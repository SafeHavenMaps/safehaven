use crate::api::AppError;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Serialize)]
pub struct RawCachedEntity {
    pub id: Option<Uuid>,
    pub entity_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub categories_ids: Option<Vec<Uuid>>,
    pub tags_ids: Option<Vec<Uuid>>,
    pub family_id: Option<Uuid>,
    pub display_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub plain_text_location: Option<String>,
}

impl TryInto<CachedEntity> for RawCachedEntity {
    type Error = sqlx::Error;

    // Error should be Error::ColumnNotFound("column_name")
    fn try_into(self) -> Result<CachedEntity, Self::Error> {
        Ok(CachedEntity {
            id: self
                .id
                .ok_or(sqlx::Error::ColumnNotFound("id".to_string()))?,
            entity_id: self
                .entity_id
                .ok_or(sqlx::Error::ColumnNotFound("entity_id".to_string()))?,
            category_id: self
                .category_id
                .ok_or(sqlx::Error::ColumnNotFound("category_id".to_string()))?,
            categories_ids: self
                .categories_ids
                .ok_or(sqlx::Error::ColumnNotFound("categories_ids".to_string()))?,
            tags_ids: self
                .tags_ids
                .ok_or(sqlx::Error::ColumnNotFound("tags_ids".to_string()))?,
            family_id: self
                .family_id
                .ok_or(sqlx::Error::ColumnNotFound("family_id".to_string()))?,
            display_name: self
                .display_name
                .ok_or(sqlx::Error::ColumnNotFound("display_name".to_string()))?,
            latitude: self
                .latitude
                .ok_or(sqlx::Error::ColumnNotFound("latitude".to_string()))?,
            longitude: self
                .longitude
                .ok_or(sqlx::Error::ColumnNotFound("longitude".to_string()))?,
            plain_text_location: self.plain_text_location.ok_or(sqlx::Error::ColumnNotFound(
                "plain_text_location".to_string(),
            ))?,
        })
    }
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct CachedEntity {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub category_id: Uuid,
    pub categories_ids: Vec<Uuid>,
    pub tags_ids: Vec<Uuid>,
    pub family_id: Uuid,
    pub display_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub plain_text_location: String,
}

pub struct FindEntitiesRequest {
    pub upper_left_lat: f64,
    pub upper_left_lon: f64,
    pub lower_right_lat: f64,
    pub lower_right_lon: f64,
    pub allow_all_families: bool,
    pub allow_all_categories: bool,
    pub allow_all_tags: bool,
    pub families_list: Vec<Uuid>,
    pub categories_list: Vec<Uuid>,
    pub tags_list: Vec<Uuid>,
}

pub struct SearchEntitiesRequest {
    pub search_query: String,
    pub allow_all_families: bool,
    pub allow_all_categories: bool,
    pub allow_all_tags: bool,
    pub families_list: Vec<Uuid>,
    pub categories_list: Vec<Uuid>,
    pub tags_list: Vec<Uuid>,
}

impl CachedEntity {
    pub async fn find_entities_in_rectangle(
        request: FindEntitiesRequest,
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, AppError> {
        let raw_entities = query_as!(
            RawCachedEntity,
            r#"SELECT * FROM fetch_entities_within_view($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#,
            request.upper_left_lat,
            request.upper_left_lon,
            request.lower_right_lat,
            request.lower_right_lon,
            request.allow_all_families,
            request.allow_all_categories,
            request.allow_all_tags,
            &request.families_list,
            &request.categories_list,
            &request.tags_list,
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)?;

        let entities: Result<Vec<CachedEntity>, sqlx::Error> = raw_entities
            .into_iter()
            .map(|raw_entity| raw_entity.try_into())
            .collect();

        entities.map_err(AppError::DatabaseError)
    }

    pub async fn search_entities(
        request: SearchEntitiesRequest,
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, AppError> {
        let raw_entities = query_as!(
            RawCachedEntity,
            r#"SELECT * FROM search_entities($1,$2,$3,$4,$5,$6,$7)"#,
            request.search_query,
            request.allow_all_families,
            request.allow_all_categories,
            request.allow_all_tags,
            &request.families_list,
            &request.categories_list,
            &request.tags_list,
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::DatabaseError)?;

        let entities: Result<Vec<CachedEntity>, sqlx::Error> = raw_entities
            .into_iter()
            .map(|raw_entity| raw_entity.try_into())
            .collect();

        entities.map_err(AppError::DatabaseError)
    }
}
