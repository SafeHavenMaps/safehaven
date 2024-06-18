use crate::api::{AppError, AppState, DbConn};
use crate::models::user::User;
use axum::async_trait;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::{Cookie, CookieJar, Expiration, SameSite};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

const EPHEMERAL_TOKEN_COOKIE_NAME: &str = "ephemeral_token";
const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";

const EPHEMERAL_TOKEN_DURATION: time::Duration = time::Duration::hours(1);
const REFRESH_TOKEN_DURATION: time::Duration = time::Duration::hours(8);
const REFRESH_TOKEN_REMEMBER_ME_DURATION: time::Duration = time::Duration::days(7);

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

    fn from_user(user: &User) -> Self {
        Self {
            admin_id: user.id,
            username: user.name.clone(),
            is_admin: user.is_admin,
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

async fn authenticate_request(
    app_state: &AppState,
    input_cookies: CookieJar,
    new_cookies: &mut Option<CookieJar>,
    mut conn: sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> Result<AdminUserIdentity, AppError> {
    // If an ephemeral token is present, try to use it
    // An invalid ephemeral token is equivalent to no token at all
    if let Some(ephemeral_token_str) = input_cookies.get(EPHEMERAL_TOKEN_COOKIE_NAME) {
        // Decode the ephemeral token
        let token_data = jsonwebtoken::decode::<AdminEphemeralTokenClaims>(
            ephemeral_token_str.value(),
            &jsonwebtoken::DecodingKey::from_secret(app_state.config.token_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        );

        // If the token is valid we just return the claims
        if let Ok(data) = token_data {
            tracing::debug!("valid ephemeral token");
            return Ok(AdminUserIdentity::from_claims(&data.claims));
        }
    }

    // If no ephemeral token is present, or the ephemeral token is invalid,
    // generate one using the refresh token.

    // Get the refresh token jwt. if missing or invalid, the user is unauthorized
    let refresh_claims = {
        let refresh_token = input_cookies
            .get(REFRESH_TOKEN_COOKIE_NAME)
            .ok_or(AppError::Unauthorized)
            .inspect_err(|_| tracing::debug!("missing refresh token"))?
            .value();

        // decode and validate refresh token claims
        match jsonwebtoken::decode::<AdminRefreshTokenClaims>(
            refresh_token,
            &jsonwebtoken::DecodingKey::from_secret(app_state.config.token_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        ) {
            Ok(jsonwebtoken::TokenData { claims, .. }) => claims,
            Err(_) => {
                tracing::debug!("invalid refresh token");
                // If the refresh token is invalid, clear it and return an error
                *new_cookies = Some(expire_cookies(app_state, input_cookies));
                return Err(AppError::Unauthorized);
            }
        }
    };

    // get the user and create corresponding claims
    let user = match User::get(refresh_claims.admin_id, &mut conn).await {
        Ok(user) => user,
        // if the user is not found, clear the cookie jar: the user was deleted
        Err(AppError::Database(sqlx::Error::RowNotFound)) => {
            tracing::debug!("cannot find the refresh token user");
            *new_cookies = Some(expire_cookies(app_state, input_cookies));
            return Err(AppError::Unauthorized);
        }
        Err(err) => {
            tracing::debug!("user get error: {:?}", err);
            return Err(err);
        }
    };

    // Regenerate and update tokens
    tracing::debug!("refreshing auth cookies");
    *new_cookies = Some(set_auth_cookies(
        app_state,
        input_cookies,
        &user,
        refresh_claims.remember_me,
    ));
    Ok(AdminUserIdentity::from_user(&user))
}

pub async fn authentication_middleware(
    State(app_state): State<AppState>,
    DbConn(conn): DbConn,
    cookies: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    // attempt to authenticate the request
    let mut new_cookies = None;
    let response = match authenticate_request(&app_state, cookies, &mut new_cookies, conn).await {
        Err(err) => return err.into_response(),
        Ok(user_identity) => {
            // attach user identity to the request
            request.extensions_mut().insert(user_identity);

            // Get the response from the middleware chain
            next.run(request).await
        }
    };

    // attach cookies to the response, regardless of success
    if let Some(new_cookies) = new_cookies {
        (new_cookies, response).into_response()
    } else {
        response
    }
}

// this helper is used to add and remove cookies
fn admin_cookie<'a>(
    base: impl Into<Cookie<'a>>,
    app_state: &AppState,
) -> cookie::CookieBuilder<'a> {
    Cookie::build(base)
        .path("/api/admin/")
        .secure(app_state.config.secure_cookie)
        .http_only(true)
        .same_site(SameSite::Strict)
}

pub fn set_auth_cookies(
    app_state: &AppState,
    cookies: CookieJar,
    auth_user: &User,
    remember_me: bool,
) -> CookieJar {
    let user_id = auth_user.id;
    let time_now = time::OffsetDateTime::now_utc();

    let ephemeral_token_exp = time_now + EPHEMERAL_TOKEN_DURATION;
    let refresh_token_exp = time_now
        + if remember_me {
            REFRESH_TOKEN_REMEMBER_ME_DURATION
        } else {
            REFRESH_TOKEN_DURATION
        };

    let ephemeral_cookie = {
        let token = app_state.generate_token(AdminEphemeralTokenClaims {
            admin_id: auth_user.id,
            username: auth_user.name.clone(),
            is_admin: auth_user.is_admin,
            iat: time_now.unix_timestamp() as usize,
            exp: ephemeral_token_exp.unix_timestamp() as usize,
        });
        admin_cookie((EPHEMERAL_TOKEN_COOKIE_NAME, token), app_state)
            .expires(Expiration::DateTime(ephemeral_token_exp))
            .build()
    };

    let refresh_cookie = {
        let token = app_state.generate_token(AdminRefreshTokenClaims {
            admin_id: user_id,
            iat: time_now.unix_timestamp() as usize,
            exp: refresh_token_exp.unix_timestamp() as usize,
            remember_me,
        });

        admin_cookie((REFRESH_TOKEN_COOKIE_NAME, token), app_state)
            .expires(if remember_me {
                Expiration::DateTime(refresh_token_exp)
            } else {
                Expiration::Session
            })
            .build()
    };
    cookies.add(ephemeral_cookie).add(refresh_cookie)
}

pub fn expire_cookies(app_state: &AppState, cookies: CookieJar) -> CookieJar {
    [EPHEMERAL_TOKEN_COOKIE_NAME, REFRESH_TOKEN_COOKIE_NAME]
        .into_iter()
        .fold(cookies, |jar, cookie_name| {
            jar.remove(admin_cookie(cookie_name, app_state))
        })
}
