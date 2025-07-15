use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowState {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
    pub display_index: Option<usize>,
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState {
            x: 100,
            y: 100,
            width: 800,
            height: 600,
            maximized: false,
            display_index: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub window_state: WindowState,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window_state: WindowState::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(config_path) = Self::config_path() {
            if let Ok(contents) = fs::read_to_string(&config_path) {
                if let Ok(config) = serde_json::from_str(&contents) {
                    return config;
                }
            }
        }
        Config::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_path) = Self::config_path() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let contents = serde_json::to_string_pretty(self)?;
            fs::write(config_path, contents)?;
        }
        Ok(())
    }

    fn config_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "editor", "editor")
            .map(|dirs| dirs.config_dir().join("window_state.json"))
    }
}