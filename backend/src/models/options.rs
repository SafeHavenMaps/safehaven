use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection};
use utoipa::ToSchema;

use crate::api::AppError;

#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct SafeHavenOptions {
    pub general: GeneralOptions,
    pub safe_mode: SafeModeConfig,
    pub cartography_init: CartographyInitConfig,
    pub cartography_cluster: CartographyClusterConfig,
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub enum ConfigurationOption {
    General(GeneralOptions),
    SafeMode(SafeModeConfig),
    CartographyInit(CartographyInitConfig),
    CartographyCluster(CartographyClusterConfig),
}

impl ConfigurationOption {
    pub fn option_name(&self) -> &'static str {
        match self {
            ConfigurationOption::General(_) => GeneralOptions::option_name(),
            ConfigurationOption::SafeMode(_) => SafeModeConfig::option_name(),
            ConfigurationOption::CartographyInit(_) => CartographyInitConfig::option_name(),
            ConfigurationOption::CartographyCluster(_) => CartographyClusterConfig::option_name(),
        }
    }
}

trait OptionConfig {
    fn option_name() -> &'static str;
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
#[serde(default)]
pub struct GeneralOptions {
    pub title: String,
    pub subtitle: Option<String>,
    pub logo_url: Option<String>,
}

impl OptionConfig for GeneralOptions {
    fn option_name() -> &'static str {
        "general"
    }
}

impl Default for GeneralOptions {
    fn default() -> Self {
        Self {
            title: "SafeHaven".to_string(),
            subtitle: Some("Carte associative".to_string()),
            logo_url: None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
#[serde(default)]
/// Safe mode configuration, when enabled, the application will require recaptcha validation for
/// almost every action making it harder to spam the application or dump the database content
pub struct SafeModeConfig {
    /// Enable safe mode
    pub enabled: bool,
    /// Title of the popup shown for first recaptcha validation
    pub popup_title: Option<String>,
    /// Message of the popup shown for first recaptcha validation
    pub popup_message: Option<String>,
    /// Secret key for recaptcha v3
    pub recaptcha_v3_secret: String,
    /// Site key for recaptcha v3
    pub recaptcha_v3_sitekey: String,
    /// Threshold for recaptcha v3
    pub recaptcha_v3_threshold: f64,
}

impl OptionConfig for SafeModeConfig {
    fn option_name() -> &'static str {
        "safe_mode"
    }
}

impl Default for SafeModeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            popup_title: None,
            popup_message: None,
            recaptcha_v3_secret: "".to_string(),
            recaptcha_v3_sitekey: "".to_string(),
            recaptcha_v3_threshold: 0.5,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
/// Displayed map initialization parameters
pub struct CartographyInitConfig {
    /// Latitude of the map center (WGS84)
    pub center_lat: f64,
    /// Longitude of the map center (WGS84)
    pub center_lng: f64,
    /// Zoom level of the map (from 2 to 20)
    pub zoom: u8,
}

impl OptionConfig for CartographyInitConfig {
    fn option_name() -> &'static str {
        "cartography_init"
    }
}

impl Default for CartographyInitConfig {
    fn default() -> Self {
        Self {
            center_lat: 47.0,
            center_lng: 2.0,
            zoom: 5,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
#[serde(default)]
/// Entity clusterization parameters
pub struct CartographyClusterConfig {
    /// Declustering speed (most of the time between 1.5 and 2, defaults to 1.65)
    pub declustering_speed: f64,
    /// Characteristic distance for clusterization
    pub characteristic_distance: f64,
    /// Minimal entity count for forming a cluster
    pub minimal_cluster_size: i32,
}

impl OptionConfig for CartographyClusterConfig {
    fn option_name() -> &'static str {
        "cartography_cluster"
    }
}

impl Default for CartographyClusterConfig {
    fn default() -> Self {
        Self {
            declustering_speed: 1.65,
            characteristic_distance: 5.,
            minimal_cluster_size: 6,
        }
    }
}

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct DatabaseOption {
    pub name: String,
    pub value: serde_json::Value,
}

impl SafeHavenOptions {
    pub async fn insert_or_update_config(conn: &mut PgConnection, to_insert: ConfigurationOption) {
        let db_option_name = to_insert.option_name();
        let db_option_value = serde_json::to_value(to_insert).unwrap();

        sqlx::query!(
            r#"INSERT INTO options (name, value) VALUES ($1, $2) ON CONFLICT (name) DO UPDATE SET value = $2"#,
            db_option_name,
            db_option_value,
        )
        .execute(conn)
        .await
        .expect("Failed to insert or update option");
    }

    async fn fetch_option<T>(conn: &mut PgConnection) -> Result<T, AppError>
    where
        T: OptionConfig,
        T: Default,
        for<'de> T: Deserialize<'de>,
    {
        let db_option_name = T::option_name();

        let db_option = sqlx::query_as!(
            DatabaseOption,
            r#"SELECT name, value FROM options WHERE name = $1"#,
            db_option_name
        )
        .fetch_optional(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(match db_option {
            Some(db_option) => serde_json::from_value(db_option.value).unwrap(),
            None => T::default(),
        })
    }

    pub async fn load(conn: &mut PgConnection) -> Self {
        let general = Self::fetch_option::<GeneralOptions>(conn)
            .await
            .expect("Failed to load general");
        let safe_mode = Self::fetch_option::<SafeModeConfig>(conn)
            .await
            .expect("Failed to load safe mode");
        let cartography_init = Self::fetch_option::<CartographyInitConfig>(conn)
            .await
            .expect("Failed to load cartography init");
        let cartography_cluster = Self::fetch_option::<CartographyClusterConfig>(conn)
            .await
            .expect("Failed to load cartography cluster");

        Self {
            general,
            safe_mode,
            cartography_init,
            cartography_cluster,
        }
    }
}
