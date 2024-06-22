use std::sync::Arc;

use crate::{api::AppState, models::icon::Icon};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use resvg::{tiny_skia, usvg, usvg::ImageHrefResolver};
use serde::Deserialize;
use usvg::ImageKind::{JPEG, PNG, SVG};

use sqlx::{pool::PoolConnection, Postgres};
use tiny_skia::{Pixmap, Transform};
use usvg::{Options, Tree};

use super::{AppError, DbConn};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/:hash", get(get_icon))
        .route("/pin", get(get_pin))
}

async fn get_icon_internal(
    state: &AppState,
    hash: String,
    conn: &mut PoolConnection<Postgres>,
) -> Result<(Vec<u8>, String), AppError> {
    // Check cache firsts
    if let Some(icon) = state.icon_cache.read().await.get(&hash) {
        return Ok(icon.clone());
    }

    // If not found in cache, query the database
    let icon = Icon::get(hash.clone(), conn).await?;
    let icon = (icon.data, icon.http_mime_type);

    // Update cache
    state.icon_cache.write().await.insert(hash, icon.clone());

    Ok(icon)
}

async fn get_icon(
    State(state): State<AppState>,
    DbConn(mut conn): DbConn,
    Path(hash): Path<String>,
) -> Result<Response, AppError> {
    let (data, mime_type) = get_icon_internal(&state, hash, &mut conn).await?;

    Ok((
        StatusCode::OK,
        [
            ("Content-Type", mime_type),
            ("Cache-Control", "public, max-age=31536000".to_owned()),
        ],
        data,
    )
        .into_response())
}

#[derive(Deserialize, Debug)]
pub struct RenderQuery {
    pub h: Option<u32>,
    pub w: Option<u32>,
    pub bc: Option<String>,
    pub fc: Option<String>,
    pub ih: Option<String>,
}

async fn get_pin(
    State(state): State<AppState>,
    DbConn(mut conn): DbConn,
    Query(query): Query<RenderQuery>,
) -> Result<Response, AppError> {
    let (border_color, fill_color, icon_hash) = (
        format!("#{}", query.bc.as_deref().unwrap_or("222222")),
        format!("#{}", query.fc.as_deref().unwrap_or("9999FF")),
        query.ih,
    );

    let icon = match icon_hash {
        None => None,
        Some(icon_hash) => Some(get_icon_internal(&state, icon_hash, &mut conn).await?),
    };

    if query.h > Some(100) || query.w > Some(100) {
        return Err(AppError::Validation("invalid_size".to_string()));
    }

    let icon_svg = match icon.as_ref() {
        None => "".to_owned(),
        Some(_) => r#"
                <image
                x="9"
                y="9"
                width="26"
                height="26"
                xlink:href="icon"
                />
            "#
        .to_owned(),
    };
    let pin_svg = format!(
        r#"
        <svg
            version="1.1"
            viewBox="0 0 44 67"
            xml:space="preserve"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
        >
            <path
                d="m21.905 1.2688c-11.397-1.86e-5 -20.637 9.5307-20.636
                21.287 0.00476 3.5178 0.85467 6.9796 2.4736 10.076 5.9268 10.527 12.063 21.068 18.111
                31.572 5.8042-10.829 13.224-21.769 18.766-32.581
                1.4143-2.9374 1.9205-5.7872 1.9231-9.0669 6.2e-5 -11.757-9.2392-21.287-20.636-21.287z"
                fill="{fill_color}"
                stroke="{border_color}"
                stroke-width="2.5"
            />
            {icon_svg}
        </svg>
        "#
    );

    let mut opt = Options::default();

    // We know that our SVG won't have DataUrl hrefs, just return None for such case.
    let resolve_data = Box::new(|_: &str, _: std::sync::Arc<Vec<u8>>, _: &usvg::Options| None);

    let parsed_image = if let Some((data, mime)) = icon {
        match mime.as_str() {
            "image/png" => Some(PNG(Arc::new(data.clone()))),
            "image/jpeg" => Some(JPEG(Arc::new(data.clone()))),
            "image/svg+xml" => {
                let tree: Tree = match Tree::from_data(&data, &Options::default()) {
                    Ok(tree) => tree,
                    Err(_) => {
                        return Err(AppError::Internal(Some("svg_renderer_failed".to_string())))
                    }
                };
                Some(SVG(tree))
            }
            _ => None,
        }
    } else {
        None
    };

    let resolve_string = Box::new(move |href: &str, _: &Options| {
        if let Some(parsed_image) = parsed_image.as_ref() {
            match href {
                "icon" => Some(parsed_image.clone()),
                _ => None,
            }
        } else {
            None
        }
    });

    // Assign new ImageHrefResolver option using our closures.
    opt.image_href_resolver = ImageHrefResolver {
        resolve_data,
        resolve_string,
    };

    let tree = match Tree::from_str(&pin_svg, &opt) {
        Ok(tree) => tree,
        Err(_) => return Err(AppError::Internal(Some("svg_renderer_failed".to_string()))),
    };

    let pixmap_size = tree.size().to_int_size();

    let (height, width, scale) = match (query.h, query.w) {
        (Some(h), Some(w)) => (h, w, None),
        (Some(h), None) => {
            let scale = h as f32 / pixmap_size.height() as f32;
            (h, (pixmap_size.width() as f32 * scale) as u32, Some(scale))
        }
        (None, Some(w)) => {
            let scale = w as f32 / pixmap_size.width() as f32;
            ((pixmap_size.height() as f32 * scale) as u32, w, Some(scale))
        }
        _ => (38, 24, None),
    };

    let scale_x = scale.unwrap_or(width as f32 / pixmap_size.width() as f32);
    let scale_y = scale.unwrap_or(height as f32 / pixmap_size.height() as f32);
    let transform = Transform::from_scale(scale_x, scale_y);

    let mut pixmap = match Pixmap::new(width, height) {
        Some(pixmap) => pixmap,
        None => {
            return Err(AppError::Internal(Some(
                "pixmap_creation_failed".to_string(),
            )))
        }
    };

    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let image_data = match pixmap.encode_png() {
        Ok(image_data) => image_data,
        Err(_) => return Err(AppError::Internal(Some("png_encoding_failed".to_string()))),
    };

    Ok((
        StatusCode::OK,
        [
            ("Content-Type", "image/png"),
            ("Cache-Control", "public, max-age=31536000"),
        ],
        image_data,
    )
        .into_response())
}
