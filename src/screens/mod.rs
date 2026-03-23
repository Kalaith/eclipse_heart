//! Screen controllers.

mod battle;
mod deck_builder;
mod menu;
mod setup;

pub use battle::BattleScreen;
pub use deck_builder::DeckBuilderScreen;
pub use menu::MenuScreen;
pub use setup::SetupScreen;

use crate::engine::MatchAction;

#[derive(Clone, Debug)]
pub enum ScreenAction {
    None,
    OpenSetup,
    OpenDeckBuilder,
    SetupCyclePlayerAMgMain,
    SetupCyclePlayerAMgSupports,
    SetupCyclePlayerABaddieMain,
    SetupCyclePlayerABaddieSupports,
    SetupCyclePlayerBMgMain,
    SetupCyclePlayerBMgSupports,
    SetupCyclePlayerBBaddieMain,
    SetupCyclePlayerBBaddieSupports,
    DeckBuilderLoadStarter { loadout_index: usize },
    DeckBuilderAddCard { card_id: String },
    DeckBuilderRemoveCard { card_id: String },
    StartConfiguredBattle,
    BackToMenu,
    ApplyMatchAction(MatchAction),
}
