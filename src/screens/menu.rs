//! Main menu screen.

use macroquad::prelude::*;

use crate::screens::ScreenAction;
use crate::state::AppState;
use crate::ui::core::{
    draw_background_texture, draw_button_frame, draw_focus_panel, draw_screen_scrim, with_alpha,
    BADDIE_PINK, MG_BLUE, PRIORITY_GOLD, TEXT_MUTED,
};
use crate::ui::layout::UiLayout;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub struct MenuScreen {
    settings_open: bool,
}

#[derive(Clone, Copy)]
enum MenuCommand {
    OpenCampaignMenu,
    OpenSetup,
    OpenDeckBuilder,
    OpenSettings,
    ExitGame,
}

impl MenuScreen {
    pub fn new() -> Self {
        Self {
            settings_open: false,
        }
    }

    pub fn set_settings_open(&mut self, open: bool) {
        self.settings_open = open;
    }

    pub fn update(&mut self, _state: &AppState) -> ScreenAction {
        if self.settings_open {
            if clicked(self.settings_back_rect()) {
                self.settings_open = false;
                return ScreenAction::None;
            }
            if clicked(self.fullscreen_toggle_rect()) {
                return ScreenAction::ToggleFullscreenMode;
            }
            return ScreenAction::None;
        }

        for (rect, command) in self.menu_buttons() {
            if clicked(rect) {
                return self.handle_menu_command(command);
            }
        }

        ScreenAction::None
    }

    pub fn draw(&self, state: &AppState) {
        if let Some(background) = state.assets.ui_background("menu") {
            draw_background_texture(background, WHITE);
        }
        if self.settings_open {
            self.draw_settings_screen(state);
        } else {
            draw_screen_scrim(0.12);
            self.draw_button_column(state);
        }
    }

    fn handle_menu_command(&mut self, command: MenuCommand) -> ScreenAction {
        match command {
            MenuCommand::OpenCampaignMenu => ScreenAction::OpenCampaignMenu,
            MenuCommand::OpenSetup => ScreenAction::OpenSetup,
            MenuCommand::OpenDeckBuilder => ScreenAction::OpenDeckBuilder,
            MenuCommand::OpenSettings => {
                self.settings_open = true;
                ScreenAction::None
            }
            MenuCommand::ExitGame => ScreenAction::ExitGame,
        }
    }

    fn draw_button_column(&self, state: &AppState) {
        let ui = UiLayout::current();
        draw_focus_panel(ui.rect(1684.0, 586.0, 670.0, 604.0), BADDIE_PINK);
        for (rect, command) in self.menu_buttons() {
            self.draw_menu_button(rect, self.menu_label(command, state));
        }
    }

    fn draw_menu_button(&self, rect: Rect, label: &str) {
        let hovered = point_in_rect(rect, mouse_position());
        let pressed = hovered && is_mouse_button_down(MouseButton::Left);
        let fill = if pressed {
            Color::new(0.20, 0.07, 0.24, 0.94)
        } else if hovered {
            Color::new(0.24, 0.10, 0.32, 0.90)
        } else {
            Color::new(0.03, 0.05, 0.14, 0.76)
        };
        let border = if hovered {
            Color::new(1.00, 0.48, 0.82, 0.98)
        } else {
            Color::new(0.60, 0.68, 1.00, 0.76)
        };
        draw_button_frame(rect, fill, border, BADDIE_PINK);

        let ui = UiLayout::current();
        let font_size = ui.font(32.0);
        let text_metrics = measure_ui_text(label, None, font_size as u16, 1.0);
        let text_x = rect.x + (rect.w - text_metrics.width) * 0.5;
        let text_y = rect.y + (rect.h + text_metrics.height) * 0.5 - ui.h(7.0);
        draw_ui_text(
            label,
            text_x + 1.0,
            text_y + 2.0,
            font_size,
            Color::new(0.0, 0.0, 0.0, 0.58),
        );
        draw_ui_text(
            label,
            text_x,
            text_y,
            font_size,
            Color::new(0.96, 0.95, 1.0, 1.0),
        );
    }

    fn draw_settings_screen(&self, state: &AppState) {
        let ui = UiLayout::current();
        let back_rect = self.settings_back_rect();
        let toggle_rect = self.fullscreen_toggle_rect();
        let panel_rect = self.settings_panel_rect();

        draw_screen_scrim(0.78);
        draw_focus_panel(panel_rect, BADDIE_PINK);

        draw_ui_text(
            state.ui_text.get("menu_settings_label"),
            panel_rect.x + ui.w(64.0),
            panel_rect.y + ui.h(112.0),
            ui.font(60.0),
            WHITE,
        );
        draw_ui_text(
            state.ui_text.get("menu_settings_display_label"),
            panel_rect.x + ui.w(70.0),
            panel_rect.y + ui.h(222.0),
            ui.font(28.0),
            PRIORITY_GOLD,
        );

        self.draw_menu_button(back_rect, state.ui_text.get("menu_settings_back"));
        self.draw_fullscreen_toggle(toggle_rect, state.saves.settings.fullscreen, state);
    }

