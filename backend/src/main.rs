mod api;
mod config;
mod doc;
mod models;

use api::AppState;
use axum::{extract::MatchedPath, http::Request, Router};
use clap::{arg, command, Args, Parser, Subcommand};
use config::SafeHavenConfig;
use std::fs;
use std::{process::exit, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;

#[derive(Parser)]
#[command(name = "safehaven", about = "SafeHaven API server")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// OpenAPI file generation
    Openapi(OpenapiArgs),
    /// Serve the application
    Serve(ServeArgs),
    /// Print the configuration
    Config(ConfigArgs),
}

#[derive(Args, Clone)]
struct OpenapiArgs {
    /// Output path for OpenAPI file
    #[arg(value_name = "path", default_value = "openapi.json")]
    path: String,
}

#[derive(Args, Clone)]
struct ServeArgs {
    /// Configuration file
    #[arg(short, long, value_name = "config", default_value = "safehaven.toml")]
    config: String,

    /// Activate ReDoc (served on /redoc endpoint)
    #[arg(short, long, default_value = "false")]
    redoc: bool,
}

#[derive(Args, Clone)]
struct ConfigArgs {}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Openapi(a) => openapi(a),
        Commands::Serve(a) => serve(a).await,
        Commands::Config(_) => {
            let config = config::load("safehaven.toml").unwrap_or_else(|e| {
                tracing::error!("Cannot load configuration: {}", e);
                exit(1);
            });
            // Print as json
            let json = serde_json::to_string_pretty(&config).unwrap();
            println!("{}", json);
        }
    };
}

fn openapi(args: &OpenapiArgs) {
    let doc = doc::ApiDoc::openapi();
    let json = doc
        .to_pretty_json()
        .expect("Failed to serialize OpenAPI document");
    fs::write(args.path.as_str(), json).expect("Failed to write OpenAPI file");
    tracing::info!("OpenAPI file saved to {}", args.path);
}

async fn build_server(app_state: AppState, config: Arc<SafeHavenConfig>) {
    tracing::info!("Starting server at {}", config.listen_addr);

    let mut app = Router::new()
        .nest("/api/", api::root::routes())
        .nest("/api/icons", api::icons::routes())
        .nest("/api/map", api::map::routes())
        .nest("/api/admin", api::admin::routes(&app_state))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    if let Some(public_path) = &config.serve_public_path {
        let serve_dir = ServeDir::new(public_path)
            .not_found_service(ServeFile::new(format!("{}/index.html", public_path)));

        app = app
            .nest_service("/", serve_dir.clone())
            .fallback_service(serve_dir);

        tracing::info!("Serving public files from {}", public_path);
    }

    let listener = TcpListener::bind(&config.listen_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn serve(args: &ServeArgs) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                if cfg!(debug_assertions) {
                    // Development build
                    "safehaven=trace,tower_http=debug,axum::rejection=trace"
                } else {
                    // Release build
                    "safehaven=info,tower_http=info,axum::rejection=warn"
                }
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::load(args.config.as_str()).unwrap_or_else(|e| {
        tracing::error!("Cannot load configuration: {}", e);
        exit(1);
    });

    let config = Arc::new(config);
    let app_state = AppState::from_config(config.clone()).await;

    let server = build_server(app_state.clone(), config);
    let db_notifier = app_state.listen_postgresql_events();

    tokio::select! {
        _ = server => {},
        _ = db_notifier => {},
    }
}
