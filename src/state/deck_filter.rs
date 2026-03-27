//! Deck-builder faceted filter state and matching.

use crate::data::{CardAlignment, CardSpeed, StoryCardDefinition};

use super::DeckSearchCardContext;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DeckFilterState {
    pub speeds: Vec<CardSpeed>,
    pub alignments: Vec<CardAlignment>,
    pub card_types: Vec<String>,
    pub owned_only: bool,
    pub missing_only: bool,
    pub in_deck_only: bool,
    pub not_in_deck_only: bool,
}

impl DeckFilterState {
    pub fn clear(&mut self) {
        self.speeds.clear();
        self.alignments.clear();
        self.card_types.clear();
        self.owned_only = false;
        self.missing_only = false;
        self.in_deck_only = false;
        self.not_in_deck_only = false;
    }

    pub fn has_active_filters(&self) -> bool {
        !self.speeds.is_empty()
            || !self.alignments.is_empty()
            || !self.card_types.is_empty()
            || self.owned_only
            || self.missing_only
            || self.in_deck_only
            || self.not_in_deck_only
    }

    pub fn toggle_speed(&mut self, speed: CardSpeed) {
        toggle_value(&mut self.speeds, speed);
    }

    pub fn toggle_alignment(&mut self, alignment: CardAlignment) {
        toggle_value(&mut self.alignments, alignment);
    }

    pub fn toggle_card_type(&mut self, card_type: &str) {
        if let Some(index) = self
            .card_types
            .iter()
            .position(|entry| entry.eq_ignore_ascii_case(card_type))
        {
            self.card_types.remove(index);
            return;
        }

        self.card_types.push(card_type.to_owned());
    }

    pub fn matches(&self, card: &StoryCardDefinition, context: DeckSearchCardContext) -> bool {
        if !self.speeds.is_empty() && !self.speeds.contains(&card.speed) {
            return false;
        }
        if !self.alignments.is_empty() && !self.alignments.contains(&card.alignment) {
            return false;
        }
        if !self.card_types.is_empty()
            && !self
                .card_types
                .iter()
                .any(|entry| entry.eq_ignore_ascii_case(&card.card_type))
        {
            return false;
        }
        if self.owned_only && context.owned_count == 0 {
            return false;
        }
        if self.missing_only && context.available_count > 0 {
            return false;
        }
        if self.in_deck_only && context.copies_in_deck == 0 {
            return false;
        }
        if self.not_in_deck_only && context.copies_in_deck > 0 {
            return false;
        }

        true
    }
}

fn toggle_value<T>(entries: &mut Vec<T>, target: T)
where
    T: Eq + Copy,
{
    if let Some(index) = entries.iter().position(|entry| *entry == target) {
        entries.remove(index);
        return;
    }

    entries.push(target);
}

#[cfg(test)]
mod tests {
    use crate::data::{CardAlignment, CardEffect, CardSpeed, StoryCardDefinition};

    use super::DeckFilterState;
    use crate::state::DeckSearchCardContext;

    fn sample_card() -> StoryCardDefinition {
        StoryCardDefinition {
            id: "sample".to_owned(),
            name: "Sample".to_owned(),
            card_type: "bond".to_owned(),
            speed: CardSpeed::DailyLife,
            alignment: CardAlignment::MagicalGirl,
            playable_in_daily_life: true,
            effects: vec![CardEffect::RevealFirstHiddenOwnSupport],
        }
    }

    #[test]
    fn facet_filters_stack_together() {
        let mut filters = DeckFilterState::default();
        filters.toggle_speed(CardSpeed::DailyLife);
        filters.toggle_alignment(CardAlignment::MagicalGirl);
        filters.toggle_card_type("bond");
        filters.owned_only = true;
        filters.in_deck_only = true;

        assert!(filters.matches(
            &sample_card(),
            DeckSearchCardContext {
                owned_count: 1,
                available_count: 0,
                copies_in_deck: 2,
            }
        ));

        assert!(!filters.matches(
            &sample_card(),
            DeckSearchCardContext {
                owned_count: 0,
                available_count: 0,
                copies_in_deck: 2,
            }
        ));
    }

    #[test]
    fn clear_resets_all_filters() {
        let mut filters = DeckFilterState::default();
        filters.toggle_speed(CardSpeed::Reaction);
        filters.missing_only = true;
        filters.clear();

        assert!(!filters.has_active_filters());
        assert!(filters.matches(&sample_card(), DeckSearchCardContext::default()));
    }
}
