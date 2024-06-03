// Note : this file is not an actual model, but instead contains a collection of statistical queries on the other models

use crate::api::AppError;
use sqlx::PgConnection;
use std::collections::HashMap;

pub async fn count_comments_entities(
    conn: &mut PgConnection,
) -> Result<
    (
        HashMap<String, (u32, u32, u32, u32)>,
        HashMap<String, (u32, u32, u32, u32)>,
    ),
    AppError,
> {
    let results = sqlx::query_as!(
        CommentEntityCounts,
        r#"
        SELECT 
            c.id AS category_id, 
            c.family_id, 
            COUNT(DISTINCT e.id) FILTER (WHERE e.moderated_at IS NOT NULL) AS "moderated_entities_count!", 
            COUNT(DISTINCT e.id) FILTER (WHERE e.moderated_at IS NULL) AS "unmoderated_entities_count!", 
            COUNT(cm.id) FILTER (WHERE cm.moderated_at IS NOT NULL) AS "moderated_comments_count!", 
            COUNT(cm.id) FILTER (WHERE cm.moderated_at IS NULL) AS "unmoderated_comments_count!"
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

struct CommentEntityCounts {
    category_id: String,
    family_id: String,
    moderated_entities_count: i64,
    unmoderated_entities_count: i64,
    moderated_comments_count: i64,
    unmoderated_comments_count: i64,
}
