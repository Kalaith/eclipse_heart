//! Shared texture loading for generated UI and card art.

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad_toolkit::assets::{load_texture_from_pack_or_file, AssetPack};

use crate::data::{CardAlignment, CardSpeed, GameContent};

const ASSET_PACK_PATH: &str = "assets.zip";

pub struct UiAssets {
    ui_backgrounds: HashMap<String, Texture2D>,
    portraits: HashMap<String, Texture2D>,
    story_card_art: HashMap<String, Texture2D>,
    card_templates: HashMap<String, Texture2D>,
    speed_badges: HashMap<String, Texture2D>,
}

impl UiAssets {
    pub async fn load(content: &GameContent) -> Self {
        let asset_pack = AssetPack::load(ASSET_PACK_PATH).await.ok();
        let mut ui_backgrounds = HashMap::new();
        for background in &content.art_catalog.ui_backgrounds {
            if let Ok(texture) =
                load_asset_texture(asset_pack.as_ref(), &background.asset_name).await
            {
                ui_backgrounds.insert(background.id.clone(), texture);
            }
        }

        let mut portraits = HashMap::new();
        for portrait in &content.art_catalog.character_portraits {
            if let Ok(texture) = load_asset_texture(asset_pack.as_ref(), &portrait.asset_name).await
            {
                portraits.insert(portrait.id.clone(), texture);
            }
        }

        let mut story_card_art = HashMap::new();
        for art in &content.art_catalog.story_card_art {
            if let Ok(texture) = load_asset_texture(asset_pack.as_ref(), &art.asset_name).await {
                story_card_art.insert(art.id.clone(), texture);
            }
        }

        let mut card_templates = HashMap::new();
        for family in &content.card_visuals.template_families {
            if let Ok(texture) = load_asset_texture(asset_pack.as_ref(), &family.asset_name).await {
                card_templates.insert(family.id.clone(), texture);
            }
        }

        let mut speed_badges = HashMap::new();
        for badge in &content.card_visuals.speed_badges {
            if let Ok(texture) = load_asset_texture(asset_pack.as_ref(), &badge.badge_asset).await {
                speed_badges.insert(badge.id.clone(), texture);
            }
        }

        Self {
            ui_backgrounds,
            portraits,
            story_card_art,
            card_templates,
            speed_badges,
        }
    }

    pub fn ui_background(&self, id: &str) -> Option<&Texture2D> {
        self.ui_backgrounds.get(id)
    }

    pub fn portrait(&self, id: &str) -> Option<&Texture2D> {
        self.portraits.get(id)
    }

    pub fn story_card_art(&self, id: &str) -> Option<&Texture2D> {
        self.story_card_art.get(id)
    }

    pub fn template_for_alignment(&self, alignment: CardAlignment) -> Option<&Texture2D> {
        let family_id = match alignment {
            CardAlignment::MagicalGirl => "magical_girl",
            CardAlignment::Baddie => "baddie",
            CardAlignment::Neutral => "neutral",
        };
        self.card_templates.get(family_id)
    }

    pub fn badge_for_speed(&self, speed: CardSpeed) -> Option<&Texture2D> {
        let badge_id = match speed {
            CardSpeed::DailyLife => "daily_life",
            CardSpeed::Reaction => "reaction",
            CardSpeed::Encounter => "encounter",
        };
        self.speed_badges.get(badge_id)
    }
}

async fn load_asset_texture(
    asset_pack: Option<&AssetPack>,
    relative_asset_path: &str,
) -> Result<Texture2D, String> {
    let packed_path = format!("assets/{relative_asset_path}");
    if let Some(pack) = asset_pack {
        if let Ok(texture) = pack.texture(&packed_path, FilterMode::Linear) {
            return Ok(texture);
        }
    }

    load_texture_from_pack_or_file(
        None,
        &asset_file_path(relative_asset_path),
        FilterMode::Linear,
    )
    .await
}

fn asset_file_path(relative_asset_path: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        format!("assets/{relative_asset_path}")
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        native_asset_path(relative_asset_path)
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn native_asset_path(relative_asset_path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(relative_asset_path)
}
