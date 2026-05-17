//! Support deck builder shell.

use macroquad::prelude::*;

use crate::data::CharacterDefinition;
use crate::screens::ScreenAction;
use crate::state::{
    card_group_label, compare_story_cards, AppState, BoosterCardGrant, CollectionCardKind,
    DeckBrowserCardStats, DeckCodeError, DeckFilterState, DeckGroupMode, DeckReplacementSuggestion,
    DeckSearchCardContext, DeckSearchQuery, DeckSortMode, DeckValidation, DeckValidationCount,
    DeckViewMode,
};
use crate::ui::card_widgets::{
    action_button, draw_story_card_preview, draw_story_card_tile, point_in_rect, section_panel,
};
use crate::ui::core::{draw_background_texture, draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

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

    pub fn update(&mut self, state: &AppState) -> ScreenAction {
        let ui = UiLayout::current();
        let mouse = mouse_position();

        if let Some(action) = self.update_import_export_dialog(state, mouse) {
            return action;
        }
        if let Some(action) = self.update_metadata_dialog(mouse) {
            return action;
        }

        if let Some(action) = self.update_rename_dialog(mouse) {
            return action;
        }

        self.update_search_input(mouse);
        self.update_filter_controls(state, mouse);
        self.update_browser_mode_controls(mouse);

        if action_button(
            ui.rect(80.0, 1328.0, 360.0, 70.0),
            state.ui_text.get("battle_back_to_menu"),
        ) {
            return ScreenAction::BackToMenu;
        }

        if action_button(
            ui.rect(80.0, 1242.0, 360.0, 70.0),
            state.ui_text.get("deck_builder_open_booster"),
        ) {
            return ScreenAction::DeckBuilderOpenBooster;
        }

        for (button_index, action) in deck_action_buttons(state).into_iter().enumerate() {
            if !action.enabled {
                continue;
            }

            if action_button(deck_action_button_rect(button_index), action.label) {
                match action.kind {
                    DeckActionKind::Create => return ScreenAction::DeckBuilderCreateEmptyDeck,
                    DeckActionKind::Rename => {
                        if let Some(deck) = state.saves.decks.selected_support_deck() {
                            self.rename_dialog = Some(DeckRenameDialog::new(&deck.name));
                        }
                    }
                    DeckActionKind::Duplicate => {
                        return ScreenAction::DeckBuilderDuplicateSelectedDeck;
                    }
                    DeckActionKind::Delete => return ScreenAction::DeckBuilderDeleteSelectedDeck,
                }
            }
        }

        for action in deck_transfer_buttons(state) {
            if !action.enabled {
                continue;
            }

            if action_button(deck_transfer_button_rect(action.kind), action.label) {
                match action.kind {
                    DeckTransferActionKind::Export => {
                        if let Some(deck) = state.saves.decks.selected_support_deck() {
                            self.import_export_dialog = Some(DeckImportExportDialog::for_export(
                                crate::state::export_deck_code(deck),
                            ));
                        }
                    }
                    DeckTransferActionKind::Import => {
                        self.import_export_dialog = Some(DeckImportExportDialog::for_import());
                    }
                }
            }
        }
        for action in deck_utility_buttons(state) {
            if !action.enabled {
                continue;
            }

            if action_button(deck_utility_button_rect(action.kind), action.label) {
                match action.kind {
                    DeckUtilityActionKind::Metadata => {
                        if let Some(deck) = state.saves.decks.selected_support_deck() {
                            self.metadata_dialog = Some(DeckMetadataDialog::new(
                                &deck.notes,
                                &deck.archetype_tags.join(", "),
                            ));
                        }
                    }
                    DeckUtilityActionKind::Undo => {
                        return ScreenAction::DeckBuilderUndoSelectedDeckChange;
                    }
                    DeckUtilityActionKind::Reset => {
                        return ScreenAction::DeckBuilderResetSelectedDeckToTemplate;
                    }
                }
            }
        }

        if point_in_rect(deck_builder_tab_rect(DeckBuilderTab::SupportCards), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.active_tab = DeckBuilderTab::SupportCards;
        }
        if point_in_rect(deck_builder_tab_rect(DeckBuilderTab::MagicalGirls), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.active_tab = DeckBuilderTab::MagicalGirls;
        }
        if point_in_rect(deck_builder_tab_rect(DeckBuilderTab::Baddies), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.active_tab = DeckBuilderTab::Baddies;
        }

        for (deck_index, deck) in state.saves.decks.support_decks.iter().enumerate() {
            let row_rect = saved_deck_row_rect(deck_index);
            if point_in_rect(row_rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_template_index = None;
                return ScreenAction::DeckBuilderSelectDeck {
                    deck_id: deck.id.clone(),
                };
            }
        }

        for (loadout_index, _) in state.content.starter_loadouts.iter().enumerate() {
            let row_rect = starter_row_rect(loadout_index);
            let create_rect = starter_create_rect(loadout_index);
            if point_in_rect(row_rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_template_index = Some(loadout_index);
            }
            if point_in_rect(create_rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_template_index = Some(loadout_index);
                return ScreenAction::DeckBuilderCreateDeckFromTemplate { loadout_index };
            }
        }

        if self.active_tab != DeckBuilderTab::SupportCards {
            return self.update_roster_layer(state, mouse);
        }

        for item in self.browser_layout_items(state) {
            if let BrowserLayoutItem::Card(card_layout) = item {
                let can_add = state.saves.decks.can_add_card(
                    &card_layout.card.id,
                    &state.content.deck_rules,
                    &state.saves.collection,
                );
                let can_remove = state.saves.decks.card_count(&card_layout.card.id) > 0;

                if point_in_rect(card_layout.add_rect, mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                    && can_add
                {
                    return ScreenAction::DeckBuilderAddCard {
                        card_id: card_layout.card.id.clone(),
                    };
                }

                if point_in_rect(card_layout.remove_rect, mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                    && can_remove
                {
                    return ScreenAction::DeckBuilderRemoveCard {
                        card_id: card_layout.card.id.clone(),
                    };
                }
            }
        }

        ScreenAction::None
    }

    fn update_rename_dialog(&mut self, mouse: (f32, f32)) -> Option<ScreenAction> {
        let dialog = self.rename_dialog.as_mut()?;

        while let Some(character) = get_char_pressed() {
            if !character.is_control() && dialog.value.chars().count() < MAX_DECK_NAME_LENGTH {
                dialog.value.push(character);
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            dialog.value.pop();
        }

        if is_key_pressed(KeyCode::Escape) {
            self.rename_dialog = None;
            return Some(ScreenAction::None);
        }

        if is_key_pressed(KeyCode::Enter) {
            let new_name = dialog.value.trim().to_owned();
            self.rename_dialog = None;
            if !new_name.is_empty() {
                return Some(ScreenAction::DeckBuilderRenameSelectedDeck { name: new_name });
            }
            return Some(ScreenAction::None);
        }

        if point_in_rect(rename_dialog_save_rect(), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            let new_name = dialog.value.trim().to_owned();
            self.rename_dialog = None;
            if !new_name.is_empty() {
                return Some(ScreenAction::DeckBuilderRenameSelectedDeck { name: new_name });
            }
            return Some(ScreenAction::None);
        }

        if point_in_rect(rename_dialog_cancel_rect(), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.rename_dialog = None;
            return Some(ScreenAction::None);
        }

        if !point_in_rect(rename_dialog_rect(), mouse) && is_mouse_button_pressed(MouseButton::Left)
        {
            self.rename_dialog = None;
            return Some(ScreenAction::None);
        }

        None
    }

    fn update_search_input(&mut self, mouse: (f32, f32)) {
        if point_in_rect(search_input_rect(), mouse) && is_mouse_button_pressed(MouseButton::Left) {
            self.search_focused = true;
        }

        if point_in_rect(search_clear_rect(), mouse) && is_mouse_button_pressed(MouseButton::Left) {
            self.search_text.clear();
            self.search_focused = true;
        }

        if !point_in_rect(search_input_rect(), mouse)
            && !point_in_rect(search_clear_rect(), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.search_focused = false;
        }

        if !self.search_focused {
            return;
        }

        while let Some(character) = get_char_pressed() {
            if !character.is_control() {
                self.search_text.push(character);
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            self.search_text.pop();
        }

        if is_key_pressed(KeyCode::Escape) {
            self.search_text.clear();
            self.search_focused = false;
        }
    }

    fn update_filter_controls(&mut self, state: &AppState, mouse: (f32, f32)) {
        if self.active_tab != DeckBuilderTab::SupportCards
            || !is_mouse_button_pressed(MouseButton::Left)
        {
            return;
        }

        for (index, button) in self.filter_buttons(state).into_iter().enumerate() {
            if point_in_rect(filter_button_rect(index), mouse) {
                self.toggle_filter(button.kind);
                return;
            }
        }

        for (index, chip) in self.active_filter_chips(state).into_iter().enumerate() {
            if point_in_rect(filter_chip_rect(index), mouse) {
                self.remove_filter_chip(chip.kind);
                return;
            }
        }

        if point_in_rect(filter_clear_all_rect(), mouse) {
            self.filters.clear();
        }
    }

    fn update_browser_mode_controls(&mut self, mouse: (f32, f32)) {
        if self.active_tab != DeckBuilderTab::SupportCards
            || !is_mouse_button_pressed(MouseButton::Left)
        {
            return;
        }

        if point_in_rect(sort_mode_rect(), mouse) {
            self.sort_mode = next_sort_mode(self.sort_mode);
        }
        if point_in_rect(group_mode_rect(), mouse) {
            self.group_mode = next_group_mode(self.group_mode);
        }
        if point_in_rect(view_mode_rect(), mouse) {
            self.view_mode = next_view_mode(self.view_mode);
        }
    }

    pub fn draw(&self, state: &AppState) {
        let ui = UiLayout::current();
        if let Some(background) = state.assets.ui_background("menu") {
            draw_background_texture(background, Color::new(1.0, 1.0, 1.0, 0.78));
        }
        let active_deck = state.saves.decks.selected_support_deck();
        let deck_name = active_deck
            .map(|deck| deck.name.as_str())
            .unwrap_or(state.ui_text.get("deck_builder_missing_deck"));
        let deck_size = active_deck.map(|deck| deck.story_cards.len()).unwrap_or(0);
        let origin_text = active_deck
            .and_then(|deck| deck.source_template_id.as_deref())
            .and_then(|template_id| {
                state
                    .content
                    .starter_loadouts
                    .iter()
                    .find(|starter| starter.id == template_id)
                    .map(|starter| {
                        format!(
                            "{}: {}",
                            state.ui_text.get("deck_builder_template_origin_label"),
                            starter.name
                        )
                    })
            })
            .unwrap_or_else(|| {
                state
                    .ui_text
                    .get("deck_builder_custom_origin_label")
                    .to_owned()
            });

        draw_text(
            state.ui_text.get("deck_builder_title"),
            ui.x(80.0),
            ui.y(96.0),
            ui.font(68.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("deck_builder_subtitle"),
            ui.x(80.0),
            ui.y(148.0),
            ui.font(30.0),
            TEXT_MUTED,
        );

        section_panel(
            ui.rect(80.0, 178.0, 400.0, 416.0),
            state.ui_text.get("deck_builder_saved_decks_label"),
            SKYBLUE,
        );
        section_panel(
            ui.rect(80.0, 612.0, 400.0, 296.0),
            state.ui_text.get("deck_builder_templates_label"),
            GOLD,
        );
        section_panel(
            ui.rect(80.0, 930.0, 400.0, 358.0),
            state.ui_text.get("deck_builder_booster_results_label"),
            PINK,
        );
        section_panel(
            ui.rect(540.0, 96.0, 1520.0, 90.0),
            state.ui_text.get("deck_builder_active_deck_label"),
            GOLD,
        );
        section_panel(
            summary_panel_rect(),
            state.ui_text.get("deck_builder_summary_label"),
            SKYBLUE,
        );
        section_panel(
            preview_panel_section_rect(),
            state.ui_text.get("deck_builder_preview_label"),
            PINK,
        );
        section_panel(
            contents_panel_section_rect(),
            match self.active_tab {
                DeckBuilderTab::SupportCards => state.ui_text.get("deck_builder_contents_label"),
                DeckBuilderTab::MagicalGirls => {
                    state.ui_text.get("deck_builder_roster_magical_girls")
                }
                DeckBuilderTab::Baddies => state.ui_text.get("deck_builder_roster_baddies"),
            },
            GOLD,
        );
        self.draw_layer_tabs(state);

        draw_text(deck_name, ui.x(570.0), ui.y(148.0), ui.font(34.0), WHITE);
        draw_text(
            &format!(
                "{}: {}/{}",
                state.ui_text.get("deck_builder_card_total_label"),
                deck_size,
                state.content.deck_rules.support_deck_size
            ),
            ui.x(1460.0),
            ui.y(148.0),
            ui.font(28.0),
            TEXT_MUTED,
        );
        draw_text(
            &origin_text,
            ui.x(570.0),
            ui.y(176.0),
            ui.font(20.0),
            TEXT_MUTED,
        );
        if let Some(deck) = active_deck {
            if !deck.archetype_tags.is_empty() {
                draw_text(
                    &format!(
                        "{}: {}",
                        state.ui_text.get("deck_builder_tags_label"),
                        deck.archetype_tags.join(", ")
                    ),
                    ui.x(980.0),
                    ui.y(176.0),
                    ui.font(20.0),
                    GOLD,
                );
            }
        }

        self.draw_saved_deck_list(state);
        self.draw_template_list(state);
        self.draw_booster_results(state);
        self.draw_support_browser_controls(state);
        match self.active_tab {
            DeckBuilderTab::SupportCards => {
                self.draw_support_card_grid(state, active_deck.is_some())
            }
            DeckBuilderTab::MagicalGirls => self.draw_roster_pool(state, true),
            DeckBuilderTab::Baddies => self.draw_roster_pool(state, false),
        }
        self.draw_validation_summary(state);

        self.draw_preview_panel(state);
        self.draw_contents_panel(state);

        if let Some(dialog) = &self.metadata_dialog {
            self.draw_metadata_dialog(state, dialog);
        }
        if let Some(dialog) = &self.import_export_dialog {
            self.draw_import_export_dialog(state, dialog);
        }

        if let Some(dialog) = &self.rename_dialog {
            self.draw_rename_dialog(state, dialog);
        }
    }

    fn update_import_export_dialog(
        &mut self,
        state: &AppState,
        mouse: (f32, f32),
    ) -> Option<ScreenAction> {
        let dialog = self.import_export_dialog.as_mut()?;

        if point_in_rect(import_export_close_rect(), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.import_export_dialog = None;
            return Some(ScreenAction::None);
        }

        if !point_in_rect(import_export_dialog_rect(), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.import_export_dialog = None;
            return Some(ScreenAction::None);
        }

        match dialog.mode {
            DeckImportExportMode::Export => {
                if point_in_rect(import_export_secondary_rect(), mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    self.import_export_dialog = None;
                    return Some(ScreenAction::None);
                }

                if point_in_rect(import_export_primary_rect(), mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    dialog.status = Some(match copy_to_clipboard(&dialog.value) {
                        Ok(()) => state
                            .ui_text
                            .get("deck_builder_export_copy_success")
                            .to_owned(),
                        Err(()) => state.ui_text.get("deck_builder_clipboard_error").to_owned(),
                    });
                    return Some(ScreenAction::None);
                }
            }
            DeckImportExportMode::Import => {
                if point_in_rect(import_export_text_rect(), mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    dialog.text_focused = true;
                }

                if point_in_rect(import_export_secondary_rect(), mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    dialog.status = Some(match read_from_clipboard() {
                        Ok(text) => {
                            dialog.value = text;
                            dialog.text_focused = true;
                            state
                                .ui_text
                                .get("deck_builder_import_paste_success")
                                .to_owned()
                        }
                        Err(()) => state.ui_text.get("deck_builder_clipboard_error").to_owned(),
                    });
                    return Some(ScreenAction::None);
                }

                if point_in_rect(import_export_primary_rect(), mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    match crate::state::import_deck_code(&dialog.value, &state.content) {
                        Ok(_) => {
                            let code = dialog.value.trim().to_owned();
                            self.import_export_dialog = None;
                            return Some(ScreenAction::DeckBuilderImportDeckCode { code });
                        }
                        Err(error) => {
                            dialog.status = Some(deck_code_error_text(state, &error));
                            return Some(ScreenAction::None);
                        }
                    }
                }

                if !point_in_rect(import_export_text_rect(), mouse)
                    && !point_in_rect(import_export_primary_rect(), mouse)
                    && !point_in_rect(import_export_secondary_rect(), mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    dialog.text_focused = false;
                }

                if dialog.text_focused {
                    while let Some(character) = get_char_pressed() {
                        if !character.is_control() {
                            dialog.value.push(character);
                        }
                    }

                    if is_key_pressed(KeyCode::Backspace) {
                        dialog.value.pop();
                    }
                    if is_key_pressed(KeyCode::Enter) {
                        dialog.value.push('\n');
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            self.import_export_dialog = None;
            return Some(ScreenAction::None);
        }

        None
    }

    fn update_metadata_dialog(&mut self, mouse: (f32, f32)) -> Option<ScreenAction> {
        let dialog = self.metadata_dialog.as_mut()?;

        if point_in_rect(metadata_notes_rect(), mouse) && is_mouse_button_pressed(MouseButton::Left)
        {
            dialog.notes_focused = true;
            dialog.tags_focused = false;
        }
        if point_in_rect(metadata_tags_rect(), mouse) && is_mouse_button_pressed(MouseButton::Left)
        {
            dialog.tags_focused = true;
            dialog.notes_focused = false;
        }
        if point_in_rect(metadata_save_rect(), mouse) && is_mouse_button_pressed(MouseButton::Left)
        {
            let notes = dialog.notes.trim().to_owned();
            let tags = split_tag_text(&dialog.tags);
            self.metadata_dialog = None;
            return Some(ScreenAction::DeckBuilderSaveMetadata { notes, tags });
        }
        if point_in_rect(metadata_cancel_rect(), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.metadata_dialog = None;
            return Some(ScreenAction::None);
        }
        if !point_in_rect(metadata_dialog_rect(), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.metadata_dialog = None;
            return Some(ScreenAction::None);
        }

        if is_key_pressed(KeyCode::Escape) {
            self.metadata_dialog = None;
            return Some(ScreenAction::None);
        }

        while let Some(character) = get_char_pressed() {
            if character.is_control() {
                continue;
            }
            if dialog.tags_focused {
                dialog.tags.push(character);
            } else {
                dialog.notes.push(character);
            }
        }
        if is_key_pressed(KeyCode::Backspace) {
            if dialog.tags_focused {
                dialog.tags.pop();
            } else {
                dialog.notes.pop();
            }
        }
        if is_key_pressed(KeyCode::Enter) {
            if dialog.tags_focused {
                dialog.tags.push_str(", ");
            } else {
                dialog.notes.push('\n');
            }
        }

        None
    }

    fn draw_saved_deck_list(&self, state: &AppState) {
        let ui = UiLayout::current();
        for (button_index, action) in deck_action_buttons(state).into_iter().enumerate() {
            let rect = deck_action_button_rect(button_index);
            draw_soft_panel(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if action.enabled { GOLD } else { DARKGRAY },
            );
            draw_text(
                action.label,
                rect.x + ui.w(12.0),
                rect.y + ui.h(28.0),
                ui.font(18.0),
                WHITE,
            );
        }
        for action in deck_transfer_buttons(state) {
            let rect = deck_transfer_button_rect(action.kind);
            draw_soft_panel(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if action.enabled { SKYBLUE } else { DARKGRAY },
            );
            draw_text(
                action.label,
                rect.x + ui.w(12.0),
                rect.y + ui.h(28.0),
                ui.font(18.0),
                WHITE,
            );
        }
        for action in deck_utility_buttons(state) {
            let rect = deck_utility_button_rect(action.kind);
            draw_soft_panel(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if action.enabled { PINK } else { DARKGRAY },
            );
            draw_text(
                action.label,
                rect.x + ui.w(10.0),
                rect.y + ui.h(24.0),
                ui.font(16.0),
                WHITE,
            );
        }

        if state.saves.decks.support_decks.is_empty() {
            draw_text(
                state.ui_text.get("deck_builder_no_saved_decks"),
                ui.x(100.0),
                ui.y(378.0),
                ui.font(22.0),
                TEXT_MUTED,
            );
            return;
        }

        let recent_deck_ids = state.saves.decks.recently_modified_deck_ids(3);
        for (deck_index, deck) in state.saves.decks.support_decks.iter().enumerate() {
            let row_rect = saved_deck_row_rect(deck_index);
            let is_selected = state
                .saves
                .decks
                .selected_support_deck()
                .map(|selected| selected.id == deck.id)
                .unwrap_or(false);
            let origin_label = deck
                .source_template_id
                .as_deref()
                .and_then(|template_id| {
                    state
                        .content
                        .starter_loadouts
                        .iter()
                        .find(|starter| starter.id == template_id)
                        .map(|starter| starter.name.as_str())
                })
                .unwrap_or(state.ui_text.get("deck_builder_custom_origin_short"));
            let recent_label = if recent_deck_ids.iter().any(|entry| entry == &deck.id) {
                state.ui_text.get("deck_builder_recently_modified_short")
            } else {
                origin_label
            };

            draw_soft_panel(
                row_rect.x,
                row_rect.y,
                row_rect.w,
                row_rect.h,
                if is_selected { SKYBLUE } else { DARKGRAY },
            );
            draw_text(
                &deck.name,
                row_rect.x + ui.w(16.0),
                row_rect.y + ui.h(28.0),
                ui.font(20.0),
                WHITE,
            );
            draw_text(
                &format!(
                    "{} {}/{}",
                    state.ui_text.get("deck_builder_card_total_label"),
                    deck.story_cards.len(),
                    state.content.deck_rules.support_deck_size
                ),
                row_rect.x + ui.w(16.0),
                row_rect.y + ui.h(50.0),
                ui.font(16.0),
                TEXT_MUTED,
            );
            draw_text(
                recent_label,
                row_rect.x + ui.w(206.0),
                row_rect.y + ui.h(50.0),
                ui.font(16.0),
                TEXT_MUTED,
            );
        }
    }

    fn draw_template_list(&self, state: &AppState) {
        let ui = UiLayout::current();
        for (loadout_index, starter) in state.content.starter_loadouts.iter().enumerate() {
            let row_rect = starter_row_rect(loadout_index);
            let create_rect = starter_create_rect(loadout_index);
            let created_decks = state
                .saves
                .decks
                .support_decks
                .iter()
                .filter(|deck| deck.source_template_id.as_deref() == Some(starter.id.as_str()))
                .count();

            draw_soft_panel(
                row_rect.x,
                row_rect.y,
                row_rect.w,
                row_rect.h,
                if self.selected_template_index == Some(loadout_index) {
                    GOLD
                } else {
                    DARKGRAY
                },
            );
            draw_text(
                &starter.name,
                row_rect.x + ui.w(16.0),
                row_rect.y + ui.h(24.0),
                ui.font(20.0),
                WHITE,
            );
            draw_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_template_playstyle_label"),
                    starter.playstyle
                ),
                row_rect.x + ui.w(16.0),
                row_rect.y + ui.h(46.0),
                ui.font(16.0),
                TEXT_MUTED,
            );
            draw_text(
                &format!(
                    "{} {}",
                    state.ui_text.get("deck_builder_template_decks_created"),
                    created_decks
                ),
                row_rect.x + ui.w(16.0),
                row_rect.y + ui.h(66.0),
                ui.font(15.0),
                TEXT_MUTED,
            );

            draw_soft_panel(
                create_rect.x,
                create_rect.y,
                create_rect.w,
                create_rect.h,
                SKYBLUE,
            );
            draw_text(
                state.ui_text.get("deck_builder_create_from_template"),
                create_rect.x + ui.w(10.0),
                create_rect.y + ui.h(36.0),
                ui.font(16.0),
                WHITE,
            );
        }
    }

    fn draw_booster_results(&self, state: &AppState) {
        let ui = UiLayout::current();
        let mouse = mouse_position();
        let mut booster_y = ui.y(992.0);
        for grant in state.last_opened_booster.iter().take(10) {
            let row_rect = Rect::new(ui.x(100.0), booster_y - ui.h(30.0), ui.w(360.0), ui.h(40.0));
            let row_hovered = point_in_rect(row_rect, mouse);
            draw_soft_panel(
                row_rect.x,
                row_rect.y,
                row_rect.w,
                row_rect.h,
                if row_hovered { PINK } else { DARKPURPLE },
            );
            draw_text(
                &format!(
                    "{}: {}",
                    collection_kind_label(state, grant.kind),
                    grant.name
                ),
                ui.x(116.0),
                booster_y,
                ui.font(18.0),
                WHITE,
            );
            booster_y += ui.h(28.0);
        }
    }

    fn draw_support_browser_controls(&self, state: &AppState) {
        if self.active_tab != DeckBuilderTab::SupportCards {
            return;
        }

        self.draw_search_bar(state);
        self.draw_filter_controls(state);
        self.draw_browser_mode_controls(state);
    }

    fn draw_search_bar(&self, state: &AppState) {
        if self.active_tab != DeckBuilderTab::SupportCards {
            return;
        }

        let ui = UiLayout::current();
        let input_rect = search_input_rect();
        let clear_rect = search_clear_rect();
        let visible_cards = self.filtered_story_cards(state);

        draw_soft_panel(
            input_rect.x,
            input_rect.y,
            input_rect.w,
            input_rect.h,
            if self.search_focused {
                SKYBLUE
            } else {
                DARKGRAY
            },
        );
        draw_text(
            if self.search_text.is_empty() {
                state.ui_text.get("deck_builder_search_placeholder")
            } else {
                &self.search_text
            },
            input_rect.x + ui.w(14.0),
            input_rect.y + ui.h(30.0),
            ui.font(20.0),
            WHITE,
        );

        draw_soft_panel(
            clear_rect.x,
            clear_rect.y,
            clear_rect.w,
            clear_rect.h,
            if self.search_text.is_empty() {
                DARKGRAY
            } else {
                PINK
            },
        );
        draw_text(
            state.ui_text.get("deck_builder_search_clear"),
            clear_rect.x + ui.w(12.0),
            clear_rect.y + ui.h(30.0),
            ui.font(18.0),
            WHITE,
        );

        draw_text(
            &format!(
                "{}: {}",
                state.ui_text.get("deck_builder_search_results_label"),
                visible_cards.len()
            ),
            ui.x(1790.0),
            ui.y(232.0),
            ui.font(18.0),
            TEXT_MUTED,
        );
    }

    fn draw_filter_controls(&self, state: &AppState) {
        if self.active_tab != DeckBuilderTab::SupportCards {
            return;
        }

        let ui = UiLayout::current();
        for (index, button) in self.filter_buttons(state).into_iter().enumerate() {
            let rect = filter_button_rect(index);
            draw_soft_panel(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if button.active { GOLD } else { DARKGRAY },
            );
            draw_text(
                &button.label,
                rect.x + ui.w(10.0),
                rect.y + ui.h(24.0),
                ui.font(16.0),
                WHITE,
            );
        }

        for (index, chip) in self.active_filter_chips(state).into_iter().enumerate() {
            let rect = filter_chip_rect(index);
            draw_soft_panel(rect.x, rect.y, rect.w, rect.h, SKYBLUE);
            draw_text(
                &format!("{} x", chip.label),
                rect.x + ui.w(10.0),
                rect.y + ui.h(22.0),
                ui.font(16.0),
                WHITE,
            );
        }

        let clear_rect = filter_clear_all_rect();
        draw_soft_panel(
            clear_rect.x,
            clear_rect.y,
            clear_rect.w,
            clear_rect.h,
            if self.filters.has_active_filters() {
                PINK
            } else {
                DARKGRAY
            },
        );
        draw_text(
            state.ui_text.get("deck_builder_filters_clear_all"),
            clear_rect.x + ui.w(10.0),
            clear_rect.y + ui.h(22.0),
            ui.font(16.0),
            WHITE,
        );
    }

    fn draw_browser_mode_controls(&self, state: &AppState) {
        if self.active_tab != DeckBuilderTab::SupportCards {
            return;
        }

        let ui = UiLayout::current();
        for (rect, label, value) in [
            (
                sort_mode_rect(),
                state.ui_text.get("deck_builder_sort_label"),
                sort_mode_label(state, self.sort_mode),
            ),
            (
                group_mode_rect(),
                state.ui_text.get("deck_builder_group_label"),
                group_mode_label(state, self.group_mode),
            ),
            (
                view_mode_rect(),
                state.ui_text.get("deck_builder_view_label"),
                view_mode_label(state, self.view_mode),
            ),
        ] {
            draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
            draw_text(
                &format!("{label}: {value}"),
                rect.x + ui.w(10.0),
                rect.y + ui.h(22.0),
                ui.font(16.0),
                WHITE,
            );
        }
    }

    fn filter_buttons(&self, state: &AppState) -> Vec<FilterButton> {
        let mut buttons = vec![
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_speed_daily"),
                FilterButtonKind::Speed(crate::data::CardSpeed::DailyLife),
                self.filters
                    .speeds
                    .contains(&crate::data::CardSpeed::DailyLife),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_speed_reaction"),
                FilterButtonKind::Speed(crate::data::CardSpeed::Reaction),
                self.filters
                    .speeds
                    .contains(&crate::data::CardSpeed::Reaction),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_speed_encounter"),
                FilterButtonKind::Speed(crate::data::CardSpeed::Encounter),
                self.filters
                    .speeds
                    .contains(&crate::data::CardSpeed::Encounter),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_align_mg"),
                FilterButtonKind::Alignment(crate::data::CardAlignment::MagicalGirl),
                self.filters
                    .alignments
                    .contains(&crate::data::CardAlignment::MagicalGirl),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_align_baddie"),
                FilterButtonKind::Alignment(crate::data::CardAlignment::Baddie),
                self.filters
                    .alignments
                    .contains(&crate::data::CardAlignment::Baddie),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_align_neutral"),
                FilterButtonKind::Alignment(crate::data::CardAlignment::Neutral),
                self.filters
                    .alignments
                    .contains(&crate::data::CardAlignment::Neutral),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_owned"),
                FilterButtonKind::OwnedOnly,
                self.filters.owned_only,
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_missing"),
                FilterButtonKind::MissingOnly,
                self.filters.missing_only,
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_in_deck"),
                FilterButtonKind::InDeckOnly,
                self.filters.in_deck_only,
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_not_in_deck"),
                FilterButtonKind::NotInDeckOnly,
                self.filters.not_in_deck_only,
            ),
        ];

        for card_type in unique_card_types(state) {
            let is_active = self
                .filters
                .card_types
                .iter()
                .any(|entry| entry.eq_ignore_ascii_case(&card_type));
            buttons.push(FilterButton::new(
                title_case_card_type(&card_type),
                FilterButtonKind::CardType(card_type),
                is_active,
            ));
        }

        buttons
    }

    fn active_filter_chips(&self, state: &AppState) -> Vec<FilterChip> {
        let mut chips = Vec::new();

        for speed in &self.filters.speeds {
            chips.push(FilterChip::new(
                speed_filter_label(state, *speed),
                FilterChipKind::Speed(*speed),
            ));
        }
        for alignment in &self.filters.alignments {
            chips.push(FilterChip::new(
                alignment_filter_label(state, *alignment),
                FilterChipKind::Alignment(*alignment),
            ));
        }
        for card_type in &self.filters.card_types {
            chips.push(FilterChip::new(
                title_case_card_type(card_type),
                FilterChipKind::CardType(card_type.clone()),
            ));
        }
        if self.filters.owned_only {
            chips.push(FilterChip::new(
                state.ui_text.get("deck_builder_filter_owned"),
                FilterChipKind::OwnedOnly,
            ));
        }
        if self.filters.missing_only {
            chips.push(FilterChip::new(
                state.ui_text.get("deck_builder_filter_missing"),
                FilterChipKind::MissingOnly,
            ));
        }
        if self.filters.in_deck_only {
            chips.push(FilterChip::new(
                state.ui_text.get("deck_builder_filter_in_deck"),
                FilterChipKind::InDeckOnly,
            ));
        }
        if self.filters.not_in_deck_only {
            chips.push(FilterChip::new(
                state.ui_text.get("deck_builder_filter_not_in_deck"),
                FilterChipKind::NotInDeckOnly,
            ));
        }

        chips
    }

    fn toggle_filter(&mut self, kind: FilterButtonKind) {
        match kind {
            FilterButtonKind::Speed(speed) => self.filters.toggle_speed(speed),
            FilterButtonKind::Alignment(alignment) => self.filters.toggle_alignment(alignment),
            FilterButtonKind::CardType(card_type) => self.filters.toggle_card_type(&card_type),
            FilterButtonKind::OwnedOnly => self.filters.owned_only = !self.filters.owned_only,
            FilterButtonKind::MissingOnly => self.filters.missing_only = !self.filters.missing_only,
            FilterButtonKind::InDeckOnly => self.filters.in_deck_only = !self.filters.in_deck_only,
            FilterButtonKind::NotInDeckOnly => {
                self.filters.not_in_deck_only = !self.filters.not_in_deck_only
            }
        }
    }

    fn remove_filter_chip(&mut self, kind: FilterChipKind) {
        match kind {
            FilterChipKind::Speed(speed) => self.filters.toggle_speed(speed),
            FilterChipKind::Alignment(alignment) => self.filters.toggle_alignment(alignment),
            FilterChipKind::CardType(card_type) => self.filters.toggle_card_type(&card_type),
            FilterChipKind::OwnedOnly => self.filters.owned_only = false,
            FilterChipKind::MissingOnly => self.filters.missing_only = false,
            FilterChipKind::InDeckOnly => self.filters.in_deck_only = false,
            FilterChipKind::NotInDeckOnly => self.filters.not_in_deck_only = false,
        }
    }

    fn filtered_story_cards<'a>(
        &self,
        state: &'a AppState,
    ) -> Vec<&'a crate::data::StoryCardDefinition> {
        let query = DeckSearchQuery::parse(&self.search_text);
        state
            .content
            .story_cards
            .iter()
            .filter(|card| {
                let context = self.story_card_search_context(state, &card.id);
                query.matches(card, context) && self.filters.matches(card, context)
            })
            .collect()
    }

    fn filtered_sorted_story_cards<'a>(
        &self,
        state: &'a AppState,
    ) -> Vec<(&'a crate::data::StoryCardDefinition, DeckBrowserCardStats)> {
        let mut cards = self
            .filtered_story_cards(state)
            .into_iter()
            .map(|card| {
                let original_index = state
                    .content
                    .story_cards
                    .iter()
                    .position(|entry| entry.id == card.id)
                    .unwrap_or(0);
                let context = self.story_card_search_context(state, &card.id);
                (
                    card,
                    DeckBrowserCardStats {
                        original_index,
                        owned_count: context.owned_count,
                        copies_in_deck: context.copies_in_deck,
                    },
                )
            })
            .collect::<Vec<_>>();

        cards.sort_by(|left, right| compare_story_cards(*left, *right, self.sort_mode));
        cards
    }

    fn browser_layout_items<'a>(&self, state: &'a AppState) -> Vec<BrowserLayoutItem<'a>> {
        let mut items = Vec::new();
        let mut current_y = browser_content_start_y();
        let columns = match self.view_mode {
            DeckViewMode::Grid => 4,
            DeckViewMode::CompactList => 1,
        };

        let sorted_cards = self.filtered_sorted_story_cards(state);
        let mut grouped_cards = Vec::<(Option<String>, Vec<_>)>::new();
        for (card, stats) in sorted_cards {
            let group_label = card_group_label(card, self.group_mode).map(|label| {
                if self.group_mode == DeckGroupMode::CardType {
                    title_case_card_type(&label)
                } else {
                    label
                }
            });

            if grouped_cards
                .last()
                .map(|(label, _)| *label == group_label)
                .unwrap_or(false)
            {
                if let Some((_, entries)) = grouped_cards.last_mut() {
                    entries.push((card, stats));
                }
            } else {
                grouped_cards.push((group_label, vec![(card, stats)]));
            }
        }

        for (group_label, entries) in grouped_cards {
            let entry_count = entries.len();
            if let Some(label) = group_label {
                let rect = Rect::new(
                    UiLayout::current().x(560.0),
                    UiLayout::current().y(current_y),
                    UiLayout::current().w(1470.0),
                    UiLayout::current().h(28.0),
                );
                items.push(BrowserLayoutItem::GroupHeader { label, rect });
                current_y += 38.0;
            }

            for (index, (card, _stats)) in entries.into_iter().enumerate() {
                let row = index / columns;
                let column = index % columns;
                let rect = browser_card_rect(self.view_mode, current_y, row, column);
                let add_rect = browser_add_rect(self.view_mode, rect);
                let remove_rect = browser_remove_rect(self.view_mode, rect);
                items.push(BrowserLayoutItem::Card(BrowserCardLayout {
                    card,
                    rect,
                    add_rect,
                    remove_rect,
                }));
            }

            let rows = (entry_count + columns - 1) / columns;
            current_y += match self.view_mode {
                DeckViewMode::Grid => rows as f32 * 134.0 + 10.0,
                DeckViewMode::CompactList => rows as f32 * 82.0 + 10.0,
            };
        }

        items
    }

    fn story_card_search_context(&self, state: &AppState, card_id: &str) -> DeckSearchCardContext {
        let owned_count = state
            .saves
            .collection
            .owned_count(CollectionCardKind::StoryCard, card_id);
        let copies_in_deck = state.saves.decks.card_count(card_id);
        DeckSearchCardContext {
            owned_count,
            available_count: state
                .saves
                .collection
                .story_cards_available_for_deck(card_id, copies_in_deck),
            copies_in_deck,
        }
    }

    fn draw_validation_summary(&self, state: &AppState) {
        let ui = UiLayout::current();
        let rect = summary_panel_content_rect();

        if let Some(deck) = state.saves.decks.selected_support_deck() {
            let validation =
                DeckValidation::for_deck(deck, &state.content.deck_rules, &state.saves.collection);
            let legal_color = if validation.is_legal { LIME } else { PINK };
            let collection_color = if validation.is_collection_complete {
                SKYBLUE
            } else {
                GOLD
            };

            draw_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_status_label"),
                    if validation.is_legal {
                        state.ui_text.get("deck_builder_legal_status")
                    } else {
                        state.ui_text.get("deck_builder_illegal_status")
                    }
                ),
                rect.x,
                rect.y,
                ui.font(24.0),
                legal_color,
            );
            draw_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_collection_label"),
                    if validation.is_collection_complete {
                        state.ui_text.get("deck_builder_collection_complete")
                    } else {
                        state.ui_text.get("deck_builder_collection_incomplete")
                    }
                ),
                rect.x,
                rect.y + ui.h(30.0),
                ui.font(20.0),
                collection_color,
            );
            draw_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_missing_count_label"),
                    validation.missing_card_total
                ),
                rect.x,
                rect.y + ui.h(52.0),
                ui.font(20.0),
                if validation.missing_card_total == 0 {
                    WHITE
                } else {
                    GOLD
                },
            );
            draw_text(
                &format!(
                    "{}: {}/{}",
                    state.ui_text.get("deck_builder_support_summary_label"),
                    validation.support_card_count,
                    validation.required_support_card_count
                ),
                rect.x,
                rect.y + ui.h(82.0),
                ui.font(20.0),
                if validation.support_card_count_valid {
                    WHITE
                } else {
                    GOLD
                },
            );
            draw_text(
                &format!(
                    "{}: {}/{}",
                    state.ui_text.get("deck_builder_magical_girl_summary_label"),
                    validation.magical_girl_roster_count,
                    validation.required_magical_girl_roster_count
                ),
                rect.x,
                rect.y + ui.h(110.0),
                ui.font(20.0),
                if validation.magical_girl_roster_valid {
                    WHITE
                } else {
                    GOLD
                },
            );
            draw_text(
                &format!(
                    "{}: {}/{}",
                    state.ui_text.get("deck_builder_baddie_summary_label"),
                    validation.baddie_roster_count,
                    validation.required_baddie_roster_count
                ),
                rect.x,
                rect.y + ui.h(138.0),
                ui.font(20.0),
                if validation.baddie_roster_valid {
                    WHITE
                } else {
                    GOLD
                },
            );

            let issues = self.validation_issue_lines(state, &validation);
            draw_text(
                state.ui_text.get("deck_builder_warnings_label"),
                rect.x,
                rect.y + ui.h(176.0),
                ui.font(22.0),
                WHITE,
            );

            if issues.is_empty() {
                draw_text(
                    state.ui_text.get("deck_builder_no_warnings"),
                    rect.x,
                    rect.y + ui.h(206.0),
                    ui.font(18.0),
                    TEXT_MUTED,
                );
                return;
            }

            let mut issue_y = rect.y + ui.h(206.0);
            for issue in issues.into_iter().take(4) {
                draw_text(&issue, rect.x, issue_y, ui.font(16.0), TEXT_MUTED);
                issue_y += ui.h(24.0);
            }
        } else {
            draw_text(
                state.ui_text.get("deck_builder_missing_deck"),
                rect.x,
                rect.y,
                ui.font(24.0),
                TEXT_MUTED,
            );
            draw_text(
                state.ui_text.get("deck_builder_summary_empty_body"),
                rect.x,
                rect.y + ui.h(34.0),
                ui.font(18.0),
                TEXT_MUTED,
            );
        }
    }

    fn draw_support_card_grid(&self, state: &AppState, has_active_deck: bool) {
        if self.active_tab != DeckBuilderTab::SupportCards {
            return;
        }

        let ui = UiLayout::current();
        let mouse = mouse_position();
        for item in self.browser_layout_items(state) {
            match item {
                BrowserLayoutItem::GroupHeader { label, rect } => {
                    draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
                    draw_text(
                        &label,
                        rect.x + ui.w(12.0),
                        rect.y + ui.h(20.0),
                        ui.font(18.0),
                        GOLD,
                    );
                }
                BrowserLayoutItem::Card(card_layout) => {
                    let card = card_layout.card;
                    let copies = state.saves.decks.card_count(&card.id);
                    let owned = state
                        .saves
                        .collection
                        .owned_count(CollectionCardKind::StoryCard, &card.id);
                    let available = state
                        .saves
                        .collection
                        .story_cards_available_for_deck(&card.id, copies);
                    let is_missing_in_deck = copies > owned as usize;
                    let hovered = point_in_rect(card_layout.rect, mouse);

                    draw_story_card_tile(
                        state,
                        card_layout.rect,
                        card,
                        &format!(
                            "{} {} | {} {}",
                            state.ui_text.get("deck_builder_owned_label"),
                            owned,
                            state.ui_text.get("deck_builder_copies_label"),
                            copies
                        ),
                        state.saves.decks.can_add_card(
                            &card.id,
                            &state.content.deck_rules,
                            &state.saves.collection,
                        ),
                        hovered,
                    );
                    if is_missing_in_deck {
                        draw_rectangle_lines(
                            card_layout.rect.x,
                            card_layout.rect.y,
                            card_layout.rect.w,
                            card_layout.rect.h,
                            ui.w(4.0),
                            GOLD,
                        );
                        draw_text(
                            state.ui_text.get("deck_builder_missing_short"),
                            card_layout.rect.x + ui.w(12.0),
                            card_layout.rect.y + ui.h(24.0),
                            ui.font(16.0),
                            GOLD,
                        );
                    }

                    draw_soft_panel(
                        card_layout.add_rect.x,
                        card_layout.add_rect.y,
                        card_layout.add_rect.w,
                        card_layout.add_rect.h,
                        if available > 0 && has_active_deck {
                            SKYBLUE
                        } else {
                            DARKGRAY
                        },
                    );
                    draw_text(
                        if available > 0 && has_active_deck {
                            state.ui_text.get("deck_builder_add_card")
                        } else {
                            state.ui_text.get("deck_builder_add_locked")
                        },
                        card_layout.add_rect.x + ui.w(12.0),
                        card_layout.add_rect.y + ui.h(24.0),
                        ui.font(16.0),
                        WHITE,
                    );

                    draw_soft_panel(
                        card_layout.remove_rect.x,
                        card_layout.remove_rect.y,
                        card_layout.remove_rect.w,
                        card_layout.remove_rect.h,
                        if copies > 0 { PINK } else { DARKGRAY },
                    );
                    draw_text(
                        if copies > 0 {
                            state.ui_text.get("deck_builder_remove_card")
                        } else {
                            state.ui_text.get("deck_builder_remove_locked")
                        },
                        card_layout.remove_rect.x + ui.w(10.0),
                        card_layout.remove_rect.y + ui.h(24.0),
                        ui.font(16.0),
                        WHITE,
                    );
                }
            }
        }
    }

    fn draw_preview_panel(&self, state: &AppState) {
        let mouse = mouse_position();

        if self.active_tab == DeckBuilderTab::SupportCards {
            for item in self.browser_layout_items(state) {
                if let BrowserLayoutItem::Card(card_layout) = item {
                    if !point_in_rect(card_layout.rect, mouse) {
                        continue;
                    }

                    let card = card_layout.card;
                    let copies = state.saves.decks.card_count(&card.id);
                    let owned = state
                        .saves
                        .collection
                        .owned_count(CollectionCardKind::StoryCard, &card.id);
                    let available = state
                        .saves
                        .collection
                        .story_cards_available_for_deck(&card.id, copies);
                    let preview_rect = preview_panel_content_rect();
                    let footer = vec![
                        format!(
                            "{}: {} / {}",
                            state.ui_text.get("deck_builder_copies_label"),
                            copies,
                            state.content.deck_rules.max_copies_per_story_card
                        ),
                        format!(
                            "{}: {}  {}: {}",
                            state.ui_text.get("deck_builder_owned_label"),
                            owned,
                            state.ui_text.get("deck_builder_available_label"),
                            available
                        ),
                    ];
                    draw_story_card_preview(state, preview_rect, card, &footer);
                    return;
                }
            }
        }

        if self.active_tab != DeckBuilderTab::SupportCards {
            self.draw_roster_preview(state);
            return;
        }

        if let Some(grant) = self.hovered_booster_result(state, mouse) {
            self.draw_collection_preview(state, grant);
            return;
        }

        if let Some(loadout_index) = self.selected_template_index {
            if let Some(starter) = state.content.starter_loadouts.get(loadout_index) {
                self.draw_template_preview(state, starter);
                return;
            }
        }

        if let Some(deck) = state.saves.decks.selected_support_deck() {
            self.draw_deck_preview(
                state,
                &deck.name,
                &deck.story_cards,
                &format!(
                    "{} {}/{}",
                    state.ui_text.get("deck_builder_card_total_label"),
                    deck.story_cards.len(),
                    state.content.deck_rules.support_deck_size
                ),
            );
            return;
        }

        self.draw_empty_preview(state);
    }

    fn draw_contents_panel(&self, state: &AppState) {
        match self.active_tab {
            DeckBuilderTab::SupportCards => self.draw_support_deck_contents(state),
            DeckBuilderTab::MagicalGirls => self.draw_roster_contents(state, true),
            DeckBuilderTab::Baddies => self.draw_roster_contents(state, false),
        }
    }

    fn draw_support_deck_contents(&self, state: &AppState) {
        let Some(deck) = state.saves.decks.selected_support_deck() else {
            return;
        };
        let ui = UiLayout::current();
        let rect = contents_panel_content_rect();
        let mut counts = std::collections::BTreeMap::<String, usize>::new();
        for card_id in &deck.story_cards {
            *counts.entry(card_id.clone()).or_insert(0) += 1;
        }

        draw_text(
            state.ui_text.get("deck_builder_contents_help"),
            rect.x,
            rect.y,
            ui.font(18.0),
            TEXT_MUTED,
        );
        if !deck.recent_story_cards.is_empty() {
            draw_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_recent_cards_label"),
                    deck.recent_story_cards
                        .iter()
                        .map(|card_id| self.story_card_name(state, card_id))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                rect.x,
                rect.y + ui.h(24.0),
                ui.font(16.0),
                SKYBLUE,
            );
        }
        if !deck.notes.is_empty() {
            draw_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_notes_label"),
                    wrap_preview_text(&deck.notes, rect.w - ui.w(20.0), ui.font(15.0), 1).join(" ")
                ),
                rect.x,
                rect.y + ui.h(46.0),
                ui.font(15.0),
                TEXT_MUTED,
            );
        }

        let mut y = rect.y + ui.h(72.0);
        for (card_id, count) in counts {
            let missing_count = count.saturating_sub(
                state
                    .saves
                    .collection
                    .owned_count(CollectionCardKind::StoryCard, &card_id) as usize,
            );
            let name = state
                .content
                .story_cards
                .iter()
                .find(|entry| entry.id == card_id)
                .map(|entry| entry.name.as_str())
                .unwrap_or(card_id.as_str());
            draw_text(
                &format!(
                    "{count}x {name}{}",
                    if missing_count > 0 {
                        format!(
                            "  {} x{}",
                            state.ui_text.get("deck_builder_missing_short"),
                            missing_count
                        )
                    } else {
                        String::new()
                    }
                ),
                rect.x,
                y,
                ui.font(18.0),
                if missing_count > 0 { GOLD } else { WHITE },
            );
            y += ui.h(22.0);
            if y > rect.y + rect.h - ui.h(20.0) {
                break;
            }
        }

        let suggestions = self.replacement_suggestion_lines(state);
        if !suggestions.is_empty() {
            let base_y = rect.y + rect.h - ui.h(80.0);
            draw_text(
                state.ui_text.get("deck_builder_replacements_label"),
                rect.x,
                base_y,
                ui.font(18.0),
                GOLD,
            );
            let mut suggestion_y = base_y + ui.h(20.0);
            for line in suggestions.into_iter().take(2) {
                draw_text(&line, rect.x, suggestion_y, ui.font(15.0), TEXT_MUTED);
                suggestion_y += ui.h(18.0);
            }
        }
    }

    fn draw_roster_contents(&self, state: &AppState, is_magical_girl_side: bool) {
        let Some(active_deck) = state.saves.decks.selected_support_deck() else {
            return;
        };
        let ui = UiLayout::current();
        let rect = contents_panel_content_rect();
        let roster = if is_magical_girl_side {
            &active_deck.magical_girl_roster
        } else {
            &active_deck.baddie_roster
        };
        let definitions = if is_magical_girl_side {
            &state.content.magical_girls
        } else {
            &state.content.baddies
        };
        let selected_slot = if is_magical_girl_side {
            self.selected_magical_girl_slot
        } else {
            self.selected_baddie_slot
        };

        draw_text(
            state.ui_text.get("deck_builder_roster_panel_help"),
            rect.x,
            rect.y,
            ui.font(18.0),
            TEXT_MUTED,
        );

        for (slot_index, character_id) in roster.iter().enumerate() {
            let slot_rect = roster_contents_slot_rect(slot_index);
            let name = definitions
                .iter()
                .find(|entry| entry.id == *character_id)
                .map(|entry| entry.name.as_str())
                .unwrap_or(character_id.as_str());
            draw_soft_panel(
                slot_rect.x,
                slot_rect.y,
                slot_rect.w,
                slot_rect.h,
                if selected_slot == Some(slot_index) {
                    GOLD
                } else {
                    DARKGRAY
                },
            );
            let owned = state.saves.collection.owned_count(
                if is_magical_girl_side {
                    CollectionCardKind::MagicalGirl
                } else {
                    CollectionCardKind::Baddie
                },
                character_id,
            );
            draw_text(
                &format!(
                    "{} {}{}",
                    slot_index + 1,
                    name,
                    if owned == 0 {
                        format!(" ({})", state.ui_text.get("deck_builder_missing_short"))
                    } else {
                        String::new()
                    }
                ),
                slot_rect.x + ui.w(12.0),
                slot_rect.y + ui.h(28.0),
                ui.font(18.0),
                if owned == 0 { GOLD } else { WHITE },
            );
        }
    }

    fn validation_issue_lines(&self, state: &AppState, validation: &DeckValidation) -> Vec<String> {
        let mut lines = Vec::new();

        if !validation.support_card_count_valid {
            lines.push(format!(
                "{} {}",
                state.ui_text.get("deck_builder_support_count_warning"),
                validation
                    .required_support_card_count
                    .saturating_sub(validation.support_card_count)
            ));
        }

        if !validation.duplicate_story_cards.is_empty() {
            lines.push(format!(
                "{}: {}",
                state.ui_text.get("deck_builder_duplicate_story_warning"),
                self.story_card_names(state, &validation.duplicate_story_cards)
            ));
        }

        if !validation.duplicate_magical_girls.is_empty() {
            lines.push(format!(
                "{}: {}",
                state
                    .ui_text
                    .get("deck_builder_duplicate_magical_girl_warning"),
                self.magical_girl_names(state, &validation.duplicate_magical_girls)
            ));
        }

        if !validation.duplicate_baddies.is_empty() {
            lines.push(format!(
                "{}: {}",
                state.ui_text.get("deck_builder_duplicate_baddie_warning"),
                self.baddie_names(state, &validation.duplicate_baddies)
            ));
        }

        if !validation.missing_story_cards.is_empty() {
            lines.push(format!(
                "{}: {}",
                state.ui_text.get("deck_builder_missing_story_warning"),
                self.story_card_missing_names(state, &validation.missing_story_cards)
            ));
        }

        if !validation.missing_magical_girls.is_empty() {
            lines.push(format!(
                "{}: {}",
                state
                    .ui_text
                    .get("deck_builder_missing_magical_girl_warning"),
                self.magical_girl_names(state, &validation.missing_magical_girls)
            ));
        }

        if !validation.missing_baddies.is_empty() {
            lines.push(format!(
                "{}: {}",
                state.ui_text.get("deck_builder_missing_baddie_warning"),
                self.baddie_names(state, &validation.missing_baddies)
            ));
        }

        lines
    }

    fn story_card_missing_names(
        &self,
        state: &AppState,
        missing_entries: &[DeckValidationCount],
    ) -> String {
        missing_entries
            .iter()
            .map(|entry| {
                format!(
                    "{} x{}",
                    self.story_card_name(state, &entry.id),
                    entry.count
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn story_card_names(&self, state: &AppState, ids: &[String]) -> String {
        ids.iter()
            .map(|id| self.story_card_name(state, id))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn magical_girl_names(&self, state: &AppState, ids: &[String]) -> String {
        ids.iter()
            .map(|id| {
                state
                    .content
                    .magical_girls
                    .iter()
                    .find(|entry| entry.id == *id)
                    .map(|entry| entry.name.clone())
                    .unwrap_or_else(|| id.clone())
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn baddie_names(&self, state: &AppState, ids: &[String]) -> String {
        ids.iter()
            .map(|id| {
                state
                    .content
                    .baddies
                    .iter()
                    .find(|entry| entry.id == *id)
                    .map(|entry| entry.name.clone())
                    .unwrap_or_else(|| id.clone())
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn story_card_name(&self, state: &AppState, id: &str) -> String {
        state
            .content
            .story_cards
            .iter()
            .find(|entry| entry.id == id)
            .map(|entry| entry.name.clone())
            .unwrap_or_else(|| id.to_owned())
    }

    fn replacement_suggestion_lines(&self, state: &AppState) -> Vec<String> {
        let Some(deck) = state.saves.decks.selected_support_deck() else {
            return Vec::new();
        };

        crate::state::suggest_story_replacements(deck, &state.content, &state.saves.collection, 2)
            .into_iter()
            .map(|suggestion| self.replacement_line(state, &suggestion))
            .collect()
    }

    fn replacement_line(&self, state: &AppState, suggestion: &DeckReplacementSuggestion) -> String {
        format!(
            "{} -> {}",
            self.story_card_name(state, &suggestion.missing_card_id),
            suggestion
                .replacement_card_ids
                .iter()
                .map(|card_id| self.story_card_name(state, card_id))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn hovered_booster_result<'a>(
        &self,
        state: &'a AppState,
        mouse: (f32, f32),
    ) -> Option<&'a BoosterCardGrant> {
        let ui = UiLayout::current();
        let mut booster_y = ui.y(992.0);
        for grant in state.last_opened_booster.iter().take(10) {
            let row_rect = Rect::new(ui.x(100.0), booster_y - ui.h(30.0), ui.w(360.0), ui.h(40.0));
            if point_in_rect(row_rect, mouse) {
                return Some(grant);
            }
            booster_y += ui.h(28.0);
        }
        None
    }

    fn draw_collection_preview(&self, state: &AppState, grant: &BoosterCardGrant) {
        let rect = preview_panel_content_rect();

        match grant.kind {
            CollectionCardKind::StoryCard => {
                if let Some(card) = state
                    .content
                    .story_cards
                    .iter()
                    .find(|card| card.id == grant.id)
                {
                    let footer = vec![format!(
                        "{}: {}",
                        state.ui_text.get("deck_builder_owned_label"),
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::StoryCard, &grant.id)
                    )];
                    draw_story_card_preview(state, rect, card, &footer);
                }
            }
            CollectionCardKind::MagicalGirl => {
                if let Some(character) = state
                    .content
                    .magical_girls
                    .iter()
                    .find(|entry| entry.id == grant.id)
                {
                    self.draw_character_preview(
                        state,
                        rect,
                        state.ui_text.get("deck_builder_kind_magical_girl"),
                        character,
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::MagicalGirl, &grant.id),
                    );
                }
            }
            CollectionCardKind::Baddie => {
                if let Some(character) = state
                    .content
                    .baddies
                    .iter()
                    .find(|entry| entry.id == grant.id)
                {
                    self.draw_character_preview(
                        state,
                        rect,
                        state.ui_text.get("deck_builder_kind_baddie"),
                        character,
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::Baddie, &grant.id),
                    );
                }
            }
        }
    }

    fn draw_character_preview(
        &self,
        state: &AppState,
        rect: Rect,
        kind_label: &str,
        character: &CharacterDefinition,
        owned: u32,
    ) {
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, SKYBLUE);
        if let Some(texture) = state.assets.portrait(&character.id) {
            draw_texture_ex(
                texture,
                rect.x + 18.0,
                rect.y + 54.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(112.0, 112.0)),
                    ..Default::default()
                },
            );
        }
        draw_text(kind_label, rect.x + 20.0, rect.y + 34.0, 24.0, GOLD);
        draw_text(&character.name, rect.x + 148.0, rect.y + 84.0, 36.0, WHITE);
        draw_text(
            &format!(
                "Power {} / {} / {}",
                character.base_power, character.transformed_power, character.final_power
            ),
            rect.x + 148.0,
            rect.y + 132.0,
            24.0,
            TEXT_MUTED,
        );
        draw_text(
            &format!(
                "Thresholds {} / {}",
                character.first_threshold, character.second_threshold
            ),
            rect.x + 148.0,
            rect.y + 168.0,
            24.0,
            TEXT_MUTED,
        );
        draw_text(
            &format!("Owned: {owned}"),
            rect.x + 148.0,
            rect.y + 220.0,
            24.0,
            WHITE,
        );
    }

    fn draw_deck_preview(
        &self,
        state: &AppState,
        title: &str,
        story_cards: &[String],
        subtitle: &str,
    ) {
        let rect = preview_panel_content_rect();
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, GOLD);
        draw_text(title, rect.x + 18.0, rect.y + 42.0, 28.0, WHITE);
        draw_text(subtitle, rect.x + 18.0, rect.y + 74.0, 20.0, TEXT_MUTED);

        let mut counts = std::collections::BTreeMap::<String, usize>::new();
        for card_id in story_cards {
            *counts.entry(card_id.clone()).or_insert(0) += 1;
        }

        let mut y = rect.y + 120.0;
        for (card_id, count) in counts {
            let name = state
                .content
                .story_cards
                .iter()
                .find(|entry| entry.id == card_id)
                .map(|entry| entry.name.as_str())
                .unwrap_or(card_id.as_str());
            draw_text(&format!("{count}x {name}"), rect.x + 18.0, y, 20.0, WHITE);
            y += 24.0;
            if y > rect.y + rect.h - 24.0 {
                break;
            }
        }
    }

    fn draw_template_preview(&self, state: &AppState, starter: &crate::data::StarterLoadout) {
        let ui = UiLayout::current();
        let rect = preview_panel_content_rect();
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, GOLD);
        draw_text(
            &starter.name,
            rect.x + ui.w(18.0),
            rect.y + ui.h(34.0),
            ui.font(26.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("deck_builder_previewing_template"),
            rect.x + ui.w(18.0),
            rect.y + ui.h(60.0),
            ui.font(18.0),
            TEXT_MUTED,
        );
        draw_text(
            &format!(
                "{}: {}",
                state.ui_text.get("deck_builder_template_playstyle_label"),
                starter.playstyle
            ),
            rect.x + ui.w(18.0),
            rect.y + ui.h(86.0),
            ui.font(18.0),
            WHITE,
        );

        let description_lines =
            wrap_preview_text(&starter.description, rect.w - ui.w(36.0), ui.font(16.0), 3);
        let mut y = rect.y + ui.h(114.0);
        draw_text(
            state.ui_text.get("deck_builder_template_description_label"),
            rect.x + ui.w(18.0),
            y,
            ui.font(18.0),
            WHITE,
        );
        y += ui.h(18.0);
        for line in description_lines {
            y += ui.h(18.0);
            draw_text(&line, rect.x + ui.w(18.0), y, ui.font(16.0), TEXT_MUTED);
        }

        y += ui.h(24.0);
        draw_text(
            state.ui_text.get("deck_builder_template_roster_seed_label"),
            rect.x + ui.w(18.0),
            y,
            ui.font(18.0),
            WHITE,
        );
        y += ui.h(20.0);
        for line in self.template_roster_lines(state, starter) {
            draw_text(&line, rect.x + ui.w(18.0), y, ui.font(16.0), TEXT_MUTED);
            y += ui.h(18.0);
        }

        y += ui.h(8.0);
        draw_text(
            &format!(
                "{} ({}/{})",
                state
                    .ui_text
                    .get("deck_builder_template_support_seed_label"),
                starter.support_deck.len(),
                state.content.deck_rules.support_deck_size
            ),
            rect.x + ui.w(18.0),
            y,
            ui.font(18.0),
            WHITE,
        );
        y += ui.h(20.0);
        for line in self.template_support_seed_lines(state, starter) {
            draw_text(&line, rect.x + ui.w(18.0), y, ui.font(15.0), TEXT_MUTED);
            y += ui.h(17.0);
            if y > rect.y + rect.h - ui.h(12.0) {
                break;
            }
        }
    }

    fn draw_empty_preview(&self, state: &AppState) {
        let ui = UiLayout::current();
        let rect = preview_panel_content_rect();
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
        draw_text(
            state.ui_text.get("deck_builder_empty_preview_title"),
            rect.x + ui.w(18.0),
            rect.y + ui.h(42.0),
            ui.font(28.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("deck_builder_empty_preview_body"),
            rect.x + ui.w(18.0),
            rect.y + ui.h(92.0),
            ui.font(20.0),
            TEXT_MUTED,
        );
    }

    fn update_roster_layer(&mut self, state: &AppState, mouse: (f32, f32)) -> ScreenAction {
        let Some(active_deck) = state.saves.decks.selected_support_deck() else {
            return ScreenAction::None;
        };

        let editing_magical_girls = self.active_tab == DeckBuilderTab::MagicalGirls;
        let selected_slot = if editing_magical_girls {
            &mut self.selected_magical_girl_slot
        } else {
            &mut self.selected_baddie_slot
        };

        for slot_index in 0..5 {
            let rect = roster_contents_slot_rect(slot_index);
            if point_in_rect(rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                *selected_slot = Some(slot_index);
            }
        }

        if editing_magical_girls {
            for (index, character) in state.content.magical_girls.iter().enumerate() {
                let rect = roster_pool_rect(true, index);
                if point_in_rect(rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                    if let Some(slot_index) = self.selected_magical_girl_slot {
                        return ScreenAction::DeckBuilderSetRosterSlot {
                            is_magical_girl_side: true,
                            slot_index,
                            character_id: character.id.clone(),
                        };
                    }
                }
            }
        } else {
            for (index, character) in state.content.baddies.iter().enumerate() {
                let rect = roster_pool_rect(false, index);
                if point_in_rect(rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                    if let Some(slot_index) = self.selected_baddie_slot {
                        return ScreenAction::DeckBuilderSetRosterSlot {
                            is_magical_girl_side: false,
                            slot_index,
                            character_id: character.id.clone(),
                        };
                    }
                }
            }
        }

        if active_deck.magical_girl_roster.is_empty() || active_deck.baddie_roster.is_empty() {
            self.selected_magical_girl_slot = None;
            self.selected_baddie_slot = None;
        }

        ScreenAction::None
    }

    fn draw_layer_tabs(&self, state: &AppState) {
        let ui = UiLayout::current();
        for tab in [
            DeckBuilderTab::SupportCards,
            DeckBuilderTab::MagicalGirls,
            DeckBuilderTab::Baddies,
        ] {
            let rect = deck_builder_tab_rect(tab);
            draw_soft_panel(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if self.active_tab == tab {
                    GOLD
                } else {
                    DARKGRAY
                },
            );
            draw_text(
                match tab {
                    DeckBuilderTab::SupportCards => {
                        state.ui_text.get("deck_builder_tab_support_cards")
                    }
                    DeckBuilderTab::MagicalGirls => {
                        state.ui_text.get("deck_builder_tab_magical_girl_roster")
                    }
                    DeckBuilderTab::Baddies => state.ui_text.get("deck_builder_tab_baddie_roster"),
                },
                rect.x + ui.w(18.0),
                rect.y + ui.h(34.0),
                ui.font(22.0),
                WHITE,
            );
        }
    }

    fn draw_roster_pool(&self, state: &AppState, is_magical_girl_side: bool) {
        let ui = UiLayout::current();
        draw_text(
            if is_magical_girl_side {
                state.ui_text.get("deck_builder_magical_girl_roster_help")
            } else {
                state.ui_text.get("deck_builder_baddie_roster_help")
            },
            ui.x(570.0),
            ui.y(226.0),
            ui.font(24.0),
            TEXT_MUTED,
        );

        let definitions = if is_magical_girl_side {
            &state.content.magical_girls
        } else {
            &state.content.baddies
        };

        for (index, character) in definitions.iter().enumerate() {
            let rect = roster_pool_rect(is_magical_girl_side, index);
            let active_deck = state.saves.decks.selected_support_deck();
            let roster = if is_magical_girl_side {
                active_deck
                    .map(|deck| deck.magical_girl_roster.as_slice())
                    .unwrap_or(&[])
            } else {
                active_deck
                    .map(|deck| deck.baddie_roster.as_slice())
                    .unwrap_or(&[])
            };
            let is_in_roster = roster.iter().any(|entry| entry == &character.id);
            draw_soft_panel(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if is_in_roster { SKYBLUE } else { GRAY },
            );
            if let Some(texture) = state.assets.portrait(&character.id) {
                draw_texture_ex(
                    texture,
                    rect.x + ui.w(6.0),
                    rect.y + ui.h(6.0),
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(ui.w(58.0), rect.h - ui.h(12.0))),
                        ..Default::default()
                    },
                );
            }
            draw_text(
                &character.name,
                rect.x + ui.w(74.0),
                rect.y + ui.h(32.0),
                ui.font(20.0),
                WHITE,
            );
            draw_text(
                &format!(
                    "{} / {} / {}",
                    character.base_power, character.transformed_power, character.final_power
                ),
                rect.x + ui.w(74.0),
                rect.y + ui.h(58.0),
                ui.font(16.0),
                TEXT_MUTED,
            );
        }
    }

    fn draw_roster_preview(&self, state: &AppState) {
        let Some(active_deck) = state.saves.decks.selected_support_deck() else {
            return;
        };
        let mouse = mouse_position();
        let preview_rect = preview_panel_content_rect();

        if self.active_tab == DeckBuilderTab::MagicalGirls {
            for (index, character) in state.content.magical_girls.iter().enumerate() {
                let rect = roster_pool_rect(true, index);
                if point_in_rect(rect, mouse) {
                    self.draw_character_preview(
                        state,
                        preview_rect,
                        state.ui_text.get("deck_builder_kind_magical_girl"),
                        character,
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::MagicalGirl, &character.id),
                    );
                    return;
                }
            }
            if let Some(slot_index) = self.selected_magical_girl_slot {
                if let Some(character_id) = active_deck.magical_girl_roster.get(slot_index) {
                    if let Some(character) = state
                        .content
                        .magical_girls
                        .iter()
                        .find(|entry| &entry.id == character_id)
                    {
                        self.draw_character_preview(
                            state,
                            preview_rect,
                            state.ui_text.get("deck_builder_kind_magical_girl"),
                            character,
                            state
                                .saves
                                .collection
                                .owned_count(CollectionCardKind::MagicalGirl, character_id),
                        );
                    }
                }
            }
        } else if self.active_tab == DeckBuilderTab::Baddies {
            for (index, character) in state.content.baddies.iter().enumerate() {
                let rect = roster_pool_rect(false, index);
                if point_in_rect(rect, mouse) {
                    self.draw_character_preview(
                        state,
                        preview_rect,
                        state.ui_text.get("deck_builder_kind_baddie"),
                        character,
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::Baddie, &character.id),
                    );
                    return;
                }
            }
            if let Some(slot_index) = self.selected_baddie_slot {
                if let Some(character_id) = active_deck.baddie_roster.get(slot_index) {
                    if let Some(character) = state
                        .content
                        .baddies
                        .iter()
                        .find(|entry| &entry.id == character_id)
                    {
                        self.draw_character_preview(
                            state,
                            preview_rect,
                            state.ui_text.get("deck_builder_kind_baddie"),
                            character,
                            state
                                .saves
                                .collection
                                .owned_count(CollectionCardKind::Baddie, character_id),
                        );
                    }
                }
            }
        }
    }

    fn draw_rename_dialog(&self, state: &AppState, dialog: &DeckRenameDialog) {
        let ui = UiLayout::current();
        let rect = rename_dialog_rect();
        draw_rectangle(
            ui.x(0.0),
            ui.y(0.0),
            ui.w(2560.0),
            ui.h(1440.0),
            Color::new(0.03, 0.04, 0.08, 0.75),
        );
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
        draw_text(
            state.ui_text.get("deck_builder_rename_prompt"),
            rect.x + ui.w(24.0),
            rect.y + ui.h(42.0),
            ui.font(26.0),
            WHITE,
        );

        let input_rect = rename_dialog_input_rect();
        draw_soft_panel(
            input_rect.x,
            input_rect.y,
            input_rect.w,
            input_rect.h,
            BLACK,
        );
        draw_text(
            if dialog.value.is_empty() {
                state.ui_text.get("deck_builder_rename_placeholder")
            } else {
                &dialog.value
            },
            input_rect.x + ui.w(14.0),
            input_rect.y + ui.h(34.0),
            ui.font(24.0),
            WHITE,
        );

        let save_rect = rename_dialog_save_rect();
        draw_soft_panel(save_rect.x, save_rect.y, save_rect.w, save_rect.h, SKYBLUE);
        draw_text(
            state.ui_text.get("deck_builder_confirm_rename"),
            save_rect.x + ui.w(14.0),
            save_rect.y + ui.h(30.0),
            ui.font(20.0),
            WHITE,
        );

        let cancel_rect = rename_dialog_cancel_rect();
        draw_soft_panel(
            cancel_rect.x,
            cancel_rect.y,
            cancel_rect.w,
            cancel_rect.h,
            PINK,
        );
        draw_text(
            state.ui_text.get("deck_builder_cancel_rename"),
            cancel_rect.x + ui.w(14.0),
            cancel_rect.y + ui.h(30.0),
            ui.font(20.0),
            WHITE,
        );
    }

    fn draw_import_export_dialog(&self, state: &AppState, dialog: &DeckImportExportDialog) {
        let ui = UiLayout::current();
        let rect = import_export_dialog_rect();
        draw_rectangle(
            ui.x(0.0),
            ui.y(0.0),
            ui.w(2560.0),
            ui.h(1440.0),
            Color::new(0.03, 0.04, 0.08, 0.78),
        );
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);

        draw_text(
            match dialog.mode {
                DeckImportExportMode::Export => {
                    state.ui_text.get("deck_builder_export_dialog_title")
                }
                DeckImportExportMode::Import => {
                    state.ui_text.get("deck_builder_import_dialog_title")
                }
            },
            rect.x + ui.w(24.0),
            rect.y + ui.h(42.0),
            ui.font(28.0),
            WHITE,
        );
        draw_text(
            match dialog.mode {
                DeckImportExportMode::Export => {
                    state.ui_text.get("deck_builder_export_dialog_body")
                }
                DeckImportExportMode::Import => {
                    state.ui_text.get("deck_builder_import_dialog_body")
                }
            },
            rect.x + ui.w(24.0),
            rect.y + ui.h(72.0),
            ui.font(20.0),
            TEXT_MUTED,
        );

        let text_rect = import_export_text_rect();
        draw_soft_panel(
            text_rect.x,
            text_rect.y,
            text_rect.w,
            text_rect.h,
            if dialog.text_focused {
                BLACK
            } else {
                DARKPURPLE
            },
        );

        let body_lines = wrap_text_block(
            if dialog.value.is_empty() && dialog.mode == DeckImportExportMode::Import {
                state.ui_text.get("deck_builder_import_placeholder")
            } else {
                &dialog.value
            },
            text_rect.w - ui.w(24.0),
            ui.font(16.0),
            14,
        );
        let mut y = text_rect.y + ui.h(26.0);
        for line in body_lines {
            draw_text(&line, text_rect.x + ui.w(12.0), y, ui.font(16.0), WHITE);
            y += ui.h(20.0);
        }

        if let Some(status) = &dialog.status {
            let status_lines = wrap_text_block(status, rect.w - ui.w(48.0), ui.font(18.0), 3);
            let mut status_y = rect.y + ui.h(548.0);
            for line in status_lines {
                draw_text(&line, rect.x + ui.w(24.0), status_y, ui.font(18.0), GOLD);
                status_y += ui.h(22.0);
            }
        }

        let primary_rect = import_export_primary_rect();
        draw_soft_panel(
            primary_rect.x,
            primary_rect.y,
            primary_rect.w,
            primary_rect.h,
            SKYBLUE,
        );
        draw_text(
            match dialog.mode {
                DeckImportExportMode::Export => state.ui_text.get("deck_builder_copy_code"),
                DeckImportExportMode::Import => state.ui_text.get("deck_builder_import_button"),
            },
            primary_rect.x + ui.w(18.0),
            primary_rect.y + ui.h(32.0),
            ui.font(22.0),
            WHITE,
        );

        let secondary_rect = import_export_secondary_rect();
        draw_soft_panel(
            secondary_rect.x,
            secondary_rect.y,
            secondary_rect.w,
            secondary_rect.h,
            if dialog.mode == DeckImportExportMode::Import {
                GOLD
            } else {
                DARKGRAY
            },
        );
        draw_text(
            match dialog.mode {
                DeckImportExportMode::Export => state.ui_text.get("deck_builder_close_dialog"),
                DeckImportExportMode::Import => state.ui_text.get("deck_builder_paste_code"),
            },
            secondary_rect.x + ui.w(18.0),
            secondary_rect.y + ui.h(32.0),
            ui.font(22.0),
            WHITE,
        );

        let close_rect = import_export_close_rect();
        draw_soft_panel(close_rect.x, close_rect.y, close_rect.w, close_rect.h, PINK);
        draw_text(
            state.ui_text.get("deck_builder_close_dialog"),
            close_rect.x + ui.w(18.0),
            close_rect.y + ui.h(32.0),
            ui.font(22.0),
            WHITE,
        );
    }

    fn draw_metadata_dialog(&self, state: &AppState, dialog: &DeckMetadataDialog) {
        let ui = UiLayout::current();
        let rect = metadata_dialog_rect();
        draw_rectangle(
            ui.x(0.0),
            ui.y(0.0),
            ui.w(2560.0),
            ui.h(1440.0),
            Color::new(0.03, 0.04, 0.08, 0.78),
        );
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
        draw_text(
            state.ui_text.get("deck_builder_metadata_title"),
            rect.x + ui.w(24.0),
            rect.y + ui.h(42.0),
            ui.font(28.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("deck_builder_metadata_tags_label"),
            rect.x + ui.w(24.0),
            rect.y + ui.h(92.0),
            ui.font(20.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("deck_builder_metadata_notes_label"),
            rect.x + ui.w(24.0),
            rect.y + ui.h(164.0),
            ui.font(20.0),
            WHITE,
        );

        let tags_rect = metadata_tags_rect();
        draw_soft_panel(
            tags_rect.x,
            tags_rect.y,
            tags_rect.w,
            tags_rect.h,
            if dialog.tags_focused {
                BLACK
            } else {
                DARKPURPLE
            },
        );
        draw_text(
            if dialog.tags.is_empty() {
                state.ui_text.get("deck_builder_metadata_tags_placeholder")
            } else {
                &dialog.tags
            },
            tags_rect.x + ui.w(12.0),
            tags_rect.y + ui.h(32.0),
            ui.font(20.0),
            WHITE,
        );

        let notes_rect = metadata_notes_rect();
        draw_soft_panel(
            notes_rect.x,
            notes_rect.y,
            notes_rect.w,
            notes_rect.h,
            if dialog.notes_focused {
                BLACK
            } else {
                DARKPURPLE
            },
        );
        let mut notes_y = notes_rect.y + ui.h(24.0);
        for line in wrap_text_block(
            if dialog.notes.is_empty() {
                state.ui_text.get("deck_builder_metadata_notes_placeholder")
            } else {
                &dialog.notes
            },
            notes_rect.w - ui.w(24.0),
            ui.font(18.0),
            10,
        ) {
            draw_text(
                &line,
                notes_rect.x + ui.w(12.0),
                notes_y,
                ui.font(18.0),
                WHITE,
            );
            notes_y += ui.h(22.0);
        }

        let save_rect = metadata_save_rect();
        draw_soft_panel(save_rect.x, save_rect.y, save_rect.w, save_rect.h, SKYBLUE);
        draw_text(
            state.ui_text.get("deck_builder_metadata_save"),
            save_rect.x + ui.w(18.0),
            save_rect.y + ui.h(32.0),
            ui.font(22.0),
            WHITE,
        );

        let cancel_rect = metadata_cancel_rect();
        draw_soft_panel(
            cancel_rect.x,
            cancel_rect.y,
            cancel_rect.w,
            cancel_rect.h,
            PINK,
        );
        draw_text(
            state.ui_text.get("deck_builder_metadata_cancel"),
            cancel_rect.x + ui.w(18.0),
            cancel_rect.y + ui.h(32.0),
            ui.font(22.0),
            WHITE,
        );
    }

    fn template_roster_lines(
        &self,
        state: &AppState,
        starter: &crate::data::StarterLoadout,
    ) -> [String; 4] {
        [
            format!(
                "{}: {}",
                state
                    .ui_text
                    .get("deck_builder_template_magical_girl_main_label"),
                self.magical_girl_names(state, std::slice::from_ref(&starter.magical_girl_main))
            ),
            format!(
                "{}: {}",
                state
                    .ui_text
                    .get("deck_builder_template_magical_girl_supports_label"),
                self.magical_girl_names(state, &starter.magical_girl_supports)
            ),
            format!(
                "{}: {}",
                state.ui_text.get("deck_builder_template_baddie_main_label"),
                self.baddie_names(state, std::slice::from_ref(&starter.prime_baddie))
            ),
            format!(
                "{}: {}",
                state
                    .ui_text
                    .get("deck_builder_template_baddie_supports_label"),
                self.baddie_names(state, &starter.baddie_supports)
            ),
        ]
    }

    fn template_support_seed_lines(
        &self,
        state: &AppState,
        starter: &crate::data::StarterLoadout,
    ) -> Vec<String> {
        let mut counts = std::collections::BTreeMap::<String, usize>::new();
        for card_id in &starter.support_deck {
            *counts.entry(card_id.clone()).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .map(|(card_id, count)| format!("{count}x {}", self.story_card_name(state, &card_id)))
            .collect()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum DeckBuilderTab {
    SupportCards,
    MagicalGirls,
    Baddies,
}

struct DeckRenameDialog {
    value: String,
}

impl DeckRenameDialog {
    fn new(current_name: &str) -> Self {
        Self {
            value: current_name.to_owned(),
        }
    }
}

struct DeckImportExportDialog {
    mode: DeckImportExportMode,
    value: String,
    status: Option<String>,
    text_focused: bool,
}

impl DeckImportExportDialog {
    fn for_export(value: String) -> Self {
        Self {
            mode: DeckImportExportMode::Export,
            value,
            status: None,
            text_focused: false,
        }
    }

    fn for_import() -> Self {
        Self {
            mode: DeckImportExportMode::Import,
            value: String::new(),
            status: None,
            text_focused: true,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum DeckImportExportMode {
    Export,
    Import,
}

struct DeckMetadataDialog {
    notes: String,
    tags: String,
    notes_focused: bool,
    tags_focused: bool,
}

impl DeckMetadataDialog {
    fn new(notes: &str, tags: &str) -> Self {
        Self {
            notes: notes.to_owned(),
            tags: tags.to_owned(),
            notes_focused: true,
            tags_focused: false,
        }
    }
}

struct DeckActionButton<'a> {
    kind: DeckActionKind,
    label: &'a str,
    enabled: bool,
}

struct DeckTransferButton<'a> {
    kind: DeckTransferActionKind,
    label: &'a str,
    enabled: bool,
}

struct DeckUtilityButton<'a> {
    kind: DeckUtilityActionKind,
    label: &'a str,
    enabled: bool,
}

struct FilterButton {
    kind: FilterButtonKind,
    label: String,
    active: bool,
}

impl FilterButton {
    fn new(label: impl Into<String>, kind: FilterButtonKind, active: bool) -> Self {
        Self {
            kind,
            label: label.into(),
            active,
        }
    }
}

struct FilterChip {
    kind: FilterChipKind,
    label: String,
}

impl FilterChip {
    fn new(label: impl Into<String>, kind: FilterChipKind) -> Self {
        Self {
            kind,
            label: label.into(),
        }
    }
}

#[derive(Clone, Copy)]
enum DeckActionKind {
    Create,
    Rename,
    Duplicate,
    Delete,
}

#[derive(Clone, Copy)]
enum DeckTransferActionKind {
    Export,
    Import,
}

#[derive(Clone, Copy)]
enum DeckUtilityActionKind {
    Metadata,
    Undo,
    Reset,
}

#[derive(Clone)]
enum FilterButtonKind {
    Speed(crate::data::CardSpeed),
    Alignment(crate::data::CardAlignment),
    CardType(String),
    OwnedOnly,
    MissingOnly,
    InDeckOnly,
    NotInDeckOnly,
}

#[derive(Clone)]
enum FilterChipKind {
    Speed(crate::data::CardSpeed),
    Alignment(crate::data::CardAlignment),
    CardType(String),
    OwnedOnly,
    MissingOnly,
    InDeckOnly,
    NotInDeckOnly,
}

enum BrowserLayoutItem<'a> {
    GroupHeader { label: String, rect: Rect },
    Card(BrowserCardLayout<'a>),
}

struct BrowserCardLayout<'a> {
    card: &'a crate::data::StoryCardDefinition,
    rect: Rect,
    add_rect: Rect,
    remove_rect: Rect,
}

fn deck_action_buttons<'a>(state: &'a AppState) -> [DeckActionButton<'a>; 4] {
    let has_selected_deck = state.saves.decks.selected_support_deck().is_some();
    [
        DeckActionButton {
            kind: DeckActionKind::Create,
            label: state.ui_text.get("deck_builder_new_deck"),
            enabled: true,
        },
        DeckActionButton {
            kind: DeckActionKind::Rename,
            label: state.ui_text.get("deck_builder_rename_deck"),
            enabled: has_selected_deck,
        },
        DeckActionButton {
            kind: DeckActionKind::Duplicate,
            label: state.ui_text.get("deck_builder_duplicate_deck"),
            enabled: has_selected_deck,
        },
        DeckActionButton {
            kind: DeckActionKind::Delete,
            label: state.ui_text.get("deck_builder_delete_deck"),
            enabled: has_selected_deck,
        },
    ]
}

fn deck_transfer_buttons<'a>(state: &'a AppState) -> [DeckTransferButton<'a>; 2] {
    let has_selected_deck = state.saves.decks.selected_support_deck().is_some();
    [
        DeckTransferButton {
            kind: DeckTransferActionKind::Export,
            label: state.ui_text.get("deck_builder_export_deck"),
            enabled: has_selected_deck,
        },
        DeckTransferButton {
            kind: DeckTransferActionKind::Import,
            label: state.ui_text.get("deck_builder_import_deck"),
            enabled: true,
        },
    ]
}

fn deck_utility_buttons<'a>(state: &'a AppState) -> [DeckUtilityButton<'a>; 3] {
    let selected_deck = state.saves.decks.selected_support_deck();
    [
        DeckUtilityButton {
            kind: DeckUtilityActionKind::Metadata,
            label: state.ui_text.get("deck_builder_metadata_button"),
            enabled: selected_deck.is_some(),
        },
        DeckUtilityButton {
            kind: DeckUtilityActionKind::Undo,
            label: state.ui_text.get("deck_builder_undo_button"),
            enabled: state.saves.decks.can_undo_selected_deck_change(),
        },
        DeckUtilityButton {
            kind: DeckUtilityActionKind::Reset,
            label: state.ui_text.get("deck_builder_reset_button"),
            enabled: selected_deck
                .and_then(|deck| deck.source_template_id.as_deref())
                .is_some(),
        },
    ]
}

