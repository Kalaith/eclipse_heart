//! Starter loadout definitions.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarterLoadout {
    pub id: String,
    pub name: String,
    pub magical_girl_main: String,
    pub magical_girl_supports: Vec<String>,
    pub prime_baddie: String,
    pub baddie_supports: Vec<String>,
    pub support_deck: Vec<String>,
}
