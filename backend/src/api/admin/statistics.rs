use std::collections::HashMap;

use crate::{
    api::{AppError, AppJson, DbConn},
    models::statistics,
};

#[utoipa::path(
    get,
    path = "/api/admin/stats/count-comments-entities",
    responses(
        (status = 200, description = "Dicts of entities and comments counts by family and category id", body = Json<(
            
            HashMap<String, (u32, u32, u32, u32)>,
            HashMap<String, (u32, u32, u32, u32)>,
        )>),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_count_comments_entities(
    DbConn(mut conn): DbConn,
) -> Result<
    AppJson<(
        HashMap<String, (u32, u32, u32, u32)>,
        HashMap<String, (u32, u32, u32, u32)>,
    )>,
    AppError,
> {
    Ok(AppJson(
        statistics::count_comments_entities(&mut conn).await?,
    ))
}
