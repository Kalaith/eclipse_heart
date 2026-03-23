//! Player-facing text catalog.

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct UiText {
    pub values: HashMap<String, String>,
}

impl UiText {
    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.values.get(key).map(String::as_str).unwrap_or(key)
    }
}
