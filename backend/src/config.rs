use std::collections::HashMap;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use number_range::NumberRange;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone)]
pub struct SafeHavenConfig {
    pub listen_addr: String,
    pub database: Database,
    pub token_secret: String,
    pub serve_public_path: Option<String>,
    pub map: MapBoot,
}

impl SafeHavenConfig {
    pub fn init(&mut self) {
        self.map.init();
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct MapBoot {
    pub center_lat: f64,
    pub center_lng: f64,
    pub zoom: u8,
    pub display_projection: String,
    pub clustering_parameters: HashMap<String, (f64, i32)>,

    #[serde(skip)]
    parsed_clustering_parameters: Option<Vec<(Vec<u8>, f64, i32)>>,
}

impl MapBoot {
    pub fn init(&mut self) {
        self.parsed_clustering_parameters = Some(
            self.clustering_parameters
                .iter()
                .map(|(k, v)| 
                    (
                        NumberRange::<u8>::default().parse_str(&k).expect("Invalid range").collect(),
                        v.0,
                        v.1
                    )
                )
                .collect()
        );
    }

    pub fn get_eps_min_for_zoom(&self, zoom: u8) -> Option<(f64, i32)> {
        self.parsed_clustering_parameters
            .as_ref()
            .expect("Not initialized")
            .iter()
            .find(|(range, _, _)| range.contains(&zoom))
            .map(|(_, eps, min)| Some((*eps, *min)))
            .unwrap_or(None)
    }
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
            map: MapBoot {
                center_lat: 47.0,
                center_lng: 2.0,
                zoom: 5,
                display_projection: "EPSG:3857".to_string(),
                clustering_parameters: {
                    let mut map = HashMap::new();
                    map.insert("1:6".to_string(), (1.0, 25));
                    map
                },
                parsed_clustering_parameters: None,
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
