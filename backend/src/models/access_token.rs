use crate::api::AppError;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use sqlx::{FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct Permissions {
    pub families_policy: PermissionPolicy,
    pub categories_policy: PermissionPolicy,
    pub tags_policy: PermissionPolicy,
    pub can_access_comments: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PermissionPolicy {
    pub allow_all: bool,
    pub allow_list: Vec<Uuid>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewOrUpdateAccessToken {
    pub token: String,
    pub permissions: Permissions,
    pub active: bool,
}

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct RawAccessToken {
    pub id: Uuid,
    pub token: String,
    pub permissions: Value,
    pub active: bool,
}

impl TryFrom<RawAccessToken> for AccessToken {
    type Error = AppError;

    fn try_from(raw: RawAccessToken) -> Result<AccessToken, AppError> {
        let permissions: Permissions = serde_json::from_value(raw.permissions)
            .map_err(|e| AppError::ValidationError(format!("Error parsing permissions: {}", e)))?;

        Ok(AccessToken {
            id: raw.id,
            token: raw.token,
            permissions,
            active: raw.active,
        })
    }
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct AccessToken {
    pub id: Uuid,
    pub token: String,
    pub permissions: Permissions,
    pub active: bool,
}

impl AccessToken {
    pub async fn new(
        access_token: NewOrUpdateAccessToken,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        let permission_value =
            to_value(access_token.permissions).expect("Failed to serialize permissions");

        sqlx::query_as!(
            RawAccessToken,
            r#"
            INSERT INTO access_tokens (token, permissions, active)
            VALUES ($1, $2, $3)
            RETURNING id, token, permissions, active
            "#,
            access_token.token,
            permission_value,
            access_token.active
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)?
        .try_into()
    }

    pub async fn update(
        given_id: Uuid,
        update: NewOrUpdateAccessToken,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        let permission_value =
            to_value(update.permissions).expect("Failed to serialize permissions");

        sqlx::query_as!(
            RawAccessToken,
            r#"
            UPDATE access_tokens
            SET token = $2, permissions = $3, active = $4
            WHERE id = $1
            RETURNING id, token, permissions, active
            "#,
            given_id,
            update.token,
            permission_value,
            update.active
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)?
        .try_into()
    }

    pub async fn delete(given_id: Uuid, conn: &mut PgConnection) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM access_tokens
            WHERE id = $1
            "#,
            given_id
        )
        .execute(conn)
        .await
        .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    pub async fn get(
        given_token: String,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        sqlx::query_as!(
            RawAccessToken,
            r#"
            SELECT id, token, permissions, active
            FROM access_tokens
            WHERE token = $1
            "#,
            given_token
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)?
        .try_into()
    }

    pub async fn get_with_id(
        given_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        sqlx::query_as!(
            RawAccessToken,
            r#"
            SELECT id, token, permissions, active
            FROM access_tokens
            WHERE id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)?
        .try_into()
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<AccessToken>, AppError> {
        sqlx::query_as!(
            RawAccessToken,
            r#"
            SELECT id, token, permissions, active
            FROM access_tokens
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)?
        .into_iter()
        .map(|raw| raw.try_into())
        .collect()
    }
}