fn collection_kind_label<'a>(state: &'a AppState, kind: CollectionCardKind) -> &'a str {
    match kind {
        CollectionCardKind::MagicalGirl => state.ui_text.get("deck_builder_kind_magical_girl"),
        CollectionCardKind::Baddie => state.ui_text.get("deck_builder_kind_baddie"),
        CollectionCardKind::StoryCard => state.ui_text.get("deck_builder_kind_story"),
    }
}

fn speed_filter_label(state: &AppState, speed: crate::data::CardSpeed) -> &str {
    match speed {
        crate::data::CardSpeed::DailyLife => state.ui_text.get("deck_builder_filter_speed_daily"),
        crate::data::CardSpeed::Reaction => state.ui_text.get("deck_builder_filter_speed_reaction"),
        crate::data::CardSpeed::Encounter => {
            state.ui_text.get("deck_builder_filter_speed_encounter")
        }
    }
}

fn alignment_filter_label(state: &AppState, alignment: crate::data::CardAlignment) -> &str {
    match alignment {
        crate::data::CardAlignment::MagicalGirl => {
            state.ui_text.get("deck_builder_filter_align_mg")
        }
        crate::data::CardAlignment::Baddie => state.ui_text.get("deck_builder_filter_align_baddie"),
        crate::data::CardAlignment::Neutral => {
            state.ui_text.get("deck_builder_filter_align_neutral")
        }
    }
}

