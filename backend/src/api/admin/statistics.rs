use crate::{
    api::{AppError, AppJson, DbConn},
    models::statistics::{self, CountResult, HomePageStats},
};

#[utoipa::path(
    get,
    path = "/api/admin/stats",
    responses(
        (status = 200, description = "Stats for the home page", body = HomePageStats),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_home_stats(
    DbConn(mut conn): DbConn,
) -> Result<AppJson<HomePageStats>, AppError> {
    Ok(AppJson(statistics::home_page_stats(&mut conn).await?))
}

#[utoipa::path(
    get,
    path = "/api/admin/stats/count-comments-entities",
    responses(
        (status = 200, description = "Dicts of entities and comments counts by family and category id", body = (HashMap<String, (u32, u32, u32, u32)>,HashMap<String, (u32, u32, u32, u32)>),),
        (status = 401, description = "Invalid permissions", body = ErrorResponse),
    )
)]
pub async fn admin_count_comments_entities(
    DbConn(mut conn): DbConn,
) -> Result<AppJson<CountResult>, AppError> {
    Ok(AppJson(
        statistics::count_comments_entities(&mut conn).await?,
    ))
}
