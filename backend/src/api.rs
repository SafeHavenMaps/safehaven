pub mod admin;
pub mod icons;
pub mod map;
pub mod root;

use crate::{
    config::SafeHavenConfig,
    models::{
        access_token::Permissions,
        options::SafeHavenOptions,
        user::{NewOrUpdatedUser, User},
    },
};
use axum::{
    async_trait,
    extract::{FromRef, FromRequest, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgConnection, Pool, Postgres};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use utoipa::ToSchema;

pub type DynOptions = Arc<RwLock<SafeHavenOptions>>;
pub type IconCache = Arc<RwLock<HashMap<String, (Vec<u8>, String)>>>;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<SafeHavenConfig>,
    pub dyn_config: DynOptions,
    pub pool: Pool<Postgres>,
    pub icon_cache: IconCache,
}

impl AppState {
    pub async fn from_config(config: Arc<SafeHavenConfig>) -> Self {
        // set up connection pool
        let pool = PgPoolOptions::new()
            .max_connections(config.database.pool_size)
            .acquire_timeout(Duration::from_secs(config.database.timeout))
            .connect(&config.database.url)
            .await
            .expect("can't connect to database");

        let mut conn = pool.acquire().await.expect("can't acquire connection");

        if User::get_users_count(&mut conn)
            .await
            .expect("can't get users count")
            == 0
        {
            tracing::info!("No users found, creating admin user");

            let admin = NewOrUpdatedUser {
                name: "admin".to_string(),
                password: Some("safehaven".to_string()),
                is_admin: true,
            };

            User::new(admin, &mut conn)
                .await
                .expect("can't create admin user");

            tracing::warn!("Default admin user created, please change the password");
        }

        tracing::info!("Loading dynamic configuration from database");
        let dyn_config = Arc::new(RwLock::new(SafeHavenOptions::load(&mut conn).await));

        Self {
            config,
            pool,
            dyn_config,
            icon_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Reload the dynamic configuration from the database
    pub async fn reload_data(&self, conn: &mut PgConnection) {
        let mut dyn_config = self.dyn_config.write().await;
        *dyn_config = SafeHavenOptions::load(conn).await;
    }

    pub async fn execute_migration(&self) {
        sqlx::migrate!()
            .run(&self.pool)
            .await
            .expect("can't run migrations");

        tracing::info!("Migrations executed");
    }

    pub fn generate_token<T>(&self, claims: T) -> String
    where
        T: serde::Serialize,
    {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.config.token_secret.as_ref()),
        )
        .expect("Could not generate token");

        token
    }
}

pub struct DbConn(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DbConn
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let conn = app_state.pool.acquire().await.map_err(|_| AppError::Pool)?;

        Ok(Self(conn))
    }
}

#[derive(Debug)]
pub enum AppError {
    Pool,
    TokenValidation,
    BadUsernameOrPassword,
    Unauthorized,
    Validation(String),
    Database(sqlx::Error),
    InvalidPagination,
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(ToSchema, Serialize)]
pub struct ErrorResponse {
    error_code: String,
    details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, details) = match self {
            AppError::Pool => (StatusCode::INTERNAL_SERVER_ERROR, "pool_error", None),
            AppError::BadUsernameOrPassword => (StatusCode::NOT_FOUND, "user_not_found", None),
            AppError::TokenValidation => (StatusCode::UNAUTHORIZED, "token_validation_error", None),
            AppError::Validation(ve) => (StatusCode::BAD_REQUEST, "validation_error", Some(ve)),
            AppError::Database(de) => match de {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "not_found", None),
                _ => {
                    tracing::error!("Sqlx error: {:?}", de);
                    (StatusCode::INTERNAL_SERVER_ERROR, "database_error", None)
                }
            },
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", None),
            AppError::InvalidPagination => (StatusCode::BAD_REQUEST, "invalid_pagination", None),
        };

        let resp = (
            status,
            AppJson(ErrorResponse {
                error_code: error_code.to_string(),
                details,
            }),
        );

        resp.into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapUserTokenClaims {
    pub perms: Permissions,
    pub exp: usize,
    pub iat: usize,
}

pub struct MapUserToken(MapUserTokenClaims);

#[async_trait]
impl<S> FromRequestParts<S> for MapUserToken
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::TokenValidation)?;

        let token_data = jsonwebtoken::decode::<MapUserTokenClaims>(
            bearer.token(),
            &jsonwebtoken::DecodingKey::from_secret(app_state.config.token_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AppError::TokenValidation)?;

        Ok(Self(token_data.claims))
    }
}
