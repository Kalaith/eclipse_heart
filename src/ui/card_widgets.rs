//! Shared card and panel widgets.

use std::cell::RefCell;

use macroquad::prelude::*;

use crate::data::{CardAlignment, CardEffect, CardSpeed, StoryCardDefinition};
use crate::state::AppState;

use super::core::{draw_panel, draw_soft_panel, TEXT_MUTED};

#[derive(Clone)]
struct ActionButtonVisual {
    rect: Rect,
    label: String,
    hovered: bool,
    pressed: bool,
}

thread_local! {
    static PENDING_ACTION_BUTTONS: RefCell<Vec<ActionButtonVisual>> = const { RefCell::new(Vec::new()) };
}

pub fn action_button(rect: Rect, label: &str) -> bool {
    let hovered = point_in_rect(rect, mouse_position());
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

    PENDING_ACTION_BUTTONS.with(|buttons| {
        buttons.borrow_mut().push(ActionButtonVisual {
            rect,
            label: label.to_owned(),
            hovered,
            pressed,
        });
    });

    clicked
}

pub fn render_action_buttons() {
    let visuals = PENDING_ACTION_BUTTONS.with(|buttons| std::mem::take(&mut *buttons.borrow_mut()));
    for visual in visuals {
        draw_action_button_visual(&visual);
    }
}

fn draw_action_button_visual(button: &ActionButtonVisual) {
    let fill = if button.pressed {
        Color::new(0.12, 0.16, 0.24, 0.96)
    } else if button.hovered {
        Color::new(0.16, 0.22, 0.30, 0.95)
    } else {
        Color::new(0.08, 0.10, 0.16, 0.94)
    };
    let outline = if button.hovered { GOLD } else { SKYBLUE };
    let shadow = Color::new(0.02, 0.03, 0.06, 0.42);

    draw_rectangle(
        button.rect.x + 4.0,
        button.rect.y + 6.0,
        button.rect.w,
        button.rect.h,
        shadow,
    );
    draw_rectangle(
        button.rect.x,
        button.rect.y,
        button.rect.w,
        button.rect.h,
        fill,
    );
    draw_rectangle_lines(
        button.rect.x,
        button.rect.y,
        button.rect.w,
        button.rect.h,
        3.0,
        outline,
    );
    draw_rectangle_lines(
        button.rect.x + 6.0,
        button.rect.y + 6.0,
        button.rect.w - 12.0,
        button.rect.h - 12.0,
        1.0,
        Color::new(1.0, 1.0, 1.0, 0.18),
    );

    let font_size = (button.rect.h * 0.36).clamp(18.0, 30.0);
    let text_metrics = measure_text(&button.label, None, font_size as u16, 1.0);
    let text_x = button.rect.x + (button.rect.w - text_metrics.width) * 0.5;
    let text_y = button.rect.y + (button.rect.h + text_metrics.height) * 0.5 - 6.0;
    draw_text(
        &button.label,
        text_x + 1.0,
        text_y + 1.0,
        font_size,
        Color::new(0.0, 0.0, 0.0, 0.55),
    );
    draw_text(&button.label, text_x, text_y, font_size, WHITE);
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

pub fn draw_story_card_tile(
    state: &AppState,
    rect: Rect,
    card: &StoryCardDefinition,
    subtitle: &str,
    enabled: bool,
    hovered: bool,
) {
    let outline = if hovered {
        GOLD
    } else if enabled {
        card_alignment_color(card.alignment)
    } else {
        GRAY
    };
    let fill = if enabled {
        Color::new(0.15, 0.15, 0.24, 0.98)
    } else {
        Color::new(0.11, 0.11, 0.18, 0.98)
    };

    if let Some(texture) = state.assets.template_for_alignment(card.alignment) {
        draw_texture_ex(
            texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect.w, rect.h)),
                ..Default::default()
            },
        );
        draw_rectangle(
            rect.x + 6.0,
            rect.y + rect.h * 0.24,
            rect.w - 12.0,
            rect.h * 0.34,
            Color::new(0.08, 0.09, 0.13, 0.88),
        );
        if let Some(art_texture) = state.assets.story_card_art(&card.id) {
            draw_texture_ex(
                art_texture,
                rect.x + 8.0,
                rect.y + rect.h * 0.245,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(rect.w - 16.0, rect.h * 0.33)),
                    ..Default::default()
                },
            );
        }
        if let Some(badge_texture) = state.assets.badge_for_speed(card.speed) {
            draw_texture_ex(
                badge_texture,
                rect.x + rect.w - 56.0,
                rect.y + 8.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(48.0, 48.0)),
                    ..Default::default()
                },
            );
        }
    } else {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    }
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, outline);

    draw_text(
        speed_label(card.speed),
        rect.x + 14.0,
        rect.y + 26.0,
        22.0,
        outline,
    );
    draw_text(
        alignment_label(card.alignment),
        rect.x + rect.w - 112.0,
        rect.y + 26.0,
        20.0,
        TEXT_MUTED,
    );

    let title_lines = wrap_text_lines(&card.name, rect.w - 28.0, 28.0, 2);
    let mut title_y = rect.y + 58.0;
    for line in title_lines {
        draw_text(&line, rect.x + 14.0, title_y, 28.0, WHITE);
        title_y += 26.0;
    }

    draw_text(
        &format!("{} | {}", card.card_type, subtitle),
        rect.x + 14.0,
        rect.y + rect.h - 16.0,
        18.0,
        TEXT_MUTED,
    );
}

