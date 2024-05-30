use axum::{
    extract::{Multipart, Path},
    Json,
};
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::{
        family::{Family, NewOrUpdateFamily},
        icon::Icon,
    },
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
    put,
    path = "/api/admin/families/:id/icon",
    request_body = Icon,
    params(
        ("id" = Uuid, Path, description = "Family identifier")
    ),
    responses(
        (status = 200, description = "Family icon updated"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_family_update_icon(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<(), AppError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|_| AppError::Validation("icon malformed".to_string()))?
        .ok_or(AppError::Validation("icon missing".to_string()))?;

    let mime = field
        .content_type()
        .ok_or(AppError::Validation("icon missing".to_string()))?
        .to_string();

    let data = field
        .bytes()
        .await
        .map_err(|_| AppError::Validation("icon missing".to_string()))?
        .to_vec();

    Icon::upsert_family(id, data, mime.to_string(), &mut conn).await?;
    Ok(())
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

#[utoipa::path(
    delete,
    path = "/api/admin/families/:id/icon",
    params(
        ("id" = Uuid, Path, description = "Family identifier")
    ),
    responses(
        (status = 200, description = "Family icon deletion successful"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_family_delete_icon(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    Icon::delete_for_family(id, &mut conn).await?;
    Ok(AppJson(()))
}
