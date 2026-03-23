//! Versioned collection save data.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionSave {
    pub version: u32,
    pub owned_magical_girls: Vec<String>,
    pub owned_baddies: Vec<String>,
    pub owned_story_cards: Vec<String>,
}

impl Default for CollectionSave {
    fn default() -> Self {
        Self {
            version: 1,
            owned_magical_girls: Vec::new(),
            owned_baddies: Vec::new(),
            owned_story_cards: Vec::new(),
        }
    }
}
