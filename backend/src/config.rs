use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone)]
pub struct SafeHavenConfig {
    pub listen_addr: String,
    pub database: Database,
    pub token_secret: String,
    pub serve_public_path: Option<String>,
    pub boot: MapBoot,
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct MapBoot {
    pub center_lat: f64,
    pub center_lng: f64,
    pub zoom: u8,
    pub display_projection: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
    pub timeout: u64,
}

impl Default for SafeHavenConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0:28669".to_string(),
            database: Database {
                url: "postgresql://postgres:postgres@localhost/safehaven".to_string(),
                pool_size: 5,
                timeout: 3,
            },
            token_secret: "SecretForValidatingAngSigngingTokens".to_string(),
            serve_public_path: None,
            boot: MapBoot {
                center_lat: 47.0,
                center_lng: 2.0,
                zoom: 5,
                display_projection: "EPSG:3857".to_string(),
            },
        }
    }
}

pub fn load(config_path: &str) -> Result<SafeHavenConfig, figment::Error> {
    Figment::from(Serialized::defaults(SafeHavenConfig::default()))
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("SAFEHAVEN_"))
        .extract()
}