    fn draw_fullscreen_toggle(&self, rect: Rect, fullscreen: bool, state: &AppState) {
        let ui = UiLayout::current();
        let hovered = point_in_rect(rect, mouse_position());
        let fill = if hovered {
            Color::new(0.08, 0.10, 0.20, 0.96)
        } else {
            Color::new(0.055, 0.066, 0.14, 0.94)
        };
        let border = if hovered { PRIORITY_GOLD } else { MG_BLUE };
        let track_width = ui.w(126.0);
        let track_height = ui.h(54.0);
        let track_x = rect.x + rect.w - track_width - ui.w(30.0);
        let track_y = rect.y + (rect.h - track_height) * 0.5;
        let knob_radius = track_height * 0.36;
        let knob_x = if fullscreen {
            track_x + track_width - track_height * 0.5
        } else {
            track_x + track_height * 0.5
        };
        let knob_y = track_y + track_height * 0.5;

        draw_button_frame(
            rect,
            fill,
            border,
            if fullscreen { MG_BLUE } else { BADDIE_PINK },
        );
        draw_ui_text(
            state.ui_text.get("menu_fullscreen_mode"),
            rect.x + ui.w(28.0),
            rect.y + ui.h(48.0),
            ui.font(32.0),
            Color::new(0.94, 0.94, 1.0, 1.0),
        );
        draw_ui_text(
            state.ui_text.get("menu_fullscreen_description"),
            rect.x + ui.w(28.0),
            rect.y + ui.h(84.0),
            ui.font(20.0),
            TEXT_MUTED,
        );
        draw_rectangle(
            track_x,
            track_y,
            track_width,
            track_height,
            if fullscreen {
                with_alpha(MG_BLUE, 0.70)
            } else {
                Color::new(0.02, 0.03, 0.08, 0.94)
            },
        );
        draw_rectangle_lines(
            track_x,
            track_y,
            track_width,
            track_height,
            1.0,
            Color::new(0.88, 0.84, 1.0, 0.58),
        );
        draw_circle(
            knob_x,
            knob_y,
            knob_radius,
            Color::new(0.96, 0.92, 1.0, 1.0),
        );
        let state_label = if fullscreen {
            state.ui_text.get("menu_toggle_on")
        } else {
            state.ui_text.get("menu_toggle_off")
        };
        let state_metrics = measure_ui_text(state_label, None, ui.font(18.0) as u16, 1.0);
        draw_ui_text(
            state_label,
            track_x + (track_width - state_metrics.width) * 0.5,
            track_y + track_height + ui.h(26.0),
            ui.font(18.0),
            if fullscreen { MG_BLUE } else { TEXT_MUTED },
        );
    }

    fn menu_buttons(&self) -> [(Rect, MenuCommand); 5] {
        let ui = UiLayout::current();
        [
            (
                ui.rect(1718.0, 622.0, 586.0, 82.0),
                MenuCommand::OpenCampaignMenu,
            ),
            (ui.rect(1718.0, 728.0, 586.0, 82.0), MenuCommand::OpenSetup),
            (
                ui.rect(1718.0, 834.0, 586.0, 82.0),
                MenuCommand::OpenDeckBuilder,
            ),
            (
                ui.rect(1718.0, 940.0, 586.0, 82.0),
                MenuCommand::OpenSettings,
            ),
            (ui.rect(1718.0, 1068.0, 586.0, 82.0), MenuCommand::ExitGame),
        ]
    }

    fn menu_label<'a>(&self, command: MenuCommand, state: &'a AppState) -> &'a str {
        match command {
            MenuCommand::OpenCampaignMenu => state.ui_text.get("menu_start_campaign"),
            MenuCommand::OpenSetup => state.ui_text.get("menu_start_battle"),
            MenuCommand::OpenDeckBuilder => state.ui_text.get("menu_open_deck_builder"),
            MenuCommand::OpenSettings => state.ui_text.get("menu_settings_label"),
            MenuCommand::ExitGame => state.ui_text.get("menu_exit_game"),
        }
    }

    fn settings_back_rect(&self) -> Rect {
        UiLayout::current().rect(2072.0, 132.0, 310.0, 76.0)
    }

    fn settings_panel_rect(&self) -> Rect {
        UiLayout::current().rect(96.0, 96.0, 2368.0, 1196.0)
    }

    fn fullscreen_toggle_rect(&self) -> Rect {
        UiLayout::current().rect(230.0, 360.0, 1460.0, 150.0)
    }
}

fn clicked(rect: Rect) -> bool {
    point_in_rect(rect, mouse_position()) && is_mouse_button_pressed(MouseButton::Left)
}

fn point_in_rect(rect: Rect, point: (f32, f32)) -> bool {
    point.0 >= rect.x
        && point.0 <= rect.x + rect.w
        && point.1 >= rect.y
        && point.1 <= rect.y + rect.h
}
