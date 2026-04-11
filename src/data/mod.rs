//! Data definitions and JSON loading.

mod art_catalog;
mod campaigns;
mod card_visuals;
mod cards;
mod loader;
mod loadouts;
mod rules;
mod ui_text;

pub use art_catalog::{ArtCatalog, CharacterArtSpec, StoryCardArtSpec, UiArtSpec};
pub use campaigns::{CampaignDefinition, CampaignEncounterDefinition, CampaignNodeDefinition};
pub use card_visuals::{CardCanvas, CardVisualSpec};
pub use cards::{CardAlignment, CardEffect, CardSpeed, CharacterDefinition, StoryCardDefinition};
pub use loader::GameContent;
pub use loadouts::StarterLoadout;
pub use rules::{DeckRules, MatchRules, ProgressionRules};
pub use ui_text::UiText;
