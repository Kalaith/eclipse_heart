//! Shared texture loading for generated UI and card art.

use std::path::PathBuf;

use std::collections::HashMap;

use macroquad::prelude::*;

use crate::data::{CardAlignment, CardSpeed, GameContent};

pub struct UiAssets {
    ui_backgrounds: HashMap<String, Texture2D>,
    portraits: HashMap<String, Texture2D>,
    story_card_art: HashMap<String, Texture2D>,
    card_templates: HashMap<String, Texture2D>,
    speed_badges: HashMap<String, Texture2D>,
}

impl UiAssets {
    pub async fn load(content: &GameContent) -> Self {
        let mut ui_backgrounds = HashMap::new();
        for background in &content.art_catalog.ui_backgrounds {
            if let Ok(texture) = load_texture(&asset_file_path(&background.asset_name)).await {
                texture.set_filter(FilterMode::Linear);
                ui_backgrounds.insert(background.id.clone(), texture);
            }
        }

        let mut portraits = HashMap::new();
        for portrait in &content.art_catalog.character_portraits {
            if let Ok(texture) = load_texture(&asset_file_path(&portrait.asset_name)).await {
                texture.set_filter(FilterMode::Linear);
                portraits.insert(portrait.id.clone(), texture);
            }
        }

        let mut story_card_art = HashMap::new();
        for art in &content.art_catalog.story_card_art {
            if let Ok(texture) = load_texture(&asset_file_path(&art.asset_name)).await {
                texture.set_filter(FilterMode::Linear);
                story_card_art.insert(art.id.clone(), texture);
            }
        }

        let mut card_templates = HashMap::new();
        for family in &content.card_visuals.template_families {
            if let Ok(texture) = load_texture(&asset_file_path(&family.asset_name)).await {
                texture.set_filter(FilterMode::Linear);
                card_templates.insert(family.id.clone(), texture);
            }
        }

        let mut speed_badges = HashMap::new();
        for badge in &content.card_visuals.speed_badges {
            if let Ok(texture) = load_texture(&asset_file_path(&badge.badge_asset)).await {
                texture.set_filter(FilterMode::Linear);
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
