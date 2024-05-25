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
    pub parent_id: Option<Uuid>,
    pub parent_display_name: Option<String>,
    pub web_mercator_x: Option<f64>,
    pub web_mercator_y: Option<f64>,
    pub plain_text_location: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PaginatedCachedEntity {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub category_id: Uuid,
    pub categories_ids: Vec<Uuid>,
    pub tags_ids: Vec<Uuid>,
    pub family_id: Uuid,
    pub display_name: String,
    pub parent_id: Option<Uuid>,
    pub parent_display_name: Option<String>,
    pub web_mercator_x: Option<f64>,
    pub web_mercator_y: Option<f64>,
    pub plain_text_location: Option<String>,
    pub total_results: i64,
    pub total_pages: i64,
    pub response_current_page: i64,
}

impl From<Vec<PaginatedCachedEntity>> for CachedEntitiesWithPagination {
    fn from(paginated_entities: Vec<PaginatedCachedEntity>) -> Self {
        let mut entities = Vec::new();
        let total_results = paginated_entities.first().map_or(0, |e| e.total_results);
        let total_pages = paginated_entities.first().map_or(0, |e| e.total_pages);
        let response_current_page = paginated_entities
            .first()
            .map_or(0, |e| e.response_current_page);

        for paginated_entity in paginated_entities {
            let entity = CachedEntity {
                id: paginated_entity.id,
                entity_id: paginated_entity.entity_id,
                category_id: paginated_entity.category_id,
                categories_ids: paginated_entity.categories_ids,
                tags_ids: paginated_entity.tags_ids,
                family_id: paginated_entity.family_id,
                display_name: paginated_entity.display_name,
                parent_id: paginated_entity.parent_id,
                parent_display_name: paginated_entity.parent_display_name,
                web_mercator_x: paginated_entity.web_mercator_x,
                web_mercator_y: paginated_entity.web_mercator_y,
                plain_text_location: paginated_entity.plain_text_location,
            };
            entities.push(entity);
        }

        CachedEntitiesWithPagination {
            entities,
            total_results,
            total_pages,
            response_current_page,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct CachedEntitiesWithPagination {
    pub entities: Vec<CachedEntity>,
    pub total_results: i64,
    pub total_pages: i64,
    pub response_current_page: i64,
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
    pub web_mercator_x: f64,
    pub web_mercator_y: f64,
    pub parent_id: Option<Uuid>,
    pub parent_display_name: Option<String>,
    pub plain_text_location: String,
    pub cluster_id: Option<i32>,
    pub cluster_center_x: Option<f64>,
    pub cluster_center_y: Option<f64>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Cluster {
    pub id: u32,
    pub center_x: f64,
    pub center_y: f64,
    pub count: i32,
}

impl Cluster {
    fn calculate_id(&mut self) {
        self.id = crc32fast::hash(
            format!("{:.4}-{:.4}-{}", self.center_x, self.center_y, self.count).as_bytes(),
        );
    }
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct EntitiesAndClusters {
    pub entities: Vec<CachedEntity>,
    pub clusters: Vec<Cluster>,
}

pub struct FindEntitiesRequest {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
    pub family_id: Uuid,

    pub allow_all_categories: bool,
    pub allow_all_tags: bool,

    pub categories_list: Vec<Uuid>,
    pub tags_list: Vec<Uuid>,

    pub exclude_categories_list: Vec<Uuid>,
    pub exclude_tags_list: Vec<Uuid>,

    pub cluster_params: Option<(f64, i32)>,

    pub active_categories: Vec<Uuid>,
    pub active_required_tags: Vec<Uuid>,
    pub active_hidden_tags: Vec<Uuid>,
}

pub struct SearchEntitiesRequest {
    pub search_query: String,
    pub family_id: Uuid,

    pub allow_all_categories: bool,
    pub allow_all_tags: bool,

    pub categories_list: Vec<Uuid>,
    pub tags_list: Vec<Uuid>,

    pub exclude_categories_list: Vec<Uuid>,
    pub exclude_tags_list: Vec<Uuid>,

    pub page: i64,
    pub page_size: i64,

    pub active_categories: Vec<Uuid>,
    pub active_required_tags: Vec<Uuid>,
    pub active_hidden_tags: Vec<Uuid>,

    pub require_locations: bool,
}

impl CachedEntity {
    /// This function fetches entities within a rectangle defined by the coordinates of the bottom left and top right corners of a view port.
    /// It also filters entities based on the user's permissions.
    /// Careful: The coordinates of the viewport are in the format of (longitude, latitude) in projection Web Mercator (EPSG:3857).
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
                parent_id,
                parent_display_name,
                web_mercator_x as "web_mercator_x!",
                web_mercator_y as "web_mercator_y!",
                plain_text_location as "plain_text_location!",
                cluster_id,
                cluster_center_x,
                cluster_center_y
            FROM fetch_entities_within_view($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16)
            "#,
            request.xmin,
            request.ymin,
            request.xmax,
            request.ymax,
            request.family_id,
            request.allow_all_categories,
            request.allow_all_tags,
            &request.categories_list,
            &request.tags_list,
            &request.exclude_categories_list,
            &request.exclude_tags_list,
            request.cluster_params.map(|(eps, _)| eps).unwrap_or(0.0),
            request.cluster_params.map(|(_, min)| min).unwrap_or(0),
            &request.active_categories,
            &request.active_required_tags,
            &request.active_hidden_tags
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
                parent_id: e.parent_id,
                parent_display_name: e.parent_display_name.clone(),
                web_mercator_x: Some(e.web_mercator_x),
                web_mercator_y: Some(e.web_mercator_y),
                plain_text_location: Some(e.plain_text_location.clone()),
            })
            .collect();

        // -- Creating the list of clusters
        let clusters = entities_with_clusters
            .iter()
            .fold(
                HashMap::new(),
                |mut acc: std::collections::HashMap<u32, Cluster>, e| {
                    if let Some(cluster_id) = e.cluster_id {
                        let cluster = acc.entry(cluster_id as u32).or_insert(Cluster {
                            id: cluster_id as u32,
                            center_x: e.cluster_center_x.unwrap(),
                            center_y: e.cluster_center_y.unwrap(),
                            count: 0,
                        });
                        cluster.count += 1;
                    }
                    acc
                },
            )
            .into_values()
            .map(|mut v| {
                v.calculate_id();
                v
            })
            .collect();

        Ok(EntitiesAndClusters { entities, clusters })
    }

    pub async fn search_entities(
        request: SearchEntitiesRequest,
        conn: &mut PgConnection,
    ) -> Result<CachedEntitiesWithPagination, AppError> {
        if (request.page_size < 1) || (request.page < 1) {
            return Err(AppError::InvalidPagination);
        }

        let results = query_as!(
            PaginatedCachedEntity,
            r#"
            SELECT 
                id as "id!",
                entity_id as "entity_id!",
                category_id as "category_id!",
                categories_ids as "categories_ids!",
                tags_ids as "tags_ids!",
                family_id as "family_id!",
                display_name as "display_name!",
                parent_id,
                parent_display_name,
                web_mercator_x as "web_mercator_x",
                web_mercator_y as "web_mercator_y",
                plain_text_location as "plain_text_location",
                total_results as "total_results!",
                total_pages as "total_pages!",
                response_current_page as "response_current_page!"
            FROM search_entities($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
            "#,
            request.search_query,
            request.family_id,
            request.allow_all_categories,
            request.allow_all_tags,
            &request.categories_list,
            &request.tags_list,
            &request.exclude_categories_list,
            &request.exclude_tags_list,
            request.page,
            request.page_size,
            &request.active_categories,
            &request.active_required_tags,
            &request.active_hidden_tags,
            request.require_locations
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(results.into())
    }
}