fn unique_card_types(state: &AppState) -> Vec<String> {
    let mut card_types = state
        .content
        .story_cards
        .iter()
        .map(|card| card.card_type.clone())
        .collect::<Vec<_>>();
    card_types.sort_by_key(|entry| entry.to_ascii_lowercase());
    card_types.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    card_types
}

fn title_case_card_type(card_type: &str) -> String {
    card_type
        .split('_')
        .map(|part| {
            let mut characters = part.chars();
            match characters.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), characters.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn next_sort_mode(mode: DeckSortMode) -> DeckSortMode {
    match mode {
        DeckSortMode::Alphabetical => DeckSortMode::Newest,
        DeckSortMode::Newest => DeckSortMode::OwnedCount,
        DeckSortMode::OwnedCount => DeckSortMode::CopiesInDeck,
        DeckSortMode::CopiesInDeck => DeckSortMode::Alphabetical,
    }
}

fn next_group_mode(mode: DeckGroupMode) -> DeckGroupMode {
    match mode {
        DeckGroupMode::None => DeckGroupMode::Alignment,
        DeckGroupMode::Alignment => DeckGroupMode::Speed,
        DeckGroupMode::Speed => DeckGroupMode::CardType,
        DeckGroupMode::CardType => DeckGroupMode::None,
    }
}

fn next_view_mode(mode: DeckViewMode) -> DeckViewMode {
    match mode {
        DeckViewMode::Grid => DeckViewMode::CompactList,
        DeckViewMode::CompactList => DeckViewMode::Grid,
    }
}

fn sort_mode_label(state: &AppState, mode: DeckSortMode) -> &str {
    match mode {
        DeckSortMode::Alphabetical => state.ui_text.get("deck_builder_sort_alphabetical"),
        DeckSortMode::Newest => state.ui_text.get("deck_builder_sort_newest"),
        DeckSortMode::OwnedCount => state.ui_text.get("deck_builder_sort_owned"),
        DeckSortMode::CopiesInDeck => state.ui_text.get("deck_builder_sort_in_deck"),
    }
}

fn group_mode_label(state: &AppState, mode: DeckGroupMode) -> &str {
    match mode {
        DeckGroupMode::None => state.ui_text.get("deck_builder_group_none"),
        DeckGroupMode::Alignment => state.ui_text.get("deck_builder_group_alignment"),
        DeckGroupMode::Speed => state.ui_text.get("deck_builder_group_speed"),
        DeckGroupMode::CardType => state.ui_text.get("deck_builder_group_card_type"),
    }
}

fn view_mode_label(state: &AppState, mode: DeckViewMode) -> &str {
    match mode {
        DeckViewMode::Grid => state.ui_text.get("deck_builder_view_grid"),
        DeckViewMode::CompactList => state.ui_text.get("deck_builder_view_list"),
    }
}

fn browser_content_start_y() -> f32 {
    362.0
}

fn browser_card_rect(view_mode: DeckViewMode, base_y: f32, row: usize, column: usize) -> Rect {
    let ui = UiLayout::current();
    match view_mode {
        DeckViewMode::Grid => Rect::new(
            ui.x(560.0 + column as f32 * 350.0),
            ui.y(base_y + row as f32 * 134.0),
            ui.w(328.0),
            ui.h(116.0),
        ),
        DeckViewMode::CompactList => Rect::new(
            ui.x(560.0),
            ui.y(base_y + row as f32 * 82.0),
            ui.w(1470.0),
            ui.h(68.0),
        ),
    }
}

fn browser_add_rect(view_mode: DeckViewMode, card_rect: Rect) -> Rect {
    let ui = UiLayout::current();
    match view_mode {
        DeckViewMode::Grid => Rect::new(
            card_rect.x + card_rect.w - ui.w(112.0),
            card_rect.y + ui.h(12.0),
            ui.w(92.0),
            ui.h(36.0),
        ),
        DeckViewMode::CompactList => Rect::new(
            card_rect.x + card_rect.w - ui.w(220.0),
            card_rect.y + ui.h(16.0),
            ui.w(92.0),
            ui.h(36.0),
        ),
    }
}

fn browser_remove_rect(view_mode: DeckViewMode, card_rect: Rect) -> Rect {
    let ui = UiLayout::current();
    match view_mode {
        DeckViewMode::Grid => Rect::new(
            card_rect.x + card_rect.w - ui.w(112.0),
            card_rect.y + ui.h(60.0),
            ui.w(92.0),
            ui.h(36.0),
        ),
        DeckViewMode::CompactList => Rect::new(
            card_rect.x + card_rect.w - ui.w(112.0),
            card_rect.y + ui.h(16.0),
            ui.w(92.0),
            ui.h(36.0),
        ),
    }
}

fn deck_action_button_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(98.0 + index as f32 * 94.0),
        ui.y(214.0),
        ui.w(84.0),
        ui.h(40.0),
    )
}

