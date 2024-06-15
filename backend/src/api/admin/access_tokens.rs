use crate::{
    api::{AppError, AppJson, DbConn},
    models::access_token::{AccessToken, AccessTokenStats, NewOrUpdateAccessToken},
};
use axum::{extract::Path, Json};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/admin/access_tokens",
    responses(
        (status = 200, description = "List of access tokens", body = Vec<AccessToken>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_access_tokens_list(
    DbConn(mut conn): DbConn,
) -> Result<AppJson<Vec<AccessToken>>, AppError> {
    Ok(AppJson(AccessToken::list(&mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/access_tokens",
    request_body = NewOrUpdateAccessToken,
    responses(
        (status = 200, description = "List of access tokens", body = AccessToken),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_access_token_new(
    DbConn(mut conn): DbConn,
    Json(new_access_token): Json<NewOrUpdateAccessToken>,
) -> Result<AppJson<AccessToken>, AppError> {
    Ok(AppJson(
        AccessToken::new(new_access_token, &mut conn).await?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/admin/access_tokens/{id}",
    params(
        ("id" = Uuid, Path, description = "Access token identifier")
    ),
    responses(
        (status = 200, description = "Access token", body = AccessToken),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_access_token_get(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<AccessToken>, AppError> {
    Ok(AppJson(AccessToken::get_with_id(id, &mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/access_tokens/{id}/stats",
    params(
        ("id" = Uuid, Path, description = "Access token identifier")
    ),
    responses(
        (status = 200, description = "Access token statistics", body = AccessTokenStats),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_access_token_get_stats(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<AccessTokenStats>, AppError> {
    Ok(AppJson(AccessToken::get_stats(id, &mut conn).await?))
}

#[utoipa::path(
    put,
    path = "/api/admin/access_tokens/{id}",
    request_body = NewOrUpdateAccessToken,
    params(
        ("id" = Uuid, Path, description = "Access token identifier")
    ),
    responses(
        (status = 200, description = "Access token", body = AccessToken),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_access_token_update(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(new_access_token): Json<NewOrUpdateAccessToken>,
) -> Result<AppJson<AccessToken>, AppError> {
    Ok(AppJson(
        AccessToken::update(id, new_access_token, &mut conn).await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/admin/access_tokens/{id}",
    params(
        ("id" = Uuid, Path, description = "Access token identifier")
    ),
    responses(
        (status = 200, description = "Deletion successful"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_access_token_delete(
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    AccessToken::delete(id, &mut conn).await?;
    Ok(AppJson(()))
}
