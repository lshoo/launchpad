use config::{Config, Environment, File};
use serde::Deserialize;

pub const CONFIG_NAME: &str = "config";
pub const CONFIG_FILE_NAME: &str = "config.yaml";
pub const ENV_PREFIX: &str = "LAUNCHPAD";

const SETTINGS_SEPARATOR: &str = "__";

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub config: ConfigInfo,
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub logging: Logging,
}

impl Settings {
    pub fn new(location: &str, env_prefix: &str) -> anyhow::Result<Self> {
        let settings = Config::builder()
            .add_source(File::with_name(location))
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator(SETTINGS_SEPARATOR)
                    .prefix_separator(SETTINGS_SEPARATOR),
            )
            .set_override("config.location", location)?
            .set_override("config.env_prefix", env_prefix)?
            .build()?;

        let settings = settings.try_deserialize()?;

        Ok(settings)
    }

    pub fn db_url(&self) -> Option<String> {
        self.database.url.clone()
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigInfo {
    pub location: Option<String>,
    pub env_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Database {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Logging {
    pub log_level: Option<String>,
}
