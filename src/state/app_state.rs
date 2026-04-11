//! Top-level app state.

use crate::data::{GameContent, UiText};
use crate::ui::assets::UiAssets;

use super::{CollectionCardKind, MatchSetup, MatchState, PersistenceBundle, PersistenceManager};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppScreen {
    Menu,
    CampaignMenu,
    CampaignHub,
    Setup,
    DeckBuilder,
    Battle,
}

#[derive(Clone, Debug)]
pub struct BoosterCardGrant {
    pub kind: CollectionCardKind,
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum BattleContext {
    Skirmish,
    Campaign { run_id: String, node_id: String },
}

pub struct AppState {
    pub screen: AppScreen,
    pub ui_text: UiText,
    pub content: GameContent,
    pub assets: UiAssets,
    pub setup: MatchSetup,
    pub match_state: Option<MatchState>,
    pub battle_context: BattleContext,
    pub campaign_notice: Option<String>,
    pub saves: PersistenceBundle,
    pub persistence: PersistenceManager,
    pub last_opened_booster: Vec<BoosterCardGrant>,
}

impl AppState {
    pub async fn new(ui_text: UiText, content: GameContent) -> Self {
        let assets = UiAssets::load(&content).await;
        let setup = MatchSetup::default_for_content(&content);
        let persistence = PersistenceManager::default_local();
        let mut saves = persistence.load_all().unwrap_or_default();
        saves.collection.ensure_full_roster_owned(
            content.magical_girls.iter().map(|entry| entry.id.clone()),
            content.baddies.iter().map(|entry| entry.id.clone()),
        );
        let magical_girl_ids = content
            .magical_girls
            .iter()
            .map(|entry| entry.id.clone())
            .collect::<Vec<_>>();
        let baddie_ids = content
            .baddies
            .iter()
            .map(|entry| entry.id.clone())
            .collect::<Vec<_>>();
        saves
            .decks
            .ensure_valid_support_decks(&magical_girl_ids, &baddie_ids);
        Self {
            screen: AppScreen::Menu,
            ui_text,
            content,
            assets,
            setup,
            match_state: None,
            battle_context: BattleContext::Skirmish,
            campaign_notice: None,
            saves,
            persistence,
            last_opened_booster: Vec::new(),
        }
    }
}
