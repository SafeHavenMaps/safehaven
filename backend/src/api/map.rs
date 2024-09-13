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
use std::collections::HashMap;
use std::fmt::Display;
use tracing::debug;
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

fn require_permission(condition: bool) -> Result<(), AppError> {
    if !condition {
        return Err(AppError::Forbidden);
    }
    Ok(())
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
    enums_constraints: HashMap<String, Vec<Value>>,
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

fn is_family_allowed_by_token(token: &MapUserTokenClaims, family_id: &Uuid) -> bool {
    let family_is_allowed = token.perms.families_policy.allow_all
        || token.perms.families_policy.allow_list.contains(family_id);

    let family_is_excluded = token
        .perms
        .families_policy
        .force_exclude
        .contains(family_id);

    family_is_allowed && !family_is_excluded
}

fn is_category_allowed_by_token(token: &MapUserTokenClaims, category_id: &Uuid) -> bool {
    let category_is_allowed = token.perms.categories_policy.allow_all
        || token
            .perms
            .categories_policy
            .allow_list
            .contains(category_id);

    let category_is_excluded = is_category_explicitly_excluded_by_token(token, category_id);

    category_is_allowed && !category_is_excluded
}

fn is_category_explicitly_excluded_by_token(
    token: &MapUserTokenClaims,
    category_id: &Uuid,
) -> bool {
    token
        .perms
        .categories_policy
        .force_exclude
        .contains(category_id)
}

fn is_tag_allowed_by_token(token: &MapUserTokenClaims, tag_id: &Uuid) -> bool {
    let tag_is_allowed =
        token.perms.tags_policy.allow_all || token.perms.tags_policy.allow_list.contains(tag_id);

    let tag_is_excluded = is_tag_explicitly_excluded_by_token(token, tag_id);

    tag_is_allowed && !tag_is_excluded
}

fn is_tag_explicitly_excluded_by_token(token: &MapUserTokenClaims, tag_id: &Uuid) -> bool {
    token.perms.tags_policy.force_exclude.contains(tag_id)
}

fn are_constraints_allowed(
    family_id: &Uuid,
    forbidden_indexed: &HashMap<Uuid, Vec<String>>,
    constraints: &HashMap<String, Vec<Value>>,
) -> Result<(), AppError> {
    let default_array = vec![];
    let forbidden_fields = forbidden_indexed.get(family_id).unwrap_or(&default_array);

    match constraints
        .iter()
        .all(|(k, v)| !forbidden_fields.contains(&k.to_string()) || v.is_empty())
    {
        true => Ok(()),
        false => Err(AppError::Forbidden),
    }
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
    // The token must allow to list entities
    require_permission(token.perms.can_list_entities)?;

    // The family must be allowed
    require_permission(is_family_allowed_by_token(&token, &request.family_id))?;

    // Check if some of the constraints are forbidden
    are_constraints_allowed(
        &request.family_id,
        &token.fam_priv_idx,
        &request.enums_constraints,
    )?;

    tracing::trace!("Received view request {}", request);

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
        geographic_restriction: token.perms.geographic_restrictions.clone(),
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
        enums_constraints: serde_json::to_value(request.enums_constraints)
            .expect("Enums should be serializable"),
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
    enums_constraints: HashMap<String, Vec<Value>>,
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
    // The token must allow to list entities
    require_permission(token.perms.can_list_entities)?;

    // The family must be allowed
    require_permission(is_family_allowed_by_token(&token, &request.family_id))?;

    // The token must allow to list entities with tag filters or the request must not have any tag filters
    require_permission(
        token.perms.can_list_with_filters
            || (request.active_required_tags.is_empty() && request.active_hidden_tags.is_empty()),
    )?;

    // The token must allow to list entities without query or the query must be at least 4 characters long
    require_permission(token.perms.can_list_without_query || request.search_query.len() >= 4)?;

    // The token must allow to list entities with enum constraints or the request must not have any enum constraints
    require_permission(
        token.perms.can_list_with_enum_constraints || request.enums_constraints.is_empty(),
    )?;

    tracing::trace!("Received search request {}", request);

    // Check if some of the constraints are forbidden
    are_constraints_allowed(
        &request.family_id,
        &token.fam_priv_idx,
        &request.enums_constraints,
    )?;

    let request = SearchEntitiesRequest {
        search_query: request.search_query,
        geographic_restriction: token.perms.geographic_restrictions.clone(),
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
        enums_constraints: serde_json::to_value(request.enums_constraints)
            .expect("Enums should be serializable"),
    };

    Ok(AppJson(
        ViewerCachedEntity::search_entities(request, &mut conn).await?,
    ))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct PublicNewEntityRequest {
    entity: PublicNewEntity,
    comment: Option<PublicNewComment>,
    hcaptcha_token: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct PublicNewEntityResponse {
    entity: PublicEntity,
    comment: Option<PublicComment>,
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
    token: MapUserTokenClaims,
    Json(request): Json<PublicNewEntityRequest>,
) -> Result<AppJson<PublicNewEntityResponse>, AppError> {
    // The token must allow to add entities
    require_permission(token.perms.can_add_entity)?;

    // The token must allow to add comments or the request must not have any comment
    require_permission(token.perms.can_add_comment || request.comment.is_none())?;

    check_captcha(state, request.hcaptcha_token).await?;

    let db_entity = PublicEntity::new(request.entity, &mut conn).await?;
    let mut db_comment = None;

    if let Some(mut comment) = request.comment {
        comment.entity_id = db_entity.id;
        db_comment = Some(PublicComment::new(comment, &mut conn).await?);
    }

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
    token: MapUserTokenClaims,
    Json(request): Json<NewCommentRequest>,
) -> Result<AppJson<PublicComment>, AppError> {
    // The token must allow to add comments
    require_permission(token.perms.can_add_comment)?;

    let target_entity = PublicEntity::get(request.comment.entity_id, &mut conn).await?;

    // The token must allow to add comments to the entity's family
    require_permission(is_family_allowed_by_token(&token, &target_entity.family_id))?;

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
    // The token must allow to access an entity
    require_permission(token.perms.can_access_entity)?;

    let entity = PublicEntity::get(id, &mut conn).await?;

    // The family must be allowed
    require_permission(is_family_allowed_by_token(&token, &entity.family_id))?;
    debug!(
        "[PERMDBG] Permission for family {} granted",
        entity.family_id
    );

    // The category must not be explicitly excluded
    require_permission(!is_category_explicitly_excluded_by_token(
        &token,
        &entity.category_id,
    ))?;
    debug!(
        "[PERMDBG] Category {} is not explicitly excluded, continuing",
        entity.family_id
    );

    // The tags must each not be explicitly excluded
    entity
        .tags
        .iter()
        .find_map(|tag_id| {
            require_permission(!is_tag_explicitly_excluded_by_token(&token, tag_id)).err()
        })
        .map_or(Ok(()), |err| Err(err))?;
    debug!("[PERMDBG] No tag is explicitly excluded, continuing");

    let parents = PublicEntity::get_parents(id, &mut conn).await?;
    let children = PublicEntity::get_children(id, &mut conn).await?;

    let authorized_children: Vec<PublicListedEntity> = children
        .into_iter()
        // filter against permissions
        .filter(|child| {
            is_category_allowed_by_token(&token, &child.category_id)
                && child
                    .tags
                    .iter()
                    .all(|tag_id| is_tag_allowed_by_token(&token, tag_id))
        })
        .collect();
    debug!(
        "[PERMDBG] Found {} authorized children",
        authorized_children.len()
    );

    // If the entity does not have any included children, we must check if it is included itself,
    // rather than simply not excluded as we did at the top of the function
    if authorized_children.is_empty() {
        debug!("[PERMDBG] No authorized children, checking entity itself");

        // The category must be allowed
        require_permission(is_category_allowed_by_token(&token, &entity.category_id))?;
        debug!(
            "[PERMDBG] Permission for category {} granted",
            entity.family_id
        );

        // The tags must each be allowed
        entity
            .tags
            .iter()
            .find_map(|tag_id| require_permission(is_tag_allowed_by_token(&token, tag_id)).err())
            .map_or(Ok(()), |err| Err(err))?;
        debug!("[PERMDBG] No tag are explicitly excluded, continuing");
    }

    let filtered_children: Vec<PublicListedEntity> = authorized_children
        .into_iter()
        // filter against request
        .filter(|child| {
            request
                .active_categories
                .iter()
                .any(|cat| *cat == child.category_id)
                && request
                    .active_required_tags
                    .iter()
                    .all(|tag| child.tags.contains(tag))
                && !request
                    .active_hidden_tags
                    .iter()
                    .any(|tag| child.tags.contains(tag))
        })
        .collect();
    debug!(
        "[PERMDBG] Found {} remaining children when applying user filters",
        filtered_children.len()
    );

    // Parents must simply not be excluded to be shown, unlike children which must be allowed in their own right
    let filtered_parents: Vec<PublicListedEntity> = parents
        .into_iter()
        // filter against permissions
        .filter(|parent| {
            !is_category_explicitly_excluded_by_token(&token, &parent.category_id)
                && !parent
                    .tags
                    .iter()
                    .any(|tag_id| is_tag_explicitly_excluded_by_token(&token, tag_id))
        })
        // filter against request
        .filter(|parent| {
            !request
                .active_hidden_tags
                .iter()
                .any(|tag| parent.tags.contains(tag))
        })
        .collect();
    debug!(
        "[PERMDBG] Found {} remaining parents when applying user filters",
        filtered_parents.len()
    );

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
