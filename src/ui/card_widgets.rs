//! Shared card and panel widgets.

use macroquad::prelude::*;
use macroquad_toolkit::ui::button;

use super::core::{draw_panel, draw_soft_panel, TEXT_MUTED};

pub fn action_button(rect: Rect, label: &str) -> bool {
    button(rect.x, rect.y, rect.w, rect.h, label)
}

pub fn disabled_card_button(rect: Rect, speed_label: &str, status_label: &str, card_name: &str) {
    draw_soft_panel(rect.x, rect.y, rect.w, rect.h, GRAY);
    draw_text(speed_label, rect.x + 18.0, rect.y + 28.0, 22.0, GOLD);
    draw_text(card_name, rect.x + 18.0, rect.y + 57.0, 24.0, WHITE);
    draw_text(
        status_label,
        rect.x + rect.w - 102.0,
        rect.y + 42.0,
        20.0,
        TEXT_MUTED,
    );
}

pub fn card_button(
    rect: Rect,
    speed_label: &str,
    status_label: &str,
    card_name: &str,
    enabled: bool,
) -> bool {
    if enabled {
        let label = format!("{speed_label} {status_label} | {card_name}");
        action_button(rect, &label)
    } else {
        disabled_card_button(rect, speed_label, status_label, card_name);
        false
    }
}

pub fn section_panel(rect: Rect, title: &str, outline: Color) {
    draw_panel(rect.x, rect.y, rect.w, rect.h, outline);
    draw_text(title, rect.x + 20.0, rect.y + 34.0, 30.0, WHITE);
}
