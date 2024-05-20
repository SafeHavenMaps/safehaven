use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::comment::{Comment, ListedComment, NewComment, UpdateComment},
};

#[utoipa::path(
    get,
    path = "/api/admin/comments/pending",
    responses(
        (status = 200, description = "List of pending comments", body = Vec<ListedComment>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_comments_pending(
    DbConn(mut conn): DbConn,
) -> Result<AppJson<Vec<ListedComment>>, AppError> {
    Ok(AppJson(Comment::pending(&mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/comments",
    request_body = NewComment,
    responses(
        (status = 200, description = "Comment created", body = Comment),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_comment_new(
    DbConn(mut conn): DbConn,
    Json(new_comment): Json<NewComment>,
) -> Result<AppJson<Comment>, AppError> {
    Ok(AppJson(Comment::new(new_comment, &mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/comments/{id}",
    params(
        ("id" = Uuid, Path, description = "Comment identifier")
    ),
    responses(
        (status = 200, description = "Comment details", body = Comment),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_comment_get(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<Comment>, AppError> {
    Ok(AppJson(Comment::get(id, &mut conn).await?))
}

#[utoipa::path(
    put,
    path = "/api/admin/comments/{id}",
    request_body = UpdateComment,
    params(
        ("id" = Uuid, Path, description = "Comment identifier")
    ),
    responses(
        (status = 200, description = "Comment updated", body = Comment),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_comment_update(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(updated_comment): Json<UpdateComment>,
) -> Result<AppJson<Comment>, AppError> {
    Ok(AppJson(
        Comment::update(id, updated_comment, &mut conn).await?,
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
    Comment::delete(id, &mut conn).await?;
    Ok(AppJson(()))
}
