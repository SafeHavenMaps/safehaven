pub mod access_tokens;
pub mod auth;
pub mod categories;
pub mod comments;
pub mod entities;
pub mod families;
pub mod options;
pub mod statistics;
pub mod tags;
pub mod users;

use crate::api::{AppError, AppJson, AppState, DbConn};
use crate::models::user::User;
use axum::extract::State;
use axum::middleware;
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{delete, get, post, put, Router},
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use auth::AdminUserIdentity;

pub fn routes(state: &AppState) -> Router<AppState> {
    let unauthenticated_router: Router<AppState> = Router::new()
        // sessions
        .route("/session", post(admin_login))
        .route("/session", delete(admin_logout));

    let authenticated_router: Router<AppState> = Router::new()
        // home statistic
        .route("/stats", get(statistics::admin_home_stats))
        // sessions
        .route("/session", get(admin_login_check))
        // options
        .route("/options", get(options::admin_options_get))
        .route("/options/:name", put(options::admin_options_update))
        .route("/options/:name", delete(options::admin_options_delete))
        // users
        .route("/users", get(users::admin_users_list))
        .route("/users", post(users::admin_user_new))
        .route("/users/:id", get(users::admin_user_get))
        .route(
            "/users/self/password",
            put(users::admin_user_change_self_password),
        )
        .route("/users/:id", put(users::admin_user_update))
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
            "/access_tokens/:id/stats",
            get(access_tokens::admin_access_token_get_stats),
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
        .route(
            "/families/:id/icon",
            post(families::admin_family_update_icon),
        )
        .route(
            "/families/:id/icon",
            delete(families::admin_family_delete_icon),
        )
        // categories
        .route("/categories", get(categories::admin_categories_list))
        .route("/categories", post(categories::admin_category_new))
        .route("/categories/:id", get(categories::admin_category_get))
        .route("/categories/:id", put(categories::admin_category_update))
        .route("/categories/:id", delete(categories::admin_category_delete))
        .route(
            "/categories/:id/icon",
            post(categories::admin_category_update_icon),
        )
        .route(
            "/categories/:id/icon",
            delete(categories::admin_category_delete_icon),
        )
        // tags
        .route("/tags", get(tags::admin_tags_list))
        .route("/tags", post(tags::admin_tag_new))
        .route("/tags/:id", get(tags::admin_tag_get))
        .route("/tags/:id", put(tags::admin_tag_update))
        .route("/tags/:id", delete(tags::admin_tag_delete))
        // entities
        .route("/entities/pending", get(entities::admin_entities_pending))
        .route("/entities/search", post(entities::admin_entities_search))
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
        // stats
        .route(
            "/stats/count-comments-entities",
            get(statistics::admin_count_comments_entities),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::authentication_middleware,
        ));

    Router::new()
        .merge(authenticated_router)
        .merge(unauthenticated_router)
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

#[utoipa::path(
    get,
    path = "/api/admin/session",
    responses(
        (status = 200, description = "Check login", body = AdminUserIdentity),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
async fn admin_login_check(
    user: AdminUserIdentity,
    DbConn(_conn): DbConn,
) -> Result<AppJson<AdminUserIdentity>, AppError> {
    Ok(AppJson(user))
}

#[utoipa::path(
    post,
    path = "/api/admin/session",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login and set cookies", body = LoginResponse),
        (status = 404, description = "User or password not found", body = ErrorResponse),
    )
)]
async fn admin_login(
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Response {
    let auth_user = match User::authenticate(request.username, request.password, &mut conn).await {
        Ok(auth_user) => auth_user,
        Err(err) => return err.into_response(),
    };

    let new_cookies = auth::set_auth_cookies(&app_state, jar, &auth_user, request.remember_me);
    let body = LoginResponse {
        is_admin: auth_user.is_admin,
    };
    (new_cookies, axum::Json(body)).into_response()
}

#[utoipa::path(
    delete,
    path = "/api/admin/session",
    responses(
        (status = 200, description = "Logout", headers(("Set-Cookie" = CookieJar, description = "Cookie jar cleaned"))),
        (status = 404, description = "User or password not found", body = ErrorResponse),
    )
)]
async fn admin_logout(State(app_state): State<AppState>, cookies: CookieJar) -> Response {
    auth::expire_cookies(&app_state, cookies).into_response()
}
