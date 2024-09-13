use crate::{api::AppError, helpers::postgis_polygons::MultiPolygon};
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use sqlx::{types::Json, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct Permissions {
    /// Restriction to a specific set of families
    pub families_policy: PermissionPolicy,

    /// Restriction to a specific set of categories
    pub categories_policy: PermissionPolicy,

    /// Restriction to a specific set of tags
    pub tags_policy: PermissionPolicy,

    /// Restriction to a specific area
    pub geographic_restrictions: Option<MultiPolygon>,

    /// Permission to list entities
    pub can_list_entities: bool,

    /// Permission to list entities with an empty or short query (can be used to list all entities)
    pub can_list_without_query: bool,

    /// Permission to list entities with filters
    pub can_list_with_filters: bool,

    /// Permission to list entities with enum constraints
    pub can_list_with_enum_constraints: bool,

    /// Permission to view an entity
    pub can_access_entity: bool,

    /// Permission to view an entity's comments
    pub can_access_comments: bool,

    /// Permission to add an entity
    pub can_add_entity: bool,

    /// Permission to add a comment to an entity
    pub can_add_comment: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PermissionPolicy {
    pub allow_all: bool,
    pub allow_list: Vec<Uuid>,
    pub force_exclude: Vec<Uuid>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewOrUpdateAccessToken {
    pub title: String,
    pub token: String,
    #[schema(value_type = Object)]
    pub permissions: Json<Permissions>,
    pub active: bool,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct AccessToken {
    pub id: Uuid,
    pub title: String,
    pub token: String,
    #[schema(value_type = Object)]
    pub permissions: Json<Permissions>,
    pub last_week_visits: i64,
    pub active: bool,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct AccessTokenStats {
    #[schema(value_type = Object)]
    pub origins: Json<Value>,

    #[schema(value_type = Object)]
    pub visits_30_days: Json<Value>,
}

impl AccessToken {
    pub async fn new(
        access_token: NewOrUpdateAccessToken,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        let permission_value =
            to_value(access_token.permissions).expect("Failed to serialize permissions");

        sqlx::query_as!(
            AccessToken,
            r#"
            INSERT INTO access_tokens (title, token, permissions, active)
            VALUES ($1, $2, $3, $4)
            RETURNING 
                id,
                title,
                token, 
                permissions as "permissions: Json<Permissions>",
                active,
                0 as "last_week_visits!"
            "#,
            access_token.title,
            access_token.token,
            permission_value,
            access_token.active
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update(
        given_id: Uuid,
        update: NewOrUpdateAccessToken,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        let permission_value =
            to_value(update.permissions).expect("Failed to serialize permissions");

        sqlx::query_as!(
            AccessToken,
            r#"
            UPDATE access_tokens
            SET title = $2, token = $3, permissions = $4, active = $5
            WHERE id = $1
            RETURNING 
                id,
                title,
                token,
                permissions as "permissions: Json<Permissions>",
                active,
                (SELECT COUNT(*) FROM access_tokens_visits WHERE token_id = id AND visited_at > NOW() - INTERVAL '1 week') as "last_week_visits!"
            "#,
            given_id,
            update.title,
            update.token,
            permission_value,
            update.active
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn delete(given_id: Uuid, conn: &mut PgConnection) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM access_tokens
            WHERE id = $1
            "#,
            given_id
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn get(
        given_token: String,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        // We don't need to count the visits here
        sqlx::query_as!(
            AccessToken,
            r#"
            SELECT
                id,
                title,
                token,
                permissions as "permissions: Json<Permissions>",
                active,
                0 as "last_week_visits!"
            FROM access_tokens
            WHERE token = $1
            "#,
            given_token
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_with_id(
        given_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<AccessToken, AppError> {
        sqlx::query_as!(
            AccessToken,
            r#"
            SELECT 
                id,
                title,
                token,
                permissions as "permissions: Json<Permissions>",
                active,
                (SELECT COUNT(*) FROM access_tokens_visits WHERE token_id = id AND visited_at > NOW() - INTERVAL '1 week') as "last_week_visits!"
            FROM access_tokens
            WHERE id = $1
            "#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<AccessToken>, AppError> {
        sqlx::query_as!(
            AccessToken,
            r#"
            SELECT
                id,
                title,
                token,
                permissions as "permissions: Json<Permissions>",
                active,
                (SELECT COUNT(*) FROM access_tokens_visits WHERE token_id = id AND visited_at > NOW() - INTERVAL '1 week') as "last_week_visits!"
            FROM access_tokens
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn register_visit(
        access_token_id: Uuid,
        referrer: Option<String>,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO access_tokens_visits (token_id, referrer)
            VALUES ($1, $2)
            "#,
            access_token_id,
            referrer
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn get_stats(
        access_token_id: Uuid,
        conn: &mut PgConnection,
    ) -> Result<AccessTokenStats, AppError> {
        sqlx::query_as!(
            AccessTokenStats,
            r#"
            SELECT
            COALESCE((
                WITH origins AS (
                    SELECT DISTINCT(COALESCE(referrer, 'unknown')) AS referrer, COUNT(*) AS total
                    FROM access_tokens_visits
                    WHERE token_id = $1
                    GROUP BY referrer
                )
                SELECT json_object_agg(referrer, total) FROM origins
            ), '{}') as "origins!",
            (
                WITH date_series AS (
                    SELECT generate_series(
                        NOW()::date - INTERVAL '30 days',
                        NOW()::date,
                        INTERVAL '1 day'
                    )::date AS visit_date
                ),
                aggregated_visits AS (
                    SELECT
                        ds.visit_date,
                        COALESCE(COUNT(atv.visited_at), 0) AS visit_count
                    FROM
                        date_series ds
                    LEFT JOIN
                        access_tokens_visits atv
                    ON
                        ds.visit_date = DATE(atv.visited_at) 
                        AND atv.token_id = $1
                    GROUP BY
                        ds.visit_date
                    ORDER BY
                        ds.visit_date
                )
                SELECT COALESCE(json_object_agg(
                    TO_CHAR(visit_date, 'YYYY-MM-DD'),
                    visit_count
                ), '{}') AS visits
                FROM aggregated_visits
            ) as "visits_30_days!";
            "#,
            access_token_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }
}
