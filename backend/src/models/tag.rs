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
    pub filter_description: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewOrUpdateTag {
    pub title: String,
    pub is_filter: bool,
    pub filter_description: Option<String>,
}

impl Tag {
    pub async fn new(tag: NewOrUpdateTag, conn: &mut PgConnection) -> Result<Tag, AppError> {
        sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO tags (title, is_filter, filter_description)
            VALUES ($1, $2, $3)
            RETURNING id, title, is_filter, filter_description
            "#,
            tag.title,
            tag.is_filter,
            tag.filter_description
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn update(
        given_id: Uuid,
        update: NewOrUpdateTag,
        conn: &mut PgConnection,
    ) -> Result<Tag, AppError> {
        sqlx::query_as!(
            Tag,
            r#"
            UPDATE tags
            SET title = $2, is_filter = $3, filter_description = $4
            WHERE id = $1
            RETURNING id, title, is_filter, filter_description
            "#,
            given_id,
            update.title,
            update.is_filter,
            update.filter_description
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
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
        .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<Tag, AppError> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, title, is_filter, filter_description
            FROM tags
            WHERE id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<Tag>, AppError> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, title, is_filter, filter_description
            FROM tags
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn list_restricted(
        ids: Vec<Uuid>,
        conn: &mut PgConnection,
    ) -> Result<Vec<Tag>, AppError> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, title, is_filter, filter_description
            FROM tags
            WHERE id = ANY($1)
            "#,
            &ids
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }
}
