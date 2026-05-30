use macroquad::prelude::*;

use crate::data::CharacterDefinition;
use crate::state::{
    AppState, BoosterCardGrant, CollectionCardKind, DeckReplacementSuggestion, DeckValidation,
    DeckValidationCount,
};
use crate::ui::card_widgets::{draw_story_card_preview, point_in_rect};
use crate::ui::core::{draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

use super::layout::*;
use super::types::*;
use super::utils::*;
use super::*;

impl DeckBuilderScreen {
    pub(super) fn draw_preview_panel(&self, state: &AppState) {
        let mouse = mouse_position();

        if self.active_tab == DeckBuilderTab::SupportCards {
            for item in self.browser_layout_items(state) {
                if let BrowserLayoutItem::Card(card_layout) = item {
                    if !point_in_rect(card_layout.rect, mouse) {
                        continue;
                    }

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
                    let preview_rect = preview_panel_content_rect();
                    let footer = vec![
                        format!(
                            "{}: {} / {}",
                            state.ui_text.get("deck_builder_copies_label"),
                            copies,
                            state.content.deck_rules.max_copies_per_story_card
                        ),
                        format!(
                            "{}: {}  {}: {}",
                            state.ui_text.get("deck_builder_owned_label"),
                            owned,
                            state.ui_text.get("deck_builder_available_label"),
                            available
                        ),
                    ];
                    draw_story_card_preview(state, preview_rect, card, &footer);
                    return;
                }
            }
        }

        if self.active_tab != DeckBuilderTab::SupportCards {
            self.draw_roster_preview(state);
            return;
        }

        if let Some(grant) = self.hovered_booster_result(state, mouse) {
            self.draw_collection_preview(state, grant);
            return;
        }

        if let Some(loadout_index) = self.selected_template_index {
            if let Some(starter) = state.content.starter_loadouts.get(loadout_index) {
                self.draw_template_preview(state, starter);
                return;
            }
        }

        if let Some(deck) = state.saves.decks.selected_support_deck() {
            self.draw_deck_preview(
                state,
                &deck.name,
                &deck.story_cards,
                &format!(
                    "{} {}/{}",
                    state.ui_text.get("deck_builder_card_total_label"),
                    deck.story_cards.len(),
                    state.content.deck_rules.support_deck_size
                ),
            );
            return;
        }

        self.draw_empty_preview(state);
    }

    pub(super) fn draw_contents_panel(&self, state: &AppState) {
        match self.active_tab {
            DeckBuilderTab::SupportCards => self.draw_support_deck_contents(state),
            DeckBuilderTab::MagicalGirls => self.draw_roster_contents(state, true),
            DeckBuilderTab::Baddies => self.draw_roster_contents(state, false),
        }
    }

    pub(super) fn draw_support_deck_contents(&self, state: &AppState) {
        let Some(deck) = state.saves.decks.selected_support_deck() else {
            return;
        };
        let ui = UiLayout::current();
        let rect = contents_panel_content_rect();
        let mut counts = std::collections::BTreeMap::<String, usize>::new();
        for card_id in &deck.story_cards {
            *counts.entry(card_id.clone()).or_insert(0) += 1;
        }

        draw_text(
            state.ui_text.get("deck_builder_contents_help"),
            rect.x,
            rect.y,
            ui.font(18.0),
            TEXT_MUTED,
        );
        if !deck.recent_story_cards.is_empty() {
            draw_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_recent_cards_label"),
                    deck.recent_story_cards
                        .iter()
                        .map(|card_id| self.story_card_name(state, card_id))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                rect.x,
                rect.y + ui.h(24.0),
                ui.font(16.0),
                SKYBLUE,
            );
        }
        if !deck.notes.is_empty() {
            draw_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("deck_builder_notes_label"),
                    wrap_preview_text(&deck.notes, rect.w - ui.w(20.0), ui.font(15.0), 1).join(" ")
                ),
                rect.x,
                rect.y + ui.h(46.0),
                ui.font(15.0),
                TEXT_MUTED,
            );
        }

        let mut y = rect.y + ui.h(72.0);
        for (card_id, count) in counts {
            let missing_count = count.saturating_sub(
                state
                    .saves
                    .collection
                    .owned_count(CollectionCardKind::StoryCard, &card_id) as usize,
            );
            let name = state
                .content
                .story_cards
                .iter()
                .find(|entry| entry.id == card_id)
                .map(|entry| entry.name.as_str())
                .unwrap_or(card_id.as_str());
            draw_text(
                &format!(
                    "{count}x {name}{}",
                    if missing_count > 0 {
                        format!(
                            "  {} x{}",
                            state.ui_text.get("deck_builder_missing_short"),
                            missing_count
                        )
                    } else {
                        String::new()
                    }
                ),
                rect.x,
                y,
                ui.font(18.0),
                if missing_count > 0 { GOLD } else { WHITE },
            );
            y += ui.h(22.0);
            if y > rect.y + rect.h - ui.h(20.0) {
                break;
            }
        }

        let suggestions = self.replacement_suggestion_lines(state);
        if !suggestions.is_empty() {
            let base_y = rect.y + rect.h - ui.h(80.0);
            draw_text(
                state.ui_text.get("deck_builder_replacements_label"),
                rect.x,
                base_y,
                ui.font(18.0),
                GOLD,
            );
            let mut suggestion_y = base_y + ui.h(20.0);
            for line in suggestions.into_iter().take(2) {
                draw_text(&line, rect.x, suggestion_y, ui.font(15.0), TEXT_MUTED);
                suggestion_y += ui.h(18.0);
            }
        }
    }

    pub(super) fn draw_roster_contents(&self, state: &AppState, is_magical_girl_side: bool) {
        let Some(active_deck) = state.saves.decks.selected_support_deck() else {
            return;
        };
        let ui = UiLayout::current();
        let rect = contents_panel_content_rect();
        let roster = if is_magical_girl_side {
            &active_deck.magical_girl_roster
        } else {
            &active_deck.baddie_roster
        };
        let definitions = if is_magical_girl_side {
            &state.content.magical_girls
        } else {
            &state.content.baddies
        };
        let selected_slot = if is_magical_girl_side {
            self.selected_magical_girl_slot
        } else {
            self.selected_baddie_slot
        };

        draw_text(
            state.ui_text.get("deck_builder_roster_panel_help"),
            rect.x,
            rect.y,
            ui.font(18.0),
            TEXT_MUTED,
        );

        for (slot_index, character_id) in roster.iter().enumerate() {
            let slot_rect = roster_contents_slot_rect(slot_index);
            let name = definitions
                .iter()
                .find(|entry| entry.id == *character_id)
                .map(|entry| entry.name.as_str())
                .unwrap_or(character_id.as_str());
            draw_soft_panel(
                slot_rect.x,
                slot_rect.y,
                slot_rect.w,
                slot_rect.h,
                if selected_slot == Some(slot_index) {
                    GOLD
                } else {
                    DARKGRAY
                },
            );
            let owned = state.saves.collection.owned_count(
                if is_magical_girl_side {
                    CollectionCardKind::MagicalGirl
                } else {
                    CollectionCardKind::Baddie
                },
                character_id,
            );
            draw_text(
                &format!(
                    "{} {}{}",
                    slot_index + 1,
                    name,
                    if owned == 0 {
                        format!(" ({})", state.ui_text.get("deck_builder_missing_short"))
                    } else {
                        String::new()
                    }
                ),
                slot_rect.x + ui.w(12.0),
                slot_rect.y + ui.h(28.0),
                ui.font(18.0),
                if owned == 0 { GOLD } else { WHITE },
            );
        }
    }

    pub(super) fn validation_issue_lines(
        &self,
        state: &AppState,
        validation: &DeckValidation,
    ) -> Vec<String> {
        let mut lines = Vec::new();

        if !validation.support_card_count_valid {
            lines.push(format!(
                "{} {}",
                state.ui_text.get("deck_builder_support_count_warning"),
                validation
                    .required_support_card_count
                    .saturating_sub(validation.support_card_count)
            ));
        }

        if !validation.duplicate_story_cards.is_empty() {
            lines.push(format!(
                "{}: {}",
                state.ui_text.get("deck_builder_duplicate_story_warning"),
                self.story_card_names(state, &validation.duplicate_story_cards)
            ));
        }

        if !validation.duplicate_magical_girls.is_empty() {
            lines.push(format!(
                "{}: {}",
                state
                    .ui_text
                    .get("deck_builder_duplicate_magical_girl_warning"),
                self.magical_girl_names(state, &validation.duplicate_magical_girls)
            ));
        }

        if !validation.duplicate_baddies.is_empty() {
            lines.push(format!(
                "{}: {}",
                state.ui_text.get("deck_builder_duplicate_baddie_warning"),
                self.baddie_names(state, &validation.duplicate_baddies)
            ));
        }

        if !validation.missing_story_cards.is_empty() {
            lines.push(format!(
                "{}: {}",
                state.ui_text.get("deck_builder_missing_story_warning"),
                self.story_card_missing_names(state, &validation.missing_story_cards)
            ));
        }

        if !validation.missing_magical_girls.is_empty() {
            lines.push(format!(
                "{}: {}",
                state
                    .ui_text
                    .get("deck_builder_missing_magical_girl_warning"),
                self.magical_girl_names(state, &validation.missing_magical_girls)
            ));
        }

        if !validation.missing_baddies.is_empty() {
            lines.push(format!(
                "{}: {}",
                state.ui_text.get("deck_builder_missing_baddie_warning"),
                self.baddie_names(state, &validation.missing_baddies)
            ));
        }

        lines
    }

    pub(super) fn story_card_missing_names(
        &self,
        state: &AppState,
        missing_entries: &[DeckValidationCount],
    ) -> String {
        missing_entries
            .iter()
            .map(|entry| {
                format!(
                    "{} x{}",
                    self.story_card_name(state, &entry.id),
                    entry.count
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(super) fn story_card_names(&self, state: &AppState, ids: &[String]) -> String {
        ids.iter()
            .map(|id| self.story_card_name(state, id))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(super) fn magical_girl_names(&self, state: &AppState, ids: &[String]) -> String {
        ids.iter()
            .map(|id| {
                state
                    .content
                    .magical_girls
                    .iter()
                    .find(|entry| entry.id == *id)
                    .map(|entry| entry.name.clone())
                    .unwrap_or_else(|| id.clone())
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(super) fn baddie_names(&self, state: &AppState, ids: &[String]) -> String {
        ids.iter()
            .map(|id| {
                state
                    .content
                    .baddies
                    .iter()
                    .find(|entry| entry.id == *id)
                    .map(|entry| entry.name.clone())
                    .unwrap_or_else(|| id.clone())
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(super) fn story_card_name(&self, state: &AppState, id: &str) -> String {
        state
            .content
            .story_cards
            .iter()
            .find(|entry| entry.id == id)
            .map(|entry| entry.name.clone())
            .unwrap_or_else(|| id.to_owned())
    }

    pub(super) fn replacement_suggestion_lines(&self, state: &AppState) -> Vec<String> {
        let Some(deck) = state.saves.decks.selected_support_deck() else {
            return Vec::new();
        };

        crate::state::suggest_story_replacements(deck, &state.content, &state.saves.collection, 2)
            .into_iter()
            .map(|suggestion| self.replacement_line(state, &suggestion))
            .collect()
    }

    pub(super) fn replacement_line(
        &self,
        state: &AppState,
        suggestion: &DeckReplacementSuggestion,
    ) -> String {
        format!(
            "{} -> {}",
            self.story_card_name(state, &suggestion.missing_card_id),
            suggestion
                .replacement_card_ids
                .iter()
                .map(|card_id| self.story_card_name(state, card_id))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    pub(super) fn hovered_booster_result<'a>(
        &self,
        state: &'a AppState,
        mouse: (f32, f32),
    ) -> Option<&'a BoosterCardGrant> {
        let ui = UiLayout::current();
        let mut booster_y = ui.y(992.0);
        for grant in state.last_opened_booster.iter().take(10) {
            let row_rect = Rect::new(ui.x(100.0), booster_y - ui.h(30.0), ui.w(360.0), ui.h(40.0));
            if point_in_rect(row_rect, mouse) {
                return Some(grant);
            }
            booster_y += ui.h(28.0);
        }
        None
    }

    pub(super) fn draw_collection_preview(&self, state: &AppState, grant: &BoosterCardGrant) {
        let rect = preview_panel_content_rect();

        match grant.kind {
            CollectionCardKind::StoryCard => {
                if let Some(card) = state
                    .content
                    .story_cards
                    .iter()
                    .find(|card| card.id == grant.id)
                {
                    let footer = vec![format!(
                        "{}: {}",
                        state.ui_text.get("deck_builder_owned_label"),
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::StoryCard, &grant.id)
                    )];
                    draw_story_card_preview(state, rect, card, &footer);
                }
            }
            CollectionCardKind::MagicalGirl => {
                if let Some(character) = state
                    .content
                    .magical_girls
                    .iter()
                    .find(|entry| entry.id == grant.id)
                {
                    self.draw_character_preview(
                        state,
                        rect,
                        state.ui_text.get("deck_builder_kind_magical_girl"),
                        character,
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::MagicalGirl, &grant.id),
                    );
                }
            }
            CollectionCardKind::Baddie => {
                if let Some(character) = state
                    .content
                    .baddies
                    .iter()
                    .find(|entry| entry.id == grant.id)
                {
                    self.draw_character_preview(
                        state,
                        rect,
                        state.ui_text.get("deck_builder_kind_baddie"),
                        character,
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::Baddie, &grant.id),
                    );
                }
            }
        }
    }

    pub(super) fn draw_character_preview(
        &self,
        state: &AppState,
        rect: Rect,
        kind_label: &str,
        character: &CharacterDefinition,
        owned: u32,
    ) {
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, SKYBLUE);
        if let Some(texture) = state.assets.portrait(&character.id) {
            draw_texture_ex(
                texture,
                rect.x + 18.0,
                rect.y + 54.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(112.0, 112.0)),
                    ..Default::default()
                },
            );
        }
        draw_text(kind_label, rect.x + 20.0, rect.y + 34.0, 24.0, GOLD);
        draw_text(&character.name, rect.x + 148.0, rect.y + 84.0, 36.0, WHITE);
        draw_text(
            &format!(
                "Power {} / {} / {}",
                character.base_power, character.transformed_power, character.final_power
            ),
            rect.x + 148.0,
            rect.y + 132.0,
            24.0,
            TEXT_MUTED,
        );
        draw_text(
            &format!(
                "Thresholds {} / {}",
                character.first_threshold, character.second_threshold
            ),
            rect.x + 148.0,
            rect.y + 168.0,
            24.0,
            TEXT_MUTED,
        );
        draw_text(
            &format!("Owned: {owned}"),
            rect.x + 148.0,
            rect.y + 220.0,
            24.0,
            WHITE,
        );
    }

    pub(super) fn draw_deck_preview(
        &self,
        state: &AppState,
        title: &str,
        story_cards: &[String],
        subtitle: &str,
    ) {
        let rect = preview_panel_content_rect();
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, GOLD);
        draw_text(title, rect.x + 18.0, rect.y + 42.0, 28.0, WHITE);
        draw_text(subtitle, rect.x + 18.0, rect.y + 74.0, 20.0, TEXT_MUTED);

        let mut counts = std::collections::BTreeMap::<String, usize>::new();
        for card_id in story_cards {
            *counts.entry(card_id.clone()).or_insert(0) += 1;
        }

        let mut y = rect.y + 120.0;
        for (card_id, count) in counts {
            let name = state
                .content
                .story_cards
                .iter()
                .find(|entry| entry.id == card_id)
                .map(|entry| entry.name.as_str())
                .unwrap_or(card_id.as_str());
            draw_text(&format!("{count}x {name}"), rect.x + 18.0, y, 20.0, WHITE);
            y += 24.0;
            if y > rect.y + rect.h - 24.0 {
                break;
            }
        }
    }

    pub(super) fn draw_template_preview(
        &self,
        state: &AppState,
        starter: &crate::data::StarterLoadout,
    ) {
        let ui = UiLayout::current();
        let rect = preview_panel_content_rect();
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, GOLD);
        draw_text(
            &starter.name,
            rect.x + ui.w(18.0),
            rect.y + ui.h(34.0),
            ui.font(26.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("deck_builder_previewing_template"),
            rect.x + ui.w(18.0),
            rect.y + ui.h(60.0),
            ui.font(18.0),
            TEXT_MUTED,
        );
        draw_text(
            &format!(
                "{}: {}",
                state.ui_text.get("deck_builder_template_playstyle_label"),
                starter.playstyle
            ),
            rect.x + ui.w(18.0),
            rect.y + ui.h(86.0),
            ui.font(18.0),
            WHITE,
        );

        let description_lines =
            wrap_preview_text(&starter.description, rect.w - ui.w(36.0), ui.font(16.0), 3);
        let mut y = rect.y + ui.h(114.0);
        draw_text(
            state.ui_text.get("deck_builder_template_description_label"),
            rect.x + ui.w(18.0),
            y,
            ui.font(18.0),
            WHITE,
        );
        y += ui.h(18.0);
        for line in description_lines {
            y += ui.h(18.0);
            draw_text(&line, rect.x + ui.w(18.0), y, ui.font(16.0), TEXT_MUTED);
        }

        y += ui.h(24.0);
        draw_text(
            state.ui_text.get("deck_builder_template_roster_seed_label"),
            rect.x + ui.w(18.0),
            y,
            ui.font(18.0),
            WHITE,
        );
        y += ui.h(20.0);
        for line in self.template_roster_lines(state, starter) {
            draw_text(&line, rect.x + ui.w(18.0), y, ui.font(16.0), TEXT_MUTED);
            y += ui.h(18.0);
        }

        y += ui.h(8.0);
        draw_text(
            &format!(
                "{} ({}/{})",
                state
                    .ui_text
                    .get("deck_builder_template_support_seed_label"),
                starter.support_deck.len(),
                state.content.deck_rules.support_deck_size
            ),
            rect.x + ui.w(18.0),
            y,
            ui.font(18.0),
            WHITE,
        );
        y += ui.h(20.0);
        for line in self.template_support_seed_lines(state, starter) {
            draw_text(&line, rect.x + ui.w(18.0), y, ui.font(15.0), TEXT_MUTED);
            y += ui.h(17.0);
            if y > rect.y + rect.h - ui.h(12.0) {
                break;
            }
        }
    }

    pub(super) fn draw_empty_preview(&self, state: &AppState) {
        let ui = UiLayout::current();
        let rect = preview_panel_content_rect();
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
        draw_text(
            state.ui_text.get("deck_builder_empty_preview_title"),
            rect.x + ui.w(18.0),
            rect.y + ui.h(42.0),
            ui.font(28.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("deck_builder_empty_preview_body"),
            rect.x + ui.w(18.0),
            rect.y + ui.h(92.0),
            ui.font(20.0),
            TEXT_MUTED,
        );
    }
}
