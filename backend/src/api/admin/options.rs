use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    api::{AdminUserToken, AppError, AppJson, AppState, DbConn},
    models::options::{ConfigurationOption, SafeHavenOptions},
};

#[utoipa::path(
    get,
    path = "/api/options",
    responses(
        (status = 200, description = "Option map", body = SafeHavenOptions),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn get(
    AdminUserToken(token): AdminUserToken,

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
    path = "/api/options/{name}",
    request_body = ConfigurationOption,
    responses(
        (status = 200, description = "Option map", body = SafeHavenOptions),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn update(
    Path(name): Path<String>,
    AdminUserToken(token): AdminUserToken,

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
