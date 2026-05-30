use crate::state::{AppState, CollectionCardKind, DeckGroupMode, DeckSortMode, DeckViewMode};

use super::types::*;

pub(super) fn deck_action_buttons<'a>(state: &'a AppState) -> [DeckActionButton<'a>; 4] {
    let has_selected_deck = state.saves.decks.selected_support_deck().is_some();
    [
        DeckActionButton {
            kind: DeckActionKind::Create,
            label: state.ui_text.get("deck_builder_new_deck"),
            enabled: true,
        },
        DeckActionButton {
            kind: DeckActionKind::Rename,
            label: state.ui_text.get("deck_builder_rename_deck"),
            enabled: has_selected_deck,
        },
        DeckActionButton {
            kind: DeckActionKind::Duplicate,
            label: state.ui_text.get("deck_builder_duplicate_deck"),
            enabled: has_selected_deck,
        },
        DeckActionButton {
            kind: DeckActionKind::Delete,
            label: state.ui_text.get("deck_builder_delete_deck"),
            enabled: has_selected_deck,
        },
    ]
}

pub(super) fn deck_transfer_buttons<'a>(state: &'a AppState) -> [DeckTransferButton<'a>; 2] {
    let has_selected_deck = state.saves.decks.selected_support_deck().is_some();
    [
        DeckTransferButton {
            kind: DeckTransferActionKind::Export,
            label: state.ui_text.get("deck_builder_export_deck"),
            enabled: has_selected_deck,
        },
        DeckTransferButton {
            kind: DeckTransferActionKind::Import,
            label: state.ui_text.get("deck_builder_import_deck"),
            enabled: true,
        },
    ]
}

pub(super) fn deck_utility_buttons<'a>(state: &'a AppState) -> [DeckUtilityButton<'a>; 3] {
    let selected_deck = state.saves.decks.selected_support_deck();
    [
        DeckUtilityButton {
            kind: DeckUtilityActionKind::Metadata,
            label: state.ui_text.get("deck_builder_metadata_button"),
            enabled: selected_deck.is_some(),
        },
        DeckUtilityButton {
            kind: DeckUtilityActionKind::Undo,
            label: state.ui_text.get("deck_builder_undo_button"),
            enabled: state.saves.decks.can_undo_selected_deck_change(),
        },
        DeckUtilityButton {
            kind: DeckUtilityActionKind::Reset,
            label: state.ui_text.get("deck_builder_reset_button"),
            enabled: selected_deck
                .and_then(|deck| deck.source_template_id.as_deref())
                .is_some(),
        },
    ]
}

pub(super) fn collection_kind_label<'a>(state: &'a AppState, kind: CollectionCardKind) -> &'a str {
    match kind {
        CollectionCardKind::MagicalGirl => state.ui_text.get("deck_builder_kind_magical_girl"),
        CollectionCardKind::Baddie => state.ui_text.get("deck_builder_kind_baddie"),
        CollectionCardKind::StoryCard => state.ui_text.get("deck_builder_kind_story"),
    }
}

pub(super) fn speed_filter_label(state: &AppState, speed: crate::data::CardSpeed) -> &str {
    match speed {
        crate::data::CardSpeed::DailyLife => state.ui_text.get("deck_builder_filter_speed_daily"),
        crate::data::CardSpeed::Reaction => state.ui_text.get("deck_builder_filter_speed_reaction"),
        crate::data::CardSpeed::Encounter => {
            state.ui_text.get("deck_builder_filter_speed_encounter")
        }
    }
}

pub(super) fn alignment_filter_label(
    state: &AppState,
    alignment: crate::data::CardAlignment,
) -> &str {
    match alignment {
        crate::data::CardAlignment::MagicalGirl => {
            state.ui_text.get("deck_builder_filter_align_mg")
        }
        crate::data::CardAlignment::Baddie => state.ui_text.get("deck_builder_filter_align_baddie"),
        crate::data::CardAlignment::Neutral => {
            state.ui_text.get("deck_builder_filter_align_neutral")
        }
    }
}

pub(super) fn unique_card_types(state: &AppState) -> Vec<String> {
    let mut card_types = state
        .content
        .story_cards
        .iter()
        .map(|card| card.card_type.clone())
        .collect::<Vec<_>>();
    card_types.sort_by_key(|entry| entry.to_ascii_lowercase());
    card_types.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    card_types
}

pub(super) fn title_case_card_type(card_type: &str) -> String {
    card_type
        .split('_')
        .map(|part| {
            let mut characters = part.chars();
            match characters.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), characters.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn next_sort_mode(mode: DeckSortMode) -> DeckSortMode {
    match mode {
        DeckSortMode::Alphabetical => DeckSortMode::Newest,
        DeckSortMode::Newest => DeckSortMode::OwnedCount,
        DeckSortMode::OwnedCount => DeckSortMode::CopiesInDeck,
        DeckSortMode::CopiesInDeck => DeckSortMode::Alphabetical,
    }
}

pub(super) fn next_group_mode(mode: DeckGroupMode) -> DeckGroupMode {
    match mode {
        DeckGroupMode::None => DeckGroupMode::Alignment,
        DeckGroupMode::Alignment => DeckGroupMode::Speed,
        DeckGroupMode::Speed => DeckGroupMode::CardType,
        DeckGroupMode::CardType => DeckGroupMode::None,
    }
}

pub(super) fn next_view_mode(mode: DeckViewMode) -> DeckViewMode {
    match mode {
        DeckViewMode::Grid => DeckViewMode::CompactList,
        DeckViewMode::CompactList => DeckViewMode::Grid,
    }
}

pub(super) fn sort_mode_label(state: &AppState, mode: DeckSortMode) -> &str {
    match mode {
        DeckSortMode::Alphabetical => state.ui_text.get("deck_builder_sort_alphabetical"),
        DeckSortMode::Newest => state.ui_text.get("deck_builder_sort_newest"),
        DeckSortMode::OwnedCount => state.ui_text.get("deck_builder_sort_owned"),
        DeckSortMode::CopiesInDeck => state.ui_text.get("deck_builder_sort_in_deck"),
    }
}

pub(super) fn group_mode_label(state: &AppState, mode: DeckGroupMode) -> &str {
    match mode {
        DeckGroupMode::None => state.ui_text.get("deck_builder_group_none"),
        DeckGroupMode::Alignment => state.ui_text.get("deck_builder_group_alignment"),
        DeckGroupMode::Speed => state.ui_text.get("deck_builder_group_speed"),
        DeckGroupMode::CardType => state.ui_text.get("deck_builder_group_card_type"),
    }
}

pub(super) fn view_mode_label(state: &AppState, mode: DeckViewMode) -> &str {
    match mode {
        DeckViewMode::Grid => state.ui_text.get("deck_builder_view_grid"),
        DeckViewMode::CompactList => state.ui_text.get("deck_builder_view_list"),
    }
}
