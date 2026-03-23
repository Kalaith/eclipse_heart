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
use crate::state::PlayerId;

#[derive(Clone, Debug)]
pub enum ScreenAction {
    None,
    OpenSetup,
    OpenDeckBuilder,
    ToggleWindowedMode,
    ExitGame,
    DeckBuilderOpenBooster,
    DeckBuilderSetRosterSlot {
        is_magical_girl_side: bool,
        slot_index: usize,
        character_id: String,
    },
    SetupCyclePlayerAMgMain,
    SetupCyclePlayerAMgSupports,
    SetupCyclePlayerABaddieMain,
    SetupCyclePlayerABaddieSupports,
    SetupCyclePlayerBMgMain,
    SetupCyclePlayerBMgSupports,
    SetupCyclePlayerBBaddieMain,
    SetupCyclePlayerBBaddieSupports,
    SetupSelectMain {
        player: PlayerId,
        is_magical_girl_side: bool,
        main_index: usize,
    },
    SetupSelectSupportPair {
        player: PlayerId,
        is_magical_girl_side: bool,
        pair_index: usize,
    },
    DeckBuilderLoadStarter {
        loadout_index: usize,
    },
    DeckBuilderAddCard {
        card_id: String,
    },
    DeckBuilderRemoveCard {
        card_id: String,
    },
    StartConfiguredBattle,
    BackToMenu,
    ApplyMatchAction(MatchAction),
}
