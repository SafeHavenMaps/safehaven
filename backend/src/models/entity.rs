use crate::api::AppError;
use crate::models::family::Family;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{types::Json, FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UnprocessedLocation {
    pub plain_text: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewEntity {
    display_name: String,
    category_id: Uuid,
    data: Value,
    last_update_by: Uuid,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct ListedEntity {
    pub id: Uuid,
    pub display_name: String,
    pub category_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct PublicEntity {
    pub id: Uuid,
    pub category_id: Uuid,
    pub family_id: Uuid,
    pub display_name: String,
    pub locations: Json<Vec<UnprocessedLocation>>,
    pub data: Value,
    pub tags: Vec<Uuid>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Entity {
    pub id: Uuid,
    pub display_name: String,
    pub category_id: Uuid,
    pub locations: Json<Vec<UnprocessedLocation>>,
    pub data: Value,
    pub hide_from_map: bool,
    pub moderation_notes: Option<String>,
    pub moderated_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub last_update_by: Option<Uuid>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct UpdateEntity {
    pub display_name: String,
    pub category_id: Uuid,
    pub data: Value,
    pub moderation_notes: Option<String>,

    pub last_update_by: Option<Uuid>,
}

#[derive(Debug)]
pub struct EntityEntity {
    pub parent_id: Uuid,
    pub child_id: Uuid,
}

impl Entity {
    pub async fn new(entity: NewEntity, conn: &mut PgConnection) -> Result<Entity, AppError> {
        let family = Family::get_from_category(entity.category_id, conn).await?;
        family.entity_form.validate_data(entity.data.clone())?;

        sqlx::query_as!(
            Entity,
            r#"
            INSERT INTO entities (display_name, category_id, data)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                display_name,
                category_id,
                locations as "locations: Json<Vec<UnprocessedLocation>>",
                data,
                hide_from_map,
                moderation_notes,
                moderated_at,
                created_at,
                updated_at,
                last_update_by
            "#,
            entity.display_name,
            entity.category_id,
            entity.data
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn update(
        id: Uuid,
        update: UpdateEntity,
        conn: &mut PgConnection,
    ) -> Result<Entity, AppError> {
        let family = Family::get_from_category(update.category_id, conn).await?;
        family.entity_form.validate_data(update.data.clone())?;

        sqlx::query_as!(
            Entity,
            r#"
            UPDATE entities
            SET display_name = $2, category_id = $3, data = $4, moderation_notes = $5, last_update_by = $6
            WHERE id = $1
            RETURNING
                id,
                display_name,
                category_id,
                locations as "locations: Json<Vec<UnprocessedLocation>>",
                data,
                hide_from_map,
                moderation_notes,
                moderated_at,
                created_at,
                updated_at,
                last_update_by
            "#,
            id,
            update.display_name,
            update.category_id,
            update.data,
            update.moderation_notes,
            update.last_update_by
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn delete(id: Uuid, conn: &mut PgConnection) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM entities
            WHERE id = $1
            "#,
            id
        )
        .execute(conn)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    pub async fn register_parent_child(
        parent_id: Uuid,
        child_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO entities_entities (parent_id, child_id)
            VALUES ($1, $2)
            "#,
            parent_id,
            child_id
        )
        .execute(conn)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    pub async fn delete_parent_child(
        parent_id: Uuid,
        child_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM entities_entities
            WHERE parent_id = $1 AND child_id = $2
            "#,
            parent_id,
            child_id
        )
        .execute(conn)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<Entity, AppError> {
        sqlx::query_as!(
            Entity,
            r#"
            SELECT e.id, e.display_name, e.category_id, 
            e.locations as "locations: Json<Vec<UnprocessedLocation>>", 
            e.data, e.hide_from_map, e.moderation_notes, e.moderated_at, 
            e.created_at, e.updated_at, e.last_update_by
            FROM entities e
            WHERE e.id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn get_public(
        given_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<PublicEntity, AppError> {
        sqlx::query_as!(
            PublicEntity,
            r#"
            SELECT e.id, c.family_id, e.category_id, e.display_name, e.data,
                e.locations as "locations: Json<Vec<UnprocessedLocation>>",
                COALESCE(
                    (SELECT array_agg(t.tag_id) FROM entity_tags t WHERE t.entity_id = e.id), 
                    array[]::uuid[]
                ) as "tags!"
            FROM entities e
            INNER JOIN categories c ON e.category_id = c.id
            WHERE e.id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn pending(conn: &mut PgConnection) -> Result<Vec<ListedEntity>, AppError> {
        sqlx::query_as!(
            ListedEntity,
            r#"
            SELECT e.id, e.display_name, e.category_id, e.created_at
            FROM entities e
            WHERE e.moderated_at IS NULL
            ORDER BY created_at
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn search(
        query: String,
        conn: &mut PgConnection,
    ) -> Result<Vec<ListedEntity>, AppError> {
        sqlx::query_as!(
            ListedEntity,
            r#"
            SELECT e.id, e.display_name, e.category_id, e.created_at
            FROM entities e
            WHERE e.full_text_search_ts @@ to_tsquery($1)
            ORDER BY created_at
            "#,
            query
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn get_children(
        given_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Vec<ListedEntity>, AppError> {
        sqlx::query_as!(
            ListedEntity,
            r#"
            SELECT e.id, e.display_name, e.category_id, e.created_at
            FROM entities e
            INNER JOIN entities_entities ee ON e.id = ee.child_id
            WHERE ee.parent_id = $1
            "#,
            given_id
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn get_parents(
        given_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Vec<ListedEntity>, AppError> {
        sqlx::query_as!(
            ListedEntity,
            r#"
            SELECT e.id, e.display_name, e.category_id, e.created_at
            FROM entities e
            INNER JOIN entities_entities ee ON e.id = ee.parent_id
            WHERE ee.child_id = $1
            "#,
            given_id
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }
}
