use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::user::{NewOrUpdatedUser, User},
};

use super::auth::AdminUserIdentity;

#[utoipa::path(
    get,
    path = "/api/admin/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_users_list(
    user: AdminUserIdentity,
    DbConn(mut conn): DbConn,
) -> Result<AppJson<Vec<User>>, AppError> {
    if !user.is_admin {
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
pub async fn admin_user_new(
    user: AdminUserIdentity,
    DbConn(mut conn): DbConn,
    Json(new_user): Json<NewOrUpdatedUser>,
) -> Result<AppJson<User>, AppError> {
    if !user.is_admin {
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
pub async fn admin_user_get(
    user: AdminUserIdentity,
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<User>, AppError> {
    if !user.is_admin {
        return Err(AppError::Unauthorized);
    }

    Ok(AppJson(User::get(id, &mut conn).await?))
}

#[utoipa::path(
    put,
    path = "/api/admin/users/self/password",
    request_body = NewOrUpdatedUser,
    responses(
        (status = 200, description = "User", body = User),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_user_change_self_password(
    user: AdminUserIdentity,
    DbConn(mut conn): DbConn,
    Json(user_with_changed_password): Json<NewOrUpdatedUser>,
) -> Result<AppJson<User>, AppError> {
    if user_with_changed_password.name != user.username
        || user_with_changed_password.is_admin != user.is_admin
    {
        return Err(AppError::Unauthorized);
    }
    Ok(AppJson(
        User::update_user(user.admin_id, user_with_changed_password, &mut conn).await?,
    ))
}

#[utoipa::path(
    put,
    path = "/api/admin/users/{id}",
    request_body = NewOrUpdatedUser,
    params(
        ("id" = Uuid, Path, description = "User identifier")
    ),
    responses(
        (status = 200, description = "User", body = User),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    )
)]
pub async fn admin_user_update(
    user: AdminUserIdentity,
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
    Json(updated_user): Json<NewOrUpdatedUser>,
) -> Result<AppJson<User>, AppError> {
    if !user.is_admin {
        return Err(AppError::Unauthorized);
    }

    Ok(AppJson(
        User::update_user(id, updated_user, &mut conn).await?,
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
pub async fn admin_user_delete(
    user: AdminUserIdentity,
    DbConn(mut conn): DbConn,
    Path(id): Path<Uuid>,
) -> Result<AppJson<()>, AppError> {
    if !user.is_admin {
        return Err(AppError::Unauthorized);
    }

    User::delete(id, &mut conn).await?;
    Ok(AppJson(()))
}