fn deck_transfer_button_rect(kind: DeckTransferActionKind) -> Rect {
    let ui = UiLayout::current();
    match kind {
        DeckTransferActionKind::Export => {
            Rect::new(ui.x(98.0), ui.y(260.0), ui.w(178.0), ui.h(40.0))
        }
        DeckTransferActionKind::Import => {
            Rect::new(ui.x(286.0), ui.y(260.0), ui.w(178.0), ui.h(40.0))
        }
    }
}

fn deck_utility_button_rect(kind: DeckUtilityActionKind) -> Rect {
    let ui = UiLayout::current();
    match kind {
        DeckUtilityActionKind::Metadata => {
            Rect::new(ui.x(98.0), ui.y(306.0), ui.w(118.0), ui.h(34.0))
        }
        DeckUtilityActionKind::Undo => Rect::new(ui.x(223.0), ui.y(306.0), ui.w(118.0), ui.h(34.0)),
        DeckUtilityActionKind::Reset => {
            Rect::new(ui.x(348.0), ui.y(306.0), ui.w(118.0), ui.h(34.0))
        }
    }
}

fn saved_deck_row_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(96.0),
        ui.y(364.0 + index as f32 * 60.0),
        ui.w(368.0),
        ui.h(52.0),
    )
}

fn starter_row_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(96.0),
        ui.y(650.0 + index as f32 * 86.0),
        ui.w(222.0),
        ui.h(70.0),
    )
}

