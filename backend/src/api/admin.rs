pub mod access_tokens;
pub mod categories;
pub mod comments;
pub mod entities;
pub mod families;
pub mod options;
pub mod tags;
pub mod users;

use crate::api::{AppError, AppState, DbConn};
use crate::models::user::User;
use axum::async_trait;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{delete, get, post, put, Router},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// Consts
const TOKEN_COOKIE_NAME: &str = "token";
const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";

pub fn routes(state: &AppState) -> Router<AppState> {
    let unauthenticated_router: Router<AppState> = Router::new()
        .route("/session", post(login))
        .route("/session", delete(logout));

    let authenticated_router: Router<AppState> = Router::new()
        // options
        .route("/options", get(options::get))
        .route("/options/:name", put(options::update))
        // users
        .route("/users", get(users::list))
        .route("/users", post(users::new))
        .route("/users/:id", get(users::get))
        .route("/users/self/password", put(users::change_self_password))
        .route("/users/:id/password", put(users::change_password))
        .route("/users/:id", delete(users::delete))
        // access_tokens
        .route("/access_tokens", get(access_tokens::list))
        .route("/access_tokens", post(access_tokens::new))
        .route("/access_tokens/:id", get(access_tokens::get))
        .route("/access_tokens/:id", put(access_tokens::update))
        .route("/access_tokens/:id", delete(access_tokens::delete))
        // families
        .route("/families", get(families::list))
        .route("/families", post(families::new))
        .route("/families/:id", get(families::get))
        .route("/families/:id", put(families::update))
        .route("/families/:id", delete(families::delete))
        // categories
        .route("/categories", get(categories::list))
        .route("/categories", post(categories::new))
        .route("/categories/:id", get(categories::get))
        .route("/categories/:id", put(categories::update))
        .route("/categories/:id", delete(categories::delete))
        // tags
        .route("/tags", get(tags::list))
        .route("/tags", post(tags::new))
        .route("/tags/:id", get(tags::get))
        .route("/tags/:id", put(tags::update))
        .route("/tags/:id", delete(tags::delete))
        // entities
        .route("/entities/pending", get(entities::pending))
        .route("/entities/search", get(entities::search))
        .route("/entities", post(entities::new))
        .route("/entities/:id", get(entities::get))
        .route("/entities/:id", put(entities::update))
        .route("/entities/:id", delete(entities::delete))
        .route("/entities/:id/comments", get(entities::get_comments))
        .route(
            "/entities/:parent_id/parent/:child_id",
            post(entities::register_parent),
        )
        .route(
            "/entities/:parent_id/parent/:child_id",
            delete(entities::remove_parent),
        )
        // comments
        .route("/comments/pending", get(comments::pending))
        .route("/comments", post(comments::new))
        .route("/comments/:id", get(comments::get))
        .route("/comments/:id", put(comments::update))
        .route("/comments/:id", delete(comments::delete))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            authentication_middleware,
        ));

    Router::new()
        .merge(authenticated_router)
        .merge(unauthenticated_router)
}

