use crate::api::{AppError, AppJson, AppState, DbConn};
use crate::helpers::hcaptcha::{self, HCaptchaValidationError};
use crate::models::comment::{PublicComment, PublicNewComment};
use crate::models::entity::{PublicEntity, PublicListedEntity, PublicNewEntity};
use crate::models::entity_cache::{
    EntitiesAndClusters, FindEntitiesRequest, SearchEntitiesRequest,
    ViewerCachedEntitiesWithPagination, ViewerCachedEntity,
};
use axum::extract::{Path, State};
use axum::middleware;
use axum::{
    routing::{post, Router},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use utoipa::ToSchema;
use uuid::Uuid;

use super::auth::{viewer_authentication_middleware, MapUserTokenClaims};

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/view", post(viewer_view_request))
        .route("/search", post(viewer_search_request))
        .route("/entities/:id", post(viewer_fetch_entity))
        .route("/entities", post(viewer_new_entity))
        .route("/comments", post(viewer_new_comment))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            viewer_authentication_middleware,
        ))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ViewRequest {
    family_id: Uuid,
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
    zoom_level: u8,
    active_categories: Vec<Uuid>,
    active_required_tags: Vec<Uuid>,
    active_hidden_tags: Vec<Uuid>,
    enums_constraints: Value,
}

impl Display for ViewRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ViewRequest {{ xmin: {}, ymin: {}, xmax: {}, ymax: {}, zoom_level: {}, family_id: {} }}",
            self.xmin, self.ymin, self.xmax, self.ymax, self.zoom_level, self.family_id
        )
    }
}

fn clusterize(
    characteristic_distance: f64,
    declustering_speed: f64,
    minimal_cluster_size: i32,
    zoom: f64,
) -> Option<(f64, i32)> {
    if zoom > 18.9 {
        return None;
    }
    Some((
        characteristic_distance * 100000. * declustering_speed.powf(-zoom),
        minimal_cluster_size,
    ))
}

fn is_token_allowed_for_family(token: &MapUserTokenClaims, family_id: &Uuid) -> bool {
    let family_is_allowed = token.perms.families_policy.allow_all
        || token.perms.families_policy.allow_list.contains(family_id);

    let family_is_excluded = token
        .perms
        .families_policy
        .force_exclude
        .contains(family_id);

    family_is_allowed && !family_is_excluded
}

#[utoipa::path(
    post,
    path = "/api/map/view",
    request_body = ViewRequest,
    responses(
        (status = 200, description = "List of entities", body = EntitiesAndClusters),
        (status = 401, description = "Invalid token", body = ErrorResponse),
    )
)]
pub async fn viewer_view_request(
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    token: MapUserTokenClaims,
    Json(request): Json<ViewRequest>,
) -> Result<AppJson<EntitiesAndClusters>, AppError> {
    tracing::trace!("Received view request {}", request);

    if !is_token_allowed_for_family(&token, &request.family_id) {
        return Err(AppError::Unauthorized);
    }

    let dyn_config = app_state.dyn_config.read().await;

    let cluster_params = clusterize(
        dyn_config.cartography_cluster.characteristic_distance,
        dyn_config.cartography_cluster.declustering_speed,
        dyn_config.cartography_cluster.minimal_cluster_size,
        request.zoom_level as f64,
    );

    // Doing the request
    let request = FindEntitiesRequest {
        xmin: request.xmin,
        ymin: request.ymin,
        xmax: request.xmax,
        ymax: request.ymax,
        family_id: request.family_id,
        allow_all_categories: token.perms.categories_policy.allow_all,
        allow_all_tags: token.perms.tags_policy.allow_all,
        categories_list: token.perms.categories_policy.allow_list.clone(),
        tags_list: token.perms.tags_policy.allow_list.clone(),
        exclude_categories_list: token.perms.categories_policy.force_exclude.clone(),
        exclude_tags_list: token.perms.tags_policy.force_exclude.clone(),
        cluster_params,
        active_categories: request.active_categories,
        active_required_tags: request.active_required_tags,
        active_hidden_tags: request.active_hidden_tags,
        enums_constraints: request.enums_constraints,
    };

    Ok(AppJson(
        ViewerCachedEntity::find_entities_in_rectangle(request, &mut conn).await?,
    ))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SearchRequest {
    search_query: String,
    family_id: Uuid,
    page: i64,
    page_size: i64,
    active_categories: Vec<Uuid>,
    active_required_tags: Vec<Uuid>,
    active_hidden_tags: Vec<Uuid>,
    require_locations: bool,
    enums_constraints: Value,
}

impl Display for SearchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SearchRequest {{ search_query: {}, family_id: {} }}",
            self.search_query, self.family_id
        )
    }
}

