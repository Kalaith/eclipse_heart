//! Support deck builder shell.

mod browser;
mod controls;
mod input;
mod layout;
mod preview;
mod render_main;
mod roster_dialogs;
mod types;
mod utils;

use macroquad::prelude::*;

use self::types::*;
use crate::state::{DeckFilterState, DeckGroupMode, DeckSortMode, DeckViewMode};

const MAX_DECK_NAME_LENGTH: usize = 28;
pub struct DeckBuilderScreen {
    selected_template_index: Option<usize>,
    active_tab: DeckBuilderTab,
    selected_magical_girl_slot: Option<usize>,
    selected_baddie_slot: Option<usize>,
    rename_dialog: Option<DeckRenameDialog>,
    import_export_dialog: Option<DeckImportExportDialog>,
    metadata_dialog: Option<DeckMetadataDialog>,
    search_text: String,
    search_focused: bool,
    filters: DeckFilterState,
    sort_mode: DeckSortMode,
    group_mode: DeckGroupMode,
    view_mode: DeckViewMode,
}

impl Default for DeckBuilderScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl DeckBuilderScreen {
    pub fn new() -> Self {
        Self {
            selected_template_index: None,
            active_tab: DeckBuilderTab::SupportCards,
            selected_magical_girl_slot: None,
            selected_baddie_slot: None,
            rename_dialog: None,
            import_export_dialog: None,
            metadata_dialog: None,
            search_text: String::new(),
            search_focused: false,
            filters: DeckFilterState::default(),
            sort_mode: DeckSortMode::Alphabetical,
            group_mode: DeckGroupMode::None,
            view_mode: DeckViewMode::Grid,
        }
    }
}
