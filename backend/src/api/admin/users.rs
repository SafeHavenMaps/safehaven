use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::user::{NewUser, User},
};

use super::AdminUserTokenClaims;

#[utoipa::path(
    get,
    path = "/api/admin/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn list(
    token: AdminUserTokenClaims,
    DbConn(mut conn): DbConn,
) -> Result<AppJson<Vec<User>>, AppError> {
    if !token.is_admin {
        return Err(AppError::Unauthorized);
    }

    Ok(AppJson(User::list(&mut conn).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/users",
    request_body = NewUser,
    responses(
        (status = 200, description = "User", body = User),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn new(
    token: AdminUserTokenClaims,
    DbConn(mut conn): DbConn,
    Json(new_user): Json<NewUser>,
) -> Result<AppJson<User>, AppError> {
    if !token.is_admin {
        return Err(AppError::Unauthorized);
    }

    Ok(AppJson(User::new(new_user, &mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User identifier")
    ),
    responses(
        (status = 200, description = "User", body = User),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn get(
    token: AdminUserTokenClaims,
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<User>, AppError> {
    if !token.is_admin {
        return Err(AppError::Unauthorized);
    }

    Ok(AppJson(User::get(id, &mut conn).await?))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ChangePassword {
    pub password: String,
}

#[utoipa::path(
    put,
    path = "/api/admin/users/self/password",
    request_body = ChangePassword,
    responses(
        (status = 200, description = "User", body = User),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn change_self_password(
    token: AdminUserTokenClaims,
    DbConn(mut conn): DbConn,
    Json(change_password): Json<ChangePassword>,
) -> Result<AppJson<()>, AppError> {
    Ok(AppJson(
        User::change_password(token.admin_id, change_password.password, &mut conn).await?,
    ))
}

#[utoipa::path(
    put,
    path = "/api/admin/users/{id}/password",
    request_body = ChangePassword,
    params(
        ("id" = Uuid, Path, description = "User identifier")
    ),
    responses(
        (status = 200, description = "User", body = User),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn change_password(
    token: AdminUserTokenClaims,
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(change_password): Json<ChangePassword>,
) -> Result<AppJson<()>, AppError> {
    if !token.is_admin {
        return Err(AppError::Unauthorized);
    }

    Ok(AppJson(
        User::change_password(id, change_password.password, &mut conn).await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/admin/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User identifier")
    ),
    responses(
        (status = 200, description = "Deletion successful"),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn delete(
    token: AdminUserTokenClaims,
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    if !token.is_admin {
        return Err(AppError::Unauthorized);
    }

    User::delete(id, &mut conn).await?;
    Ok(AppJson(()))
}