pub fn draw_story_card_preview(
    state: &AppState,
    rect: Rect,
    card: &StoryCardDefinition,
    footer_lines: &[String],
) {
    let accent = card_alignment_color(card.alignment);
    if let Some(texture) = state.assets.template_for_alignment(card.alignment) {
        draw_texture_ex(
            texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect.w, rect.h)),
                ..Default::default()
            },
        );
    } else {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.10, 0.10, 0.16, 0.99),
        );
    }
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 4.0, accent);
    if let Some(art_texture) = state.assets.story_card_art(&card.id) {
        draw_texture_ex(
            art_texture,
            rect.x + 18.0,
            rect.y + 54.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect.w - 36.0, 146.0)),
                ..Default::default()
            },
        );
        draw_rectangle(
            rect.x + 18.0,
            rect.y + 54.0,
            rect.w - 36.0,
            146.0,
            Color::new(0.08, 0.09, 0.13, 0.14),
        );
    }

    draw_text(
        speed_label(card.speed),
        rect.x + 20.0,
        rect.y + 34.0,
        26.0,
        accent,
    );
    draw_text(
        alignment_label(card.alignment),
        rect.x + rect.w - 130.0,
        rect.y + 34.0,
        22.0,
        TEXT_MUTED,
    );

    let title_lines = wrap_text_lines(&card.name, rect.w - 40.0, 38.0, 3);
    let mut title_y = rect.y + 226.0;
    for line in title_lines {
        draw_text(&line, rect.x + 20.0, title_y, 38.0, WHITE);
        title_y += 36.0;
    }

    draw_text(
        &card.card_type,
        rect.x + 20.0,
        rect.y + 344.0,
        22.0,
        TEXT_MUTED,
    );

    let effect_lines = card
        .effects
        .iter()
        .flat_map(|effect| wrap_text_lines(&describe_effect(effect), rect.w - 40.0, 28.0, 3))
        .collect::<Vec<_>>();
    let mut effect_y = rect.y + 388.0;
    for line in effect_lines {
        draw_text(&line, rect.x + 20.0, effect_y, 28.0, WHITE);
        effect_y += 30.0;
    }

    let mut footer_y = rect.y + rect.h - 76.0;
    for line in footer_lines {
        draw_text(line, rect.x + 20.0, footer_y, 20.0, TEXT_MUTED);
        footer_y += 22.0;
    }
}

pub fn point_in_rect(rect: Rect, point: (f32, f32)) -> bool {
    point.0 >= rect.x
        && point.0 <= rect.x + rect.w
        && point.1 >= rect.y
        && point.1 <= rect.y + rect.h
}

fn speed_label(speed: CardSpeed) -> &'static str {
    match speed {
        CardSpeed::DailyLife => "Daily",
        CardSpeed::Reaction => "Reaction",
        CardSpeed::Encounter => "Encounter",
    }
}

fn alignment_label(alignment: CardAlignment) -> &'static str {
    match alignment {
        CardAlignment::MagicalGirl => "MG",
        CardAlignment::Baddie => "Baddie",
        CardAlignment::Neutral => "Neutral",
    }
}

fn card_alignment_color(alignment: CardAlignment) -> Color {
    match alignment {
        CardAlignment::MagicalGirl => SKYBLUE,
        CardAlignment::Baddie => PINK,
        CardAlignment::Neutral => GOLD,
    }
}

fn describe_effect(effect: &CardEffect) -> String {
    match effect {
        CardEffect::GainMainRadiance { amount } => format!("Gain {amount} main Radiance."),
        CardEffect::GainRevealedSupportRadiance { amount } => {
            format!("Gain {amount} Radiance on revealed supports.")
        }
        CardEffect::ReduceOpponentMainRadiance { amount } => {
            format!("Opponent main loses {amount} Radiance.")
        }
        CardEffect::GainPrimeDread { amount } => format!("Gain {amount} Prime Dread."),
        CardEffect::GainRevealedSupportDread { amount } => {
            format!("Gain {amount} Dread on revealed supports.")
        }
        CardEffect::ReduceOpponentPrimeDread { amount } => {
            format!("Opponent Prime loses {amount} Dread.")
        }
        CardEffect::GainMainPowerThisEncounter { amount } => {
            format!("Gain {amount} main power this Encounter.")
        }
        CardEffect::GainMainPowerNextEncounter { amount } => {
            format!("Gain {amount} main power next Encounter.")
        }
        CardEffect::ReduceOpponentMainPowerThisEncounter { amount } => {
            format!("Opponent main loses {amount} power this Encounter.")
        }
        CardEffect::GainPrimePowerThisEncounter { amount } => {
            format!("Gain {amount} Prime power this Encounter.")
        }
        CardEffect::GainRevealedSupportPowerThisEncounter { amount } => {
            format!("Revealed supports gain {amount} power this Encounter.")
        }
        CardEffect::GainFirstRevealedSupportRadiance { amount } => {
            format!("First revealed support gains {amount} Radiance.")
        }
        CardEffect::ExhaustFirstRevealedOpponentSupport => {
            "Exhaust the first revealed opponent support.".to_owned()
        }
        CardEffect::RevealFirstHiddenOwnSupport => "Reveal your first hidden support.".to_owned(),
    }
}

fn wrap_text_lines(text: &str, max_width: f32, font_size: f32, max_lines: usize) -> Vec<String> {
    let mut wrapped = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_owned()
        } else {
            format!("{current} {word}")
        };

        if measure_text(&candidate, None, font_size as u16, 1.0).width <= max_width {
            current = candidate;
        } else {
            if !current.is_empty() {
                wrapped.push(current);
            }
            current = word.to_owned();
            if wrapped.len() + 1 == max_lines {
                break;
            }
        }
    }

    if !current.is_empty() && wrapped.len() < max_lines {
        wrapped.push(current);
    }

    wrapped
}
