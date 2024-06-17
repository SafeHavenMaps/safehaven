use crate::api::{AppError, AppState, DbConn};
use crate::models::user::User;
use axum::async_trait;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::{Cookie, CookieJar, Expiration, SameSite};
use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// Consts
const EPHEMERAL_TOKEN_COOKIE_NAME: &str = "ephemeral_token";
const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";

const REFRESH_TOKEN_MAX_INACTIVE_DAYS: i64 = 3;
const REFRESH_TOKEN_MAX_INACTIVE_DAYS_REMEMBER_ME: i64 = 31;

#[derive(Clone, Serialize, ToSchema)]
pub struct AdminUserIdentity {
    pub admin_id: Uuid,
    pub username: String,
    pub is_admin: bool,
}

impl AdminUserIdentity {
    fn from_claims(claims: &AdminEphemeralTokenClaims) -> Self {
        Self {
            admin_id: claims.admin_id,
            username: claims.username.clone(),
            is_admin: claims.is_admin,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AdminUserIdentity
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Extract the claims from the request extensions
        let claims = parts
            .extensions
            .get::<AdminUserIdentity>()
            .ok_or(AppError::Unauthorized)?;

        Ok(claims.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
struct AdminEphemeralTokenClaims {
    admin_id: Uuid,
    username: String,
    is_admin: bool,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct AdminRefreshTokenClaims {
    pub admin_id: Uuid,
    pub remember_me: bool,
    pub exp: usize,
    pub iat: usize,
}

pub async fn authentication_middleware(
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    cookies: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Get the token from the cookies
    let token = cookies
        .get(EPHEMERAL_TOKEN_COOKIE_NAME)
        .ok_or_else(|| AppError::Unauthorized.into_response())?
        .value();

    // 1) Decode the ephemeral token:
    //   - If valid, continue on
    //   - If invalid,
    //    If

    // Decode the ephemeral token
    let token_data = jsonwebtoken::decode::<AdminEphemeralTokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(app_state.config.token_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    );

    let (ephemeral_claims, update_cookie) = match token_data {
        // If the token is valid we just return the claims
        Ok(data) => (data.claims, false),

        // If the ephemeral token is invalid try to use the refresh token to generate a new one
        Err(_) => {
            // get the refresh token
            let refresh_token = cookies
                .get(REFRESH_TOKEN_COOKIE_NAME)
                .ok_or_else(|| AppError::Unauthorized.into_response())?
                .value();

            // decode and validate refresh token claims
            let Ok(jsonwebtoken::TokenData { claims, .. }) =
                jsonwebtoken::decode::<AdminRefreshTokenClaims>(
                    refresh_token,
                    &jsonwebtoken::DecodingKey::from_secret(app_state.config.token_secret.as_ref()),
                    &jsonwebtoken::Validation::default(),
                )
            else {
                // If the refresh token is invalid return an error
                let purged_jar = expire_cookies(cookies);
                return Ok((purged_jar, AppError::Unauthorized).into_response());
            };

            // get the user and create corresponding claims
            match User::get(claims.admin_id, &mut conn).await {
                Ok(user) => {
                    let new_ephemeral_claims = create_user_claim(&user);
                    (new_ephemeral_claims, true)
                }
                // if the user is not found, clear the cookie jar
                Err(AppError::Database(sqlx::Error::RowNotFound)) => {
                    let purged_jar = expire_cookies(cookies);
                    return Ok((purged_jar, AppError::Unauthorized).into_response());
                }
                Err(err) => return Err(err.into_response()),
            }
        }
    };

    // put the token claims in the request
    request
        .extensions_mut()
        .insert(AdminUserIdentity::from_claims(&ephemeral_claims));

    // Execute the chain
    let response = next.run(request).await;

    Ok(if update_cookie {
        // If a new ephemerala token is generated we need to send it back to the client
        let ephemeral_cookie = create_ephemeral_cookie(ephemeral_claims, &app_state);
        let new_cookies = cookies.add(ephemeral_cookie);
        (new_cookies, response).into_response()
    } else {
        // Otherwise just return the response
        response
    })
}

pub fn expire_cookies(cookies: CookieJar) -> CookieJar {
    cookies
        .remove(EPHEMERAL_TOKEN_COOKIE_NAME)
        .remove(REFRESH_TOKEN_COOKIE_NAME)
}

fn create_user_claim(user: &User) -> AdminEphemeralTokenClaims {
    AdminEphemeralTokenClaims {
        admin_id: user.id,
        username: user.name.clone(),
        is_admin: user.is_admin,
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + TimeDelta::hours(1)).timestamp() as usize,
    }
}

fn create_ephemeral_cookie<'a>(
    claims: AdminEphemeralTokenClaims,
    app_state: &AppState,
) -> Cookie<'a> {
    let token = app_state.generate_refresh_token(claims);

    Cookie::build((EPHEMERAL_TOKEN_COOKIE_NAME, token))
        .path("/api/admin/")
        .secure(app_state.config.secure_cookie)
        .http_only(true)
        .same_site(SameSite::Strict)
        .build()
}

fn create_refresh_cookie<'a>(user_id: Uuid, app_state: &AppState, remember_me: bool) -> Cookie<'a> {
    // sadly, the cookie library uses `time` and the jwt library uses `chrono`
    let inactive_days = if remember_me {
        REFRESH_TOKEN_MAX_INACTIVE_DAYS_REMEMBER_ME
    } else {
        REFRESH_TOKEN_MAX_INACTIVE_DAYS
    };
    let time_now = time::OffsetDateTime::now_utc();
    let token_exp = time_now + time::Duration::days(inactive_days);

    let token = app_state.generate_refresh_token(AdminRefreshTokenClaims {
        admin_id: user_id,
        iat: time_now.unix_timestamp() as usize,
        exp: token_exp.unix_timestamp() as usize,
        remember_me,
    });

    Cookie::build((REFRESH_TOKEN_COOKIE_NAME, token))
        .path("/api/admin/")
        .secure(app_state.config.secure_cookie)
        .http_only(true)
        .same_site(SameSite::Strict)
        .expires(if remember_me {
            Expiration::DateTime(time_now + time::Duration::days(inactive_days))
        } else {
            Expiration::Session
        })
        .build()
}

pub fn login(
    app_state: &AppState,
    cookies: CookieJar,
    auth_user: &User,
    remember_me: bool,
) -> CookieJar {
    let auth_cookie = create_ephemeral_cookie(create_user_claim(auth_user), app_state);
    let refresh_cookie = create_refresh_cookie(auth_user.id, app_state, remember_me);
    cookies.add(auth_cookie).add(refresh_cookie)
}
