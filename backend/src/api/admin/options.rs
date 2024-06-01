use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    api::{AppError, AppJson, AppState, DbConn},
    models::options::{ConfigurationOption, SafeHavenOptions},
};

use super::AdminUserTokenClaims;

#[utoipa::path(
    get,
    path = "/api/admin/options",
    responses(
        (status = 200, description = "Option map", body = SafeHavenOptions),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_options_get(
    token: AdminUserTokenClaims,
    State(app_state): State<AppState>,
) -> Result<AppJson<SafeHavenOptions>, AppError> {
    if !token.is_admin {
        return Err(AppError::Unauthorized);
    }

    let dyn_config = app_state.dyn_config.read().await.clone();

    Ok(AppJson(dyn_config))
}

#[utoipa::path(
    put,
    path = "/api/admin/options/{name}",
    params(
        ("name" = String, Path, description = "Option group name")
    ),
    request_body = ConfigurationOption,
    responses(
        (status = 200, description = "Option map", body = SafeHavenOptions),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_options_update(
    Path(name): Path<String>,
    token: AdminUserTokenClaims,
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    Json(config): Json<ConfigurationOption>,
) -> Result<AppJson<SafeHavenOptions>, AppError> {
    if !token.is_admin {
        return Err(AppError::Unauthorized);
    }

    if config.option_name() != name {
        return Err(AppError::Validation("Option name mismatch".to_string()));
    }

    SafeHavenOptions::insert_or_update_config(&mut conn, config).await;
    app_state.reload_data(&mut conn).await;

    let dyn_config = app_state.dyn_config.read().await.clone();

    Ok(AppJson(dyn_config))
}

#[utoipa::path(
    delete,
    path = "/api/admin/options/{name}",
    params(
        ("name" = String, Path, description = "Option group name")
    ),
    responses(
        (status = 200, description = "Option map", body = SafeHavenOptions),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_options_delete(
    Path(name): Path<String>,
    token: AdminUserTokenClaims,
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
) -> Result<AppJson<SafeHavenOptions>, AppError> {
    if !token.is_admin {
        return Err(AppError::Unauthorized);
    }

    SafeHavenOptions::delete(&mut conn, name).await?;
    app_state.reload_data(&mut conn).await;

    let dyn_config = app_state.dyn_config.read().await.clone();

    Ok(AppJson(dyn_config))
}
