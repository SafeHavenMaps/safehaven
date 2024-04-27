use crate::api::{AppError, AppJson, AppState, DbConn, MapUserToken};
use crate::models::comment::{Comment, NewComment, PublicComment};
use crate::models::entity::{Entity, ListedEntity, NewEntity, PublicEntity};
use crate::models::entity_cache::{
    CachedEntity, EntitiesAndClusters, FindEntitiesRequest, SearchEntitiesRequest,
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
    family_id: Uuid,
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
    zoom_level: u8,
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
    if zoom > 26.9 {
        return None;
    }
    Some((
        characteristic_distance * 100000. * declustering_speed.powf(-zoom),
        minimal_cluster_size,
    ))
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
    State(app_state): State<AppState>,
    DbConn(mut conn): DbConn,
    MapUserToken(token): MapUserToken,
    Json(request): Json<ViewRequest>,
) -> Result<AppJson<EntitiesAndClusters>, AppError> {
    let cluster_config = &app_state.config.cartography.cluster;
    let cluster_params = clusterize(
        cluster_config.characteristic_distance,
        cluster_config.declustering_speed,
        cluster_config.minimal_cluster_size,
        request.zoom_level as f64,
    );

    tracing::trace!("Received view request {}", request);

    // Checking if the family is allowed
    // We prefer readability here so we decompose the conditions
    let family_is_allowed = token.perms.families_policy.allow_all
        || token
            .perms
            .families_policy
            .allow_list
            .contains(&request.family_id);

    let family_is_excluded = token
        .perms
        .families_policy
        .force_exclude
        .contains(&request.family_id);

    if !family_is_allowed || family_is_excluded {
        return Err(AppError::Unauthorized);
    }

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
