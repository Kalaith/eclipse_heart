use crate::state::{AppState, DeckCodeError};
use macroquad_toolkit::ui::measure_ui_text;

pub(super) fn wrap_preview_text(
    text: &str,
    max_width: f32,
    font_size: f32,
    max_lines: usize,
) -> Vec<String> {
    let mut wrapped = Vec::new();
    let mut current = String::new();
    let words = text.split_whitespace().collect::<Vec<_>>();
    let mut index = 0;

    while index < words.len() {
        let word = words[index];
        let candidate = if current.is_empty() {
            word.to_owned()
        } else {
            format!("{current} {word}")
        };

        if measure_ui_text(&candidate, None, font_size as u16, 1.0).width <= max_width {
            current = candidate;
            index += 1;
            continue;
        }

        if !current.is_empty() {
            wrapped.push(current);
        }
        if wrapped.len() + 1 == max_lines {
            let remaining = words[index..].join(" ");
            wrapped.push(remaining);
            return wrapped;
        }
        current = word.to_owned();
        index += 1;
    }

    if !current.is_empty() && wrapped.len() < max_lines {
        wrapped.push(current);
    }

    wrapped
}

pub(super) fn wrap_text_block(
    text: &str,
    max_width: f32,
    font_size: f32,
    max_lines: usize,
) -> Vec<String> {
    let mut wrapped = macroquad_toolkit::ui::wrap_text(text, max_width, font_size);
    if wrapped.is_empty() {
        wrapped.push(String::new());
    }
    wrapped.truncate(max_lines);
    wrapped
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn copy_to_clipboard(text: &str) -> Result<(), ()> {
    let mut clipboard = arboard::Clipboard::new().map_err(|_| ())?;
    clipboard.set_text(text.to_owned()).map_err(|_| ())
}

#[cfg(target_arch = "wasm32")]
pub(super) fn copy_to_clipboard(_text: &str) -> Result<(), ()> {
    Err(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn read_from_clipboard() -> Result<String, ()> {
    let mut clipboard = arboard::Clipboard::new().map_err(|_| ())?;
    clipboard.get_text().map_err(|_| ())
}

#[cfg(target_arch = "wasm32")]
pub(super) fn read_from_clipboard() -> Result<String, ()> {
    Err(())
}

pub(super) fn split_tag_text(text: &str) -> Vec<String> {
    text.split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(str::to_owned)
        .collect()
}

pub(super) fn deck_code_error_text(state: &AppState, error: &DeckCodeError) -> String {
    match error {
        DeckCodeError::Empty => state
            .ui_text
            .get("deck_builder_import_error_empty")
            .to_owned(),
        DeckCodeError::InvalidFormat => state
            .ui_text
            .get("deck_builder_import_error_invalid_format")
            .to_owned(),
        DeckCodeError::UnsupportedVersion(version) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_unsupported_version"),
            version
        ),
        DeckCodeError::InvalidMagicalGirlRosterCount(count) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_invalid_mg_count"),
            count
        ),
        DeckCodeError::InvalidBaddieRosterCount(count) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_invalid_baddie_count"),
            count
        ),
        DeckCodeError::UnknownStoryCard(card_id) => format!(
            "{}: {}",
            state.ui_text.get("deck_builder_import_error_unknown_story"),
            card_id
        ),
        DeckCodeError::UnknownMagicalGirl(character_id) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_unknown_magical_girl"),
            character_id
        ),
        DeckCodeError::UnknownBaddie(character_id) => format!(
            "{}: {}",
            state
                .ui_text
                .get("deck_builder_import_error_unknown_baddie"),
            character_id
        ),
    }
}
