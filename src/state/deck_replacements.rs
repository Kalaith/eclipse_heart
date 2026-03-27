//! Ownership-aware replacement suggestions for incomplete decks.

use crate::data::{GameContent, StoryCardDefinition};

use super::{CollectionCardKind, CollectionSave, DeckPreset};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeckReplacementSuggestion {
    pub missing_card_id: String,
    pub replacement_card_ids: Vec<String>,
}

pub fn suggest_story_replacements(
    deck: &DeckPreset,
    content: &GameContent,
    collection: &CollectionSave,
    max_per_missing: usize,
) -> Vec<DeckReplacementSuggestion> {
    let mut suggestions = Vec::new();
    for missing_entry in
        super::DeckValidation::for_deck(deck, &content.deck_rules, collection).missing_story_cards
    {
        let Some(missing_card) = content
            .story_cards
            .iter()
            .find(|card| card.id == missing_entry.id)
        else {
            continue;
        };

        let mut candidates = content
            .story_cards
            .iter()
            .filter(|candidate| candidate.id != missing_card.id)
            .filter(|candidate| {
                collection.owned_count(CollectionCardKind::StoryCard, &candidate.id)
                    > deck
                        .story_cards
                        .iter()
                        .filter(|entry| *entry == &candidate.id)
                        .count() as u32
            })
            .filter(|candidate| {
                deck.story_cards
                    .iter()
                    .filter(|entry| *entry == &candidate.id)
                    .count()
                    < content.deck_rules.max_copies_per_story_card
            })
            .filter(|candidate| candidate.speed == missing_card.speed)
            .map(|candidate| replacement_score(deck, collection, missing_card, candidate))
            .collect::<Vec<_>>();

        candidates.sort_by(|left, right| {
            right
                .0
                .cmp(&left.0)
                .then_with(|| left.1.name.cmp(&right.1.name))
        });
        let replacement_card_ids = candidates
            .into_iter()
            .take(max_per_missing)
            .map(|(_, card)| card.id.clone())
            .collect::<Vec<_>>();

        if !replacement_card_ids.is_empty() {
            suggestions.push(DeckReplacementSuggestion {
                missing_card_id: missing_entry.id,
                replacement_card_ids,
            });
        }
    }

    suggestions
}

fn replacement_score<'a>(
    deck: &DeckPreset,
    collection: &CollectionSave,
    missing_card: &StoryCardDefinition,
    candidate: &'a StoryCardDefinition,
) -> (u8, &'a StoryCardDefinition) {
    let mut score = 0;
    if candidate.alignment == missing_card.alignment {
        score += 4;
    }
    if candidate
        .card_type
        .eq_ignore_ascii_case(&missing_card.card_type)
    {
        score += 2;
    }
    if collection.owned_count(CollectionCardKind::StoryCard, &candidate.id) > 0 {
        score += 1;
    }
    if deck.story_cards.iter().any(|entry| entry == &candidate.id) {
        score += 1;
    }
    (score, candidate)
}

#[cfg(test)]
mod tests {
    use crate::data::{CardAlignment, CardSpeed, GameContent, StoryCardDefinition};
    use crate::state::{CollectionCardKind, CollectionSave, DeckPreset};

    use super::suggest_story_replacements;

    fn card(
        id: &str,
        speed: CardSpeed,
        alignment: CardAlignment,
        card_type: &str,
    ) -> StoryCardDefinition {
        StoryCardDefinition {
            id: id.to_owned(),
            name: id.to_owned(),
            card_type: card_type.to_owned(),
            speed,
            alignment,
            playable_in_daily_life: true,
            effects: Vec::new(),
        }
    }

    #[test]
    fn replacement_suggestions_prefer_matching_alignment_and_type() {
        let mut content = GameContent::default();
        content.deck_rules.support_deck_size = 3;
        content.deck_rules.max_copies_per_story_card = 2;
        content.story_cards = vec![
            card(
                "missing",
                CardSpeed::Reaction,
                CardAlignment::MagicalGirl,
                "guard",
            ),
            card(
                "best_fit",
                CardSpeed::Reaction,
                CardAlignment::MagicalGirl,
                "guard",
            ),
            card(
                "same_speed_only",
                CardSpeed::Reaction,
                CardAlignment::Neutral,
                "setup",
            ),
            card(
                "wrong_speed",
                CardSpeed::Encounter,
                CardAlignment::MagicalGirl,
                "guard",
            ),
        ];

        let deck = DeckPreset {
            id: "deck_1".to_owned(),
            name: "Test".to_owned(),
            source_template_id: None,
            notes: String::new(),
            archetype_tags: Vec::new(),
            story_cards: vec![
                "missing".to_owned(),
                "missing".to_owned(),
                "best_fit".to_owned(),
            ],
            magical_girl_roster: vec!["mg1".to_owned(); 5],
            baddie_roster: vec!["b1".to_owned(); 5],
            created_at_unix: 1,
            updated_at_unix: 1,
            recent_story_cards: Vec::new(),
        };

        let mut collection = CollectionSave::default();
        collection.add_owned(CollectionCardKind::StoryCard, "missing", 1);
        collection.add_owned(CollectionCardKind::StoryCard, "best_fit", 2);
        collection.add_owned(CollectionCardKind::StoryCard, "same_speed_only", 2);

        let suggestions = suggest_story_replacements(&deck, &content, &collection, 2);

        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].missing_card_id, "missing");
        assert_eq!(
            suggestions[0].replacement_card_ids,
            vec!["best_fit".to_owned(), "same_speed_only".to_owned()]
        );
    }
}
