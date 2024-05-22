use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::{
        comment::Comment,
        entity::{Entity, ListedEntity, NewEntity, UpdateEntity},
    },
};

use super::AdminUserTokenClaims;

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub search: String,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/api/admin/entities/search",
    params(
        ("search" = String, Query, description = "Search query"),
        ("page" = i64, Query, description = "Current page (default: 1)"),
        ("page_size" = i64, Query, description = "Number of items per page (default: 20)")
    ),
    responses(
        (status = 200, description = "Search results for entities", body = Vec<ListedEntity>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_entities_search(
    DbConn(mut conn): DbConn,
    Query(query): Query<SearchQuery>,
) -> Result<AppJson<Vec<ListedEntity>>, AppError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    Ok(AppJson(
        Entity::search(query.search, page, page_size, &mut conn).await?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/admin/entities/pending",
    responses(
        (status = 200, description = "List of pending entities", body = Vec<ListedEntity>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_entities_pending(
    DbConn(mut conn): DbConn,
) -> Result<AppJson<Vec<ListedEntity>>, AppError> {
    Ok(AppJson(Entity::pending(&mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/entities",
    request_body = NewEntity,
    responses(
        (status = 200, description = "Entity created", body = Entity),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_entity_new(
    DbConn(mut conn): DbConn,
    Json(new_entity): Json<NewEntity>,
) -> Result<AppJson<Entity>, AppError> {
    Ok(AppJson(Entity::new(new_entity, &mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/entities/{id}",
    params(
        ("id" = Uuid, Path, description = "Entity identifier")
    ),
    responses(
        (status = 200, description = "Entity details", body = Entity),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_entity_get(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<Entity>, AppError> {
    Ok(AppJson(Entity::get(id, &mut conn).await?))
}

#[utoipa::path(
    put,
    path = "/api/admin/entities/{id}",
    request_body = UpdateEntity,
    params(
        ("id" = Uuid, Path, description = "Entity identifier")
    ),
    responses(
        (status = 200, description = "Entity updated", body = Entity),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_entity_update(
    token: AdminUserTokenClaims,
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(mut updated_entity): Json<UpdateEntity>,
) -> Result<AppJson<Entity>, AppError> {
    // Ensure the last_update_by field is set to the current admin's ID
    updated_entity.last_update_by = Some(token.admin_id);

    Ok(AppJson(
        Entity::update(id, updated_entity, &mut conn).await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/admin/entities/{id}",
    params(
        ("id" = Uuid, Path, description = "Entity identifier")
    ),
    responses(
        (status = 200, description = "Entity deleted successfully"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_entity_delete(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    Entity::delete(id, &mut conn).await?;
    Ok(AppJson(()))
}

#[utoipa::path(
    get,
    path = "/api/admin/entities/{id}/comments",
    params(
        ("id" = Uuid, Path, description = "Entity identifier")
    ),
    responses(
        (status = 200, description = "List of comments for the entity", body = Vec<Comment>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_entity_get_comments(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<Vec<Comment>>, AppError> {
    Ok(AppJson(Comment::list_for_entity(id, &mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/entities/{parent_id}/parent/{child_id}",
    params(
        ("parent_id" = Uuid, Path, description = "Parent entity identifier"),
        ("child_id" = Uuid, Path, description = "Child entity identifier")
    ),
    responses(
        (status = 200, description = "Parent entity registered successfully"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_entity_register_parent(
    DbConn(mut conn): DbConn,
    Path((parent_id, child_id)): Path<(Uuid, Uuid)>,
) -> Result<AppJson<()>, AppError> {
    Entity::register_parent_child(parent_id, child_id, &mut conn).await?;
    Ok(AppJson(()))
}

#[utoipa::path(
    delete,
    path = "/api/admin/entities/{parent_id}/parent/{child_id}",
    params(
        ("parent_id" = Uuid, Path, description = "Parent entity identifier"),
        ("child_id" = Uuid, Path, description = "Child entity identifier")
    ),
    responses(
        (status = 200, description = "Parent-child relationship removed successfully"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Relationship not found", body = ErrorResponse),
    )
)]
pub async fn admin_entity_remove_parent(
    DbConn(mut conn): DbConn,
    Path((parent_id, child_id)): Path<(Uuid, Uuid)>,
) -> Result<AppJson<()>, AppError> {
    Entity::delete_parent_child(parent_id, child_id, &mut conn).await?;
    Ok(AppJson(()))
}
