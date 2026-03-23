//! Top-level app state.

use crate::data::{GameContent, UiText};

use super::{MatchSetup, MatchState, PersistenceBundle, PersistenceManager};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppScreen {
    Menu,
    Setup,
    DeckBuilder,
    Battle,
}

pub struct AppState {
    pub screen: AppScreen,
    pub ui_text: UiText,
    pub content: GameContent,
    pub setup: MatchSetup,
    pub match_state: Option<MatchState>,
    pub saves: PersistenceBundle,
    pub persistence: PersistenceManager,
}

impl AppState {
    pub fn new(ui_text: UiText, content: GameContent) -> Self {
        let setup = MatchSetup::default_for_content(&content);
        let persistence = PersistenceManager::default_local();
        let mut saves = persistence.load_all().unwrap_or_default();
        saves
            .decks
            .ensure_active_support_deck(&content.starter_loadouts);
        Self {
            screen: AppScreen::Menu,
            ui_text,
            content,
            setup,
            match_state: None,
            saves,
            persistence,
        }
    }
}
