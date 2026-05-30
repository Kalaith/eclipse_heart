use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::prelude::{draw_rectangle, draw_text, measure_text, Color, Rect, WHITE};

use crate::screens::ScreenAction;
use crate::ui::card_widgets::{action_button, point_in_rect};
use crate::ui::core::draw_soft_panel;
use crate::ui::layout::UiLayout;

use super::Game;

impl Game {
    pub(super) fn escape_menu_action(&self) -> ScreenAction {
        let mouse = mouse_position();
        for (rect, action) in self.escape_menu_buttons() {
            if point_in_rect(rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                return action;
            }
        }
        ScreenAction::None
    }

    pub(super) fn draw_escape_menu(&self) {
        let ui = UiLayout::current();
        let backdrop = Color::new(0.02, 0.03, 0.06, 0.74);
        let panel_rect = ui.rect(916.0, 300.0, 728.0, 772.0);

        draw_rectangle(0.0, 0.0, ui.w(2560.0), ui.h(1440.0), backdrop);
        draw_soft_panel(
            panel_rect.x,
            panel_rect.y,
            panel_rect.w,
            panel_rect.h,
            WHITE,
        );
        draw_rectangle(
            panel_rect.x + 18.0,
            panel_rect.y + 18.0,
            panel_rect.w - 36.0,
            ui.h(108.0),
            Color::new(0.05, 0.06, 0.11, 0.84),
        );

        let title = self.state.ui_text.get("escape_menu_title");
        let title_metrics = measure_text(title, None, ui.font(46.0) as u16, 1.0);
        draw_text(
            title,
            panel_rect.x + (panel_rect.w - title_metrics.width) * 0.5,
            panel_rect.y + ui.h(84.0),
            ui.font(46.0),
            WHITE,
        );

        for (rect, action) in self.escape_menu_buttons() {
            let label = match action {
                ScreenAction::EscapeMenuSave => self.state.ui_text.get("escape_menu_save"),
                ScreenAction::EscapeMenuExitToMainMenu => {
                    self.state.ui_text.get("escape_menu_exit_to_menu")
                }
                ScreenAction::ToggleEscapeMenu => self.state.ui_text.get("escape_menu_resume"),
                ScreenAction::ExitGame => self.state.ui_text.get("menu_exit_game"),
                _ => continue,
            };
            action_button(rect, label);
        }
    }

    pub(super) fn escape_menu_buttons(&self) -> [(Rect, ScreenAction); 4] {
        let ui = UiLayout::current();
        [
            (
                ui.rect(986.0, 468.0, 588.0, 96.0),
                ScreenAction::EscapeMenuSave,
            ),
            (
                ui.rect(986.0, 596.0, 588.0, 96.0),
                ScreenAction::EscapeMenuExitToMainMenu,
            ),
            (
                ui.rect(986.0, 724.0, 588.0, 96.0),
                ScreenAction::ToggleEscapeMenu,
            ),
            (ui.rect(986.0, 852.0, 588.0, 96.0), ScreenAction::ExitGame),
        ]
    }
}
