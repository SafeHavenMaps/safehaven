use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::{
        category::{Category, NewOrUpdateCategory},
        icon::Icon,
    },
};

#[utoipa::path(
    get,
    path = "/api/admin/categories",
    responses(
        (status = 200, description = "List of categories", body = Vec<Category>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_categories_list(
    DbConn(mut conn): DbConn,
) -> Result<AppJson<Vec<Category>>, AppError> {
    Ok(AppJson(Category::list(&mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/categories",
    request_body = NewOrUpdateCategory,
    responses(
        (status = 200, description = "Category created", body = Category),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_category_new(
    DbConn(mut conn): DbConn,
    Json(new_category): Json<NewOrUpdateCategory>,
) -> Result<AppJson<Category>, AppError> {
    Ok(AppJson(Category::new(new_category, &mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/categories/{id}",
    params(
        ("id" = Uuid, Path, description = "Category identifier")
    ),
    responses(
        (status = 200, description = "Category details", body = Category),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_category_get(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<Category>, AppError> {
    Ok(AppJson(Category::get(id, &mut conn).await?))
}

#[utoipa::path(
    put,
    path = "/api/admin/categories/{id}",
    request_body = NewOrUpdateCategory,
    params(
        ("id" = Uuid, Path, description = "Category identifier")
    ),
    responses(
        (status = 200, description = "Category updated", body = Category),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_category_update(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(update_category): Json<NewOrUpdateCategory>,
) -> Result<AppJson<Category>, AppError> {
    Ok(AppJson(
        Category::update(id, update_category, &mut conn).await?,
    ))
}

#[utoipa::path(
    put,
    path = "/api/admin/categories/:id/icon",
    request_body = Icon,
    params(
        ("id" = Uuid, Path, description = "Category identifier")
    ),
    responses(
        (status = 200, description = "Category icon updated"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_category_update_icon(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(icon): Json<Icon>,
) -> Result<(), AppError> {
    Ok(AppJson(
        Icon::upsert_category(id, icon.data, icon.http_mime_type, &mut conn).await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/admin/categories/{id}",
    params(
        ("id" = Uuid, Path, description = "Category identifier")
    ),
    responses(
        (status = 200, description = "Category deletion successful"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_category_delete(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    Category::delete(id, &mut conn).await?;
    Ok(AppJson(()))
}

#[utoipa::path(
    delete,
    path = "/api/admin/categories/:id/icon",
    params(
        ("id" = Uuid, Path, description = "Category identifier")
    ),
    responses(
        (status = 200, description = "Category icon deletion successful"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_category_delete_icon(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    Icon::delete_for_category(id, &mut conn).await?;
    Ok(AppJson(()))
}
