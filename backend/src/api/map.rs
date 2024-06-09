use crate::api::{AppError, AppJson, AppState, DbConn, MapUserToken};
use crate::models::comment::{PublicComment, PublicNewComment};
use crate::models::entity::{PublicEntity, PublicListedEntity, PublicNewEntity};
use crate::models::entity_cache::{
    CachedEntitiesWithPagination, CachedEntity, EntitiesAndClusters, FindEntitiesRequest,
    SearchEntitiesRequest,
};
use axum::extract::{Path, State};
use axum::{
    routing::{get, post, Router},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;
use uuid::Uuid;

use super::MapUserTokenClaims;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/view", post(viewer_view_request))
        .route("/search", post(viewer_search_request))
        .route("/entities/:id", get(viewer_fetch_entity))
        .route("/entities", post(viewer_new_entity))
        .route("/comments", post(viewer_new_comment))
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
    MapUserToken(token): MapUserToken,
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
    };

    Ok(AppJson(
        CachedEntity::find_entities_in_rectangle(request, &mut conn).await?,
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
    MapUserToken(token): MapUserToken,
    Json(request): Json<SearchRequest>,
) -> Result<AppJson<CachedEntitiesWithPagination>, AppError> {
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
    };

    Ok(AppJson(
        CachedEntity::search_entities(request, &mut conn).await?,
    ))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct NewEntityRequest {
    entity: PublicNewEntity,
}

#[utoipa::path(
    post,
    path = "/api/map/entities",
    request_body = NewEntityRequest,
    responses(
        (status = 200, description = "Entity", body = PublicEntity),
        (status = 401, description = "Invalid token", body = ErrorResponse),
    )
)]
async fn viewer_new_entity(
    DbConn(mut conn): DbConn,
    MapUserToken(_token): MapUserToken,
    Json(request): Json<NewEntityRequest>,
) -> Result<AppJson<PublicEntity>, AppError> {
    let db_entity = PublicEntity::new(request.entity, &mut conn).await?;
    Ok(AppJson(db_entity))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct NewCommentRequest {
    comment: PublicNewComment,
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
    MapUserToken(_token): MapUserToken,
    Json(request): Json<NewCommentRequest>,
) -> Result<AppJson<PublicComment>, AppError> {
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

#[utoipa::path(
    get,
    path = "/api/map/entities/{id}",
    responses(
        (status = 200, description = "Entity", body = FetchedEntity),
        (status = 401, description = "Invalid token", body = ErrorResponse),
        (status = 404, description = "Entity not found", body = ErrorResponse),
    )
)]
async fn viewer_fetch_entity(
    DbConn(mut conn): DbConn,
    MapUserToken(token): MapUserToken,
    Path(id): Path<Uuid>,
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

    if !can_read_entity {
        return Err(AppError::Unauthorized);
    }

    let comments = match token.perms.can_access_comments {
        true => PublicComment::list_for_public_entity(id, &entity.comment_form, &mut conn).await?,
        false => vec![],
    };

    let parents = PublicEntity::get_parents(id, &mut conn).await?;
    let children = PublicEntity::get_children(id, &mut conn).await?;

    Ok(AppJson(FetchedEntity {
        entity,
        comments,
        parents,
        children,
    }))
}
