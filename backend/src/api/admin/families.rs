use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::family::{Family, NewOrUpdateFamily},
};

#[utoipa::path(
    get,
    path = "/api/admin/families",
    responses(
        (status = 200, description = "List of families", body = Vec<Family>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_families_list(
    DbConn(mut conn): DbConn,
) -> Result<AppJson<Vec<Family>>, AppError> {
    Ok(AppJson(Family::list(&mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/families",
    request_body = NewOrUpdateFamily,
    responses(
        (status = 200, description = "Family created", body = Family),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_family_new(
    DbConn(mut conn): DbConn,
    Json(new_family): Json<NewOrUpdateFamily>,
) -> Result<AppJson<Family>, AppError> {
    Ok(AppJson(Family::new(new_family, &mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/families/{id}",
    params(
        ("id" = Uuid, Path, description = "Family identifier")
    ),
    responses(
        (status = 200, description = "Family details", body = Family),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_family_get(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<Family>, AppError> {
    Ok(AppJson(Family::get(id, &mut conn).await?))
}

#[utoipa::path(
    put,
    path = "/api/admin/families/{id}",
    request_body = NewOrUpdateFamily,
    params(
        ("id" = Uuid, Path, description = "Family identifier")
    ),
    responses(
        (status = 200, description = "Family updated", body = Family),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_family_update(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(new_family): Json<NewOrUpdateFamily>,
) -> Result<AppJson<Family>, AppError> {
    Ok(AppJson(Family::update(id, new_family, &mut conn).await?))
}

#[utoipa::path(
    delete,
    path = "/api/admin/families/{id}",
    params(
        ("id" = Uuid, Path, description = "Family identifier")
    ),
    responses(
        (status = 200, description = "Family deleted successfully"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_family_delete(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    Family::delete(id, &mut conn).await?;
    Ok(AppJson(()))
}