fn starter_create_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(328.0),
        ui.y(650.0 + index as f32 * 86.0),
        ui.w(136.0),
        ui.h(70.0),
    )
}

fn deck_builder_tab_rect(tab: DeckBuilderTab) -> Rect {
    let ui = UiLayout::current();
    match tab {
        DeckBuilderTab::SupportCards => {
            Rect::new(ui.x(560.0), ui.y(194.0), ui.w(250.0), ui.h(44.0))
        }
        DeckBuilderTab::MagicalGirls => {
            Rect::new(ui.x(826.0), ui.y(194.0), ui.w(290.0), ui.h(44.0))
        }
        DeckBuilderTab::Baddies => Rect::new(ui.x(1132.0), ui.y(194.0), ui.w(230.0), ui.h(44.0)),
    }
}

fn roster_pool_rect(is_magical_girl_side: bool, index: usize) -> Rect {
    let ui = UiLayout::current();
    let _ = is_magical_girl_side;
    let row = index / 4;
    let column = index % 4;
    Rect::new(
        ui.x(560.0 + column as f32 * 370.0),
        ui.y(274.0 + row as f32 * 94.0),
        ui.w(348.0),
        ui.h(74.0),
    )
}

fn summary_panel_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2090.0, 96.0, 390.0, 324.0)
}

