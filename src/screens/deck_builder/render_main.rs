use macroquad::prelude::*;

use crate::state::AppState;
use crate::ui::card_widgets::{point_in_rect, section_panel};
use crate::ui::core::{draw_background_texture, draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

use super::controls::*;
use super::layout::*;
use super::types::*;
use super::*;

impl DeckBuilderScreen {
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

    pub(super) fn draw_saved_deck_list(&self, state: &AppState) {
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

    pub(super) fn draw_template_list(&self, state: &AppState) {
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

    pub(super) fn draw_booster_results(&self, state: &AppState) {
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

    pub(super) fn draw_support_browser_controls(&self, state: &AppState) {
        if self.active_tab != DeckBuilderTab::SupportCards {
            return;
        }

        self.draw_search_bar(state);
        self.draw_filter_controls(state);
        self.draw_browser_mode_controls(state);
    }

    pub(super) fn draw_search_bar(&self, state: &AppState) {
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

    pub(super) fn draw_filter_controls(&self, state: &AppState) {
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

    pub(super) fn draw_browser_mode_controls(&self, state: &AppState) {
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
}
