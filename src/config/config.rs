use super::window_state::WindowState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use sdl3::video::WindowBuilder;
use sdl3::VideoSubsystem;

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
    
    pub fn create_window_builder(&mut self, video_subsystem: &VideoSubsystem, title: &str) -> Result<WindowBuilder, Box<dyn std::error::Error>> {
        let mut window_builder = video_subsystem
            .window(title, self.window_state.width, self.window_state.height);
        if let Some((x, y)) = self.window_state.position {
            window_builder.position(x, y);
        }
        window_builder.resizable();
        if self.window_state.maximized {
            window_builder.maximized();
        }
        Ok(window_builder)
    }
}