async fn authentication_middleware(
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    cookies: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Get the token from the cookies
    let token = cookies
        .get(TOKEN_COOKIE_NAME)
        .ok_or(AppError::Unauthorized)?
        .value();

    // Decode the token
    let token_data = jsonwebtoken::decode::<AdminUserTokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(app_state.config.token_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    );

    let (token_data, new_token) = match token_data {
        // If the token is valid we just return the claims
        Ok(data) => (data.claims, false),

        // If the token is invalid we try to use the refresh token to generate a new one
        // If the refresh token is invalid we return an error
        Err(_) => {
            let refresh_token = cookies
                .get(REFRESH_TOKEN_COOKIE_NAME)
                .ok_or(AppError::Unauthorized)?
                .value();

            let refresh_token_data = match jsonwebtoken::decode::<AdminRefreshUserTokenClaims>(
                refresh_token,
                &jsonwebtoken::DecodingKey::from_secret(app_state.config.token_secret.as_ref()),
                &jsonwebtoken::Validation::default(),
            ) {
                Ok(data) => data.claims,
                Err(_) => return Err(AppError::Unauthorized),
            };

            let user = User::get(refresh_token_data.admin_id, &mut conn).await?;
            let new_token = create_user_claim(&user);

            (new_token, true)
        }
    };

    // put the token claims in the request
    request.extensions_mut().insert(token_data.clone());

    // Execute the chain
    let response = next.run(request).await;

    match new_token {
        // If a new token is generated we need to set it in the response
        true => {
            let auth_cookie = create_user_cookie(token_data, &app_state);
            let jar = cookies.remove(TOKEN_COOKIE_NAME).add(auth_cookie);
            Ok((jar, response).into_response())
        }

        // Otherwise we just return the response
        false => Ok(response),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminUserTokenClaims {
    pub admin_id: Uuid,
    pub username: String,
    pub is_admin: bool,
    pub exp: usize,
    pub iat: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for AdminUserTokenClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Extract the claims from the request extensions
        let claims = parts
            .extensions
            .get::<AdminUserTokenClaims>()
            .ok_or(AppError::Unauthorized)?;

        Ok(claims.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AdminRefreshUserTokenClaims {
    pub admin_id: Uuid,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
    remember_me: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginResponse {
    is_admin: bool,
}

struct AppJsonWCookies<T> {
    pub body: T,
    pub jar: CookieJar,
}

impl<T> IntoResponse for AppJsonWCookies<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        (self.jar, axum::Json(self.body)).into_response()
    }
}

#[utoipa::path(
    post,
    path = "/api/admin/session",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login", body = LoginResponse, headers(("Set-Cookie" = CookieJar, description = "Cookie jar with auth cookie inside"))),
        (status = 404, description = "User or password not found", body = ErrorResponse),
    )
)]
async fn login(
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    Json(request): Json<LoginRequest>,
) -> Result<AppJsonWCookies<LoginResponse>, AppError> {
    let auth_user = User::authenticate(request.username, request.password, &mut conn).await?;

    let auth_cookie = create_user_cookie(create_user_claim(&auth_user), &app_state);
    let refresh_cookie = create_refresh_cookie(auth_user.id, &app_state);
    let jar = CookieJar::new().add(auth_cookie).add(refresh_cookie);

    Ok(AppJsonWCookies {
        body: LoginResponse {
            is_admin: auth_user.is_admin,
        },
        jar,
    })
}

#[utoipa::path(
    delete,
    path = "/api/admin/session",
    responses(
        (status = 200, description = "Logout", description = "Cookie jar cleaned"),
        (status = 404, description = "User or password not found", body = ErrorResponse),
    )
)]
async fn logout(cookies: CookieJar) -> Response {
    let jar = cookies
        .remove(TOKEN_COOKIE_NAME)
        .remove(REFRESH_TOKEN_COOKIE_NAME);
    jar.into_response()
}

fn create_user_claim(user: &User) -> AdminUserTokenClaims {
    AdminUserTokenClaims {
        admin_id: user.id,
        username: user.name.clone(),
        is_admin: user.is_admin,
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + TimeDelta::try_hours(1).expect("valid duration")).timestamp() as usize,
    }
}

fn create_user_cookie<'a>(claims: AdminUserTokenClaims, app_state: &AppState) -> Cookie<'a> {
    let token = app_state.generate_token(claims);

    Cookie::build((TOKEN_COOKIE_NAME, token))
        .path("/api/admin/")
        .secure(app_state.config.secure_cookie)
        .http_only(true)
        .same_site(SameSite::Strict)
        .build()
}

fn create_refresh_cookie<'a>(user_id: Uuid, app_state: &AppState) -> Cookie<'a> {
    let token = app_state.generate_token(AdminRefreshUserTokenClaims {
        admin_id: user_id,
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + TimeDelta::try_days(7).expect("valid duration")).timestamp() as usize,
    });

    Cookie::build((REFRESH_TOKEN_COOKIE_NAME, token))
        .path("/api/admin/")
        .secure(app_state.config.secure_cookie)
        .http_only(true)
        .same_site(SameSite::Strict)
        .build()
}