fn summary_panel_content_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2110.0, 138.0, 350.0, 250.0)
}

fn preview_panel_section_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2090.0, 442.0, 390.0, 454.0)
}

fn preview_panel_content_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2120.0, 482.0, 330.0, 374.0)
}

fn contents_panel_section_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2090.0, 918.0, 390.0, 370.0)
}

fn contents_panel_content_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2110.0, 958.0, 350.0, 300.0)
}

fn roster_contents_slot_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(2110.0),
        ui.y(992.0 + index as f32 * 56.0),
        ui.w(350.0),
        ui.h(46.0),
    )
}

fn search_input_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1100.0, 194.0, 690.0, 44.0)
}

fn search_clear_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1806.0, 194.0, 126.0, 44.0)
}

fn sort_mode_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(560.0, 246.0, 156.0, 30.0)
}

fn group_mode_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(732.0, 246.0, 156.0, 30.0)
}

fn view_mode_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(904.0, 246.0, 156.0, 30.0)
}

fn filter_button_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    let buttons_per_row = 8;
    let row = index / buttons_per_row;
    let column = index % buttons_per_row;
    Rect::new(
        ui.x(1100.0 + column as f32 * 108.0),
        ui.y(246.0 + row as f32 * 38.0),
        ui.w(100.0),
        ui.h(30.0),
    )
}

