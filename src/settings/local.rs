use bevy::{ecs::system::Resource, prelude::*};
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::time::Duration;

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().expect("config cannot be loaded!");
}

#[derive(Default, Debug, Deserialize, Clone, Resource)]
pub struct Settings {
    pub default_window_settings: DefaultWindowSettings,
    pub field_settings: FieldSettings,
    pub rule: Rule,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct DefaultWindowSettings {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub clear_color: Color,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct FieldSettings {
    pub block_size: f32,
    pub cell_color_from: Color,
    pub cell_color_to: Color,
    pub updating_frequency: Duration,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Rule {
    pub survive: Vec<u32>,
    pub birth: Vec<u32>,
    pub health: u32,
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

    pub fn window_plugin(self) -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                resolution: (
                    self.default_window_settings.width,
                    self.default_window_settings.height,
                )
                    .into(),
                title: self.default_window_settings.name,
                ..default()
            }),
            ..default()
        }
    }
}

// --- Raw types below ---

#[derive(Debug, Deserialize, Clone)]
struct RawSettings {
    default_window_settings: RawDefaultWindowSettings,
    field_settings: RawFieldSettings,
    rule: Rule,
}

#[derive(Debug, Deserialize, Clone)]
struct RawDefaultWindowSettings {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub clear_color: (f32, f32, f32),
}

#[derive(Debug, Deserialize, Clone)]
struct RawFieldSettings {
    pub block_size: f32,
    pub cell_color_gradient: ((f32, f32, f32), (f32, f32, f32)),
    pub updating_frequency: u64,
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

impl From<RawFieldSettings> for FieldSettings {
    fn from(raw_settings: RawFieldSettings) -> Self {
        let ((r_from, g_from, b_from), (r_to, g_to, b_to)) = raw_settings.cell_color_gradient;
        FieldSettings {
            block_size: raw_settings.block_size,
            cell_color_from: Color::rgb(r_from, g_from, b_from),
            cell_color_to: Color::rgb(r_to, g_to, b_to),
            updating_frequency: Duration::from_millis(raw_settings.updating_frequency),
        }
    }
}

impl From<RawSettings> for Settings {
    fn from(raw_settings: RawSettings) -> Self {
        Settings {
            default_window_settings: DefaultWindowSettings::from(
                raw_settings.default_window_settings,
            ),
            field_settings: FieldSettings::from(raw_settings.field_settings),
            rule: raw_settings.rule,
        }
    }
}
