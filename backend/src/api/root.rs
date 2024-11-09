use std::collections::HashMap;

use crate::api::auth::MapUserTokenClaims;
use crate::api::{AppError, AppJson, AppState, DbConn};
use crate::models::options::{
    CartographyInitConfig, CartographySourceConfig, GeneralOptions, InitPopupOptions,
};
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
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(status))
        .route("/version", get(version))
        .route("/bootstrap/:token", get(bootstrap))
}

#[derive(Serialize, ToSchema)]
pub struct StatusResponse {
    status: &'static str,
    general: GeneralOptions,
    init_popup: InitPopupOptions,
    cartography_init: CartographyInitConfig,
    cartography_source: CartographySourceConfig,
    safe_mode: SafeMode,
}

#[derive(Serialize, ToSchema)]
pub struct SafeMode {
    enabled: bool,
    hcaptcha_sitekey: String,
}

#[utoipa::path(
    get,
    path = "/api/status",
    responses(
        (status = 200, description = "Status check and pre init", body = StatusResponse)
    )
)]
pub async fn status(State(app_state): State<AppState>) -> AppJson<StatusResponse> {
    let config = app_state.dyn_config.read().await.clone();

    AppJson(StatusResponse {
        status: "ok",
        general: config.general,
        init_popup: config.init_popup,
        cartography_init: config.cartography_init,
        cartography_source: config.cartography_source,
        safe_mode: SafeMode {
            enabled: config.safe_mode.enabled,
            hcaptcha_sitekey: config.safe_mode.hcaptcha_sitekey,
        },
    })
}

#[derive(Serialize, ToSchema)]
pub struct BootstrapPermissions {
    /// Permission to list entities
    can_list_entities: bool,

    /// Permission to list entities with an empty or short query (can be used to list all entities)
    can_list_without_query: bool,

    /// Permission to list entities with filters
    can_list_with_filters: bool,

    /// Permission to list entities with enum constraints
    can_list_with_enum_constraints: bool,

    /// Permission to view an entity
    can_access_entity: bool,

    /// Permission to view an entity's comments
    can_access_comments: bool,

    /// Permission to add an entity
    can_add_entity: bool,

    /// Permission to add a comment to an entity
    can_add_comment: bool,
}

#[derive(Serialize, ToSchema)]
pub struct BootstrapResponse {
    /// Signed token for subsequent requests
    signed_token: String,

    /// List of known families
    families: Vec<Family>,

    /// List of known categories
    categories: Vec<Category>,

    /// List of allowed categories
    allowed_categories: Vec<Uuid>,

    /// List of allowed tags
    allowed_tags: Vec<Uuid>,

    /// Permissions
    permissions: BootstrapPermissions,

    /// List of tags
    tags: Vec<Tag>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct BootstrapQueryParams {
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub referrer: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SafeHavenVersionResponse {
    version: &'static str,
    git_hash: &'static str,
    github_latest_version: Option<String>,
}

async fn get_github_last_version() -> Option<String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/repos/SafeHavenMaps/safehaven/releases/latest")
        .header("User-Agent", "SafeHaven")
        .send()
        .await
        .ok()?;
    let json: serde_json::Value = response.json().await.ok()?;
    let version = json.get("tag_name")?.as_str()?;
    Some(version.to_string())
}

#[utoipa::path(get, path = "/api/version")]
pub async fn version() -> AppJson<SafeHavenVersionResponse> {
    let github_latest_version = get_github_last_version().await;
    AppJson(SafeHavenVersionResponse {
        version: env!("SH_VERSION"),
        git_hash: env!("SH_GITHASH"),
        github_latest_version,
    })
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
async fn bootstrap(
    State(app_state): State<AppState>,
    Path(token): Path<String>,
    Query(query): Query<BootstrapQueryParams>,
    DbConn(mut conn): DbConn,
) -> Result<AppJson<BootstrapResponse>, AppError> {
    tracing::trace!("Bootstrapping");
    let access_token = AccessToken::get(token, &mut conn).await?;
    if !access_token.active {
        return Err(AppError::NotFound);
    }

    // Process the token request
    let perms: crate::models::access_token::Permissions = access_token.permissions.0;

    tracing::trace!("Bootstrapping: found access token");

    let families = Family::list_restricted(&perms.families_policy, &mut conn).await?;
    tracing::trace!("Loaded {} families", families.len());

    let families_ids: Vec<_> = families.iter().map(|f| f.id).collect();

    let fam_priv_idx: HashMap<Uuid, Vec<String>> =
        Family::get_privately_indexed_fields_for_families(&families);

    let signed_token = app_state.generate_token(MapUserTokenClaims {
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + TimeDelta::try_minutes(2).expect("valid duration")).timestamp() as usize,
        fam_priv_idx,
        perms: perms.clone(),
    });
    tracing::trace!("Generated access token");

    let categories = Category::list_except_with_families(
        &perms.categories_policy.force_exclude,
        families_ids,
        &mut conn,
    )
    .await?;
    tracing::trace!("Loaded {} categories", categories.len());

    let tags = Tag::list_except(&perms.tags_policy.force_exclude, &mut conn).await?;
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

    let allowed_categories = match perms.categories_policy.allow_all {
        true => categories.iter().map(|c| c.id).collect(),
        false => perms.categories_policy.allow_list.clone(),
    };

    let allowed_tags = match perms.tags_policy.allow_all {
        true => tags.iter().map(|t| t.id).collect(),
        false => perms.tags_policy.allow_list.clone(),
    };

    let resp = BootstrapResponse {
        signed_token,
        families,
        categories,
        allowed_categories,
        allowed_tags,
        permissions: BootstrapPermissions {
            can_list_entities: perms.can_list_entities,
            can_list_without_query: perms.can_list_without_query,
            can_list_with_filters: perms.can_list_with_filters,
            can_list_with_enum_constraints: perms.can_list_with_enum_constraints,
            can_access_entity: perms.can_access_entity,
            can_access_comments: perms.can_access_comments,
            can_add_entity: perms.can_add_entity,
            can_add_comment: perms.can_add_comment,
        },
        tags,
    };

    Ok(AppJson(resp))
}
