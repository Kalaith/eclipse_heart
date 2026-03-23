//! Match setup screen.

use macroquad::prelude::*;

use crate::screens::ScreenAction;
use crate::state::AppState;
use crate::ui::card_widgets::{action_button, section_panel};
use crate::ui::core::{draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

pub struct SetupScreen;

impl SetupScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, state: &AppState) -> ScreenAction {
        let ui = UiLayout::current();
        let width = ui.w(420.0);
        let height = ui.h(58.0);
        let left = ui.x(80.0);
        let right = ui.x(1360.0);
        let mut left_y = ui.y(760.0);
        let mut right_y = ui.y(760.0);

        if action_button(
            Rect::new(left, left_y, width, height),
            state.ui_text.get("setup_cycle_player_a_mg_main"),
        ) {
            return ScreenAction::SetupCyclePlayerAMgMain;
        }
        left_y += ui.h(72.0);
        if action_button(
            Rect::new(left, left_y, width, height),
            state.ui_text.get("setup_cycle_player_a_mg_supports"),
        ) {
            return ScreenAction::SetupCyclePlayerAMgSupports;
        }
        left_y += ui.h(72.0);
        if action_button(
            Rect::new(left, left_y, width, height),
            state.ui_text.get("setup_cycle_player_a_baddie_main"),
        ) {
            return ScreenAction::SetupCyclePlayerABaddieMain;
        }
        left_y += ui.h(72.0);
        if action_button(
            Rect::new(left, left_y, width, height),
            state.ui_text.get("setup_cycle_player_a_baddie_supports"),
        ) {
            return ScreenAction::SetupCyclePlayerABaddieSupports;
        }

        if action_button(
            Rect::new(right, right_y, width, height),
            state.ui_text.get("setup_cycle_player_b_mg_main"),
        ) {
            return ScreenAction::SetupCyclePlayerBMgMain;
        }
        right_y += ui.h(72.0);
        if action_button(
            Rect::new(right, right_y, width, height),
            state.ui_text.get("setup_cycle_player_b_mg_supports"),
        ) {
            return ScreenAction::SetupCyclePlayerBMgSupports;
        }
        right_y += ui.h(72.0);
        if action_button(
            Rect::new(right, right_y, width, height),
            state.ui_text.get("setup_cycle_player_b_baddie_main"),
        ) {
            return ScreenAction::SetupCyclePlayerBBaddieMain;
        }
        right_y += ui.h(72.0);
        if action_button(
            Rect::new(right, right_y, width, height),
            state.ui_text.get("setup_cycle_player_b_baddie_supports"),
        ) {
            return ScreenAction::SetupCyclePlayerBBaddieSupports;
        }

        if action_button(
            ui.rect(2080.0, 1328.0, 400.0, 70.0),
            state.ui_text.get("setup_start_match"),
        ) {
            return ScreenAction::StartConfiguredBattle;
        }

        if action_button(
            ui.rect(80.0, 1328.0, 360.0, 70.0),
            state.ui_text.get("battle_back_to_menu"),
        ) {
            return ScreenAction::BackToMenu;
        }

        ScreenAction::None
    }

    pub fn draw(&self, state: &AppState) {
        let ui = UiLayout::current();
        let setup = &state.setup;

        draw_text(
            state.ui_text.get("setup_title"),
            ui.x(80.0),
            ui.y(96.0),
            ui.font(68.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("setup_subtitle"),
            ui.x(80.0),
            ui.y(150.0),
            ui.font(30.0),
            TEXT_MUTED,
        );

        self.draw_side_box(
            state,
            ui.x(80.0),
            ui.y(220.0),
            ui.w(1120.0),
            ui.h(200.0),
            state.ui_text.get("setup_player_a_mg_side"),
            setup.player_a_mg_main_name(&state.content),
            &setup.player_a_mg_support_names(&state.content).join(", "),
        );
        self.draw_side_box(
            state,
            ui.x(80.0),
            ui.y(460.0),
            ui.w(1120.0),
            ui.h(200.0),
            state.ui_text.get("setup_player_a_baddie_side"),
            setup.player_a_baddie_main_name(&state.content),
            &setup
                .player_a_baddie_support_names(&state.content)
                .join(", "),
        );
        self.draw_side_box(
            state,
            ui.x(1360.0),
            ui.y(220.0),
            ui.w(1120.0),
            ui.h(200.0),
            state.ui_text.get("setup_player_b_mg_side"),
            setup.player_b_mg_main_name(&state.content),
            &setup.player_b_mg_support_names(&state.content).join(", "),
        );
        self.draw_side_box(
            state,
            ui.x(1360.0),
            ui.y(460.0),
            ui.w(1120.0),
            ui.h(200.0),
            state.ui_text.get("setup_player_b_baddie_side"),
            setup.player_b_baddie_main_name(&state.content),
            &setup
                .player_b_baddie_support_names(&state.content)
                .join(", "),
        );

        draw_text(
            state.ui_text.get("setup_hidden_support_note"),
            ui.x(80.0),
            ui.y(708.0),
            ui.font(24.0),
            GOLD,
        );
    }

    fn draw_side_box(
        &self,
        state: &AppState,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        label: &str,
        main: &str,
        supports: &str,
    ) {
        section_panel(Rect::new(x, y, width, height), label, GRAY);
        draw_text(
            &format!("{}: {main}", state.ui_text.get("setup_main_label")),
            x + 20.0,
            y + 84.0,
            32.0,
            SKYBLUE,
        );
        draw_soft_panel(x + 18.0, y + 108.0, width - 36.0, height - 126.0, DARKGRAY);
        draw_text(
            &format!(
                "{}: {supports}",
                state.ui_text.get("setup_hidden_supports_label")
            ),
            x + 34.0,
            y + height - 36.0,
            24.0,
            TEXT_MUTED,
        );
    }
}
