pub mod access_tokens;
pub mod categories;
pub mod comments;
pub mod entities;
pub mod families;
pub mod options;
pub mod tags;
pub mod users;

use crate::api::{AppError, AppJson, AppState, DbConn};
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
        // sessions
        .route("/session", post(admin_login))
        .route("/session", delete(admin_logout));

    let authenticated_router: Router<AppState> = Router::new()
        // sessions
        .route("/session", get(admin_login_check))
        // options
        .route("/options", get(options::admin_options_get))
        .route("/options/:name", put(options::admin_options_update))
        // users
        .route("/users", get(users::admin_users_list))
        .route("/users", post(users::admin_user_new))
        .route("/users/:id", get(users::admin_user_get))
        .route(
            "/users/self/password",
            put(users::admin_user_change_self_password),
        )
        .route("/users/:id/password", put(users::admin_user_update))
        .route("/users/:id", delete(users::admin_user_delete))
        // access_tokens
        .route(
            "/access_tokens",
            get(access_tokens::admin_access_tokens_list),
        )
        .route(
            "/access_tokens",
            post(access_tokens::admin_access_token_new),
        )
        .route(
            "/access_tokens/:id",
            get(access_tokens::admin_access_token_get),
        )
        .route(
            "/access_tokens/:id",
            put(access_tokens::admin_access_token_update),
        )
        .route(
            "/access_tokens/:id",
            delete(access_tokens::admin_access_token_delete),
        )
        // families
        .route("/families", get(families::admin_families_list))
        .route("/families", post(families::admin_family_new))
        .route("/families/:id", get(families::admin_family_get))
        .route("/families/:id", put(families::admin_family_update))
        .route("/families/:id", delete(families::admin_family_delete))
        // categories
        .route("/categories", get(categories::admin_categories_list))
        .route("/categories", post(categories::admin_category_new))
        .route("/categories/:id", get(categories::admin_category_get))
        .route("/categories/:id", put(categories::admin_category_update))
        .route("/categories/:id", delete(categories::admin_category_delete))
        // tags
        .route("/tags", get(tags::admin_tags_list))
        .route("/tags", post(tags::admin_tag_new))
        .route("/tags/:id", get(tags::admin_tag_get))
        .route("/tags/:id", put(tags::admin_tag_update))
        .route("/tags/:id", delete(tags::admin_tag_delete))
        // entities
        .route("/entities/pending", get(entities::admin_entities_pending))
        .route("/entities/search", get(entities::admin_entities_search))
        .route("/entities", post(entities::admin_entity_new))
        .route("/entities/:id", get(entities::admin_entity_get))
        .route("/entities/:id", put(entities::admin_entity_update))
        .route("/entities/:id", delete(entities::admin_entity_delete))
        .route(
            "/entities/:id/comments",
            get(entities::admin_entity_get_comments),
        )
        .route(
            "/entities/:parent_id/parent/:child_id",
            post(entities::admin_entity_register_parent),
        )
        .route(
            "/entities/:parent_id/parent/:child_id",
            delete(entities::admin_entity_remove_parent),
        )
        // comments
        .route("/comments/pending", get(comments::admin_comments_pending))
        .route("/comments", post(comments::admin_comment_new))
        .route("/comments/:id", get(comments::admin_comment_get))
        .route("/comments/:id", put(comments::admin_comment_update))
        .route("/comments/:id", delete(comments::admin_comment_delete))
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
                Err(_) => {
                    let purged_jar = expire_cookies(cookies);
                    return Ok((purged_jar, AppError::Unauthorized).into_response());
                }
            };

            let user = User::get(refresh_token_data.admin_id, &mut conn).await?;
            let new_token_data = create_user_claim(&user);

            (new_token_data, true)
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

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
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
    get,
    path = "/api/admin/session",
    responses(
        (status = 200, description = "Check login", body = AdminUserTokenClaims),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
async fn admin_login_check(
    token_claims: AdminUserTokenClaims,
    DbConn(_conn): DbConn,
) -> Result<AppJson<AdminUserTokenClaims>, AppError> {
    Ok(AppJson(token_claims))
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
async fn admin_login(
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
        (status = 200, description = "Logout", headers(("Set-Cookie" = CookieJar, description = "Cookie jar cleaned"))),
        (status = 404, description = "User or password not found", body = ErrorResponse),
    )
)]
async fn admin_logout(cookies: CookieJar) -> Response {
    expire_cookies(cookies).into_response()
}

fn expire_cookies(cookies: CookieJar) -> CookieJar {
    cookies
        .remove(TOKEN_COOKIE_NAME)
        .remove(REFRESH_TOKEN_COOKIE_NAME)
        .add(
            Cookie::build((TOKEN_COOKIE_NAME, ""))
                .path("/api/admin/")
                .http_only(true)
                .same_site(SameSite::Strict)
                .removal(),
        )
        .add(
            Cookie::build((REFRESH_TOKEN_COOKIE_NAME, ""))
                .path("/api/admin/")
                .http_only(true)
                .same_site(SameSite::Strict)
                .removal(),
        )
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
