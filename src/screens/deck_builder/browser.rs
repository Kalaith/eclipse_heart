use macroquad::prelude::*;

use crate::state::{
    card_group_label, compare_story_cards, AppState, CollectionCardKind, DeckBrowserCardStats,
    DeckGroupMode, DeckSearchCardContext, DeckSearchQuery, DeckValidation, DeckViewMode,
};
use crate::ui::card_widgets::{draw_story_card_tile, point_in_rect};
use crate::ui::core::{draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

use super::controls::*;
use super::layout::*;
use super::types::*;
use super::*;
use macroquad_toolkit::ui::draw_ui_text;

impl DeckBuilderScreen {
    pub(super) fn filter_buttons(&self, state: &AppState) -> Vec<FilterButton> {
        let mut buttons = vec![
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_speed_daily"),
                FilterButtonKind::Speed(crate::data::CardSpeed::DailyLife),
                self.filters
                    .speeds
                    .contains(&crate::data::CardSpeed::DailyLife),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_speed_reaction"),
                FilterButtonKind::Speed(crate::data::CardSpeed::Reaction),
                self.filters
                    .speeds
                    .contains(&crate::data::CardSpeed::Reaction),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_speed_encounter"),
                FilterButtonKind::Speed(crate::data::CardSpeed::Encounter),
                self.filters
                    .speeds
                    .contains(&crate::data::CardSpeed::Encounter),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_align_mg"),
                FilterButtonKind::Alignment(crate::data::CardAlignment::MagicalGirl),
                self.filters
                    .alignments
                    .contains(&crate::data::CardAlignment::MagicalGirl),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_align_baddie"),
                FilterButtonKind::Alignment(crate::data::CardAlignment::Baddie),
                self.filters
                    .alignments
                    .contains(&crate::data::CardAlignment::Baddie),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_align_neutral"),
                FilterButtonKind::Alignment(crate::data::CardAlignment::Neutral),
                self.filters
                    .alignments
                    .contains(&crate::data::CardAlignment::Neutral),
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_owned"),
                FilterButtonKind::OwnedOnly,
                self.filters.owned_only,
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_missing"),
                FilterButtonKind::MissingOnly,
                self.filters.missing_only,
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_in_deck"),
                FilterButtonKind::InDeckOnly,
                self.filters.in_deck_only,
            ),
            FilterButton::new(
                state.ui_text.get("deck_builder_filter_not_in_deck"),
                FilterButtonKind::NotInDeckOnly,
                self.filters.not_in_deck_only,
            ),
        ];

        for card_type in unique_card_types(state) {
            let is_active = self
                .filters
                .card_types
                .iter()
                .any(|entry| entry.eq_ignore_ascii_case(&card_type));
            buttons.push(FilterButton::new(
                title_case_card_type(&card_type),
                FilterButtonKind::CardType(card_type),
                is_active,
            ));
        }

        buttons
    }

    pub(super) fn active_filter_chips(&self, state: &AppState) -> Vec<FilterChip> {
        let mut chips = Vec::new();

        for speed in &self.filters.speeds {
            chips.push(FilterChip::new(
                speed_filter_label(state, *speed),
                FilterChipKind::Speed(*speed),
            ));
        }
        for alignment in &self.filters.alignments {
            chips.push(FilterChip::new(
                alignment_filter_label(state, *alignment),
                FilterChipKind::Alignment(*alignment),
            ));
        }
        for card_type in &self.filters.card_types {
            chips.push(FilterChip::new(
                title_case_card_type(card_type),
                FilterChipKind::CardType(card_type.clone()),
            ));
        }
        if self.filters.owned_only {
            chips.push(FilterChip::new(
                state.ui_text.get("deck_builder_filter_owned"),
                FilterChipKind::OwnedOnly,
            ));
        }
        if self.filters.missing_only {
            chips.push(FilterChip::new(
                state.ui_text.get("deck_builder_filter_missing"),
                FilterChipKind::MissingOnly,
            ));
        }
        if self.filters.in_deck_only {
            chips.push(FilterChip::new(
                state.ui_text.get("deck_builder_filter_in_deck"),
                FilterChipKind::InDeckOnly,
            ));
        }
        if self.filters.not_in_deck_only {
            chips.push(FilterChip::new(
                state.ui_text.get("deck_builder_filter_not_in_deck"),
                FilterChipKind::NotInDeckOnly,
            ));
        }

        chips
    }

    pub(super) fn toggle_filter(&mut self, kind: FilterButtonKind) {
        match kind {
            FilterButtonKind::Speed(speed) => self.filters.toggle_speed(speed),
            FilterButtonKind::Alignment(alignment) => self.filters.toggle_alignment(alignment),
            FilterButtonKind::CardType(card_type) => self.filters.toggle_card_type(&card_type),
            FilterButtonKind::OwnedOnly => self.filters.owned_only = !self.filters.owned_only,
            FilterButtonKind::MissingOnly => self.filters.missing_only = !self.filters.missing_only,
            FilterButtonKind::InDeckOnly => self.filters.in_deck_only = !self.filters.in_deck_only,
            FilterButtonKind::NotInDeckOnly => {
                self.filters.not_in_deck_only = !self.filters.not_in_deck_only
            }
        }
    }

    pub(super) fn remove_filter_chip(&mut self, kind: FilterChipKind) {
        match kind {
            FilterChipKind::Speed(speed) => self.filters.toggle_speed(speed),
            FilterChipKind::Alignment(alignment) => self.filters.toggle_alignment(alignment),
            FilterChipKind::CardType(card_type) => self.filters.toggle_card_type(&card_type),
            FilterChipKind::OwnedOnly => self.filters.owned_only = false,
            FilterChipKind::MissingOnly => self.filters.missing_only = false,
            FilterChipKind::InDeckOnly => self.filters.in_deck_only = false,
            FilterChipKind::NotInDeckOnly => self.filters.not_in_deck_only = false,
        }
    }

    pub(super) fn filtered_story_cards<'a>(
        &self,
        state: &'a AppState,
    ) -> Vec<&'a crate::data::StoryCardDefinition> {
        let query = DeckSearchQuery::parse(&self.search_text);
        state
            .content
            .story_cards
            .iter()
            .filter(|card| {
                let context = self.story_card_search_context(state, &card.id);
                query.matches(card, context) && self.filters.matches(card, context)
            })
            .collect()
    }

    pub(super) fn filtered_sorted_story_cards<'a>(
        &self,
        state: &'a AppState,
    ) -> Vec<(&'a crate::data::StoryCardDefinition, DeckBrowserCardStats)> {
        let mut cards = self
            .filtered_story_cards(state)
            .into_iter()
            .map(|card| {
                let original_index = state
                    .content
                    .story_cards
                    .iter()
                    .position(|entry| entry.id == card.id)
                    .unwrap_or(0);
                let context = self.story_card_search_context(state, &card.id);
                (
                    card,
                    DeckBrowserCardStats {
                        original_index,
                        owned_count: context.owned_count,
                        copies_in_deck: context.copies_in_deck,
                    },
                )
            })
            .collect::<Vec<_>>();

        cards.sort_by(|left, right| compare_story_cards(*left, *right, self.sort_mode));
        cards
    }

    pub(super) fn browser_layout_items<'a>(
        &self,
        state: &'a AppState,
    ) -> Vec<BrowserLayoutItem<'a>> {
        let mut items = Vec::new();
        let mut current_y = browser_content_start_y();
        let columns = match self.view_mode {
            DeckViewMode::Grid => 4,
            DeckViewMode::CompactList => 1,
        };

        let sorted_cards = self.filtered_sorted_story_cards(state);
        let mut grouped_cards = Vec::<(Option<String>, Vec<_>)>::new();
        for (card, stats) in sorted_cards {
            let group_label = card_group_label(card, self.group_mode).map(|label| {
                if self.group_mode == DeckGroupMode::CardType {
                    title_case_card_type(&label)
                } else {
                    label
                }
            });

            if grouped_cards
                .last()
                .map(|(label, _)| *label == group_label)
                .unwrap_or(false)
            {
                if let Some((_, entries)) = grouped_cards.last_mut() {
                    entries.push((card, stats));
                }
            } else {
                grouped_cards.push((group_label, vec![(card, stats)]));
            }
        }

        for (group_label, entries) in grouped_cards {
            let entry_count = entries.len();
            if let Some(label) = group_label {
                let rect = Rect::new(
                    UiLayout::current().x(560.0),
                    UiLayout::current().y(current_y),
                    UiLayout::current().w(1470.0),
                    UiLayout::current().h(28.0),
                );
                items.push(BrowserLayoutItem::GroupHeader { label, rect });
                current_y += 38.0;
            }

            for (index, (card, _stats)) in entries.into_iter().enumerate() {
                let row = index / columns;
                let column = index % columns;
                let rect = browser_card_rect(self.view_mode, current_y, row, column);
                let add_rect = browser_add_rect(self.view_mode, rect);
                let remove_rect = browser_remove_rect(self.view_mode, rect);
                items.push(BrowserLayoutItem::Card(BrowserCardLayout {
                    card,
                    rect,
                    add_rect,
                    remove_rect,
                }));
            }

            let rows = entry_count.div_ceil(columns);
            current_y += match self.view_mode {
                DeckViewMode::Grid => rows as f32 * 134.0 + 10.0,
                DeckViewMode::CompactList => rows as f32 * 82.0 + 10.0,
            };
        }

        items
    }

    pub(super) fn story_card_search_context(
        &self,
        state: &AppState,
        card_id: &str,
    ) -> DeckSearchCardContext {
        let owned_count = state
            .saves
            .collection
            .owned_count(CollectionCardKind::StoryCard, card_id);
        let copies_in_deck = state.saves.decks.card_count(card_id);
        DeckSearchCardContext {
            owned_count,
            available_count: state
                .saves
                .collection
                .story_cards_available_for_deck(card_id, copies_in_deck),
            copies_in_deck,
        }
    }

    pub(super) fn draw_validation_summary(&self, state: &AppState) {
        let ui = UiLayout::current();
        let rect = summary_panel_content_rect();

        if let Some(deck) = state.saves.decks.selected_support_deck() {
            let validation =
                DeckValidation::for_deck(deck, &state.content.deck_rules, &state.saves.collection);
            let legal_color = if validation.is_legal { LIME } else { PINK };
            let collection_color = if validation.is_collection_complete {
                SKYBLUE
            } else {
                GOLD
            };

            draw_ui_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_status_label"),
                    if validation.is_legal {
                        state.ui_text.get("deck_builder_legal_status")
                    } else {
                        state.ui_text.get("deck_builder_illegal_status")
                    }
                ),
                rect.x,
                rect.y,
                ui.font(24.0),
                legal_color,
            );
            draw_ui_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_collection_label"),
                    if validation.is_collection_complete {
                        state.ui_text.get("deck_builder_collection_complete")
                    } else {
                        state.ui_text.get("deck_builder_collection_incomplete")
                    }
                ),
                rect.x,
                rect.y + ui.h(30.0),
                ui.font(20.0),
                collection_color,
            );
            draw_ui_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_missing_count_label"),
                    validation.missing_card_total
                ),
                rect.x,
                rect.y + ui.h(52.0),
                ui.font(20.0),
                if validation.missing_card_total == 0 {
                    WHITE
                } else {
                    GOLD
                },
            );
            draw_ui_text(
                &format!(
                    "{}: {}/{}",
                    state.ui_text.get("deck_builder_support_summary_label"),
                    validation.support_card_count,
                    validation.required_support_card_count
                ),
                rect.x,
                rect.y + ui.h(82.0),
                ui.font(20.0),
                if validation.support_card_count_valid {
                    WHITE
                } else {
                    GOLD
                },
            );
            draw_ui_text(
                &format!(
                    "{}: {}/{}",
                    state.ui_text.get("deck_builder_magical_girl_summary_label"),
                    validation.magical_girl_roster_count,
                    validation.required_magical_girl_roster_count
                ),
                rect.x,
                rect.y + ui.h(110.0),
                ui.font(20.0),
                if validation.magical_girl_roster_valid {
                    WHITE
                } else {
                    GOLD
                },
            );
            draw_ui_text(
                &format!(
                    "{}: {}/{}",
                    state.ui_text.get("deck_builder_baddie_summary_label"),
                    validation.baddie_roster_count,
                    validation.required_baddie_roster_count
                ),
                rect.x,
                rect.y + ui.h(138.0),
                ui.font(20.0),
                if validation.baddie_roster_valid {
                    WHITE
                } else {
                    GOLD
                },
            );

            let issues = self.validation_issue_lines(state, &validation);
            draw_ui_text(
                state.ui_text.get("deck_builder_warnings_label"),
                rect.x,
                rect.y + ui.h(176.0),
                ui.font(22.0),
                WHITE,
            );

            if issues.is_empty() {
                draw_ui_text(
                    state.ui_text.get("deck_builder_no_warnings"),
                    rect.x,
                    rect.y + ui.h(206.0),
                    ui.font(18.0),
                    TEXT_MUTED,
                );
                return;
            }

            let mut issue_y = rect.y + ui.h(206.0);
            for issue in issues.into_iter().take(4) {
                draw_ui_text(&issue, rect.x, issue_y, ui.font(16.0), TEXT_MUTED);
                issue_y += ui.h(24.0);
            }
        } else {
            draw_ui_text(
                state.ui_text.get("deck_builder_missing_deck"),
                rect.x,
                rect.y,
                ui.font(24.0),
                TEXT_MUTED,
            );
            draw_ui_text(
                state.ui_text.get("deck_builder_summary_empty_body"),
                rect.x,
                rect.y + ui.h(34.0),
                ui.font(18.0),
                TEXT_MUTED,
            );
        }
    }

    pub(super) fn draw_support_card_grid(&self, state: &AppState, has_active_deck: bool) {
        if self.active_tab != DeckBuilderTab::SupportCards {
            return;
        }

        let ui = UiLayout::current();
        let mouse = mouse_position();
        for item in self.browser_layout_items(state) {
            match item {
                BrowserLayoutItem::GroupHeader { label, rect } => {
                    draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
                    draw_ui_text(
                        &label,
                        rect.x + ui.w(12.0),
                        rect.y + ui.h(20.0),
                        ui.font(18.0),
                        GOLD,
                    );
                }
                BrowserLayoutItem::Card(card_layout) => {
                    let card = card_layout.card;
                    let copies = state.saves.decks.card_count(&card.id);
                    let owned = state
                        .saves
                        .collection
                        .owned_count(CollectionCardKind::StoryCard, &card.id);
                    let available = state
                        .saves
                        .collection
                        .story_cards_available_for_deck(&card.id, copies);
                    let is_missing_in_deck = copies > owned as usize;
                    let hovered = point_in_rect(card_layout.rect, mouse);

                    draw_story_card_tile(
                        state,
                        card_layout.rect,
                        card,
                        &format!(
                            "{} {} | {} {}",
                            state.ui_text.get("deck_builder_owned_label"),
                            owned,
                            state.ui_text.get("deck_builder_copies_label"),
                            copies
                        ),
                        state.saves.decks.can_add_card(
                            &card.id,
                            &state.content.deck_rules,
                            &state.saves.collection,
                        ),
                        hovered,
                    );
                    if is_missing_in_deck {
                        draw_rectangle_lines(
                            card_layout.rect.x,
                            card_layout.rect.y,
                            card_layout.rect.w,
                            card_layout.rect.h,
                            ui.w(4.0),
                            GOLD,
                        );
                        draw_ui_text(
                            state.ui_text.get("deck_builder_missing_short"),
                            card_layout.rect.x + ui.w(12.0),
                            card_layout.rect.y + ui.h(24.0),
                            ui.font(16.0),
                            GOLD,
                        );
                    }

                    draw_soft_panel(
                        card_layout.add_rect.x,
                        card_layout.add_rect.y,
                        card_layout.add_rect.w,
                        card_layout.add_rect.h,
                        if available > 0 && has_active_deck {
                            SKYBLUE
                        } else {
                            DARKGRAY
                        },
                    );
                    draw_ui_text(
                        if available > 0 && has_active_deck {
                            state.ui_text.get("deck_builder_add_card")
                        } else {
                            state.ui_text.get("deck_builder_add_locked")
                        },
                        card_layout.add_rect.x + ui.w(12.0),
                        card_layout.add_rect.y + ui.h(24.0),
                        ui.font(16.0),
                        WHITE,
                    );

                    draw_soft_panel(
                        card_layout.remove_rect.x,
                        card_layout.remove_rect.y,
                        card_layout.remove_rect.w,
                        card_layout.remove_rect.h,
                        if copies > 0 { PINK } else { DARKGRAY },
                    );
                    draw_ui_text(
                        if copies > 0 {
                            state.ui_text.get("deck_builder_remove_card")
                        } else {
                            state.ui_text.get("deck_builder_remove_locked")
                        },
                        card_layout.remove_rect.x + ui.w(10.0),
                        card_layout.remove_rect.y + ui.h(24.0),
                        ui.font(16.0),
                        WHITE,
                    );
                }
            }
        }
    }
}
