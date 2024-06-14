use crate::api::AppError;
use crate::models::family::Family;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use sqlx::{types::Json, Acquire, FromRow, PgConnection, Postgres, Transaction};
use utoipa::ToSchema;
use uuid::Uuid;

use super::family::Form;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UnprocessedLocation {
    pub plain_text: String,
    pub lat: f64,
    pub long: f64,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct PublicNewEntity {
    pub display_name: String,
    pub category_id: Uuid,
    pub locations: Vec<UnprocessedLocation>,
    pub data: Value,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct PublicListedEntity {
    pub id: Uuid,
    pub display_name: String,
    pub category_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct PublicEntity {
    pub id: Uuid,
    pub display_name: String,
    pub category_id: Uuid,
    pub family_id: Uuid,
    pub locations: Json<Vec<UnprocessedLocation>>,
    pub data: Value,
    pub tags: Vec<Uuid>,
    pub entity_form: Json<Form>,
    pub comment_form: Json<Form>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl PublicEntity {
    /// Remove all data that is not user_facing from the data object using the entity_form
    fn cleanup_data(&mut self) {
        let data = self.data.as_object_mut().expect("data is not an object");
        let non_user_facing_fields: Vec<String> = self
            .entity_form
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
        entity: PublicNewEntity,
        conn: &mut PgConnection,
    ) -> Result<PublicEntity, AppError> {
        let family = Family::get_from_category(entity.category_id, conn).await?;
        family.entity_form.validate_data(entity.data.clone())?;

        let locations = to_value(entity.locations).unwrap();

        sqlx::query_as!(
            PublicEntity,
            r#"
            WITH inserted AS (
                INSERT INTO entities (display_name, category_id, locations, data)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            ) 
            SELECT 
                i.id, 
                i.category_id, 
                i.display_name, 
                i.data,
                i.locations AS "locations: Json<Vec<UnprocessedLocation>>",
                i.created_at,
                i.updated_at,
                array[]::uuid[] AS "tags!", 
                c.family_id,
                f.entity_form AS "entity_form: Json<Form>", 
                f.comment_form AS "comment_form: Json<Form>"
            FROM inserted i
            JOIN categories c ON c.id = i.category_id
            JOIN families f ON f.id = c.family_id
            "#,
            entity.display_name,
            entity.category_id,
            locations,
            entity.data,
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<PublicEntity, AppError> {
        let mut public_entity = sqlx::query_as!(
            PublicEntity,
            r#"
            SELECT e.id, c.family_id, e.category_id, e.display_name, e.data, e.created_at, e.updated_at,
                e.locations as "locations: Json<Vec<UnprocessedLocation>>",
                COALESCE(
                    (SELECT array_agg(t.tag_id) FROM entity_tags t WHERE t.entity_id = e.id), 
                    array[]::uuid[]
                ) as "tags!",
                f.entity_form as "entity_form: Json<Form>",
                f.comment_form as "comment_form: Json<Form>"
            FROM entities e
            INNER JOIN categories c ON e.category_id = c.id
            INNER JOIN families f ON c.family_id = f.id
            WHERE e.id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)?;

        public_entity.cleanup_data();
        Ok(public_entity)
    }

    pub async fn get_children(
        given_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Vec<PublicListedEntity>, AppError> {
        sqlx::query_as!(
            PublicListedEntity,
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
        .map_err(AppError::Database)
    }

    pub async fn get_parents(
        given_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Vec<PublicListedEntity>, AppError> {
        sqlx::query_as!(
            PublicListedEntity,
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
        .map_err(AppError::Database)
    }
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct AdminNewOrUpdateEntity {
    pub display_name: String,
    pub category_id: Uuid,
    pub locations: Json<Vec<UnprocessedLocation>>,
    pub data: Value,
    pub tags: Vec<Uuid>,
    pub hidden: bool,
    pub moderation_notes: Option<String>,
    pub moderated: bool,
    pub version: i32,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct AdminListedEntity {
    pub id: Uuid,
    pub display_name: String,
    pub category_id: Uuid,
    pub tags: Vec<Uuid>,
    pub hidden: bool,
    pub moderation_notes: Option<String>,
    pub moderated: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct AdminEntity {
    pub id: Uuid,
    pub display_name: String,
    pub category_id: Uuid,
    pub family_id: Uuid,
    pub locations: Json<Vec<UnprocessedLocation>>,
    pub data: Value,
    pub tags: Vec<Uuid>,
    pub hidden: bool,
    pub moderation_notes: Option<String>,
    pub moderated: bool,
    pub version: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl AdminEntity {
    pub async fn new(
        new_entity: AdminNewOrUpdateEntity,
        conn: &mut PgConnection,
    ) -> Result<AdminEntity, AppError> {
        // Start a database transaction
        let mut tx: Transaction<'_, Postgres> = conn.begin().await.map_err(AppError::Database)?;

        // Validate the new data against the form from the corresponding family
        let family = Family::get_from_category(new_entity.category_id, &mut tx).await?;
        family.entity_form.validate_data(new_entity.data.clone())?;

        // Serialize locations to JSON
        let locations = to_value(new_entity.locations).unwrap();

        // Insert the new entity using a CTE (WITH clause) and fetch the result
        let mut created_entity = sqlx::query_as!(
            AdminEntity,
            r#"
            WITH inserted AS (
                INSERT INTO entities (display_name, category_id, locations, data, hidden, moderation_notes, moderated)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
            )
            SELECT 
                i.id,
                i.display_name,
                i.category_id,
                i.locations AS "locations: Json<Vec<UnprocessedLocation>>",
                i.data,
                i.hidden,
                i.moderation_notes,
                i.moderated,
                i.created_at,
                i.updated_at,
                i.version,
                c.family_id,
                COALESCE(array(
                    SELECT tag_id
                    FROM entity_tags
                    WHERE entity_id = i.id
                ), array[]::uuid[]) AS "tags!"
            FROM inserted i
            JOIN categories c ON c.id = i.category_id
            "#,
            new_entity.display_name,
            new_entity.category_id,
            locations,
            new_entity.data,
            new_entity.hidden,
            new_entity.moderation_notes,
            new_entity.moderated
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        // Handle the many-to-many relationship for tags
        sqlx::query!(
            r#"
                SELECT replace_tags_for_entity($1, $2)
                "#,
            created_entity.id,
            &new_entity.tags
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        // Commit the transaction if all operations succeeded
        tx.commit().await.map_err(AppError::Database)?;

        created_entity.tags = new_entity.tags;
        Ok(created_entity)
    }

    pub async fn update(
        id: Uuid,
        update: AdminNewOrUpdateEntity,
        conn: &mut PgConnection,
    ) -> Result<AdminEntity, AppError> {
        // Start a database transaction using the Acquire trait
        let mut tx: Transaction<'_, Postgres> = conn.begin().await.map_err(AppError::Database)?;

        // Validate the new data against the form from the corresponding family
        let family = Family::get_from_category(update.category_id, &mut tx).await?;
        family.entity_form.validate_data(update.data.clone())?;

        // Serialize locations to JSON
        let locations = to_value(update.locations).unwrap();

        // Handle the many-to-many relationship for tags
        sqlx::query!(
            r#"
            SELECT replace_tags_for_entity($1, $2)
            "#,
            id,
            &update.tags
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        // Update the entity itselfand return the updated record using a CTE
        let updated_entity = sqlx::query_as!(
            AdminEntity,
            r#"
            WITH updated AS (
                UPDATE entities
                SET 
                    display_name = $2, 
                    category_id = $3, 
                    locations = $4, 
                    data = $5, 
                    hidden = $6, 
                    moderation_notes = $7, 
                    moderated = $8,
                    version = $9
                WHERE id = $1
                RETURNING *
            )
            SELECT 
                u.id,
                u.display_name,
                u.category_id,
                u.locations AS "locations: Json<Vec<UnprocessedLocation>>",
                u.data,
                u.hidden,
                u.moderation_notes,
                u.moderated,
                u.created_at,
                u.updated_at,
                u.version,
                c.family_id,
                COALESCE(array(
                    SELECT tag_id
                    FROM entity_tags
                    WHERE entity_id = u.id
                ), array[]::uuid[]) AS "tags!"
            FROM updated u
            JOIN categories c ON c.id = u.category_id
            "#,
            id,
            update.display_name,
            update.category_id,
            locations,
            update.data,
            update.hidden,
            update.moderation_notes,
            update.moderated,
            update.version
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        // Commit the transaction if all operations succeeded
        tx.commit().await.map_err(AppError::Database)?;

        Ok(updated_entity)
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
        .map_err(AppError::Database)?;

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
        .map_err(AppError::Database)?;

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
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<AdminEntity, AppError> {
        sqlx::query_as!(
            AdminEntity,
            r#"
            SELECT e.id, c.family_id, e.display_name, e.category_id, 
                e.locations as "locations: Json<Vec<UnprocessedLocation>>", 
                e.data, e.hidden, e.moderation_notes, e.moderated, 
                e.created_at, e.updated_at, e.version,
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
        .map_err(AppError::Database)
    }

    pub async fn pending(conn: &mut PgConnection) -> Result<Vec<AdminListedEntity>, AppError> {
        sqlx::query_as!(
            AdminListedEntity,
            r#"
            SELECT e.id, e.display_name, e.category_id, e.created_at, e.hidden,
                    e.moderation_notes, e.moderated, e.updated_at,
                    COALESCE(
                        (SELECT array_agg(t.tag_id) FROM entity_tags t WHERE t.entity_id = e.id), 
                        array[]::uuid[]
                    ) as "tags!"
            FROM entities e
            WHERE NOT e.moderated
            ORDER BY created_at
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }
}
