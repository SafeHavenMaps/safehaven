pub mod access_tokens;
pub mod categories;
pub mod comments;
pub mod entities;
pub mod families;
pub mod options;
pub mod tags;
pub mod users;

use super::AdminUserTokenClaims;
use crate::api::{AppError, AppJson, AppState, DbConn};
use crate::models::user::User;
use axum::extract::State;
use axum::{
    routing::{delete, get, post, put, Router},
    Json,
};
use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        // options
        .route("/options", get(options::get))
        .route("/options/:name", put(options::update))
        // login
        .route("/login", post(login))
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
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginResponse {
    token: String,
}

#[utoipa::path(
    post,
    path = "/api/admin/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login", body = LoginResponse),
        (status = 404, description = "User or password not found", body = ErrorResponse),
    )
)]
async fn login(
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    Json(request): Json<LoginRequest>,
) -> Result<AppJson<LoginResponse>, AppError> {
    let auth_user = User::authenticate(request.username, request.password, &mut conn).await?;
    let token = app_state.generate_token(AdminUserTokenClaims {
        admin_id: auth_user.id,
        username: auth_user.name,
        is_admin: auth_user.is_admin,
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + TimeDelta::try_hours(1).expect("valid duration")).timestamp() as usize,
    });

    Ok(AppJson(LoginResponse { token }))
}
