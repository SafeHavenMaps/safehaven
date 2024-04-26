use crate::api::{AppError, AppJson, AppState, DbConn, MapUserToken};
use crate::models::comment::{Comment, NewComment, PublicComment};
use crate::models::entity::{Entity, ListedEntity, NewEntity, PublicEntity};
use crate::models::entity_cache::{CachedEntity, EntitiesAndClusters, FindEntitiesRequest, SearchEntitiesRequest};
use axum::extract::Path;
use axum::{
    routing::{get, post, Router},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/view", post(view_request))
        .route("/search", post(search_request))
        .route("/entities/:id", get(fetch_entity))
        .route("/entities", post(new_entity))
        .route("/comments", post(new_comment))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ViewRequest {
    upper_left_lat: f64,
    upper_left_lon: f64,
    lower_right_lat: f64,
    lower_right_lon: f64,
    zoom_level: u8,
}

impl ViewRequest {
    /// Returns the epsilon and min points for the DBSCAN clustering algorithm
    /// Zoom goes from 1 to 28 (OpenLayers default zoom levels)
    /// For each zoom level, we have a different epsilon and min points
    pub fn get_eps_and_min_from_zoom(&self) -> (f64, i32) {
        match self.zoom_level {
            1..=5 => (1.0, 25),
            6..=8 => (0.13, 10),
            9 => (0.07, 10),
            10 => (0.04, 10),
            11..=12 => (0.01, 10),
            13..=15 => (0.005, 10),
            16..=18 => (0.0025, 10),
            19..=21 => (0.001, 10),
            22..=24 => (0.0005, 10),
            25..=28 => (0.0001, 10),
            _ => (0.0001, 10),
        }
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
pub async fn view_request(
    DbConn(mut conn): DbConn,
    MapUserToken(token): MapUserToken,
    Json(request): Json<ViewRequest>,
) -> Result<AppJson<EntitiesAndClusters>, AppError> {
    let (eps, min) = request.get_eps_and_min_from_zoom();
    let request = FindEntitiesRequest {
        upper_left_lat: request.upper_left_lat,
        upper_left_lon: request.upper_left_lon,
        lower_right_lat: request.lower_right_lat,
        lower_right_lon: request.lower_right_lon,

        allow_all_families: token.perms.families_policy.allow_all,
        allow_all_categories: token.perms.categories_policy.allow_all,

        allow_all_tags: token.perms.tags_policy.allow_all,
        families_list: token.perms.families_policy.allow_list.clone(),
        categories_list: token.perms.categories_policy.allow_list.clone(),
        tags_list: token.perms.tags_policy.allow_list.clone(),

        exclude_families_list: token.perms.families_policy.force_exclude.clone(),
        exclude_categories_list: token.perms.categories_policy.force_exclude.clone(),
        exclude_tags_list: token.perms.tags_policy.force_exclude.clone(),

        cluster_eps: eps,
        cluster_min_points: min,
    };

    Ok(AppJson(
        CachedEntity::find_entities_in_rectangle(request, &mut conn).await?,
    ))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SearchRequest {
    search_query: String,
}

#[utoipa::path(
    post,
    path = "/api/map/search",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "List of entities", body = Vec<CachedEntity>),
        (status = 401, description = "Invalid token", body = ErrorResponse),
    )
)]
async fn search_request(
    DbConn(mut conn): DbConn,
    MapUserToken(token): MapUserToken,
    Json(request): Json<SearchRequest>,
) -> Result<AppJson<Vec<CachedEntity>>, AppError> {
    let request = SearchEntitiesRequest {
        search_query: request.search_query,
        allow_all_families: token.perms.families_policy.allow_all,
        allow_all_categories: token.perms.categories_policy.allow_all,
        allow_all_tags: token.perms.tags_policy.allow_all,
        families_list: token.perms.families_policy.allow_list.clone(),
        categories_list: token.perms.categories_policy.allow_list.clone(),
        tags_list: token.perms.tags_policy.allow_list.clone(),
        exclude_families_list: token.perms.families_policy.force_exclude.clone(),
        exclude_categories_list: token.perms.categories_policy.force_exclude.clone(),
        exclude_tags_list: token.perms.tags_policy.force_exclude.clone(),
    };

    Ok(AppJson(
        CachedEntity::search_entities(request, &mut conn).await?,
    ))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct NewEntityRequest {
    entity: NewEntity,
}

#[utoipa::path(
    post,
    path = "/api/map/entities",
    request_body = NewEntityRequest,
    responses(
        (status = 200, description = "Entity", body = Entity),
        (status = 401, description = "Invalid token", body = ErrorResponse),
    )
)]
async fn new_entity(
    DbConn(mut conn): DbConn,
    MapUserToken(_token): MapUserToken,
    Json(request): Json<NewEntityRequest>,
) -> Result<AppJson<Entity>, AppError> {
    let db_entity = Entity::new(request.entity, &mut conn).await?;
    Ok(AppJson(db_entity))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct NewCommentRequest {
    comment: NewComment,
}

#[utoipa::path(
    post,
    path = "/api/map/comments",
    request_body = NewEntityRequest,
    responses(
        (status = 200, description = "Comment", body = Comment),
        (status = 401, description = "Invalid token", body = ErrorResponse),
    )
)]
async fn new_comment(
    DbConn(mut conn): DbConn,
    MapUserToken(_token): MapUserToken,
    Json(request): Json<NewCommentRequest>,
) -> Result<AppJson<Comment>, AppError> {
    let db_comment = Comment::new(request.comment, &mut conn).await?;
    Ok(AppJson(db_comment))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct FetchedEntity {
    pub entity: PublicEntity,
    pub comments: Vec<PublicComment>,
    pub parents: Vec<ListedEntity>,
    pub children: Vec<ListedEntity>,
}

#[utoipa::path(
    get,
    path = "/api/map/entities/:id",
    responses(
        (status = 200, description = "Entity", body = FetchedEntity),
        (status = 401, description = "Invalid token", body = ErrorResponse),
        (status = 404, description = "Entity not found", body = ErrorResponse),
    )
)]
async fn fetch_entity(
    DbConn(mut conn): DbConn,
    MapUserToken(token): MapUserToken,
    Path(id): Path<Uuid>,
) -> Result<AppJson<FetchedEntity>, AppError> {
    let entity = Entity::get_public(id, &mut conn).await?;

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
        true => Comment::list_for_public_entity(id, &entity.comment_form, &mut conn).await?,
        false => vec![],
    };

    let parents = Entity::get_parents(id, &mut conn).await?;
    let children = Entity::get_children(id, &mut conn).await?;

    Ok(AppJson(FetchedEntity {
        entity,
        comments,
        parents,
        children,
    }))
}
