//! Main menu screen.

use macroquad::prelude::*;

use crate::screens::ScreenAction;
use crate::state::AppState;
use crate::ui::card_widgets::action_button;
use crate::ui::core::{draw_background_texture, draw_panel, draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

pub struct MenuScreen;

impl MenuScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, state: &AppState) -> ScreenAction {
        let ui = UiLayout::current();
        let campaign_rect = ui.rect(1040.0, 720.0, 480.0, 78.0);
        let setup_rect = ui.rect(1040.0, 816.0, 480.0, 78.0);
        let deck_rect = ui.rect(1040.0, 912.0, 480.0, 78.0);
        let exit_rect = ui.rect(1040.0, 1090.0, 480.0, 78.0);
        let checkbox_rect = ui.rect(1040.0, 1018.0, 44.0, 44.0);

        if action_button(campaign_rect, state.ui_text.get("menu_start_campaign")) {
            return ScreenAction::OpenCampaignMenu;
        }

        if action_button(setup_rect, state.ui_text.get("menu_start_battle")) {
            return ScreenAction::OpenSetup;
        }

        if action_button(deck_rect, state.ui_text.get("menu_open_deck_builder")) {
            return ScreenAction::OpenDeckBuilder;
        }

        if point_in_rect(checkbox_rect, mouse_position())
            && is_mouse_button_pressed(MouseButton::Left)
        {
            return ScreenAction::ToggleWindowedMode;
        }

        if action_button(exit_rect, state.ui_text.get("menu_exit_game")) {
            return ScreenAction::ExitGame;
        }

        ScreenAction::None
    }

    pub fn draw(&self, state: &AppState) {
        let ui = UiLayout::current();
        let title = state.ui_text.get("menu_title");
        let subtitle = state.ui_text.get("menu_subtitle");
        if let Some(background) = state.assets.ui_background("menu") {
            draw_background_texture(background, Color::new(1.0, 1.0, 1.0, 0.92));
        }
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

        draw_text(
            state.ui_text.get("menu_settings_label"),
            ui.x(1040.0),
            ui.y(992.0),
            ui.font(28.0),
            WHITE,
        );
        self.draw_checkbox(
            ui.rect(1040.0, 1018.0, 44.0, 44.0),
            !state.saves.settings.fullscreen,
        );
        draw_text(
            state.ui_text.get("menu_windowed_mode"),
            ui.x(1100.0),
            ui.y(1050.0),
            ui.font(26.0),
            TEXT_MUTED,
        );
    }

    fn draw_checkbox(&self, rect: Rect, checked: bool) {
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, WHITE);
        if checked {
            draw_line(
                rect.x + 10.0,
                rect.y + 22.0,
                rect.x + 20.0,
                rect.y + 34.0,
                4.0,
                GOLD,
            );
            draw_line(
                rect.x + 20.0,
                rect.y + 34.0,
                rect.x + 34.0,
                rect.y + 10.0,
                4.0,
                GOLD,
            );
        }
    }
}

fn point_in_rect(rect: Rect, point: (f32, f32)) -> bool {
    point.0 >= rect.x
        && point.0 <= rect.x + rect.w
        && point.1 >= rect.y
        && point.1 <= rect.y + rect.h
}
