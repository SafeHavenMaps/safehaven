use std::collections::HashMap;

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

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct CachedClusteredEntity {
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
    pub cluster_id: Option<i32>,
    pub cluster_center_lat: Option<f64>,
    pub cluster_center_lon: Option<f64>
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Cluster {
    pub id: i32,
    pub center_lat: f64,
    pub center_lon: f64,
    pub count: i32,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct EntitiesAndClusters {
    pub entities: Vec<CachedEntity>,
    pub clusters: Vec<Cluster>,
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

    pub exclude_families_list: Vec<Uuid>,
    pub exclude_categories_list: Vec<Uuid>,
    pub exclude_tags_list: Vec<Uuid>,

    pub cluster_eps: f64,
    pub cluster_min_points: i32,
}

pub struct SearchEntitiesRequest {
    pub search_query: String,

    pub allow_all_families: bool,
    pub allow_all_categories: bool,
    pub allow_all_tags: bool,

    pub families_list: Vec<Uuid>,
    pub categories_list: Vec<Uuid>,
    pub tags_list: Vec<Uuid>,

    pub exclude_families_list: Vec<Uuid>,
    pub exclude_categories_list: Vec<Uuid>,
    pub exclude_tags_list: Vec<Uuid>,
}


impl CachedEntity {
    pub async fn find_entities_in_rectangle(
        request: FindEntitiesRequest,
        conn: &mut PgConnection,
    ) -> Result<EntitiesAndClusters, AppError> {
        let entities_with_clusters = query_as!(
            CachedClusteredEntity,
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
                plain_text_location as "plain_text_location!",
                cluster_id,
                cluster_center_lat,
                cluster_center_lon
            FROM fetch_entities_within_view($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
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
            &request.exclude_families_list,
            &request.exclude_categories_list,
            &request.exclude_tags_list,
            request.cluster_eps,
            request.cluster_min_points,
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)?;

        // If the entity is not clustered, it will have a None cluster_id.
        // We isolate entities without clusters and put them in a separate vector.
        // Then we create a vector of clusters and put everyting in a EntitiesAndClusters struct

        // -- Creating the list of entities
        let entities = entities_with_clusters
            .iter()
            .filter(|e| e.cluster_id.is_none())
            .map(|e| CachedEntity {
                id: e.id,
                entity_id: e.entity_id,
                category_id: e.category_id,
                categories_ids: e.categories_ids.clone(),
                tags_ids: e.tags_ids.clone(),
                family_id: e.family_id,
                display_name: e.display_name.clone(),
                latitude: e.latitude,
                longitude: e.longitude,
                plain_text_location: e.plain_text_location.clone(),
            })
            .collect();
        
        // -- Creating the list of clusters
        let clusters = entities_with_clusters
            .iter()
            .fold(HashMap::new(), |mut acc: std::collections::HashMap<i32, Cluster>, e| {
                if let Some(cluster_id) = e.cluster_id {
                    let cluster = acc.entry(cluster_id).or_insert(Cluster {
                        id: cluster_id,
                        center_lat: e.cluster_center_lat.unwrap(),
                        center_lon: e.cluster_center_lon.unwrap(),
                        count: 0,
                    });
                    cluster.count += 1;
                }
                acc
            })
            .into_iter()
            .map(|(_, v)| v)
            .collect();
        
        Ok(EntitiesAndClusters {
            entities,
            clusters,
        })
    }

    pub async fn search_entities(
        request: SearchEntitiesRequest,
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, AppError> {
        query_as!(
            Self,
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
            FROM search_entities($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#,
            request.search_query,
            request.allow_all_families,
            request.allow_all_categories,
            request.allow_all_tags,
            &request.families_list,
            &request.categories_list,
            &request.tags_list,
            &request.exclude_families_list,
            &request.exclude_categories_list,
            &request.exclude_tags_list,
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }
}
