//! Versioned deck and roster preset save data.

use serde::{Deserialize, Serialize};

use crate::data::{DeckRules, StarterLoadout};

use super::CollectionSave;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DeckPreset {
    pub id: String,
    pub name: String,
    pub story_cards: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecksSave {
    pub version: u32,
    pub support_decks: Vec<DeckPreset>,
    pub roster_presets: Vec<String>,
}

impl Default for DecksSave {
    fn default() -> Self {
        Self {
            version: 1,
            support_decks: Vec::new(),
            roster_presets: Vec::new(),
        }
    }
}

impl DecksSave {
    pub fn ensure_active_support_deck(&mut self, starters: &[StarterLoadout]) {
        if !self.support_decks.is_empty() {
            return;
        }

        if let Some(starter) = starters.first() {
            self.support_decks.push(DeckPreset {
                id: starter.id.clone(),
                name: starter.name.clone(),
                story_cards: starter.support_deck.clone(),
            });
        } else {
            self.support_decks.push(DeckPreset {
                id: "prototype_support_deck".to_owned(),
                name: "Prototype Support Deck".to_owned(),
                story_cards: Vec::new(),
            });
        }
    }

    pub fn active_support_deck(&self) -> Option<&DeckPreset> {
        self.support_decks.first()
    }

    pub fn active_support_deck_mut(&mut self) -> Option<&mut DeckPreset> {
        self.support_decks.first_mut()
    }

    pub fn card_count(&self, card_id: &str) -> usize {
        self.active_support_deck()
            .map(|deck| {
                deck.story_cards
                    .iter()
                    .filter(|entry| entry.as_str() == card_id)
                    .count()
            })
            .unwrap_or(0)
    }

    pub fn can_add_card(
        &self,
        card_id: &str,
        deck_rules: &DeckRules,
        collection: &CollectionSave,
    ) -> bool {
        let Some(deck) = self.active_support_deck() else {
            return false;
        };

        deck.story_cards.len() < deck_rules.support_deck_size
            && self.card_count(card_id) < deck_rules.max_copies_per_story_card
            && collection.story_cards_available_for_deck(card_id, self.card_count(card_id)) > 0
    }

    pub fn add_card(
        &mut self,
        card_id: &str,
        deck_rules: &DeckRules,
        collection: &CollectionSave,
    ) -> bool {
        if !self.can_add_card(card_id, deck_rules, collection) {
            return false;
        }

        if let Some(deck) = self.active_support_deck_mut() {
            deck.story_cards.push(card_id.to_owned());
            return true;
        }

        false
    }

    pub fn remove_card(&mut self, card_id: &str) -> bool {
        let Some(deck) = self.active_support_deck_mut() else {
            return false;
        };

        let Some(index) = deck.story_cards.iter().position(|entry| entry == card_id) else {
            return false;
        };

        deck.story_cards.remove(index);
        true
    }

    pub fn load_starter_into_active(&mut self, starter: &StarterLoadout) {
        self.ensure_active_support_deck(std::slice::from_ref(starter));
        if let Some(deck) = self.active_support_deck_mut() {
            deck.id = starter.id.clone();
            deck.name = starter.name.clone();
            deck.story_cards = starter.support_deck.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{DeckRules, StarterLoadout};

    use super::{CollectionSave, DecksSave};
    use crate::state::CollectionCardKind;

    fn sample_starter() -> StarterLoadout {
        StarterLoadout {
            id: "starter_alpha".to_owned(),
            name: "Starter Alpha".to_owned(),
            magical_girl_main: "yuki".to_owned(),
            magical_girl_supports: vec!["hana".to_owned(), "riri".to_owned()],
            prime_baddie: "noctra".to_owned(),
            baddie_supports: vec!["glass_crow".to_owned(), "thorn_waltz".to_owned()],
            support_deck: vec!["quiet_lunch_on_the_rooftop".to_owned()],
        }
    }

    #[test]
    fn deck_save_creates_default_active_deck_and_respects_copy_limit() {
        let starter = sample_starter();
        let mut decks = DecksSave::default();
        let rules = DeckRules {
            support_deck_size: 5,
            max_copies_per_story_card: 2,
            universal_copy_limit: true,
        };

        decks.ensure_active_support_deck(std::slice::from_ref(&starter));
        assert_eq!(
            decks.active_support_deck().map(|deck| deck.name.as_str()),
            Some("Starter Alpha")
        );

        let mut collection = CollectionSave::default();
        collection.add_owned(CollectionCardKind::StoryCard, "not_on_my_watch", 2);

        assert!(decks.add_card("not_on_my_watch", &rules, &collection));
        assert!(decks.add_card("not_on_my_watch", &rules, &collection));
        assert!(!decks.add_card("not_on_my_watch", &rules, &collection));

        assert!(decks.remove_card("not_on_my_watch"));
        assert_eq!(decks.card_count("not_on_my_watch"), 1);
    }
}
