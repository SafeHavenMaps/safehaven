use crate::api::AppError;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

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
        query_as!(
            CachedEntity,
            r#"
            SELECT
                id as "id!",
                entity_id as "entity_id!",
                category_id as "category_id!",
                categories_ids as "categories_ids!",
                tags_ids as "tags_ids!",
                family_id as "family_id!",
                display_name as "display_name!",
                latitude as "latitude!",
                longitude as "longitude!",
                plain_text_location as "plain_text_location!"
            FROM fetch_entities_within_view($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            "#,
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
        .map_err(AppError::DatabaseError)
    }

    pub async fn search_entities(
        request: SearchEntitiesRequest,
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, AppError> {
        query_as!(
            CachedEntity,
            r#"
            SELECT 
                id as "id!",
                entity_id as "entity_id!",
                category_id as "category_id!",
                categories_ids as "categories_ids!",
                tags_ids as "tags_ids!",
                family_id as "family_id!",
                display_name as "display_name!",
                latitude as "latitude!",
                longitude as "longitude!",
                plain_text_location as "plain_text_location!"
            FROM search_entities($1,$2,$3,$4,$5,$6,$7)"#,
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
        .map_err(AppError::DatabaseError)
    }
}
