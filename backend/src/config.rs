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
    pub cartography: CartographyConfig,
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct CartographyInitConfig {
    ///Displayed map initialization parameters
    pub center_lat: f64,
    pub center_lng: f64,
    pub zoom: u8,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct CartographyClusterConfig {
    ///Entity clusterization parameters
    pub declustering_speed: f64,
    pub characteristic_distance: f64,
    pub minimal_cluster_size: i32,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct CartographyConfig {
    /// Map configuration
    pub init: CartographyInitConfig,
    pub cluster: CartographyClusterConfig,
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
            token_secret: "SecretForValidatingAngSigningTokens".to_string(),
            serve_public_path: None,
            cartography: CartographyConfig {
                init: CartographyInitConfig {
                    center_lat: 47.0,
                    center_lng: 2.0,
                    zoom: 5,
                },
                cluster: CartographyClusterConfig {
                    characteristic_distance: 5.,
                    declustering_speed: 1.65,
                    minimal_cluster_size: 6,
                },
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
