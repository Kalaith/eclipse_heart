//! Versioned settings save data.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsSave {
    pub version: u32,
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
}

impl Default for SettingsSave {
    fn default() -> Self {
        Self {
            version: 1,
            window_width: 2560,
            window_height: 1440,
            fullscreen: true,
        }
    }
}
