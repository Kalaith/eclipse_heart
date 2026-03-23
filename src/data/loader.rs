//! JSON loading helpers.

use std::collections::HashMap;

use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::data::{
    CardVisualSpec, CharacterDefinition, DeckRules, MatchRules, ProgressionRules, StarterLoadout,
    StoryCardDefinition, UiText,
};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse json: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Clone, Debug)]
pub struct GameContent {
    pub magical_girls: Vec<CharacterDefinition>,
    pub baddies: Vec<CharacterDefinition>,
    pub story_cards: Vec<StoryCardDefinition>,
    pub rules: MatchRules,
    pub deck_rules: DeckRules,
    pub progression_rules: ProgressionRules,
    pub starter_loadouts: Vec<StarterLoadout>,
    pub card_visuals: CardVisualSpec,
}

impl Default for GameContent {
    fn default() -> Self {
        Self {
            magical_girls: Vec::new(),
            baddies: Vec::new(),
            story_cards: Vec::new(),
            rules: MatchRules::default(),
            deck_rules: DeckRules::default(),
            progression_rules: ProgressionRules::default(),
            starter_loadouts: Vec::new(),
            card_visuals: CardVisualSpec {
                canvas: crate::data::CardCanvas {
                    width: 0,
                    height: 0,
                    safe_margin: 0,
                },
                zones: Vec::new(),
                template_families: Vec::new(),
                speed_badges: Vec::new(),
            },
        }
    }
}

impl GameContent {
    pub fn load() -> Result<Self, LoadError> {
        let magical_girls =
            load_json::<Vec<CharacterDefinition>>("assets/data/magical_girls/prototype_set.json")?;
        let baddies =
            load_json::<Vec<CharacterDefinition>>("assets/data/baddies/prototype_set.json")?;
        let story_cards =
            load_json::<Vec<StoryCardDefinition>>("assets/data/story_cards/prototype_set.json")?;
        let rules = load_json::<MatchRules>("assets/data/rules/match_rules.json")?;
        let deck_rules = load_json::<DeckRules>("assets/data/rules/deck_rules.json")?;
        let progression_rules =
            load_json::<ProgressionRules>("assets/data/rules/progression_rules.json")?;
        let starter_loadouts = load_json::<Vec<StarterLoadout>>(
            "assets/data/starter_loadouts/prototype_starters.json",
        )?;
        let card_visuals = load_json::<CardVisualSpec>("assets/data/card_visuals.json")?;
        Ok(Self {
            magical_girls,
            baddies,
            story_cards,
            rules,
            deck_rules,
            progression_rules,
            starter_loadouts,
            card_visuals,
        })
    }
}

impl UiText {
    pub fn load() -> Result<Self, LoadError> {
        let values = load_json::<HashMap<String, String>>("assets/data/ui_text.json")?;
        Ok(Self { values })
    }
}

fn load_json<T>(path: &str) -> Result<T, LoadError>
where
    T: DeserializeOwned,
{
    let text = std::fs::read_to_string(path)?;
    let value = serde_json::from_str::<T>(&text)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use crate::data::UiText;

    use super::GameContent;

    #[test]
    fn loads_all_core_content_files() {
        let content = GameContent::load().expect("content loads");

        assert_eq!(content.magical_girls.len(), 5);
        assert_eq!(content.baddies.len(), 5);
        assert_eq!(content.story_cards.len(), 32);
        assert_eq!(content.deck_rules.support_deck_size, 40);
        assert!(content.progression_rules.overflow_is_lost);
        assert!(!content.starter_loadouts.is_empty());
        assert_eq!(content.card_visuals.canvas.width, 750);
        assert_eq!(content.card_visuals.template_families.len(), 3);
        assert_eq!(content.card_visuals.speed_badges.len(), 3);
    }

    #[test]
    fn loads_ui_text_catalog() {
        let ui_text = UiText::load().expect("ui text loads");

        assert_eq!(ui_text.get("menu_title"), "Eclipse Heart");
        assert_eq!(ui_text.get("deck_builder_title"), "Deck Builder");
        assert_eq!(ui_text.get("battle_stage_label"), "Stage");
    }
}
