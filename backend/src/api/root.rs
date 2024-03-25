use crate::api::{AppError, AppJson, AppState, DbConn, MapUserTokenClaims};
use crate::models::{access_token::AccessToken, category::Category, family::Family, tag::Tag};
use axum::extract::State;
use axum::{
    extract::Path,
    routing::{get, Router},
};
use chrono::{TimeDelta, Utc};
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/bootstrap/:token", get(boostrap))
}

#[derive(Serialize, ToSchema)]
pub struct HealthCheckResponse {
    status: &'static str,
}

#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Health check", body = HealthCheckResponse)
    )
)]
pub async fn health_check() -> AppJson<HealthCheckResponse> {
    AppJson(HealthCheckResponse { status: "ok" })
}

#[derive(Serialize, ToSchema)]
pub struct BootstrapResponse {
    signed_token: String,
    families: Vec<Family>,
    categories: Vec<Category>,
    tags: Vec<Tag>,
}

#[utoipa::path(
    get,
    path = "/api/bootstrap/{token}",
    params(
        ("token" = String, Path, description = "Access token")
    ),
    responses(
        (status = 200, description = "Bootstraping data", body = BootstrapResponse)
    )
)]
async fn boostrap(
    State(app_state): State<AppState>,
    Path(token): Path<String>,
    DbConn(mut conn): DbConn,
) -> Result<AppJson<BootstrapResponse>, AppError> {
    let access_token = AccessToken::get(token, &mut conn).await?;
    let perms: crate::models::access_token::Permissions = access_token.permissions.0;

    let signed_token = app_state.generate_token(MapUserTokenClaims {
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + TimeDelta::try_hours(1).expect("valid duration")).timestamp() as usize,
        perms: perms.clone(),
    });

    let categories = match perms.categories_policy.allow_all {
        true => Category::list(&mut conn).await?,
        false => Category::list_restricted(perms.categories_policy.allow_list, &mut conn).await?,
    };

    let families = match perms.families_policy.allow_all {
        true => Family::list(&mut conn).await?,
        false => Family::list_restricted(perms.families_policy.allow_list, &mut conn).await?,
    };

    let tags = match perms.tags_policy.allow_all {
        true => Tag::list(&mut conn).await?,
        false => Tag::list_restricted(perms.tags_policy.allow_list, &mut conn).await?,
    };

    let resp = BootstrapResponse {
        signed_token,
        families,
        categories,
        tags,
    };

    Ok(AppJson(resp))
}
