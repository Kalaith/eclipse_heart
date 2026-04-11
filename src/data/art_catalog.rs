//! Procedural art asset catalog for card illustrations, portraits, and UI backdrops.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArtCatalog {
    pub ui_backgrounds: Vec<UiArtSpec>,
    pub character_portraits: Vec<CharacterArtSpec>,
    pub story_card_art: Vec<StoryCardArtSpec>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiArtSpec {
    pub id: String,
    pub asset_name: String,
    pub tone: String,
    pub motifs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterArtSpec {
    pub id: String,
    pub role: String,
    pub asset_name: String,
    pub accent_color: String,
    pub motifs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryCardArtSpec {
    pub id: String,
    pub asset_name: String,
    pub alignment: String,
    pub speed: String,
    pub card_type: String,
    pub motifs: Vec<String>,
}
