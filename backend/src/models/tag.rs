use crate::api::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Tag {
    pub id: Uuid,
    pub title: String,
    pub is_filter: bool,
    pub is_primary_filter: bool,
    pub default_filter_status: bool,
    pub filter_description: Option<String>,
    pub fill_color: String,
    pub border_color: String,
    pub version: i32,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewOrUpdateTag {
    pub title: String,
    pub is_filter: bool,
    pub is_primary_filter: bool,
    pub filter_description: Option<String>,
    pub default_filter_status: bool,
    pub version: Option<i32>,
    pub fill_color: String,
    pub border_color: String,
}

impl Tag {
    pub async fn new(tag: NewOrUpdateTag, conn: &mut PgConnection) -> Result<Tag, AppError> {
        sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO tags (title, is_filter, is_primary_filter, 
                filter_description, default_filter_status, fill_color, border_color)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, title, is_filter, is_primary_filter, filter_description,
                default_filter_status, version, fill_color, border_color
            "#,
            tag.title,
            tag.is_filter,
            tag.is_primary_filter,
            tag.filter_description,
            tag.default_filter_status,
            tag.fill_color,
            tag.border_color,
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update(
        given_id: Uuid,
        update: NewOrUpdateTag,
        conn: &mut PgConnection,
    ) -> Result<Tag, AppError> {
        // Check if the version is provided
        if update.version.is_none() {
            return Err(AppError::Validation("Version is required".to_string()));
        }

        sqlx::query_as!(
            Tag,
            r#"
            UPDATE tags
            SET title = $2, is_filter = $3, is_primary_filter = $4, filter_description = $5, 
                default_filter_status = $6, version = $7, fill_color = $8, border_color = $9
            WHERE id = $1
            RETURNING id, title, is_filter, is_primary_filter, filter_description, 
                default_filter_status, version, fill_color, border_color
            "#,
            given_id,
            update.title,
            update.is_filter,
            update.is_primary_filter,
            update.filter_description,
            update.default_filter_status,
            update.version,
            update.fill_color,
            update.border_color
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn delete(given_id: Uuid, conn: &mut PgConnection) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM tags
            WHERE id = $1
            "#,
            given_id
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<Tag, AppError> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, title, is_filter, is_primary_filter, filter_description, 
                default_filter_status, version, fill_color, border_color
            FROM tags
            WHERE id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<Tag>, AppError> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, title, is_filter, is_primary_filter, filter_description,
                default_filter_status, version, fill_color, border_color
            FROM tags
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list_except(
        excluded_tags: &Vec<Uuid>,
        conn: &mut PgConnection,
    ) -> Result<Vec<Tag>, AppError> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, title, is_filter, is_primary_filter, filter_description,
                default_filter_status, version, fill_color, border_color
            FROM tags
            WHERE NOT (id = ANY($1))
            "#,
            &excluded_tags
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }
}
