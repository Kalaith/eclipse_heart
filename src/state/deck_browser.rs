//! Deck-builder browser sort, group, and view modes.

use std::cmp::Ordering;

use crate::data::{CardAlignment, CardSpeed, StoryCardDefinition};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeckSortMode {
    Alphabetical,
    Newest,
    OwnedCount,
    CopiesInDeck,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeckGroupMode {
    None,
    Alignment,
    Speed,
    CardType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeckViewMode {
    Grid,
    CompactList,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeckBrowserCardStats {
    pub original_index: usize,
    pub owned_count: u32,
    pub copies_in_deck: usize,
}

pub fn compare_story_cards(
    left: (&StoryCardDefinition, DeckBrowserCardStats),
    right: (&StoryCardDefinition, DeckBrowserCardStats),
    mode: DeckSortMode,
) -> Ordering {
    match mode {
        DeckSortMode::Alphabetical => left
            .0
            .name
            .cmp(&right.0.name)
            .then(left.0.id.cmp(&right.0.id)),
        DeckSortMode::Newest => right
            .1
            .original_index
            .cmp(&left.1.original_index)
            .then(left.0.name.cmp(&right.0.name)),
        DeckSortMode::OwnedCount => right
            .1
            .owned_count
            .cmp(&left.1.owned_count)
            .then(left.0.name.cmp(&right.0.name)),
        DeckSortMode::CopiesInDeck => right
            .1
            .copies_in_deck
            .cmp(&left.1.copies_in_deck)
            .then(left.0.name.cmp(&right.0.name)),
    }
}

pub fn card_group_label(card: &StoryCardDefinition, mode: DeckGroupMode) -> Option<String> {
    match mode {
        DeckGroupMode::None => None,
        DeckGroupMode::Alignment => Some(match card.alignment {
            CardAlignment::MagicalGirl => "Magical Girl".to_owned(),
            CardAlignment::Baddie => "Baddie".to_owned(),
            CardAlignment::Neutral => "Neutral".to_owned(),
        }),
        DeckGroupMode::Speed => Some(match card.speed {
            CardSpeed::DailyLife => "Daily".to_owned(),
            CardSpeed::Reaction => "Reaction".to_owned(),
            CardSpeed::Encounter => "Encounter".to_owned(),
        }),
        DeckGroupMode::CardType => Some(card.card_type.clone()),
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::data::{CardAlignment, CardEffect, CardSpeed, StoryCardDefinition};

    use super::{
        card_group_label, compare_story_cards, DeckBrowserCardStats, DeckGroupMode, DeckSortMode,
    };

    fn card(
        id: &str,
        name: &str,
        speed: CardSpeed,
        alignment: CardAlignment,
        card_type: &str,
    ) -> StoryCardDefinition {
        StoryCardDefinition {
            id: id.to_owned(),
            name: name.to_owned(),
            card_type: card_type.to_owned(),
            speed,
            alignment,
            playable_in_daily_life: speed == CardSpeed::DailyLife,
            effects: vec![CardEffect::RevealFirstHiddenOwnSupport],
        }
    }

    #[test]
    fn sorting_uses_requested_mode() {
        let alpha = card(
            "a",
            "Alpha",
            CardSpeed::DailyLife,
            CardAlignment::MagicalGirl,
            "bond",
        );
        let beta = card(
            "b",
            "Beta",
            CardSpeed::Reaction,
            CardAlignment::Baddie,
            "scheme",
        );

        assert_eq!(
            compare_story_cards(
                (
                    &alpha,
                    DeckBrowserCardStats {
                        original_index: 0,
                        owned_count: 1,
                        copies_in_deck: 0
                    }
                ),
                (
                    &beta,
                    DeckBrowserCardStats {
                        original_index: 1,
                        owned_count: 3,
                        copies_in_deck: 2
                    }
                ),
                DeckSortMode::Alphabetical
            ),
            Ordering::Less
        );
        assert_eq!(
            compare_story_cards(
                (
                    &alpha,
                    DeckBrowserCardStats {
                        original_index: 0,
                        owned_count: 1,
                        copies_in_deck: 0
                    }
                ),
                (
                    &beta,
                    DeckBrowserCardStats {
                        original_index: 1,
                        owned_count: 3,
                        copies_in_deck: 2
                    }
                ),
                DeckSortMode::Newest
            ),
            Ordering::Greater
        );
    }

    #[test]
    fn grouping_returns_expected_labels() {
        let sample = card(
            "a",
            "Alpha",
            CardSpeed::DailyLife,
            CardAlignment::MagicalGirl,
            "bond",
        );

        assert_eq!(
            card_group_label(&sample, DeckGroupMode::Alignment),
            Some("Magical Girl".to_owned())
        );
        assert_eq!(
            card_group_label(&sample, DeckGroupMode::Speed),
            Some("Daily".to_owned())
        );
        assert_eq!(
            card_group_label(&sample, DeckGroupMode::CardType),
            Some("bond".to_owned())
        );
    }
}
