//! Support deck builder shell.

use macroquad::prelude::*;

use crate::screens::ScreenAction;
use crate::state::AppState;
use crate::ui::card_widgets::{action_button, section_panel};
use crate::ui::core::{draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

pub struct DeckBuilderScreen;

impl DeckBuilderScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, state: &AppState) -> ScreenAction {
        let ui = UiLayout::current();

        if action_button(
            ui.rect(80.0, 1328.0, 360.0, 70.0),
            state.ui_text.get("battle_back_to_menu"),
        ) {
            return ScreenAction::BackToMenu;
        }

        let mut starter_y = ui.y(178.0);
        for (loadout_index, starter) in state.content.starter_loadouts.iter().enumerate() {
            if action_button(
                Rect::new(ui.x(80.0), starter_y, ui.w(400.0), ui.h(58.0)),
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_load_starter"),
                    starter.name
                ),
            ) {
                return ScreenAction::DeckBuilderLoadStarter { loadout_index };
            }
            starter_y += ui.h(72.0);
        }

        for (index, card) in state.content.story_cards.iter().enumerate() {
            let row = index % 10;
            let column = index / 10;
            let base_x = ui.x(540.0 + column as f32 * 970.0);
            let base_y = ui.y(196.0 + row as f32 * 108.0);

            if action_button(
                Rect::new(base_x + ui.w(700.0), base_y, ui.w(92.0), ui.h(52.0)),
                state.ui_text.get("deck_builder_add_card"),
            ) {
                return ScreenAction::DeckBuilderAddCard {
                    card_id: card.id.clone(),
                };
            }

            if action_button(
                Rect::new(base_x + ui.w(804.0), base_y, ui.w(92.0), ui.h(52.0)),
                state.ui_text.get("deck_builder_remove_card"),
            ) {
                return ScreenAction::DeckBuilderRemoveCard {
                    card_id: card.id.clone(),
                };
            }
        }

        ScreenAction::None
    }

    pub fn draw(&self, state: &AppState) {
        let ui = UiLayout::current();
        let active_deck = state.saves.decks.active_support_deck();
        let deck_name = active_deck
            .map(|deck| deck.name.as_str())
            .unwrap_or(state.ui_text.get("deck_builder_missing_deck"));
        let deck_size = active_deck.map(|deck| deck.story_cards.len()).unwrap_or(0);

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
            ui.rect(80.0, 178.0, 400.0, 1110.0),
            state.ui_text.get("deck_builder_starters_label"),
            SKYBLUE,
        );
        section_panel(
            ui.rect(540.0, 96.0, 1940.0, 90.0),
            state.ui_text.get("deck_builder_active_deck_label"),
            GOLD,
        );

        draw_text(deck_name, ui.x(570.0), ui.y(154.0), ui.font(34.0), WHITE);
        draw_text(
            &format!(
                "{}: {}/{}",
                state.ui_text.get("deck_builder_card_total_label"),
                deck_size,
                state.content.deck_rules.support_deck_size
            ),
            ui.x(1840.0),
            ui.y(154.0),
            ui.font(28.0),
            TEXT_MUTED,
        );

        let mut starter_y = ui.y(250.0);
        for starter in &state.content.starter_loadouts {
            draw_soft_panel(
                ui.x(100.0),
                starter_y - ui.h(34.0),
                ui.w(360.0),
                ui.h(56.0),
                DARKGRAY,
            );
            draw_text(
                &starter.name,
                ui.x(120.0),
                starter_y,
                ui.font(26.0),
                TEXT_MUTED,
            );
            starter_y += ui.h(72.0);
        }

        for (index, card) in state.content.story_cards.iter().enumerate() {
            let row = index % 10;
            let column = index / 10;
            let base_x = ui.x(540.0 + column as f32 * 970.0);
            let base_y = ui.y(196.0 + row as f32 * 108.0);
            let copies = state.saves.decks.card_count(&card.id);

            draw_soft_panel(
                base_x,
                base_y - ui.h(36.0),
                ui.w(920.0),
                ui.h(72.0),
                DARKGRAY,
            );
            draw_text(
                &card.name,
                base_x + ui.w(18.0),
                base_y,
                ui.font(26.0),
                WHITE,
            );
            draw_text(
                card_speed_label(state, card.speed),
                base_x + ui.w(420.0),
                base_y,
                ui.font(22.0),
                SKYBLUE,
            );
            draw_text(
                &format!(
                    "{}: {} / {}",
                    state.ui_text.get("deck_builder_copies_label"),
                    copies,
                    state.content.deck_rules.max_copies_per_story_card
                ),
                base_x + ui.w(520.0),
                base_y,
                ui.font(22.0),
                TEXT_MUTED,
            );
        }
    }
}

fn card_speed_label<'a>(state: &'a AppState, speed: crate::data::CardSpeed) -> &'a str {
    match speed {
        crate::data::CardSpeed::DailyLife => state.ui_text.get("battle_speed_daily"),
        crate::data::CardSpeed::Reaction => state.ui_text.get("battle_speed_reaction"),
        crate::data::CardSpeed::Encounter => state.ui_text.get("battle_speed_encounter"),
    }
}
