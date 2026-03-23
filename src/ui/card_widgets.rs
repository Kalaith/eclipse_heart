//! Shared card and panel widgets.

use macroquad::prelude::*;
use macroquad_toolkit::ui::button;

use crate::data::{CardAlignment, CardEffect, CardSpeed, StoryCardDefinition};

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

pub fn draw_story_card_tile(
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

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
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
    rect: Rect,
    card: &StoryCardDefinition,
    footer_lines: &[String],
) {
    let accent = card_alignment_color(card.alignment);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.10, 0.10, 0.16, 0.99));
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 4.0, accent);

    draw_text(speed_label(card.speed), rect.x + 20.0, rect.y + 34.0, 26.0, accent);
    draw_text(
        alignment_label(card.alignment),
        rect.x + rect.w - 130.0,
        rect.y + 34.0,
        22.0,
        TEXT_MUTED,
    );

    let title_lines = wrap_text_lines(&card.name, rect.w - 40.0, 38.0, 3);
    let mut title_y = rect.y + 78.0;
    for line in title_lines {
        draw_text(&line, rect.x + 20.0, title_y, 38.0, WHITE);
        title_y += 36.0;
    }

    draw_text(
        &card.card_type,
        rect.x + 20.0,
        rect.y + 176.0,
        22.0,
        TEXT_MUTED,
    );

    let effect_lines = card
        .effects
        .iter()
        .flat_map(|effect| wrap_text_lines(&describe_effect(effect), rect.w - 40.0, 28.0, 3))
        .collect::<Vec<_>>();
    let mut effect_y = rect.y + 226.0;
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
