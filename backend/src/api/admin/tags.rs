use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::tag::{NewOrUpdateTag, Tag},
};

#[utoipa::path(
    get,
    path = "/api/admin/tags",
    responses(
        (status = 200, description = "List of tags", body = Vec<Tag>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn list(DbConn(mut conn): DbConn) -> Result<AppJson<Vec<Tag>>, AppError> {
    Ok(AppJson(Tag::list(&mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/tags",
    request_body = NewOrUpdateTag,
    responses(
        (status = 200, description = "Tag created", body = Tag),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn new(
    DbConn(mut conn): DbConn,
    Json(new_tag): Json<NewOrUpdateTag>,
) -> Result<AppJson<Tag>, AppError> {
    Ok(AppJson(Tag::new(new_tag, &mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/tags/{id}",
    params(
        ("id" = Uuid, Path, description = "Tag identifier")
    ),
    responses(
        (status = 200, description = "Tag details", body = Tag),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn get(DbConn(mut conn): DbConn, Path(id): Path<Uuid>) -> Result<AppJson<Tag>, AppError> {
    Ok(AppJson(Tag::get(id, &mut conn).await?))
}

#[utoipa::path(
    put,
    path = "/api/admin/tags/{id}",
    request_body = NewOrUpdateTag,
    params(
        ("id" = Uuid, Path, description = "Tag identifier")
    ),
    responses(
        (status = 200, description = "Tag updated", body = Tag),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn update(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(new_tag): Json<NewOrUpdateTag>,
) -> Result<AppJson<Tag>, AppError> {
    Ok(AppJson(Tag::update(id, new_tag, &mut conn).await?))
}

#[utoipa::path(
    delete,
    path = "/api/admin/tags/{id}",
    params(
        ("id" = Uuid, Path, description = "Tag identifier")
    ),
    responses(
        (status = 200, description = "Tag deleted successfully"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn delete(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    Tag::delete(id, &mut conn).await?;
    Ok(AppJson(()))
}
