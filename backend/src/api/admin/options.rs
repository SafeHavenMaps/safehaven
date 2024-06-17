use axum::extract::{Path, State};

use crate::{
    api::{AppError, AppJson, AppState, DbConn},
    models::options::{ConfigurationOption, SafeHavenOptions},
};

use super::auth::AdminUserIdentity;

#[utoipa::path(
    get,
    path = "/api/admin/options",
    responses(
        (status = 200, description = "Option map", body = SafeHavenOptions),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_options_get(
    State(app_state): State<AppState>,
) -> Result<AppJson<SafeHavenOptions>, AppError> {
    let dyn_config = app_state.dyn_config.read().await.clone();

    Ok(AppJson(dyn_config))
}

fn deserialize_option<T>(value: serde_json::Value) -> Result<T, AppError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(value).map_err(|_| AppError::Validation("Bad request".to_owned()))
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
    user: AdminUserIdentity,
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    raw_body: axum::body::Bytes,
) -> Result<AppJson<SafeHavenOptions>, AppError> {
    // Deserialize raw_body into a serde_json::Value first
    let value: serde_json::Value = serde_json::from_slice(&raw_body)
        .map_err(|_| AppError::Validation("Bad request".to_owned()))?;

    // Use the name parameter to determine the correct variant
    let config: ConfigurationOption = match name.as_str() {
        "general" => ConfigurationOption::General(deserialize_option(value)?),
        "safe_mode" => ConfigurationOption::SafeMode(deserialize_option(value)?),
        "cartography_init" => ConfigurationOption::CartographyInit(deserialize_option(value)?),
        "cartography_cluster" => {
            ConfigurationOption::CartographyCluster(deserialize_option(value)?)
        }
        _ => {
            return Err(AppError::Validation(format!(
                "Unknown configuration option: {}",
                name
            )));
        }
    };

    if !user.is_admin {
        return Err(AppError::Unauthorized);
    }

    if config.option_name() != name {
        return Err(AppError::Validation("Option name mismatch".to_string()));
    }

    SafeHavenOptions::insert_or_update_config(&mut conn, config).await;

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
    user: AdminUserIdentity,
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
) -> Result<AppJson<SafeHavenOptions>, AppError> {
    if !user.is_admin {
        return Err(AppError::Unauthorized);
    }

    SafeHavenOptions::delete(&mut conn, name).await?;

    let dyn_config = app_state.dyn_config.read().await.clone();

    Ok(AppJson(dyn_config))
}
