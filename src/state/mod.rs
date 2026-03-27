//! Runtime and save state.

mod app_state;
mod campaigns;
mod collection;
mod deck_browser;
mod deck_filter;
mod deck_import_export;
mod deck_replacements;
mod deck_search;
mod deck_validation;
mod decks;
mod match_state;
mod persistence;
mod profile;
mod settings;

pub use app_state::{AppScreen, AppState, BattleContext, BoosterCardGrant};
pub use campaigns::{CampaignRunSave, CampaignRunStatus, CampaignSaveBundle};
pub use collection::{CollectionCardKind, CollectionSave};
pub use deck_browser::{
    card_group_label, compare_story_cards, DeckBrowserCardStats, DeckGroupMode, DeckSortMode,
    DeckViewMode,
};
pub use deck_filter::DeckFilterState;
pub use deck_import_export::{export_deck_code, import_deck_code, DeckCodeError, ImportedDeck};
pub use deck_replacements::{suggest_story_replacements, DeckReplacementSuggestion};
pub use deck_search::{DeckSearchCardContext, DeckSearchQuery};
pub use deck_validation::{DeckValidation, DeckValidationCount};
pub use decks::{DeckPreset, DecksSave};
pub use match_state::{
    opposing, CharacterStage, MatchPhase, MatchSetup, MatchState, PlayerId, ReactionState,
    ResourceKind, SideState, StackItem, StackItemKind, SupportState,
};
pub use persistence::{PersistenceBundle, PersistenceManager};
pub use profile::ProfileSave;
pub use settings::SettingsSave;
