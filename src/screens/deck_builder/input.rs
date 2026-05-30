use macroquad::prelude::*;

use crate::screens::ScreenAction;
use crate::state::AppState;
use crate::ui::card_widgets::{action_button, point_in_rect};
use crate::ui::layout::UiLayout;

use super::controls::*;
use super::layout::*;
use super::types::*;
use super::utils::*;
use super::*;

impl DeckBuilderScreen {
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

    pub(super) fn update_rename_dialog(&mut self, mouse: (f32, f32)) -> Option<ScreenAction> {
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

    pub(super) fn update_search_input(&mut self, mouse: (f32, f32)) {
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

    pub(super) fn update_filter_controls(&mut self, state: &AppState, mouse: (f32, f32)) {
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

    pub(super) fn update_browser_mode_controls(&mut self, mouse: (f32, f32)) {
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

    pub(super) fn update_import_export_dialog(
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

    pub(super) fn update_metadata_dialog(&mut self, mouse: (f32, f32)) -> Option<ScreenAction> {
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
    pub(super) fn update_roster_layer(
        &mut self,
        state: &AppState,
        mouse: (f32, f32),
    ) -> ScreenAction {
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
}
