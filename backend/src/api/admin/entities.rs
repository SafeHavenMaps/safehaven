use axum::{
    extract::{Path, Query},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::{
        comment::AdminComment,
        entity::{AdminEntity, AdminListedEntity, AdminNewOrUpdateEntity, UnprocessedLocation},
        entity_cache::{
            AdminCachedEntitiesWithPagination, AdminCachedEntity, AdminSearchEntitiesRequest,
        },
    },
};

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct AdminSearchRequest {
    pub family: Uuid,
    pub search: String,
    pub active_categories_ids: Vec<Uuid>,
    pub required_tags_ids: Vec<Uuid>,
    pub excluded_tags_ids: Vec<Uuid>,
    pub enums_constraints: Value,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct AdminEntityWithRelations {
    pub id: Uuid,
    pub display_name: String,
    pub category_id: Uuid,
    pub family_id: Uuid,
    #[schema(value_type = Object)]
    pub locations: sqlx::types::Json<Vec<UnprocessedLocation>>,
    pub data: Value,
    pub tags: Vec<Uuid>,
    pub hidden: bool,
    pub moderation_notes: Option<String>,
    pub moderated: bool,
    pub version: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub parents: Vec<AdminListedEntity>,
    pub children: Vec<AdminListedEntity>,
}

#[utoipa::path(
    post,
    path = "/api/admin/entities/search",
    request_body = AdminSearchRequest,
    params(
        ("page" = i64, Query, description = "Current page (default: 1)"),
        ("page_size" = i64, Query, description = "Number of items per page (default: 20)")
    ),
    responses(
        (status = 200, description = "Search results for entities", body = PaginatedVec<AdminCachedEntity>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_entities_search(
    DbConn(mut conn): DbConn,
    Query(search_query): Query<SearchQuery>,
    Json(search_req): Json<AdminSearchRequest>,
) -> Result<AppJson<AdminCachedEntitiesWithPagination>, AppError> {
    let page = search_query.page.unwrap_or(1);
    let page_size = search_query.page_size.unwrap_or(20);

    Ok(AppJson(
        AdminCachedEntity::search_entities(
            AdminSearchEntitiesRequest {
                search_query: search_req.search,
                family_id: search_req.family,
                page,
                page_size,
                active_categories_ids: search_req.active_categories_ids,
                required_tags_ids: search_req.required_tags_ids,
                excluded_tags_ids: search_req.excluded_tags_ids,
                enums_constraints: search_req.enums_constraints,
            },
            &mut conn,
        )
        .await?,
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
    DbConn(mut conn): DbConn,
    Json(new_entity): Json<AdminNewOrUpdateEntity>,
) -> Result<AppJson<AdminEntity>, AppError> {
    Ok(AppJson(AdminEntity::new(new_entity, &mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/entities/{id}",
    params(
        ("id" = Uuid, Path, description = "Entity identifier")
    ),
    responses(
        (status = 200, description = "Entity details", body = AdminEntityWithRelations),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_entity_get(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<AdminEntityWithRelations>, AppError> {
    // Fetch the main AdminEntity
    let admin_entity = AdminEntity::get(id, &mut conn).await?;
    // Fetch related children and parents
    let children = AdminEntity::get_children(id, &mut conn).await?;
    let parents = AdminEntity::get_parents(id, &mut conn).await?;
    // Recompose AdminEntityWithRelations
    let admin_entity_with_relations = AdminEntityWithRelations {
        id: admin_entity.id,
        display_name: admin_entity.display_name,
        category_id: admin_entity.category_id,
        family_id: admin_entity.family_id,
        locations: admin_entity.locations,
        data: admin_entity.data,
        tags: admin_entity.tags,
        hidden: admin_entity.hidden,
        moderation_notes: admin_entity.moderation_notes,
        moderated: admin_entity.moderated,
        version: admin_entity.version,
        created_at: admin_entity.created_at,
        updated_at: admin_entity.updated_at,
        parents,
        children,
    };
    // Return the recomposed entity
    Ok(AppJson(admin_entity_with_relations))
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
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(updated_entity): Json<AdminNewOrUpdateEntity>,
) -> Result<AppJson<AdminEntity>, AppError> {
    Ok(AppJson(
        AdminEntity::update(id, updated_entity, &mut conn).await?,
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
        (status = 200, description = "List of comments for the entity", body = Vec<AdminComment>),
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
