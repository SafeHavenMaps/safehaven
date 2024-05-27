use crate::api::AppError;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use sqlx::{types::Json, PgConnection};
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
    pub force_exclude: Vec<Uuid>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewOrUpdateAccessToken {
    pub title: String,
    pub token: String,
    pub permissions: Json<Permissions>,
    pub active: bool,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct AccessToken {
    pub id: Uuid,
    pub title: String,
    pub token: String,
    pub permissions: Json<Permissions>,
    pub last_week_visits: i64,
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
            AccessToken,
            r#"
            INSERT INTO access_tokens (title, token, permissions, active)
            VALUES ($1, $2, $3, $4)
            RETURNING 
                id,
                title,
                token, 
                permissions as "permissions: Json<Permissions>",
                active,
                0 as "last_week_visits!"
            "#,
            access_token.title,
            access_token.token,
            permission_value,
            access_token.active
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update(
        given_id: Uuid,
        update: NewOrUpdateAccessToken,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        let permission_value =
            to_value(update.permissions).expect("Failed to serialize permissions");

        sqlx::query_as!(
            AccessToken,
            r#"
            UPDATE access_tokens
            SET title = $2, token = $3, permissions = $4, active = $5
            WHERE id = $1
            RETURNING 
                id,
                title,
                token,
                permissions as "permissions: Json<Permissions>",
                active,
                (SELECT COUNT(*) FROM access_tokens_visits WHERE token_id = id AND visited_at > NOW() - INTERVAL '1 week') as "last_week_visits!"
            "#,
            given_id,
            update.title,
            update.token,
            permission_value,
            update.active
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
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
        .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn get(
        given_token: String,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        // We don't need to count the visits here
        sqlx::query_as!(
            AccessToken,
            r#"
            SELECT
                id,
                title,
                token,
                permissions as "permissions: Json<Permissions>",
                active,
                0 as "last_week_visits!"
            FROM access_tokens
            WHERE token = $1
            "#,
            given_token
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_with_id(
        given_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        sqlx::query_as!(
            AccessToken,
            r#"
            SELECT 
                id,
                title,
                token,
                permissions as "permissions: Json<Permissions>",
                active,
                (SELECT COUNT(*) FROM access_tokens_visits WHERE token_id = id AND visited_at > NOW() - INTERVAL '1 week') as "last_week_visits!"
            FROM access_tokens
            WHERE id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<AccessToken>, AppError> {
        sqlx::query_as!(
            AccessToken,
            r#"
            SELECT
                id,
                title,
                token,
                permissions as "permissions: Json<Permissions>",
                active,
                (SELECT COUNT(*) FROM access_tokens_visits WHERE token_id = id AND visited_at > NOW() - INTERVAL '1 week') as "last_week_visits!"
            FROM access_tokens
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn register_visit(
        access_token_id: Uuid,
        referrer: Option<String>,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO access_tokens_visits (token_id, referrer)
            VALUES ($1, $2)
            "#,
            access_token_id,
            referrer
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }
}
