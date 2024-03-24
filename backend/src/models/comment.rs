use super::family::Family;
use crate::api::AppError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewComment {
    pub entity_id: Uuid,
    pub author: String,
    pub text: String,
    pub data: Value,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct ListedComment {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub entity_display_name: String,
    pub entity_category_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct PublicComment {
    pub id: Uuid,
    pub author: String,
    pub text: String,
    pub data: Value,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct Comment {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub author: String,
    pub text: String,
    pub data: Value,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub moderated_at: Option<chrono::NaiveDateTime>,
    pub moderated_by: Option<Uuid>,
}

#[derive(Deserialize, Serialize, ToSchema, Default)]
pub struct UpdateComment {
    pub entity_id: Uuid,
    pub author: String,
    pub text: String,
    pub data: Value,
    pub moderated_at: Option<chrono::NaiveDateTime>,
    pub moderated_by: Option<Uuid>,
}

impl Comment {
    pub async fn new(comment: NewComment, conn: &mut PgConnection) -> Result<Comment, AppError> {
        let family = Family::get_from_entity(comment.entity_id, conn).await?;
        family.comment_form.validate_data(comment.data.clone())?;

        sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (entity_id, author, text, data)
            VALUES ($1, $2, $3, $4)
            RETURNING id, entity_id, author, text, data, created_at, updated_at, moderated_at, moderated_by
            "#,
            comment.entity_id,
            comment.author,
            comment.text,
            comment.data
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn update(
        id: Uuid,
        update: UpdateComment,
        conn: &mut PgConnection,
    ) -> Result<Comment, AppError> {
        let family = Family::get_from_entity(update.entity_id, conn).await?;
        family.comment_form.validate_data(update.data.clone())?;

        sqlx::query_as!(
            Comment,
            r#"
            UPDATE comments
            SET entity_id = $2, author = $3, text = $4, data = $5, moderated_at = $6, moderated_by = $7
            WHERE id = $1
            RETURNING id, entity_id, author, text, data, created_at, updated_at, moderated_at, moderated_by
            "#,
            id,
            update.entity_id,
            update.author,
            update.text,
            update.data,
            update.moderated_at,
            update.moderated_by
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<Comment, AppError> {
        sqlx::query_as!(
            Comment,
            r#"
            SELECT id, entity_id, author, text, data, created_at, updated_at, moderated_at, moderated_by
            FROM comments
            WHERE id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn delete(given_id: Uuid, conn: &mut PgConnection) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM comments
            WHERE id = $1
            "#,
            given_id
        )
        .execute(conn)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    pub async fn list_for_public_entity(
        given_entity_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Vec<PublicComment>, AppError> {
        sqlx::query_as!(
            PublicComment,
            r#"
            SELECT c.id, c.author, c.text, c.data, c.created_at, c.updated_at
            FROM comments c
            INNER JOIN entities e ON c.entity_id = e.id
            WHERE c.entity_id = $1
            ORDER BY created_at
            "#,
            given_entity_id
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn list_for_entity(
        given_entity_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Vec<ListedComment>, AppError> {
        sqlx::query_as!(
            ListedComment,
            r#"
            SELECT c.id, c.entity_id, e.display_name as entity_display_name, e.category_id as entity_category_id, c.created_at
            FROM comments c
            INNER JOIN entities e ON c.entity_id = e.id
            WHERE c.entity_id = $1
            ORDER BY created_at
            "#,
            given_entity_id
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn pending(conn: &mut PgConnection) -> Result<Vec<ListedComment>, AppError> {
        sqlx::query_as!(
            ListedComment,
            r#"
            SELECT c.id, c.entity_id, e.display_name as entity_display_name, e.category_id as entity_category_id, c.created_at
            FROM comments c
            INNER JOIN entities e ON c.entity_id = e.id
            WHERE c.moderated_at IS NULL
            ORDER BY created_at
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }
}