#[utoipa::path(
    post,
    path = "/api/map/search",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "List of entities", body = CachedEntitiesWithPagination),
        (status = 401, description = "Invalid token", body = ErrorResponse),
    )
)]
async fn viewer_search_request(
    DbConn(mut conn): DbConn,
    token: MapUserTokenClaims,
    Json(request): Json<SearchRequest>,
) -> Result<AppJson<ViewerCachedEntitiesWithPagination>, AppError> {
    tracing::trace!("Received search request {}", request);

    if !is_token_allowed_for_family(&token, &request.family_id) {
        return Err(AppError::Unauthorized);
    }

    let request = SearchEntitiesRequest {
        search_query: request.search_query,
        family_id: request.family_id,
        allow_all_categories: token.perms.categories_policy.allow_all,
        allow_all_tags: token.perms.tags_policy.allow_all,
        categories_list: token.perms.categories_policy.allow_list.clone(),
        tags_list: token.perms.tags_policy.allow_list.clone(),
        exclude_categories_list: token.perms.categories_policy.force_exclude.clone(),
        exclude_tags_list: token.perms.tags_policy.force_exclude.clone(),
        page: request.page,
        page_size: request.page_size,
        active_categories: request.active_categories,
        active_required_tags: request.active_required_tags,
        active_hidden_tags: request.active_hidden_tags,
        require_locations: request.require_locations,
        enums_constraints: request.enums_constraints,
    };

    Ok(AppJson(
        ViewerCachedEntity::search_entities(request, &mut conn).await?,
    ))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct PublicNewEntityRequest {
    entity: PublicNewEntity,
    comment: PublicNewComment,
    hcaptcha_token: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct PublicNewEntityResponse {
    entity: PublicEntity,
    comment: PublicComment,
}

async fn check_captcha(state: AppState, response: Option<String>) -> Result<(), AppError> {
    let dyn_config = state.dyn_config.read().await;

    if dyn_config.safe_mode.enabled {
        match response {
            Some(response) => {
                hcaptcha::check_hcaptcha(
                    response,
                    dyn_config.safe_mode.hcaptcha_secret.clone(),
                    None,
                )
                .await
                .map_err(|e| match e {
                    HCaptchaValidationError::NetworkError() => {
                        AppError::Validation("HCaptcha network error".to_string())
                    }
                    HCaptchaValidationError::HCaptchaError(errors) => {
                        AppError::Validation(format!("HCaptcha errors: {:?}", errors))
                    }
                })?;
            }
            None => {
                return Err(AppError::Validation(
                    "HCaptcha token is required".to_string(),
                ));
            }
        }
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/map/entities",
    request_body = PublicNewEntityRequest,
    responses(
        (status = 200, description = "Entity", body = PublicNewEntityResponse),
        (status = 401, description = "Invalid token", body = ErrorResponse),
    )
)]
async fn viewer_new_entity(
    DbConn(mut conn): DbConn,
    State(state): State<AppState>,
    Json(request): Json<PublicNewEntityRequest>,
) -> Result<AppJson<PublicNewEntityResponse>, AppError> {
    check_captcha(state, request.hcaptcha_token).await?;

    let db_entity = PublicEntity::new(request.entity, &mut conn).await?;

    let mut new_comment = request.comment;
    new_comment.entity_id = db_entity.id;

    let db_comment = PublicComment::new(new_comment, &mut conn).await?;

    Ok(AppJson(PublicNewEntityResponse {
        entity: db_entity,
        comment: db_comment,
    }))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct NewCommentRequest {
    comment: PublicNewComment,
    hcaptcha_token: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/map/comments",
    request_body = NewCommentRequest,
    responses(
        (status = 200, description = "Comment", body = PublicComment),
        (status = 401, description = "Invalid token", body = ErrorResponse),
    )
)]
async fn viewer_new_comment(
    DbConn(mut conn): DbConn,
    State(state): State<AppState>,
    Json(request): Json<NewCommentRequest>,
) -> Result<AppJson<PublicComment>, AppError> {
    check_captcha(state, request.hcaptcha_token).await?;
    let db_comment = PublicComment::new(request.comment, &mut conn).await?;
    Ok(AppJson(db_comment))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct FetchedEntity {
    pub entity: PublicEntity,
    pub comments: Vec<PublicComment>,
    pub parents: Vec<PublicListedEntity>,
    pub children: Vec<PublicListedEntity>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct FetchEntityRequest {
    pub active_categories: Vec<Uuid>,
    pub active_required_tags: Vec<Uuid>,
    pub active_hidden_tags: Vec<Uuid>,
}

#[utoipa::path(
    post,
    path = "/api/map/entities/{id}",
    responses(
        (status = 200, description = "Entity", body = FetchedEntity),
        (status = 401, description = "Invalid token", body = ErrorResponse),
        (status = 404, description = "Entity not found", body = ErrorResponse),
    )
)]
async fn viewer_fetch_entity(
    DbConn(mut conn): DbConn,
    token: MapUserTokenClaims,
    Path(id): Path<Uuid>,
    Json(request): Json<FetchEntityRequest>,
) -> Result<AppJson<FetchedEntity>, AppError> {
    let entity = PublicEntity::get(id, &mut conn).await?;

    let can_read_entity = (token.perms.families_policy.allow_all
        || token
            .perms
            .families_policy
            .allow_list
            .contains(&entity.family_id))
        && (token.perms.categories_policy.allow_all
            || token
                .perms
                .categories_policy
                .allow_list
                .contains(&entity.category_id))
        && (token.perms.tags_policy.allow_all
            || entity
                .tags
                .iter()
                .all(|tag| token.perms.tags_policy.allow_list.contains(tag)));

    let parents = PublicEntity::get_parents(id, &mut conn).await?;
    let children = PublicEntity::get_children(id, &mut conn).await?;

    let filtered_children: Vec<PublicListedEntity> = children
        .into_iter()
        // filter against permissions
        .filter(|child| {
            (token.perms.categories_policy.allow_all
                || token
                    .perms
                    .categories_policy
                    .allow_list
                    .contains(&child.category_id))
                && (token.perms.tags_policy.allow_all
                    || child
                        .tags
                        .iter()
                        .all(|tag| token.perms.tags_policy.allow_list.contains(tag)))
        })
        // filter against request
        .filter(|child| {
            request
                .active_categories
                .iter()
                .any(|cat| *cat == child.category_id)
                && (request.active_required_tags.is_empty()
                    || request
                        .active_required_tags
                        .iter()
                        .any(|tag| child.tags.contains(tag)))
                && !request
                    .active_hidden_tags
                    .iter()
                    .any(|tag| child.tags.contains(tag))
        })
        .collect();

    let filtered_parents: Vec<PublicListedEntity> = parents
        .into_iter()
        // filter against permissions
        .filter(|parent| {
            (token.perms.categories_policy.allow_all
                || token
                    .perms
                    .categories_policy
                    .allow_list
                    .contains(&parent.category_id))
                && (token.perms.tags_policy.allow_all
                    || parent
                        .tags
                        .iter()
                        .all(|tag| token.perms.tags_policy.allow_list.contains(tag)))
        })
        // filter against request
        .filter(|parent| {
            request
                .active_categories
                .iter()
                .any(|cat| *cat == parent.category_id)
                && (request.active_required_tags.is_empty()
                    || request
                        .active_required_tags
                        .iter()
                        .any(|tag| parent.tags.contains(tag)))
                && !request
                    .active_hidden_tags
                    .iter()
                    .any(|tag| parent.tags.contains(tag))
        })
        .collect();

    if !can_read_entity && filtered_children.is_empty() {
        return Err(AppError::Unauthorized);
    }

    let comments = match token.perms.can_access_comments {
        true => PublicComment::list_for_public_entity(id, &entity.comment_form, &mut conn).await?,
        false => vec![],
    };

    Ok(AppJson(FetchedEntity {
        entity,
        comments,
        parents: filtered_parents,
        children: filtered_children,
    }))
}
