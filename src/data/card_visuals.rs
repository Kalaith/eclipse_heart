//! Card template and layout metadata for renderer-facing visuals.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardVisualSpec {
    pub canvas: CardCanvas,
    pub zones: Vec<LayoutZone>,
    pub template_families: Vec<TemplateFamilySpec>,
    pub speed_badges: Vec<SpeedBadgeSpec>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardCanvas {
    pub width: u32,
    pub height: u32,
    pub safe_margin: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutZone {
    pub id: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateFamilySpec {
    pub id: String,
    pub display_name: String,
    pub asset_name: String,
    pub background_asset: String,
    pub frame_asset: String,
    pub ornament_asset: String,
    pub textbox_asset: String,
    pub art_mask_asset: String,
    pub gloss_asset: String,
    pub palette: Vec<String>,
    pub motifs: Vec<String>,
    pub tone: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeedBadgeSpec {
    pub id: String,
    pub short_label: String,
    pub accent_color: String,
    pub base_asset: String,
    pub icon_asset: String,
    pub badge_asset: String,
}
