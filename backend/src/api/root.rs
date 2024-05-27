use crate::api::{AppError, AppJson, AppState, DbConn, MapUserTokenClaims};
use crate::models::options::{CartographyInitConfig, GeneralOptions};
use crate::models::{access_token::AccessToken, category::Category, family::Family, tag::Tag};
use axum::extract::{Query, State};
use axum::{
    extract::Path,
    routing::{get, Router},
};
use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use tokio::task;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(status))
        .route("/bootstrap/:token", get(boostrap))
}

#[derive(Serialize, ToSchema)]
pub struct StatusResponse {
    status: &'static str,
    general: GeneralOptions,
    safe_mode: Option<SafeMode>,
}

#[derive(Serialize, ToSchema)]
pub struct SafeMode {
    recaptcha_v3_sitekey: String,
    popup_title: String,
    popup_message: String,
}

#[utoipa::path(
    get,
    path = "/api/status",
    responses(
        (status = 200, description = "Status check and pre init", body = StatusResponse)
    )
)]
pub async fn status(State(app_state): State<AppState>) -> AppJson<StatusResponse> {
    let safe_mode = app_state.dyn_config.read().await.safe_mode.clone();

    AppJson(StatusResponse {
        status: "ok",
        general: app_state.dyn_config.read().await.general.clone(),
        safe_mode: match safe_mode.enabled {
            true => Some(SafeMode {
                recaptcha_v3_sitekey: safe_mode.recaptcha_v3_sitekey,
                popup_title: safe_mode.popup_title.unwrap_or("Safe mode".to_string()),
                popup_message: safe_mode
                    .popup_message
                    .unwrap_or("Click to continue".to_string()),
            }),
            false => None,
        },
    })
}

#[derive(Serialize, ToSchema)]
pub struct BootstrapResponse {
    signed_token: String,
    families: Vec<Family>,
    categories: Vec<Category>,
    tags: Vec<Tag>,
    cartography_init_config: CartographyInitConfig,
}

#[serde_as]
#[derive(Deserialize)]
pub struct BootstrapQueryParams {
    #[serde_as(as = "NoneAsEmptyString")]
    pub referrer: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/bootstrap/{token}",
    params(
        ("token" = String, Path, description = "Access token"),
        ("referrer" = Option<String>, Query, description = "The referrer URL to register the visit")
    ),
    responses(
        (status = 200, description = "Bootstraping data", body = BootstrapResponse)
    )
)]
async fn boostrap(
    State(app_state): State<AppState>,
    Path(token): Path<String>,
    Query(query): Query<BootstrapQueryParams>,
    DbConn(mut conn): DbConn,
) -> Result<AppJson<BootstrapResponse>, AppError> {
    tracing::trace!("Bootstrapping");
    let access_token = AccessToken::get(token, &mut conn).await?;

    // Process the token request
    let perms: crate::models::access_token::Permissions = access_token.permissions.0;

    tracing::trace!("Bootstrapping: found access token");

    let signed_token = app_state.generate_token(MapUserTokenClaims {
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + TimeDelta::try_hours(1).expect("valid duration")).timestamp() as usize,
        perms: perms.clone(),
    });
    tracing::trace!("Generated access token");

    let families = match perms.families_policy.allow_all {
        true => Family::list(&mut conn).await?,
        false => Family::list_restricted(perms.families_policy.allow_list, &mut conn).await?,
    };
    tracing::trace!("Loaded {} families", families.len());

    let families_ids: Vec<_> = families.iter().map(|f| f.id).collect();

    let categories = match perms.categories_policy.allow_all {
        true => Category::list_with_families(families_ids, &mut conn).await?,
        false => {
            Category::list_restricted(perms.categories_policy.allow_list, families_ids, &mut conn)
                .await?
        }
    };
    tracing::trace!("Loaded {} categories", categories.len());

    let tags = match perms.tags_policy.allow_all {
        true => Tag::list(&mut conn).await?,
        false => Tag::list_restricted(perms.tags_policy.allow_list, &mut conn).await?,
    };
    tracing::trace!("Loaded {} tags", tags.len());

    // Register visit in background to avoid blocking the response
    task::spawn(async move {
        tracing::trace!("Registering visit");
        if let Err(e) =
            AccessToken::register_visit(access_token.id, query.referrer, &mut conn).await
        {
            tracing::error!("Failed to register visit: {:?}", e);
        }
    });

    let dyn_config = app_state.dyn_config.read().await;

    let resp = BootstrapResponse {
        signed_token,
        families,
        categories,
        tags,
        cartography_init_config: dyn_config.cartography_init.clone(),
    };

    Ok(AppJson(resp))
}
