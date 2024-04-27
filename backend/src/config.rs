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
                clustering_parameters: {
                    let characteric_distance: f64 = 5.; //in hundreds of kms
                    let declustering_speed: f64 = 1.65;
                    let minimal_cluster_size: i32 = 6;
                    let mut params = HashMap::new();
                    for k in 2..29 {
                        params.insert(
                            k.to_string(),
                            (
                                characteric_distance * 100000. * declustering_speed.powf(-k as f64),
                                minimal_cluster_size,
                            ),
                        );
                    }
                    //params.insert("1:4".to_string(), (90000., 6));
                    // params.insert("5:7".to_string(), (0.5, 6));
                    // params.insert("8:20".to_string(), (0.3, 6));
                    params
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
