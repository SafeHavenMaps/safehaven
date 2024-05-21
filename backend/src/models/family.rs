use crate::api::AppError;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use sqlx::{types::Json, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Form {
    pub title: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum FieldType {
    SingleLineText,
    MultiLineText,
    Number,
    Boolean,
    DiscreteScore,
    Date,
    EnumSingleOption,
    EnumMultiOption,
    EventList,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Field {
    /// The key is used to identify the field in the data object
    pub key: String,

    /// The display name is used to show the field in the UI
    pub display_name: String,

    /// The help text is used to show additional information about the field in the UI
    pub help: Option<String>,

    /// The type of the field
    pub field_type: FieldType,

    /// Used to store detail about the field that relevent
    /// only for the frontend. For instance, if the field is an enum
    /// use it to store possible values. If it is a SingleLineText, specify
    /// if it's an email, a phone number, etc...
    pub field_type_metadata: Option<Value>,

    /// Sets if the field is indexed (used in full text search)
    pub indexed: bool,

    /// Sets if the field is mandatory
    pub mandatory: bool,

    /// Sets if the field is displayed to the final user
    pub user_facing: bool,

    /// Form page number for the given field
    pub form_page: u8,

    /// The weight of the field in the form (when displayed, ordered by weight)
    pub form_weight: u8,

    /// The weight of the field in the display (when displayed, ordered by weight)
    pub display_weight: u8,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Family {
    pub id: Uuid,
    pub title: String,
    pub icon: Option<String>,
    pub entity_form: Json<Form>,
    pub comment_form: Json<Form>,
    pub sort_order: i32,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewOrUpdateFamily {
    pub title: String,
    pub entity_form: Form,
    pub comment_form: Form,
    pub icon: Option<String>,
    pub sort_order: i32,
}

impl Form {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.title.is_empty() {
            return Err(AppError::Validation("Title cannot be empty".to_string()));
        }

        if self.fields.is_empty() {
            return Err(AppError::Validation(
                "Form must have at least one field".to_string(),
            ));
        }

        let mut keys = Vec::new();
        for field in self.fields.iter() {
            if keys.contains(&field.key) {
                return Err(AppError::Validation(format!(
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
                    AppError::Validation(format!("Mandatory field {} is missing", field.key))
                })?;
            }
        }
        Ok(())
    }
}

impl Field {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.key.is_empty() {
            return Err(AppError::Validation("Key cannot be empty".to_string()));
        }

        if self.display_name.is_empty() {
            return Err(AppError::Validation(
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
            INSERT INTO families (title, entity_form, comment_form, icon, sort_order)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING 
                id,
                title,
                icon,
                entity_form as "entity_form: Json<Form>",
                comment_form as "comment_form: Json<Form>",
                sort_order
            "#,
            family.title,
            entity_form,
            comment_form,
            family.icon,
            family.sort_order
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
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
            SET title = $2, entity_form = $3, comment_form = $4, icon = $5, sort_order = $6
            WHERE id = $1
            RETURNING 
                id,
                title,
                icon,
                entity_form as "entity_form: Json<Form>",
                comment_form as "comment_form: Json<Form>",
                sort_order
            "#,
            id,
            update.title,
            entity_form,
            comment_form,
            update.icon,
            update.sort_order
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
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
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<Family, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT id, title, icon, 
                entity_form as "entity_form: Json<Form>", 
                comment_form as "comment_form: Json<Form>",
                sort_order
            FROM families
            WHERE id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<Family>, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT id, title, icon,
                entity_form as "entity_form: Json<Form>", 
                comment_form as "comment_form: Json<Form>",
                sort_order
            FROM families
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list_restricted(
        ids: Vec<Uuid>,
        conn: &mut PgConnection,
    ) -> Result<Vec<Family>, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT id, title, icon,
                entity_form as "entity_form: Json<Form>", 
                comment_form as "comment_form: Json<Form>",
                sort_order
            FROM families
            WHERE id = ANY($1)
            "#,
            &ids
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_from_category(
        category_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Family, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT families.id, families.title, families.icon,
                families.entity_form as "entity_form: Json<Form>", 
                families.comment_form as "comment_form: Json<Form>",
                families.sort_order
            FROM families
            JOIN categories ON families.id = categories.family_id
            WHERE categories.id = $1
            "#,
            category_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_from_entity(
        entity_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<Family, AppError> {
        sqlx::query_as!(
            Family,
            r#"
            SELECT families.id, families.title, families.icon,
                families.entity_form as "entity_form: Json<Form>", 
                families.comment_form as "comment_form: Json<Form>",
                families.sort_order
            FROM families
            JOIN categories ON families.id = categories.family_id
            JOIN entities ON categories.id = entities.category_id
            WHERE entities.id = $1
            "#,
            entity_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }
}
