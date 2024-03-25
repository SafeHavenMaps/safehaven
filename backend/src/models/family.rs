use crate::api::AppError;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use sqlx::{types::Json, FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Form {
    pub title: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum FieldType {
    SingleLineText = 0,
    MultiLineText = 1,
    Number = 2,
    DiscreteScore = 3,
    Date = 4,
    EventList = 10,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Field {
    pub key: String,
    pub display_name: String,
    pub help: Option<String>,
    pub field_type: FieldType,
    pub indexed: bool,
    pub mandatory: bool,
    pub form_weight: u8,
    pub display_weight: u8,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct Family {
    pub id: Uuid,
    pub title: String,
    pub entity_form: Json<Form>,
    pub comment_form: Json<Form>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewOrUpdateFamily {
    pub title: String,
    pub entity_form: Form,
    pub comment_form: Form,
}

impl Form {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.title.is_empty() {
            return Err(AppError::ValidationError(
                "Title cannot be empty".to_string(),
            ));
        }

        if self.fields.is_empty() {
            return Err(AppError::ValidationError(
                "Form must have at least one field".to_string(),
            ));
        }

        let mut keys = Vec::new();
        for field in self.fields.iter() {
            if keys.contains(&field.key) {
                return Err(AppError::ValidationError(format!(
                    "Duplicate key: {}",
                    field.key
                )));
            }
            keys.push(field.key.clone());
            field.validate()?;
        }

        Ok(())
    }

    pub fn validate_data(&self, data: Value) -> Result<(), AppError> {
        for field in self.fields.iter() {
            if field.mandatory {
                data.get(&field.key).ok_or_else(|| {
                    AppError::ValidationError(format!("Mandatory field {} is missing", field.key))
                })?;
            }
        }
        Ok(())
    }
}

impl Field {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.key.is_empty() {
            return Err(AppError::ValidationError("Key cannot be empty".to_string()));
        }

        if self.display_name.is_empty() {
            return Err(AppError::ValidationError(
                "Display name cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl Family {
    pub async fn new(
        family: NewOrUpdateFamily,
        conn: &mut PgConnection,
    ) -> Result<Family, AppError> {
        family.entity_form.validate()?;
        family.comment_form.validate()?;

        let entity_form = to_value(family.entity_form).unwrap();
        let comment_form = to_value(family.comment_form).unwrap();

        sqlx::query_as!(
            Family,
            r#"
            INSERT INTO families (title, entity_form, comment_form)
            VALUES ($1, $2, $3)
            RETURNING 
                id,
                title,
                entity_form as "entity_form: Json<Form>",
                comment_form as "comment_form: Json<Form>"
            "#,
            family.title,
            entity_form,
            comment_form
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn update(
        id: Uuid,
        update: NewOrUpdateFamily,
        conn: &mut PgConnection,
    ) -> Result<Family, AppError> {
        update.entity_form.validate()?;
        update.comment_form.validate()?;

        let entity_form = to_value(update.entity_form).unwrap();
        let comment_form = to_value(update.comment_form).unwrap();

        sqlx::query_as!(
            Family,
            r#"
            UPDATE families
            SET title = $2, entity_form = $3, comment_form = $4
            WHERE id = $1
            RETURNING 
                id,
                title,
                entity_form as "entity_form: Json<Form>",
                comment_form as "comment_form: Json<Form>"
            "#,
            id,
            update.title,
            entity_form,
            comment_form
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn delete(given_id: Uuid, conn: &mut PgConnection) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM families
            WHERE id = $1
            "#,
            given_id
        )
        .execute(conn)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<Family, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT id, title, 
                entity_form as "entity_form: Json<Form>", 
                comment_form as "comment_form: Json<Form>"
            FROM families
            WHERE id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<Family>, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT id, title, 
                entity_form as "entity_form: Json<Form>", 
                comment_form as "comment_form: Json<Form>"
            FROM families
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn list_restricted(
        ids: Vec<Uuid>,
        conn: &mut PgConnection,
    ) -> Result<Vec<Family>, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT id, title, 
                entity_form as "entity_form: Json<Form>", 
                comment_form as "comment_form: Json<Form>"
            FROM families
            WHERE id = ANY($1)
            "#,
            &ids
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn get_from_category(
        category_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Family, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT families.id, families.title, 
                families.entity_form as "entity_form: Json<Form>", 
                families.comment_form as "comment_form: Json<Form>"
            FROM families
            JOIN categories ON families.id = categories.family_id
            WHERE categories.id = $1
            "#,
            category_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn get_from_entity(
        entity_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Family, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT families.id, families.title, 
                families.entity_form as "entity_form: Json<Form>", 
                families.comment_form as "comment_form: Json<Form>"
            FROM families
            JOIN categories ON families.id = categories.family_id
            JOIN entities ON categories.id = entities.category_id
            WHERE entities.id = $1
            "#,
            entity_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }
}
