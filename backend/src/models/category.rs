use crate::api::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewOrUpdateCategory {
    pub title: String,
    pub family_id: Uuid,
    pub default_status: bool,
    pub fill_color: String,
    pub border_color: String,
    pub version: Option<i32>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Category {
    pub id: Uuid,
    pub title: String,
    pub family_id: Uuid,
    pub default_status: bool,
    pub icon_hash: Option<String>,
    pub fill_color: String,
    pub border_color: String,
    pub version: i32,
}

impl Category {
    pub async fn new(
        category: NewOrUpdateCategory,
        conn: &mut PgConnection,
    ) -> Result<Category, AppError> {
        sqlx::query_as!(
            Category,
            r#"
            INSERT INTO categories (title, family_id, default_status, fill_color, border_color)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                title,
                family_id,
                default_status,
                (SELECT hash FROM icons WHERE id = icon_id) as icon_hash,
                fill_color,
                border_color,
                version
            "#,
            category.title,
            category.family_id,
            category.default_status,
            category.fill_color,
            category.border_color
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<Category, AppError> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT
                id,
                title,
                family_id,
                default_status,
                (SELECT hash FROM icons WHERE id = icon_id) as icon_hash,
                fill_color,
                border_color,
                version
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
        update: NewOrUpdateCategory,
        conn: &mut PgConnection,
    ) -> Result<Category, AppError> {
        if update.version.is_none() {
            return Err(AppError::Validation("Version is required".to_string()));
        }

        sqlx::query_as!(
            Category,
            r#"
            UPDATE categories
            SET title = $2, family_id = $3, default_status = $4, fill_color = $5, border_color = $6, version = $7
            WHERE id = $1
            RETURNING
                id,
                title,
                family_id,
                default_status,
                (SELECT hash FROM icons WHERE id = icon_id) as icon_hash,
                fill_color,
                border_color,
                version
            "#,
            id,
            update.title,
            update.family_id,
            update.default_status,
            update.fill_color,
            update.border_color,
            update.version
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
            SELECT
                id,
                title,
                family_id,
                default_status,
                (SELECT hash FROM icons WHERE id = icon_id) as icon_hash,
                fill_color,
                border_color,
                version
            FROM categories
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list_except_with_families(
        excluded_categories: &Vec<Uuid>,
        families: Vec<Uuid>,
        conn: &mut PgConnection,
    ) -> Result<Vec<Category>, AppError> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT
                id,
                title,
                family_id,
                default_status,
                (SELECT hash FROM icons WHERE id = icon_id) as icon_hash,
                fill_color,
                border_color,
                version
            FROM categories
            WHERE NOT (id = ANY($1)) AND family_id = ANY($2)
            "#,
            &excluded_categories,
            &families
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }
}
