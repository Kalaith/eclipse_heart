//! Cross-platform save timestamp helpers.

pub fn current_unix_timestamp() -> i64 {
    macroquad::miniquad::date::now().floor() as i64
}
