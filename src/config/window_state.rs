use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowState {
    pub position: Option<(i32, i32)>,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState {
            position: None,
            width: 800,
            height: 600,
            maximized: false,
        }
    }
}