fn filter_chip_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(1100.0 + index as f32 * 140.0),
        ui.y(324.0),
        ui.w(132.0),
        ui.h(28.0),
    )
}

fn filter_clear_all_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1940.0, 324.0, 92.0, 28.0)
}

fn rename_dialog_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(910.0, 520.0, 740.0, 220.0)
}

fn rename_dialog_input_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(934.0, 574.0, 692.0, 54.0)
}

fn rename_dialog_save_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(934.0, 650.0, 220.0, 48.0)
}

fn rename_dialog_cancel_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1170.0, 650.0, 220.0, 48.0)
}

fn import_export_dialog_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(720.0, 360.0, 1120.0, 700.0)
}

fn import_export_text_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(750.0, 438.0, 1060.0, 420.0)
}

fn import_export_primary_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(750.0, 928.0, 240.0, 54.0)
}

fn import_export_secondary_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1012.0, 928.0, 240.0, 54.0)
}

fn import_export_close_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1570.0, 928.0, 240.0, 54.0)
}

fn metadata_dialog_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(820.0, 360.0, 920.0, 680.0)
}

fn metadata_tags_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(850.0, 472.0, 860.0, 54.0)
}

fn metadata_notes_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(850.0, 544.0, 860.0, 340.0)
}

fn metadata_save_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(850.0, 920.0, 220.0, 54.0)
}

