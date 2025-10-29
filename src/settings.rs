use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WatchSettings {
    pub time_format_24h: bool,
    pub date_format_us: bool, // true for MM/DD, false for DD/MM
    pub auto_light_duration: u64, // seconds
    pub alarm_enabled: bool,
    pub alarm_time: Option<String>, // HH:MM format
}

impl Default for WatchSettings {
    fn default() -> Self {
        Self {
            time_format_24h: false, // default to 12-hour format
            date_format_us: true,   // default to US date format (MM/DD)
            auto_light_duration: 1, // in seconds
            alarm_enabled: false,
            alarm_time: None,
        }
    }
}

impl WatchSettings {
    pub fn load() -> Result<Self> {
        let path = "casiotoy.json";
        if Path::new(path).exists() {
            let data = fs::read_to_string(path)?;
            let settings: WatchSettings = serde_json::from_str(&data)?;
            Ok(settings)
        } else {
            // create default settings if none exist
            let settings = WatchSettings::default();
            settings.save()?;
            Ok(settings)
        }
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write("casiotoy.json", data)?;
        Ok(())
    }
}
