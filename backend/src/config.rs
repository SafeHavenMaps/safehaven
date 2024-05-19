use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct SafeHavenConfig {
    /// Address to listen on
    pub listen_addr: String,
    /// Database configuration
    pub database: Database,
    /// Secret for JWT validation
    pub token_secret: String,
    /// Path to serve public files from
    pub serve_public_path: Option<String>,
    /// Secure cookie flag
    pub secure_cookie: bool,
}

#[derive(Deserialize, Serialize, Clone)]
/// Database configuration
pub struct Database {
    /// Database URL ('postgresql://user:password@host/database')
    pub url: String,
    /// Database connection pool size (default to 5)
    pub pool_size: u32,
    /// Database connection timeout in seconds (default to 3)
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
            secure_cookie: false,
        }
    }
}

pub fn load(config_path: &str) -> Result<SafeHavenConfig, figment::Error> {
    Figment::from(Serialized::defaults(SafeHavenConfig::default()))
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("SAFEHAVEN_"))
        .extract()
}
