use crate::{api::AppState, models::icon::Icon};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, Router},
    Json,
};
use serde_json::json;

use super::DbConn;

pub fn routes() -> Router<AppState> {
    Router::new().route("/:hash", get(get_icon))
}

async fn get_icon(
    State(state): State<AppState>,
    DbConn(mut conn): DbConn,
    Path(hash): Path<String>,
) -> Response {
    // Check cache firsts
    if let Some(icon) = state.icon_cache.read().await.get(&hash) {
        return (
            StatusCode::OK,
            [("Content-Type", icon.1.clone())],
            icon.0.clone(),
        )
            .into_response();
    }

    // If not found in cache, query the database
    match Icon::get(hash.clone(), &mut conn).await {
        Ok(icon) => {
            // Update cache
            state
                .icon_cache
                .write()
                .await
                .insert(hash, (icon.data.clone(), icon.http_mime_type.clone()));

            (
                StatusCode::OK,
                [("Content-Type", icon.http_mime_type.clone())],
                icon.data,
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Icon not found"})),
        )
            .into_response(),
    }
}
