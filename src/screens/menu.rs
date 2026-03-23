//! Main menu screen.

use macroquad::prelude::*;

use crate::screens::ScreenAction;
use crate::state::AppState;
use crate::ui::card_widgets::action_button;
use crate::ui::core::{draw_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

pub struct MenuScreen;

impl MenuScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, state: &AppState) -> ScreenAction {
        let ui = UiLayout::current();
        let setup_rect = ui.rect(1040.0, 760.0, 480.0, 78.0);
        let deck_rect = ui.rect(1040.0, 856.0, 480.0, 78.0);

        if action_button(setup_rect, state.ui_text.get("menu_start_battle")) {
            return ScreenAction::OpenSetup;
        }

        if action_button(deck_rect, state.ui_text.get("menu_open_deck_builder")) {
            return ScreenAction::OpenDeckBuilder;
        }

        ScreenAction::None
    }

    pub fn draw(&self, state: &AppState) {
        let ui = UiLayout::current();
        let title = state.ui_text.get("menu_title");
        let subtitle = state.ui_text.get("menu_subtitle");
        draw_panel(ui.x(80.0), ui.y(84.0), ui.w(2400.0), ui.h(560.0), SKYBLUE);
        draw_text(title, ui.x(140.0), ui.y(210.0), ui.font(106.0), WHITE);
        draw_text(
            subtitle,
            ui.x(140.0),
            ui.y(300.0),
            ui.font(42.0),
            TEXT_MUTED,
        );
        draw_text(
            state.ui_text.get("menu_detail"),
            ui.x(140.0),
            ui.y(380.0),
            ui.font(32.0),
            GOLD,
        );
    }
}
