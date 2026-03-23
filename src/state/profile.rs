//! Versioned player profile save data.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileSave {
    pub version: u32,
    pub player_name: String,
    pub total_matches_played: u32,
    pub total_wins: u32,
}

impl Default for ProfileSave {
    fn default() -> Self {
        Self {
            version: 1,
            player_name: "Player".to_owned(),
            total_matches_played: 0,
            total_wins: 0,
        }
    }
}
