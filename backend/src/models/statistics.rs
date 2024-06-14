// Note : this file is not an actual model, but instead contains a collection of statistical queries on the other models

use crate::api::AppError;
use serde::Serialize;
use serde_json::Value;
use sqlx::PgConnection;
use std::collections::HashMap;
use utoipa::ToSchema;

pub type CountResult = (
    HashMap<String, (u32, u32, u32, u32)>,
    HashMap<String, (u32, u32, u32, u32)>,
);

struct CommentEntityCounts {
    category_id: String,
    family_id: String,
    moderated_entities_count: i64,
    unmoderated_entities_count: i64,
    moderated_comments_count: i64,
    unmoderated_comments_count: i64,
}

pub async fn count_comments_entities(conn: &mut PgConnection) -> Result<CountResult, AppError> {
    let results = sqlx::query_as!(
        CommentEntityCounts,
        r#"
        SELECT 
            c.id AS category_id, 
            c.family_id, 
            COUNT(DISTINCT e.id) FILTER (WHERE e.moderated) AS "moderated_entities_count!", 
            COUNT(DISTINCT e.id) FILTER (WHERE NOT e.moderated) AS "unmoderated_entities_count!", 
            COUNT(cm.id) FILTER (WHERE cm.moderated) AS "moderated_comments_count!", 
            COUNT(cm.id) FILTER (WHERE NOT cm.moderated) AS "unmoderated_comments_count!"
        FROM public.categories c
        JOIN public.entities e ON c.id = e.category_id
        LEFT JOIN public.comments cm ON e.id = cm.entity_id
        GROUP BY c.id, c.family_id
        "#
    )
    .fetch_all(conn)
    .await
    .map_err(AppError::Database)?;

    let (family_map, category_map) = results.into_iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut family_map, mut category_map), result| {
            family_map
                .entry(result.family_id)
                .and_modify(|e: &mut (u32, u32, u32, u32)| {
                    e.0 += result.moderated_entities_count as u32;
                    e.1 += result.unmoderated_entities_count as u32;
                    e.2 += result.moderated_comments_count as u32;
                    e.3 += result.unmoderated_comments_count as u32;
                })
                .or_insert((
                    result.moderated_entities_count as u32,
                    result.unmoderated_entities_count as u32,
                    result.moderated_comments_count as u32,
                    result.unmoderated_comments_count as u32,
                ));

            category_map
                .entry(result.category_id)
                .and_modify(|e: &mut (u32, u32, u32, u32)| {
                    e.0 += result.moderated_entities_count as u32;
                    e.1 += result.unmoderated_entities_count as u32;
                    e.2 += result.moderated_comments_count as u32;
                    e.3 += result.unmoderated_comments_count as u32;
                })
                .or_insert((
                    result.moderated_entities_count as u32,
                    result.unmoderated_entities_count as u32,
                    result.moderated_comments_count as u32,
                    result.unmoderated_comments_count as u32,
                ));

            (family_map, category_map)
        },
    );

    Ok((family_map, category_map))
}

#[derive(Serialize, Debug, ToSchema)]
pub struct HomePageStats {
    pub total_entities: i64,
    pub total_comments: i64,

    pub pending_entities: i64,
    pub pending_comments: i64,

    pub total_visits_30_days: i64,
    pub total_visits_7_days: i64,

    pub visits_30_days: Value,
}

pub async fn home_page_stats(conn: &mut PgConnection) -> Result<HomePageStats, AppError> {
    let stats = sqlx::query_as!(
        HomePageStats,
        r#"
        SELECT
            (SELECT COUNT(*) FROM entities WHERE moderated) as "total_entities!",
            (SELECT COUNT(*) FROM comments WHERE moderated) as "total_comments!",
            (SELECT COUNT(*) FROM entities WHERE NOT moderated) as "pending_entities!",
            (SELECT COUNT(*) FROM comments WHERE NOT moderated) as "pending_comments!",
            (SELECT COUNT(*) FROM access_tokens_visits WHERE visited_at >= NOW()::date - INTERVAL '30 days') as "total_visits_30_days!",
            (SELECT COUNT(*) FROM access_tokens_visits WHERE visited_at >= NOW()::date - INTERVAL '7 days') as "total_visits_7_days!",
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
                    WHERE
                        ds.visit_date >= NOW()::date - INTERVAL '30 days'
                    GROUP BY
                        ds.visit_date
                    ORDER BY
                        ds.visit_date
                )
                SELECT json_object_agg(
                    TO_CHAR(visit_date, 'YYYY-MM-DD'),
                    visit_count
                ) AS visits
                FROM aggregated_visits
            )
            as "visits_30_days!"
        "#
    )
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)?;

    Ok(stats)
}
