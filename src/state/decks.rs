//! Versioned deck and roster preset save data.

use serde::{Deserialize, Serialize};

use crate::data::{DeckRules, StarterLoadout};

use super::CollectionSave;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DeckPreset {
    pub id: String,
    pub name: String,
    pub story_cards: Vec<String>,
    #[serde(default)]
    pub magical_girl_roster: Vec<String>,
    #[serde(default)]
    pub baddie_roster: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecksSave {
    pub version: u32,
    pub support_decks: Vec<DeckPreset>,
    #[serde(default)]
    pub active_support_deck_index: usize,
    pub roster_presets: Vec<String>,
}

impl Default for DecksSave {
    fn default() -> Self {
        Self {
            version: 1,
            support_decks: Vec::new(),
            active_support_deck_index: 0,
            roster_presets: Vec::new(),
        }
    }
}

impl DecksSave {
    pub fn ensure_active_support_deck(
        &mut self,
        starters: &[StarterLoadout],
        magical_girl_ids: &[String],
        baddie_ids: &[String],
    ) {
        if !self.support_decks.is_empty() {
            self.active_support_deck_index = self
                .active_support_deck_index
                .min(self.support_decks.len().saturating_sub(1));
            for deck in &mut self.support_decks {
                fill_missing_roster(&mut deck.magical_girl_roster, magical_girl_ids);
                fill_missing_roster(&mut deck.baddie_roster, baddie_ids);
            }
            return;
        }

        if let Some(starter) = starters.first() {
            self.support_decks.push(DeckPreset {
                id: starter.id.clone(),
                name: starter.name.clone(),
                story_cards: starter.support_deck.clone(),
                magical_girl_roster: magical_girl_ids.to_vec(),
                baddie_roster: baddie_ids.to_vec(),
            });
        } else {
            self.support_decks.push(DeckPreset {
                id: "prototype_support_deck".to_owned(),
                name: "Prototype Support Deck".to_owned(),
                story_cards: Vec::new(),
                magical_girl_roster: magical_girl_ids.to_vec(),
                baddie_roster: baddie_ids.to_vec(),
            });
        }
        self.active_support_deck_index = 0;
    }

    pub fn active_support_deck(&self) -> Option<&DeckPreset> {
        self.support_decks.get(self.active_support_deck_index)
    }

    pub fn active_support_deck_mut(&mut self) -> Option<&mut DeckPreset> {
        self.support_decks.get_mut(self.active_support_deck_index)
    }

    pub fn preset_for_starter(&self, starter_id: &str) -> Option<&DeckPreset> {
        self.support_decks.iter().find(|deck| deck.id == starter_id)
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

    pub fn edit_starter_deck(
        &mut self,
        starter: &StarterLoadout,
        magical_girl_ids: &[String],
        baddie_ids: &[String],
    ) {
        if let Some(existing_index) = self
            .support_decks
            .iter()
            .position(|deck| deck.id == starter.id)
        {
            self.active_support_deck_index = existing_index;
            if let Some(deck) = self.active_support_deck_mut() {
                fill_missing_roster(&mut deck.magical_girl_roster, magical_girl_ids);
                fill_missing_roster(&mut deck.baddie_roster, baddie_ids);
            }
            return;
        }

        self.support_decks.push(DeckPreset {
            id: starter.id.clone(),
            name: starter.name.clone(),
            story_cards: starter.support_deck.clone(),
            magical_girl_roster: magical_girl_ids.to_vec(),
            baddie_roster: baddie_ids.to_vec(),
        });
        self.active_support_deck_index = self.support_decks.len().saturating_sub(1);
    }

    pub fn set_roster_slot(
        &mut self,
        is_magical_girl_side: bool,
        slot_index: usize,
        character_id: &str,
    ) -> bool {
        let Some(deck) = self.active_support_deck_mut() else {
            return false;
        };

        let roster = if is_magical_girl_side {
            &mut deck.magical_girl_roster
        } else {
            &mut deck.baddie_roster
        };

        if slot_index >= roster.len() || roster.iter().any(|entry| entry == character_id) {
            return false;
        }

        roster[slot_index] = character_id.to_owned();
        true
    }
}

fn fill_missing_roster(roster: &mut Vec<String>, fallback: &[String]) {
    if roster.len() == fallback.len() && roster.iter().all(|entry| fallback.contains(entry)) {
        return;
    }

    *roster = fallback.to_vec();
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
        decks.ensure_active_support_deck(
            std::slice::from_ref(&starter),
            &["yuki".to_owned(), "hana".to_owned()],
            &["noctra".to_owned(), "glass_crow".to_owned()],
        );
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

    #[test]
    fn edit_starter_deck_preserves_existing_edits_for_that_starter() {
        let starter = sample_starter();
        let mut decks = DecksSave::default();

        decks.edit_starter_deck(
            &starter,
            &["yuki".to_owned(), "hana".to_owned()],
            &["noctra".to_owned(), "glass_crow".to_owned()],
        );
        decks
            .active_support_deck_mut()
            .expect("active deck")
            .story_cards
            .push("not_on_my_watch".to_owned());

        decks.edit_starter_deck(
            &starter,
            &["yuki".to_owned(), "hana".to_owned()],
            &["noctra".to_owned(), "glass_crow".to_owned()],
        );

        let active = decks.active_support_deck().expect("active deck");
        assert_eq!(active.id, starter.id);
        assert_eq!(active.story_cards.len(), 2);
        assert!(active
            .story_cards
            .iter()
            .any(|card| card == "not_on_my_watch"));
    }

    #[test]
    fn set_roster_slot_rejects_duplicates_and_updates_slot() {
        let starter = sample_starter();
        let mut decks = DecksSave::default();
        let magical_girls = ["yuki".to_owned(), "hana".to_owned(), "riri".to_owned()];
        let baddies = ["noctra".to_owned(), "glass_crow".to_owned(), "thorn_waltz".to_owned()];
        decks.ensure_active_support_deck(std::slice::from_ref(&starter), &magical_girls, &baddies);

        assert!(!decks.set_roster_slot(true, 0, "hana"));
        assert!(decks.set_roster_slot(true, 0, "riri"));
        assert_eq!(
            decks
                .active_support_deck()
                .expect("active deck")
                .magical_girl_roster[0],
            "riri"
        );
    }
}
