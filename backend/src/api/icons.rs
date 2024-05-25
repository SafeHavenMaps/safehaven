use crate::{
    api::AppState,
    models::{category::Category, family::Family},
};
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
    Router::new()
        .route("/families/:hash", get(get_family_icon))
        .route("/categories/:hash", get(get_category_icon))
}

async fn get_family_icon(
    State(state): State<AppState>,
    DbConn(mut conn): DbConn,
    Path(hash): Path<String>,
) -> Response {
    // Check cache first
    if let Some(icon) = state.icon_cache.read().await.get(&hash) {
        return (
            StatusCode::OK,
            [("Content-Type", "image/svg+xml")],
            icon.clone(),
        )
            .into_response();
    }

    // If not found in cache, query the database
    match Family::get_icon_content(hash.clone(), &mut conn).await {
        Ok(icon) => {
            // Update cache
            state.icon_cache.write().await.insert(hash, icon.clone());

            (StatusCode::OK, [("Content-Type", "image/svg+xml")], icon).into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Icon not found"})),
        )
            .into_response(),
    }
}

async fn get_category_icon(
    State(state): State<AppState>,
    DbConn(mut conn): DbConn,
    Path(hash): Path<String>,
) -> Response {
    // Check cache first
    if let Some(icon) = state.icon_cache.read().await.get(&hash) {
        return (
            StatusCode::OK,
            [("Content-Type", "image/svg+xml")],
            icon.clone(),
        )
            .into_response();
    }

    // If not found in cache, query the database
    match Category::get_icon_content(hash.clone(), &mut conn).await {
        Ok(icon) => {
            // Update cache
            state.icon_cache.write().await.insert(hash, icon.clone());

            (StatusCode::OK, [("Content-Type", "image/svg+xml")], icon).into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Icon not found"})),
        )
            .into_response(),
    }
}
