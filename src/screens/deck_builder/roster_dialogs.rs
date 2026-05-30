use macroquad::prelude::*;

use crate::state::{AppState, CollectionCardKind};
use crate::ui::card_widgets::point_in_rect;
use crate::ui::core::{draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

use super::layout::*;
use super::types::*;
use super::utils::*;
use super::*;

impl DeckBuilderScreen {
    pub(super) fn draw_layer_tabs(&self, state: &AppState) {
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

    pub(super) fn draw_roster_pool(&self, state: &AppState, is_magical_girl_side: bool) {
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

    pub(super) fn draw_roster_preview(&self, state: &AppState) {
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

    pub(super) fn draw_rename_dialog(&self, state: &AppState, dialog: &DeckRenameDialog) {
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

    pub(super) fn draw_import_export_dialog(
        &self,
        state: &AppState,
        dialog: &DeckImportExportDialog,
    ) {
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

    pub(super) fn draw_metadata_dialog(&self, state: &AppState, dialog: &DeckMetadataDialog) {
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

    pub(super) fn template_roster_lines(
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

    pub(super) fn template_support_seed_lines(
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
