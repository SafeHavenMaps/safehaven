pub mod admin;
pub mod auth;
pub mod icons;
pub mod map;
pub mod root;

use crate::{
    config::SafeHavenConfig,
    models::{
        options::SafeHavenOptions,
        user::{NewOrUpdatedUser, User},
    },
};
use axum::{
    async_trait,
    extract::{FromRef, FromRequest, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::{
    postgres::{PgListener, PgPoolOptions},
    PgConnection, Pool, Postgres,
};
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

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("can't run migrations");

        tracing::info!("Migrations executed");

        let mut conn = pool.acquire().await.expect("can't acquire connection");

        if User::get_users_count(&mut conn)
            .await
            .expect("can't get users count")
            == 0
        {
            let user =
                std::env::var("SAFEHAVEN_DEFAULT_USER").unwrap_or_else(|_| "admin".to_string());
            let password = match std::env::var("SAFEHAVEN_DEFAULT_PASSWORD") {
                Ok(password) => password,
                Err(_) => {
                    use rand::distributions::{Alphanumeric, DistString};
                    Alphanumeric.sample_string(&mut rand::thread_rng(), 32)
                }
            };

            tracing::info!(
                "No users found, creating admin user {} with password {}",
                &user,
                &password
            );

            let admin = NewOrUpdatedUser {
                name: user,
                password: Some(password),
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

    pub async fn listen_postgresql_events(&self) {
        loop {
            match self.internal_listen().await {
                Ok(_) => break,
                Err(e) => {
                    tracing::error!("Error listening for PostgreSQL notifications: {:?}", e);
                }
            }

            // Sleep for 10 seconds to try to reconnect if something goes wrong
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }

    async fn internal_listen(&self) -> Result<(), sqlx::Error> {
        let mut listener = PgListener::connect_with(&self.pool).await?;

        listener.listen("reload_options").await?;

        tracing::info!("Listening for PostgreSQL notifications");

        loop {
            while let Some(notification) = listener.try_recv().await? {
                match notification.channel() {
                    "reload_options" => {
                        tracing::info!("Received notification to reload options");

                        self.reload_data(
                            &mut self
                                .pool
                                .acquire()
                                .await
                                .expect("Couldn't acquire connection"),
                        )
                        .await;
                    }
                    _ => {
                        tracing::warn!(
                            "Received notification from unknown channel : {:?}",
                            notification.channel()
                        )
                    }
                }
            }
        }
    }

    /// Reload the dynamic configuration from the database
    async fn reload_data(&self, conn: &mut PgConnection) {
        let mut dyn_config = self.dyn_config.write().await;
        *dyn_config = SafeHavenOptions::load(conn).await;
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
    Forbidden,
    Validation(String),
    Database(sqlx::Error),
    InvalidPagination,
    Internal(Option<String>),
    NotFound,
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
            AppError::NotFound => (StatusCode::NOT_FOUND, "not_found", None),
            AppError::Database(de) => match de {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "not_found", None),
                _ => {
                    // Check if error contains an sh_code_* within the SQL error message and return it
                    // as a bad request error with the error code set
                    if let Some(sh_code) = de
                        .to_string()
                        .split_whitespace()
                        .find(|s| s.starts_with("sh_code"))
                    {
                        // Return the code as a bad request, remove the prefix
                        return (
                            StatusCode::BAD_REQUEST,
                            AppJson(ErrorResponse {
                                error_code: sh_code[8..].to_string(),
                                details: None,
                            }),
                        )
                            .into_response();
                    }

                    tracing::error!("Sqlx error: {:?}", de);
                    (StatusCode::INTERNAL_SERVER_ERROR, "database_error", None)
                }
            },
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", None),
            AppError::InvalidPagination => (StatusCode::BAD_REQUEST, "invalid_pagination", None),
            AppError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", e),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden", None),
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
