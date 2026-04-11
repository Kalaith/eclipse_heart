//! Screen controllers.

mod battle;
mod campaign_hub;
mod campaign_menu;
mod deck_builder;
mod menu;
mod setup;

pub use battle::BattleScreen;
pub use campaign_hub::CampaignHubScreen;
pub use campaign_menu::CampaignMenuScreen;
pub use deck_builder::DeckBuilderScreen;
pub use menu::MenuScreen;
pub use setup::SetupScreen;

use crate::engine::MatchAction;
use crate::state::PlayerId;

#[derive(Clone, Debug)]
pub enum ScreenAction {
    None,
    OpenCampaignMenu,
    OpenSetup,
    OpenDeckBuilder,
    ToggleEscapeMenu,
    EscapeMenuSave,
    EscapeMenuExitToMainMenu,
    ToggleWindowedMode,
    ExitGame,
    DeckBuilderOpenBooster,
    DeckBuilderCreateEmptyDeck,
    DeckBuilderSelectDeck {
        deck_id: String,
    },
    DeckBuilderRenameSelectedDeck {
        name: String,
    },
    DeckBuilderDuplicateSelectedDeck,
    DeckBuilderDeleteSelectedDeck,
    DeckBuilderImportDeckCode {
        code: String,
    },
    DeckBuilderSaveMetadata {
        notes: String,
        tags: Vec<String>,
    },
    DeckBuilderUndoSelectedDeckChange,
    DeckBuilderResetSelectedDeckToTemplate,
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
    SetupUseSelectedDeck {
        player: PlayerId,
    },
    SetupClearAssignedDeck {
        player: PlayerId,
    },
    CampaignSelectRun {
        run_id: String,
    },
    CampaignToggleSupportSelection {
        character_id: String,
    },
    CampaignStartNewRun,
    CampaignContinueRun,
    CampaignAbandonRun,
    CampaignStartEncounter,
    DeckBuilderCreateDeckFromTemplate {
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
