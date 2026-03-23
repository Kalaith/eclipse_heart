//! Data definitions and JSON loading.

mod card_visuals;
mod cards;
mod loader;
mod loadouts;
mod rules;
mod ui_text;

pub use card_visuals::{CardCanvas, CardVisualSpec};
pub use cards::{CardEffect, CardSpeed, CharacterDefinition, StoryCardDefinition};
pub use loader::GameContent;
pub use loadouts::StarterLoadout;
pub use rules::{DeckRules, MatchRules, ProgressionRules};
pub use ui_text::UiText;
