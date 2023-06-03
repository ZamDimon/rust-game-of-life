use config::{Config, ConfigError, File};
use serde::Deserialize;
use bevy::prelude::{Color};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub default_window_settings: DefaultWindowSettings,
    pub field_settings: FieldSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultWindowSettings {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub clear_color: Color,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FieldSettings {
    pub block_size: f32,
}

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Initializing a config entity
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Default".into());
        let mut config = Config::new();
        config.set("env", env.clone())?;

        config.merge(File::with_name(CONFIG_FILE_PATH))?;
        config.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;
        
        // Try putting into the raw structures first
        let raw_settings: RawSettings = config.try_into()?;
        Ok(Settings::from(raw_settings))
    }
}

// --- Raw types below ---

#[derive(Debug, Deserialize, Clone)]
struct RawSettings {
    default_window_settings: RawDefaultWindowSettings,
    field_settings: FieldSettings,
}

#[derive(Debug, Deserialize, Clone)]
struct RawDefaultWindowSettings {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub clear_color: (f32, f32, f32),
}

impl From<RawDefaultWindowSettings> for DefaultWindowSettings {
    fn from(raw_settings: RawDefaultWindowSettings) -> Self {
        let (r, g, b) = raw_settings.clear_color;
        DefaultWindowSettings { 
            name: raw_settings.name, 
            width: raw_settings.width, 
            height: raw_settings.height, 
            clear_color: Color::rgb(r, g, b),
        }
    }
}

impl From<RawSettings> for Settings {
    fn from(raw_settings: RawSettings) -> Self {
        Settings { 
            default_window_settings: DefaultWindowSettings::from(raw_settings.default_window_settings),
            field_settings: raw_settings.field_settings,
        }
    }
}