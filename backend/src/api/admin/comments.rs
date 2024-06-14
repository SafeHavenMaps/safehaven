use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::comment::{AdminComment, AdminListedComment, AdminNewOrUpdateComment},
};

#[utoipa::path(
    get,
    path = "/api/admin/comments/pending",
    responses(
        (status = 200, description = "List of pending comments", body = Vec<AdminListedComment>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_comments_pending(
    DbConn(mut conn): DbConn,
) -> Result<AppJson<Vec<AdminListedComment>>, AppError> {
    Ok(AppJson(AdminComment::pending(&mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/comments",
    request_body = AdminNewOrUpdateComment,
    responses(
        (status = 200, description = "Comment created", body = AdminComment),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_comment_new(
    DbConn(mut conn): DbConn,
    Json(new_comment): Json<AdminNewOrUpdateComment>,
) -> Result<AppJson<AdminComment>, AppError> {
    Ok(AppJson(AdminComment::new(new_comment, &mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/comments/{id}",
    params(
        ("id" = Uuid, Path, description = "Comment identifier")
    ),
    responses(
        (status = 200, description = "Comment details", body = AdminComment),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_comment_get(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<AdminComment>, AppError> {
    Ok(AppJson(AdminComment::get(id, &mut conn).await?))
}

#[utoipa::path(
    put,
    path = "/api/admin/comments/{id}",
    request_body = AdminNewOrUpdateComment,
    params(
        ("id" = Uuid, Path, description = "Comment identifier")
    ),
    responses(
        (status = 200, description = "Comment updated", body = AdminComment),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_comment_update(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(updated_comment): Json<AdminNewOrUpdateComment>,
) -> Result<AppJson<AdminComment>, AppError> {
    Ok(AppJson(
        AdminComment::update(id, updated_comment, &mut conn).await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/admin/comments/{id}",
    params(
        ("id" = Uuid, Path, description = "Comment identifier")
    ),
    responses(
        (status = 200, description = "Comment deleted successfully"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_comment_delete(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    AdminComment::delete(id, &mut conn).await?;
    Ok(AppJson(()))
}
