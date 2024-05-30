use sqlx::PgConnection;
use uuid::Uuid;

use crate::api::AppError;

pub struct Icon {
    pub data: Vec<u8>,
    pub http_mime_type: String,
}

impl Icon {
    pub async fn get(hash: String, conn: &mut PgConnection) -> Result<Icon, AppError> {
        let icon = sqlx::query_as!(
            Icon,
            r#"SELECT data, http_mime_type FROM icons WHERE hash = $1"#,
            hash
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(icon)
    }

    pub async fn delete_for_family(
        family_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"DELETE FROM icons WHERE id = (SELECT icon_id FROM families WHERE id = $1)"#,
            family_id
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn delete_for_category(
        category_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"DELETE FROM icons WHERE id = (SELECT icon_id FROM categories WHERE id = $1)"#,
            category_id
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn upsert_family(
        family_id: Uuid,
        data: Vec<u8>,
        http_mime_type: String,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"SELECT upsert_row_icon($1, $2, $3, 'families')"#,
            family_id,
            data,
            http_mime_type
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn upsert_category(
        category_id: Uuid,
        data: Vec<u8>,
        http_mime_type: String,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"SELECT upsert_row_icon($1, $2, $3, 'categories')"#,
            category_id,
            data,
            http_mime_type
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }
}
