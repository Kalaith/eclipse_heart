//! Runtime and save state.

mod app_state;
mod collection;
mod decks;
mod match_state;
mod persistence;
mod profile;
mod settings;

pub use app_state::{AppScreen, AppState, BoosterCardGrant};
pub use collection::{CollectionCardKind, CollectionSave};
pub use decks::DecksSave;
pub use match_state::{
    opposing, CharacterStage, MatchPhase, MatchSetup, MatchState, PlayerId, ReactionState,
    ResourceKind, SideState, StackItem, StackItemKind, SupportState,
};
pub use persistence::{PersistenceBundle, PersistenceManager};
pub use profile::ProfileSave;
pub use settings::SettingsSave;
