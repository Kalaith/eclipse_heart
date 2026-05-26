//! Main menu screen.

use macroquad::prelude::*;

use crate::screens::ScreenAction;
use crate::state::AppState;
use crate::ui::core::draw_background_texture;
use crate::ui::layout::UiLayout;

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
        let glow = Color::new(0.75, 0.16, 0.52, if hovered { 0.34 } else { 0.20 });

        draw_rectangle(
            rect.x + 8.0,
            rect.y + 10.0,
            rect.w,
            rect.h,
            Color::new(0.01, 0.01, 0.04, 0.48),
        );
        draw_rectangle(rect.x - 2.0, rect.y - 2.0, rect.w + 4.0, rect.h + 4.0, glow);
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle(
            rect.x,
            rect.y,
            6.0,
            rect.h,
            Color::new(0.93, 0.18, 0.58, 0.86),
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, border);
        draw_rectangle_lines(
            rect.x + 10.0,
            rect.y + 10.0,
            rect.w - 20.0,
            rect.h - 20.0,
            1.0,
            Color::new(1.0, 1.0, 1.0, 0.18),
        );

        let ui = UiLayout::current();
        let font_size = ui.font(32.0);
        let text_metrics = measure_text(label, None, font_size as u16, 1.0);
        let text_x = rect.x + (rect.w - text_metrics.width) * 0.5;
        let text_y = rect.y + (rect.h + text_metrics.height) * 0.5 - ui.h(7.0);
        draw_text(
            label,
            text_x + 1.0,
            text_y + 2.0,
            font_size,
            Color::new(0.0, 0.0, 0.0, 0.58),
        );
        draw_text(
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

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.01, 0.01, 0.05, 0.78),
        );
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            ui.h(164.0),
            Color::new(0.03, 0.04, 0.12, 0.92),
        );
        draw_rectangle(
            0.0,
            ui.h(158.0),
            screen_width(),
            ui.h(6.0),
            Color::new(0.93, 0.18, 0.58, 0.92),
        );

        draw_text(
            state.ui_text.get("menu_settings_label"),
            ui.x(220.0),
            ui.y(104.0),
            ui.font(58.0),
            WHITE,
        );
        self.draw_menu_button(back_rect, state.ui_text.get("menu_settings_back"));
        self.draw_settings_section_backdrop();
        self.draw_fullscreen_toggle(toggle_rect, state.saves.settings.fullscreen, state);
    }

    fn draw_settings_section_backdrop(&self) {
        let ui = UiLayout::current();
        let content_y = ui.y(246.0);

        draw_rectangle(
            0.0,
            content_y,
            screen_width(),
            screen_height() - content_y,
            Color::new(0.02, 0.03, 0.10, 0.52),
        );
        draw_rectangle(
            ui.x(220.0),
            content_y,
            ui.w(8.0),
            screen_height() - content_y,
            Color::new(0.93, 0.18, 0.58, 0.78),
        );
        draw_line(
            0.0,
            content_y,
            screen_width(),
            content_y,
            2.0,
            Color::new(0.52, 0.58, 0.96, 0.36),
        );
    }

    fn draw_fullscreen_toggle(&self, rect: Rect, fullscreen: bool, state: &AppState) {
        let ui = UiLayout::current();
        let hovered = point_in_rect(rect, mouse_position());
        let fill = if hovered {
            Color::new(0.11, 0.08, 0.22, 0.92)
        } else {
            Color::new(0.05, 0.06, 0.14, 0.88)
        };
        let border = if hovered {
            Color::new(1.00, 0.48, 0.82, 0.98)
        } else {
            Color::new(0.52, 0.58, 0.96, 0.66)
        };
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

        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, border);
        draw_text(
            state.ui_text.get("menu_fullscreen_mode"),
            rect.x + ui.w(28.0),
            rect.y + ui.h(54.0),
            ui.font(32.0),
            Color::new(0.94, 0.94, 1.0, 1.0),
        );
        draw_rectangle(
            track_x,
            track_y,
            track_width,
            track_height,
            if fullscreen {
                Color::new(0.58, 0.14, 0.56, 0.96)
            } else {
                Color::new(0.02, 0.03, 0.08, 0.94)
            },
        );
        draw_rectangle_lines(
            track_x,
            track_y,
            track_width,
            track_height,
            2.0,
            Color::new(0.88, 0.84, 1.0, 0.76),
        );
        draw_circle(
            knob_x,
            knob_y,
            knob_radius,
            Color::new(0.96, 0.92, 1.0, 1.0),
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
        UiLayout::current().rect(1964.0, 46.0, 340.0, 76.0)
    }

    fn fullscreen_toggle_rect(&self) -> Rect {
        UiLayout::current().rect(300.0, 330.0, 1960.0, 112.0)
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
