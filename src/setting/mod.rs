use chrono::{Date, NaiveDate, Utc};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    /// Name for Company Using the Log Collector?
    name: String,

    /// Sink Type.
    sink: HashMap<String, String>,

    /// Store
    stores: Vec<HashMap<String, String>>,

    /// Expiry of license
    expires: String,

    /// License for using the log collector
    #[serde(with = "indexmap::serde_seq")]
    license: IndexMap<String, String>,
}

impl Settings {
    /// Create Settings from Config File.
    pub fn from_file(path: &str, demo: bool) -> anyhow::Result<Self> {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name(path))?;
        let parsed_settings: Settings = settings.try_into()?;

        let license_id = parsed_settings
            .get_license()
            .get("license_id")
            .ok_or(anyhow::Error::msg("No License id provided"))?
            .to_string();

        let email = parsed_settings
            .get_license()
            .get("email")
            .ok_or(anyhow::Error::msg("No key `email` provided"))?
            .to_string();

        if !demo
            && !lic_gen::verify_lic_id(
                create_license_string(parsed_settings.get_license()),
                email,
                license_id,
            )
        {
            return Err(anyhow::Error::msg("Unverified License"));
        }
        Ok(parsed_settings)
    }

    /// Get the `name` property
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_stores(&self) -> &[HashMap<String, String>] {
        &self.stores
    }

    /// Get License
    pub fn get_license(&self) -> &IndexMap<String, String> {
        &self.license
    }

    pub fn get_sink(&self) -> &HashMap<String, String> {
        &self.sink
    }
    /// Get Expires
    pub fn get_expires(&self) -> anyhow::Result<Date<Utc>> {
        Ok(Date::from_utc(
            NaiveDate::parse_from_str(&self.expires, "%Y-%m-%d")?,
            Utc,
        ))
    }
}

pub fn create_license_string(map: &IndexMap<String, String>) -> String {
    map.values()
        .map(|v| format!("{}\n", v))
        .collect::<String>()
        .trim_end()
        .to_string()
}
