use super::family::{Family, Form};
use crate::api::AppError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct PublicNewComment {
    pub entity_id: Uuid,
    pub author: String,
    pub text: String,
    pub data: Value,
    pub entity_category_id: Uuid,
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

impl PublicComment {
    /// Remove all data that is not user_facing from the data object using the comment_form
    pub fn cleanup_data(&mut self, comment_form: &Form) {
        let data = self.data.as_object_mut().expect("data is not an object");
        let non_user_facing_fields: Vec<String> = comment_form
            .fields
            .iter()
            .filter(|field| !field.user_facing)
            .map(|field| field.key.clone()) // Assuming each field has a 'name' attribute
            .collect();

        for field_name in non_user_facing_fields.iter() {
            data.remove(field_name);
        }
    }

    pub async fn new(
        comment: PublicNewComment,
        conn: &mut PgConnection,
    ) -> Result<PublicComment, AppError> {
        let family = Family::get_from_entity(comment.entity_id, conn).await?;
        family
            .comment_form
            .validate_data(&comment.data, comment.entity_category_id)?;

        sqlx::query_as!(
            PublicComment,
            r#"
            INSERT INTO comments (entity_id, author, text, data)
            VALUES ($1, $2, $3, $4)
            RETURNING id, author, text, data, created_at, updated_at
            "#,
            comment.entity_id,
            comment.author,
            comment.text,
            comment.data
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list_for_public_entity(
        given_entity_id: Uuid,
        comment_form: &Form,
        conn: &mut PgConnection,
    ) -> Result<Vec<PublicComment>, AppError> {
        let result = sqlx::query_as!(
            PublicComment,
            r#"
            SELECT id, author, text, data, created_at, updated_at
            FROM comments 
            WHERE entity_id = $1 AND moderated
            ORDER BY created_at
            "#,
            given_entity_id
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)?
        .into_iter()
        .map(|mut comment| {
            comment.cleanup_data(comment_form);
            comment
        })
        .collect();

        Ok(result)
    }
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct AdminComment {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub author: String,
    pub text: String,
    pub data: Value,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub moderated: bool,
    pub version: i32,
    pub entity_display_name: String,
    pub entity_category_id: Uuid,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct AdminListedComment {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub author: String,
    pub entity_display_name: String,
    pub entity_category_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub moderated: bool,
}

#[derive(Deserialize, Serialize, ToSchema, Default)]
pub struct AdminNewOrUpdateComment {
    pub entity_id: Uuid,
    pub author: String,
    pub text: String,
    pub data: Value,
    pub moderated: bool,
    pub version: i32,
    pub entity_category_id: Uuid,
}

impl AdminComment {
    pub async fn new(
        new_comment: AdminNewOrUpdateComment,
        conn: &mut PgConnection,
    ) -> Result<AdminComment, AppError> {
        let family = Family::get_from_entity(new_comment.entity_id, conn).await?;
        family
            .comment_form
            .validate_data(&new_comment.data, new_comment.entity_category_id)?;

        sqlx::query_as!(
            AdminComment,
            r#"
            WITH inserted AS (
                INSERT INTO comments (entity_id, author, text, data, moderated)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            )
            SELECT i.id, i.entity_id, i.author, i.text, i.data, i.created_at, i.updated_at, i.moderated, i.version, 
                display_name AS entity_display_name, category_id AS entity_category_id
            FROM inserted i
            JOIN entities e 
            ON e.id = entity_id
            "#,
            new_comment.entity_id,
            new_comment.author,
            new_comment.text,
            new_comment.data,
            new_comment.moderated
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update(
        id: Uuid,
        update: AdminNewOrUpdateComment,
        conn: &mut PgConnection,
    ) -> Result<AdminComment, AppError> {
        let family = Family::get_from_entity(update.entity_id, conn).await?;
        family
            .comment_form
            .validate_data(&update.data, update.entity_category_id)?;

        sqlx::query_as!(
            AdminComment,
            r#"
            WITH inserted AS (
                UPDATE comments
                SET 
                    entity_id = $2,
                    author = $3,
                    text = $4,
                    data = $5,
                    moderated = $6,
                    version = $7
                WHERE id = $1
                RETURNING *
            )
            SELECT i.id, i.entity_id, i.author, i.text, i.data, i.created_at, i.updated_at, i.moderated, i.version, 
                e.display_name AS entity_display_name, e.category_id AS entity_category_id
            FROM inserted i
            JOIN entities e 
            ON e.id = entity_id
            "#,
            id,
            update.entity_id,
            update.author,
            update.text,
            update.data,
            update.moderated,
            update.version
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<AdminComment, AppError> {
        sqlx::query_as!(
            AdminComment,
            r#"
            SELECT c.id, c.entity_id, c.author, c.text, c.data, c.created_at, c.updated_at, c.moderated, c.version,
                e.display_name AS entity_display_name, e.category_id AS entity_category_id
            FROM comments c
            JOIN entities e ON e.id = c.entity_id
            WHERE c.id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
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
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn list_for_entity(
        given_entity_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Vec<AdminComment>, AppError> {
        sqlx::query_as!(
            AdminComment,
            r#"
            SELECT c.id, c.entity_id, c.author, c.text, c.data, c.created_at, c.updated_at, c.moderated, c.version,
                e.display_name AS entity_display_name, e.category_id AS entity_category_id
            FROM comments c
            JOIN entities e ON e.id = c.entity_id
            WHERE entity_id = $1
            ORDER BY created_at
            "#,
            given_entity_id
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn pending(conn: &mut PgConnection) -> Result<Vec<AdminListedComment>, AppError> {
        sqlx::query_as!(
            AdminListedComment,
            r#"
            SELECT c.id, c.entity_id, e.display_name AS entity_display_name, e.category_id AS entity_category_id, c.created_at,
                c.author, c.moderated, c.updated_at
            FROM comments c
            INNER JOIN entities e ON c.entity_id = e.id
            WHERE NOT c.moderated
            ORDER BY created_at
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }
}