fn metadata_cancel_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1088.0, 920.0, 220.0, 54.0)
}

fn wrap_preview_text(text: &str, max_width: f32, font_size: f32, max_lines: usize) -> Vec<String> {
    let mut wrapped = Vec::new();
    let mut current = String::new();
    let words = text.split_whitespace().collect::<Vec<_>>();
    let mut index = 0;

    while index < words.len() {
        let word = words[index];
        let candidate = if current.is_empty() {
            word.to_owned()
        } else {
            format!("{current} {word}")
        };

        if measure_text(&candidate, None, font_size as u16, 1.0).width <= max_width {
            current = candidate;
            index += 1;
            continue;
        }

        if !current.is_empty() {
            wrapped.push(current);
        }
        if wrapped.len() + 1 == max_lines {
            let remaining = words[index..].join(" ");
            wrapped.push(remaining);
            return wrapped;
        }
        current = word.to_owned();
        index += 1;
    }

    if !current.is_empty() && wrapped.len() < max_lines {
        wrapped.push(current);
    }

    wrapped
}

fn wrap_text_block(text: &str, max_width: f32, font_size: f32, max_lines: usize) -> Vec<String> {
    let mut wrapped = macroquad_toolkit::ui::wrap_text(text, max_width, font_size);
    if wrapped.is_empty() {
        wrapped.push(String::new());
    }
    wrapped.truncate(max_lines);
    wrapped
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_to_clipboard(text: &str) -> Result<(), ()> {
    let mut clipboard = arboard::Clipboard::new().map_err(|_| ())?;
    clipboard.set_text(text.to_owned()).map_err(|_| ())
}

#[cfg(target_arch = "wasm32")]
fn copy_to_clipboard(_text: &str) -> Result<(), ()> {
    Err(())
}

#[cfg(not(target_arch = "wasm32"))]
fn read_from_clipboard() -> Result<String, ()> {
    let mut clipboard = arboard::Clipboard::new().map_err(|_| ())?;
    clipboard.get_text().map_err(|_| ())
}

#[cfg(target_arch = "wasm32")]
fn read_from_clipboard() -> Result<String, ()> {
    Err(())
}

fn split_tag_text(text: &str) -> Vec<String> {
    text.split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(str::to_owned)
        .collect()
}

fn deck_code_error_text(state: &AppState, error: &DeckCodeError) -> String {
    match error {
        DeckCodeError::Empty => state
            .ui_text
            .get("deck_builder_import_error_empty")
            .to_owned(),
        DeckCodeError::InvalidFormat => state
            .ui_text
            .get("deck_builder_import_error_invalid_format")
            .to_owned(),
        DeckCodeError::UnsupportedVersion(version) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_unsupported_version"),
            version
        ),
        DeckCodeError::InvalidMagicalGirlRosterCount(count) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_invalid_mg_count"),
            count
        ),
        DeckCodeError::InvalidBaddieRosterCount(count) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_invalid_baddie_count"),
            count
        ),
        DeckCodeError::UnknownStoryCard(card_id) => format!(
            "{}: {}",
            state.ui_text.get("deck_builder_import_error_unknown_story"),
            card_id
        ),
        DeckCodeError::UnknownMagicalGirl(character_id) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_unknown_magical_girl"),
            character_id
        ),
        DeckCodeError::UnknownBaddie(character_id) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_unknown_baddie"),
            character_id
        ),
    }
}
