use crate::api::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewCategory {
    pub title: String,
    pub family_id: Uuid,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct UpdateCategory {
    pub title: String,
    pub family_id: Uuid,
    pub default_status: bool,
    pub icon: Option<String>,
    pub fill_color: String,
    pub border_color: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Category {
    pub id: Uuid,
    pub title: String,
    pub family_id: Uuid,
    pub default_status: bool,
    pub icon: Option<String>,
    pub fill_color: String,
    pub border_color: String,
}

impl Category {
    pub async fn new(category: NewCategory, conn: &mut PgConnection) -> Result<Category, AppError> {
        sqlx::query_as!(
            Category,
            r#"
            INSERT INTO categories (title, family_id)
            VALUES ($1, $2)
            RETURNING id, title, family_id, default_status, icon, fill_color, border_color
            "#,
            category.title,
            category.family_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<Category, AppError> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, title, family_id, default_status, icon, fill_color, border_color
            FROM categories
            WHERE id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update(
        id: Uuid,
        update: UpdateCategory,
        conn: &mut PgConnection,
    ) -> Result<Category, AppError> {
        sqlx::query_as!(
            Category,
            r#"
            UPDATE categories
            SET title = $2, family_id = $3, default_status = $4, icon = $5, fill_color = $6, border_color = $7
            WHERE id = $1
            RETURNING id, title, family_id, default_status, icon, fill_color, border_color
            "#,
            id,
            update.title,
            update.family_id,
            update.default_status,
            update.icon,
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
            DELETE FROM categories
            WHERE id = $1
            "#,
            given_id
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<Category>, AppError> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, title, family_id, default_status, icon, fill_color, border_color
            FROM categories
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list_with_families(families: Vec<Uuid>, conn: &mut PgConnection) -> Result<Vec<Category>, AppError> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, title, family_id, default_status, icon, fill_color, border_color
            FROM categories
            WHERE family_id = ANY($1)
            "#,
            &families
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list_restricted(
        ids: Vec<Uuid>,
        families: Vec<Uuid>,
        conn: &mut PgConnection,
    ) -> Result<Vec<Category>, AppError> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, title, family_id, default_status, icon, fill_color, border_color
            FROM categories
            WHERE id = ANY($1) AND family_id = ANY($2)
            "#,
            &ids,
            &families
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }
}
