use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::{
        comment::AdminComment,
        entity::{AdminEntity, AdminListedEntity, AdminNewOrUpdateEntity},
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
        (status = 200, description = "Search results for entities", body = Vec<AdminListedEntity>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_entities_search(
    DbConn(mut conn): DbConn,
    Query(query): Query<SearchQuery>,
) -> Result<AppJson<Vec<AdminListedEntity>>, AppError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    Ok(AppJson(
        AdminEntity::search(query.search, page, page_size, &mut conn).await?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/admin/entities/pending",
    responses(
        (status = 200, description = "List of pending entities", body = Vec<AdminListedEntity>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_entities_pending(
    DbConn(mut conn): DbConn,
) -> Result<AppJson<Vec<AdminListedEntity>>, AppError> {
    Ok(AppJson(AdminEntity::pending(&mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/entities",
    request_body = AdminNewOrUpdateEntity,
    responses(
        (status = 200, description = "Entity created", body = AdminEntity),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_entity_new(
    token: AdminUserTokenClaims,
    DbConn(mut conn): DbConn,
    Json(new_entity): Json<AdminNewOrUpdateEntity>,
) -> Result<AppJson<AdminEntity>, AppError> {
    Ok(AppJson(
        AdminEntity::new(new_entity, token.admin_id, &mut conn).await?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/admin/entities/{id}",
    params(
        ("id" = Uuid, Path, description = "Entity identifier")
    ),
    responses(
        (status = 200, description = "Entity details", body = AdminEntity),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_entity_get(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<AdminEntity>, AppError> {
    Ok(AppJson(AdminEntity::get(id, &mut conn).await?))
}

#[utoipa::path(
    put,
    path = "/api/admin/entities/{id}",
    request_body = AdminNewOrUpdateEntity,
    params(
        ("id" = Uuid, Path, description = "Entity identifier")
    ),
    responses(
        (status = 200, description = "Entity updated", body = AdminEntity),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_entity_update(
    token: AdminUserTokenClaims,
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(updated_entity): Json<AdminNewOrUpdateEntity>,
) -> Result<AppJson<AdminEntity>, AppError> {
    Ok(AppJson(
        AdminEntity::update(id, updated_entity, token.admin_id, &mut conn).await?,
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
    AdminEntity::delete(id, &mut conn).await?;
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
) -> Result<AppJson<Vec<AdminComment>>, AppError> {
    Ok(AppJson(AdminComment::list_for_entity(id, &mut conn).await?))
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
    AdminEntity::register_parent_child(parent_id, child_id, &mut conn).await?;
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
    AdminEntity::delete_parent_child(parent_id, child_id, &mut conn).await?;
    Ok(AppJson(()))
}
