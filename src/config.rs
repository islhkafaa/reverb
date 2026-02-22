use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const SUPPORTED_EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "ogg"];

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub music_dir: PathBuf,
    pub volume_increment: f32,
    pub colors: ColorConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorConfig {
    pub primary: String,
    pub secondary: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            primary: "#6496ff".to_string(),
            secondary: "#ff64c8".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_dir = dirs::config_dir()
            .map(|d| d.join("reverb"))
            .unwrap_or_else(|| PathBuf::from("."));
        let config_path = config_dir.join("config.toml");

        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }

        Self::default()
    }

    pub fn new(music_dir_override: Option<PathBuf>) -> Self {
        let mut config = Self::load();
        if let Some(m_dir) = music_dir_override {
            config.music_dir = m_dir;
        }
        config
    }
}

impl Default for Config {
    fn default() -> Self {
        let music_dir = dirs::audio_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join("Music")))
            .unwrap_or_else(|| PathBuf::from("."));

        Self {
            music_dir,
            volume_increment: 0.1,
            colors: ColorConfig::default(),
        }
    }
}
