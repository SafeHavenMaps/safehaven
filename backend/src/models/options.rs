use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection};
use utoipa::ToSchema;

use crate::api::AppError;

#[derive(Deserialize, Serialize, Clone, ToSchema, Debug)]
pub struct SafeHavenOptions {
    pub general: GeneralOptions,
    pub init_popup: InitPopupOptions,
    pub safe_mode: SafeModeConfig,
    pub cartography_init: CartographyInitConfig,
    pub cartography_source: CartographySourceConfig,
    pub cartography_cluster: CartographyClusterConfig,
}

#[derive(Deserialize, Serialize, Clone, ToSchema, Debug)]
#[serde(untagged)]
pub enum ConfigurationOption {
    General(GeneralOptions),
    InitPopup(InitPopupOptions),
    SafeMode(SafeModeConfig),
    CartographyInit(CartographyInitConfig),
    CartographySource(CartographySourceConfig),
    CartographyCluster(CartographyClusterConfig),
}

impl ConfigurationOption {
    pub fn option_name(&self) -> &'static str {
        match self {
            ConfigurationOption::General(_) => GeneralOptions::option_name(),
            ConfigurationOption::InitPopup(_) => InitPopupOptions::option_name(),
            ConfigurationOption::SafeMode(_) => SafeModeConfig::option_name(),
            ConfigurationOption::CartographyInit(_) => CartographyInitConfig::option_name(),
            ConfigurationOption::CartographySource(_) => CartographySourceConfig::option_name(),
            ConfigurationOption::CartographyCluster(_) => CartographyClusterConfig::option_name(),
        }
    }
}

trait OptionConfig {
    fn option_name() -> &'static str;
}

#[derive(Deserialize, Serialize, Clone, ToSchema, Debug)]
#[serde(default)]
pub struct GeneralOptions {
    pub title: String,
    pub subtitle: Option<String>,
    pub logo_url: Option<String>,
    pub information: Option<String>,
    pub redirect_url: Option<String>,
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
            information: None,
            redirect_url: None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema, Debug, Default)]
#[serde(default)]
pub struct InitPopupOptions {
    pub popup: Option<String>,
    pub popup_check_text: Option<String>,
}

impl OptionConfig for InitPopupOptions {
    fn option_name() -> &'static str {
        "init_popup"
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema, Debug)]
#[serde(default)]
/// Safe mode configuration, when enabled, the application will require hcaptcha validation for
/// almost every action making it harder to spam the application or dump the database content
pub struct SafeModeConfig {
    /// Enable safe mode
    pub enabled: bool,
    /// Secret key for hcaptcha
    pub hcaptcha_secret: String,
    /// Site key for hcaptcha
    pub hcaptcha_sitekey: String,
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
            hcaptcha_sitekey: "".to_string(),
            hcaptcha_secret: "".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema, Debug)]
#[serde(default)]
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

#[derive(Deserialize, Serialize, Clone, ToSchema, Debug)]
#[serde(default)]
pub struct CartographySourceConfig {
    /// Light mode map url
    pub light_map_url: String,
    /// Dark mode map url
    pub dark_map_url: String,
    /// Light mode map attributions
    pub light_map_attributions: String,
    /// Dark mode map attributions
    pub dark_map_attributions: String,
}

impl OptionConfig for CartographySourceConfig {
    fn option_name() -> &'static str {
        "cartography_source"
    }
}

impl Default for CartographySourceConfig {
    fn default() -> Self {
        Self {
            light_map_url: "https://tile.openstreetmap.org/{z}/{x}/{y}.png".to_string(),
            light_map_attributions: "Map data © OpenStreetMap contributors".to_string(),
            dark_map_url: "https://tile.openstreetmap.org/{z}/{x}/{y}.png".to_string(),
            dark_map_attributions: "Map data © OpenStreetMap contributors".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema, Debug)]
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

    pub async fn delete(conn: &mut PgConnection, to_delete: String) -> Result<(), AppError> {
        let db_option_name = to_delete;
        sqlx::query!(
            r#"
            DELETE FROM options
            WHERE name = $1
            "#,
            db_option_name
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(())
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
        let init_popup = Self::fetch_option::<InitPopupOptions>(conn)
            .await
            .expect("Failed to load init popup");
        let safe_mode = Self::fetch_option::<SafeModeConfig>(conn)
            .await
            .expect("Failed to load safe mode");
        let cartography_init = Self::fetch_option::<CartographyInitConfig>(conn)
            .await
            .expect("Failed to load cartography init");
        let cartography_source = Self::fetch_option::<CartographySourceConfig>(conn)
            .await
            .expect("Failed to load cartography source");
        let cartography_cluster = Self::fetch_option::<CartographyClusterConfig>(conn)
            .await
            .expect("Failed to load cartography cluster");

        Self {
            general,
            init_popup,
            safe_mode,
            cartography_init,
            cartography_source,
            cartography_cluster,
        }
    }
}
