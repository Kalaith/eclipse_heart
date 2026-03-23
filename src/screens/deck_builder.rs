//! Support deck builder shell.

use macroquad::prelude::*;

use crate::data::CharacterDefinition;
use crate::screens::ScreenAction;
use crate::state::{AppState, CollectionCardKind};
use crate::ui::card_widgets::{
    action_button, draw_story_card_preview, draw_story_card_tile, point_in_rect, section_panel,
};
use crate::ui::core::{draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

pub struct DeckBuilderScreen {
    selected_starter_index: Option<usize>,
    active_layer: DeckBuilderLayer,
    selected_magical_girl_slot: Option<usize>,
    selected_baddie_slot: Option<usize>,
}

impl DeckBuilderScreen {
    pub fn new() -> Self {
        Self {
            selected_starter_index: None,
            active_layer: DeckBuilderLayer::SupportCards,
            selected_magical_girl_slot: None,
            selected_baddie_slot: None,
        }
    }

    pub fn update(&mut self, state: &AppState) -> ScreenAction {
        let ui = UiLayout::current();
        let mouse = mouse_position();

        if action_button(
            ui.rect(80.0, 1328.0, 360.0, 70.0),
            state.ui_text.get("battle_back_to_menu"),
        ) {
            return ScreenAction::BackToMenu;
        }

        if action_button(
            ui.rect(80.0, 1242.0, 360.0, 70.0),
            state.ui_text.get("deck_builder_open_booster"),
        ) {
            return ScreenAction::DeckBuilderOpenBooster;
        }

        if point_in_rect(deck_builder_tab_rect(DeckBuilderLayer::SupportCards), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.active_layer = DeckBuilderLayer::SupportCards;
        }
        if point_in_rect(deck_builder_tab_rect(DeckBuilderLayer::Roster), mouse)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.active_layer = DeckBuilderLayer::Roster;
        }

        for (loadout_index, _starter) in state.content.starter_loadouts.iter().enumerate() {
            let row_rect = starter_row_rect(loadout_index);
            let load_rect = starter_edit_rect(loadout_index);
            if point_in_rect(row_rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_starter_index = Some(loadout_index);
            }
            if point_in_rect(load_rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                return ScreenAction::DeckBuilderLoadStarter { loadout_index };
            }
        }

        if self.active_layer == DeckBuilderLayer::Roster {
            return self.update_roster_layer(state, mouse);
        }

        for (index, card) in state.content.story_cards.iter().enumerate() {
            let row = index / 4;
            let column = index % 4;
            let tile_x = ui.x(560.0 + column as f32 * 350.0);
            let tile_y = ui.y(232.0 + row as f32 * 134.0);
            let rect = Rect::new(tile_x, tile_y, ui.w(328.0), ui.h(116.0));
            let can_add = state.saves.decks.can_add_card(
                &card.id,
                &state.content.deck_rules,
                &state.saves.collection,
            );
            let can_remove = state.saves.decks.card_count(&card.id) > 0;

            let add_rect = Rect::new(
                rect.x + rect.w - ui.w(112.0),
                rect.y + ui.h(12.0),
                ui.w(92.0),
                ui.h(36.0),
            );
            let remove_rect = Rect::new(
                rect.x + rect.w - ui.w(112.0),
                rect.y + ui.h(60.0),
                ui.w(92.0),
                ui.h(36.0),
            );

            if point_in_rect(add_rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                if can_add {
                    return ScreenAction::DeckBuilderAddCard {
                        card_id: card.id.clone(),
                    };
                }
            }

            if point_in_rect(remove_rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                if can_remove {
                    return ScreenAction::DeckBuilderRemoveCard {
                        card_id: card.id.clone(),
                    };
                }
            }
        }

        ScreenAction::None
    }

    pub fn draw(&self, state: &AppState) {
        let ui = UiLayout::current();
        let active_deck = state.saves.decks.active_support_deck();
        let deck_name = active_deck
            .map(|deck| deck.name.as_str())
            .unwrap_or(state.ui_text.get("deck_builder_missing_deck"));
        let deck_size = active_deck.map(|deck| deck.story_cards.len()).unwrap_or(0);

        draw_text(
            state.ui_text.get("deck_builder_title"),
            ui.x(80.0),
            ui.y(96.0),
            ui.font(68.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("deck_builder_subtitle"),
            ui.x(80.0),
            ui.y(148.0),
            ui.font(30.0),
            TEXT_MUTED,
        );

        section_panel(
            ui.rect(80.0, 178.0, 400.0, 730.0),
            state.ui_text.get("deck_builder_starters_label"),
            SKYBLUE,
        );
        section_panel(
            ui.rect(80.0, 930.0, 400.0, 358.0),
            state.ui_text.get("deck_builder_booster_results_label"),
            PINK,
        );
        section_panel(
            ui.rect(540.0, 96.0, 1520.0, 90.0),
            state.ui_text.get("deck_builder_active_deck_label"),
            GOLD,
        );
        section_panel(
            ui.rect(2090.0, 96.0, 390.0, 1192.0),
            state.ui_text.get("deck_builder_preview_label"),
            PINK,
        );
        self.draw_layer_tabs(state);

        draw_text(deck_name, ui.x(570.0), ui.y(154.0), ui.font(34.0), WHITE);
        draw_text(
            &format!(
                "{}: {}/{}",
                state.ui_text.get("deck_builder_card_total_label"),
                deck_size,
                state.content.deck_rules.support_deck_size
            ),
            ui.x(1500.0),
            ui.y(154.0),
            ui.font(28.0),
            TEXT_MUTED,
        );

        let mouse = mouse_position();
        for (loadout_index, starter) in state.content.starter_loadouts.iter().enumerate() {
            let row_rect = starter_row_rect(loadout_index);
            let edit_rect = starter_edit_rect(loadout_index);
            let row_hovered = point_in_rect(row_rect, mouse);
            let saved_preset = state.saves.decks.preset_for_starter(&starter.id);
            let shown_card_count = saved_preset
                .map(|deck| deck.story_cards.len())
                .unwrap_or(starter.support_deck.len());
            draw_soft_panel(
                row_rect.x,
                row_rect.y,
                row_rect.w,
                row_rect.h,
                if self.selected_starter_index == Some(loadout_index) {
                    SKYBLUE
                } else if row_hovered {
                    GRAY
                } else {
                    DARKGRAY
                },
            );
            draw_text(
                &starter.name,
                row_rect.x + ui.w(16.0),
                row_rect.y + ui.h(30.0),
                ui.font(22.0),
                WHITE,
            );
            draw_text(
                &format!(
                    "{} {}/{}",
                    state.ui_text.get("deck_builder_card_total_label"),
                    shown_card_count,
                    state.content.deck_rules.support_deck_size
                ),
                row_rect.x + ui.w(16.0),
                row_rect.y + ui.h(52.0),
                ui.font(16.0),
                TEXT_MUTED,
            );

            draw_soft_panel(
                edit_rect.x,
                edit_rect.y,
                edit_rect.w,
                edit_rect.h,
                if point_in_rect(edit_rect, mouse) {
                    GOLD
                } else {
                    PINK
                },
            );
            draw_text(
                state.ui_text.get("deck_builder_edit_starter"),
                edit_rect.x + ui.w(12.0),
                edit_rect.y + ui.h(28.0),
                ui.font(18.0),
                WHITE,
            );
        }

        let mut booster_y = ui.y(992.0);
        let mut hovered_booster = None;
        for grant in state.last_opened_booster.iter().take(10) {
            let row_rect = Rect::new(ui.x(100.0), booster_y - ui.h(30.0), ui.w(360.0), ui.h(40.0));
            let row_hovered = point_in_rect(row_rect, mouse);
            if row_hovered {
                hovered_booster = Some(grant);
            }
            draw_soft_panel(
                row_rect.x,
                row_rect.y,
                row_rect.w,
                row_rect.h,
                if row_hovered { PINK } else { DARKPURPLE },
            );
            draw_text(
                &format!(
                    "{}: {}",
                    collection_kind_label(state, grant.kind),
                    grant.name
                ),
                ui.x(116.0),
                booster_y,
                ui.font(18.0),
                WHITE,
            );
            booster_y += ui.h(28.0);
        }

        let mut hovered_card = None;
        if self.active_layer == DeckBuilderLayer::SupportCards {
            for (index, card) in state.content.story_cards.iter().enumerate() {
                let row = index / 4;
                let column = index % 4;
                let base_x = ui.x(560.0 + column as f32 * 350.0);
                let base_y = ui.y(232.0 + row as f32 * 134.0);
                let copies = state.saves.decks.card_count(&card.id);
                let owned = state
                    .saves
                    .collection
                    .owned_count(CollectionCardKind::StoryCard, &card.id);
                let available = state
                    .saves
                    .collection
                    .story_cards_available_for_deck(&card.id, copies);
                let rect = Rect::new(base_x, base_y, ui.w(328.0), ui.h(116.0));
                let hovered = point_in_rect(rect, mouse);
                if hovered {
                    hovered_card = Some((card, copies, owned, available));
                }

                draw_story_card_tile(
                    rect,
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

                draw_soft_panel(
                    rect.x + rect.w - ui.w(112.0),
                    rect.y + ui.h(12.0),
                    ui.w(92.0),
                    ui.h(36.0),
                    if available > 0 { SKYBLUE } else { DARKGRAY },
                );
                draw_text(
                    if available > 0 {
                        state.ui_text.get("deck_builder_add_card")
                    } else {
                        state.ui_text.get("deck_builder_add_locked")
                    },
                    rect.x + rect.w - ui.w(96.0),
                    rect.y + ui.h(36.0),
                    ui.font(16.0),
                    WHITE,
                );

                draw_soft_panel(
                    rect.x + rect.w - ui.w(112.0),
                    rect.y + ui.h(60.0),
                    ui.w(92.0),
                    ui.h(36.0),
                    if copies > 0 { PINK } else { DARKGRAY },
                );
                draw_text(
                    if copies > 0 {
                        state.ui_text.get("deck_builder_remove_card")
                    } else {
                        state.ui_text.get("deck_builder_remove_locked")
                    },
                    rect.x + rect.w - ui.w(94.0),
                    rect.y + ui.h(84.0),
                    ui.font(16.0),
                    WHITE,
                );
            }
        }

        if self.active_layer == DeckBuilderLayer::Roster {
            self.draw_roster_layer(state);
            self.draw_roster_preview(state);
        } else if let Some((card, copies, owned, available)) = hovered_card {
            let preview_rect = ui.rect(2120.0, 136.0, 330.0, 1120.0);
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
            draw_story_card_preview(preview_rect, card, &footer);
        } else if let Some(grant) = hovered_booster {
            self.draw_collection_preview(state, grant);
        } else if let Some(loadout_index) = self.selected_starter_index {
            if let Some(starter) = state.content.starter_loadouts.get(loadout_index) {
                let shown_cards = state
                    .saves
                    .decks
                    .preset_for_starter(&starter.id)
                    .map(|deck| deck.story_cards.as_slice())
                    .unwrap_or(starter.support_deck.as_slice());
                let shown_count = shown_cards.len();
                self.draw_deck_preview(
                    state,
                    &starter.name,
                    shown_cards,
                    &format!(
                        "{} | {} {}/{}",
                        state.ui_text.get("deck_builder_previewing_starter"),
                        state.ui_text.get("deck_builder_card_total_label"),
                        shown_count,
                        state.content.deck_rules.support_deck_size
                    ),
                );
            }
        } else if let Some(deck) = active_deck {
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
        }
    }

    fn draw_collection_preview(&self, state: &AppState, grant: &crate::state::BoosterCardGrant) {
        let ui = UiLayout::current();
        let rect = ui.rect(2120.0, 136.0, 330.0, 1120.0);

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
                    draw_story_card_preview(rect, card, &footer);
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

    fn draw_character_preview(
        &self,
        rect: Rect,
        kind_label: &str,
        character: &CharacterDefinition,
        owned: u32,
    ) {
        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, SKYBLUE);
        draw_text(kind_label, rect.x + 20.0, rect.y + 34.0, 24.0, GOLD);
        draw_text(&character.name, rect.x + 20.0, rect.y + 84.0, 36.0, WHITE);
        draw_text(
            &format!(
                "Power {} / {} / {}",
                character.base_power, character.transformed_power, character.final_power
            ),
            rect.x + 20.0,
            rect.y + 132.0,
            24.0,
            TEXT_MUTED,
        );
        draw_text(
            &format!(
                "Thresholds {} / {}",
                character.first_threshold, character.second_threshold
            ),
            rect.x + 20.0,
            rect.y + 168.0,
            24.0,
            TEXT_MUTED,
        );
        draw_text(
            &format!("Owned: {owned}"),
            rect.x + 20.0,
            rect.y + 220.0,
            24.0,
            WHITE,
        );
    }

    fn draw_deck_preview(
        &self,
        state: &AppState,
        title: &str,
        story_cards: &[String],
        subtitle: &str,
    ) {
        let ui = UiLayout::current();
        let rect = ui.rect(2120.0, 136.0, 330.0, 1120.0);
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

    fn update_roster_layer(&mut self, state: &AppState, mouse: (f32, f32)) -> ScreenAction {
        let Some(active_deck) = state.saves.decks.active_support_deck() else {
            return ScreenAction::None;
        };

        for (slot_index, rect) in roster_slot_rects(true).into_iter().enumerate() {
            if point_in_rect(rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_magical_girl_slot = Some(slot_index);
            }
        }

        for (slot_index, rect) in roster_slot_rects(false).into_iter().enumerate() {
            if point_in_rect(rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_baddie_slot = Some(slot_index);
            }
        }

        for (index, character) in state.content.magical_girls.iter().enumerate() {
            let rect = roster_pool_rect(true, index);
            if point_in_rect(rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                if let Some(slot_index) = self.selected_magical_girl_slot {
                    return ScreenAction::DeckBuilderSetRosterSlot {
                        is_magical_girl_side: true,
                        slot_index,
                        character_id: character.id.clone(),
                    };
                }
            }
        }

        for (index, character) in state.content.baddies.iter().enumerate() {
            let rect = roster_pool_rect(false, index);
            if point_in_rect(rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                if let Some(slot_index) = self.selected_baddie_slot {
                    return ScreenAction::DeckBuilderSetRosterSlot {
                        is_magical_girl_side: false,
                        slot_index,
                        character_id: character.id.clone(),
                    };
                }
            }
        }

        if active_deck.magical_girl_roster.is_empty() || active_deck.baddie_roster.is_empty() {
            self.selected_magical_girl_slot = None;
            self.selected_baddie_slot = None;
        }

        ScreenAction::None
    }

    fn draw_layer_tabs(&self, state: &AppState) {
        let ui = UiLayout::current();
        for layer in [DeckBuilderLayer::SupportCards, DeckBuilderLayer::Roster] {
            let rect = deck_builder_tab_rect(layer);
            draw_soft_panel(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if self.active_layer == layer {
                    GOLD
                } else {
                    DARKGRAY
                },
            );
            draw_text(
                match layer {
                    DeckBuilderLayer::SupportCards => {
                        state.ui_text.get("deck_builder_tab_support_cards")
                    }
                    DeckBuilderLayer::Roster => state.ui_text.get("deck_builder_tab_roster"),
                },
                rect.x + ui.w(18.0),
                rect.y + ui.h(34.0),
                ui.font(22.0),
                WHITE,
            );
        }
    }

    fn draw_roster_layer(&self, state: &AppState) {
        let Some(active_deck) = state.saves.decks.active_support_deck() else {
            return;
        };
        let ui = UiLayout::current();
        draw_text(
            state.ui_text.get("deck_builder_roster_help"),
            ui.x(570.0),
            ui.y(226.0),
            ui.font(24.0),
            TEXT_MUTED,
        );

        self.draw_roster_column(
            state,
            true,
            &active_deck.magical_girl_roster,
            self.selected_magical_girl_slot,
        );
        self.draw_roster_column(
            state,
            false,
            &active_deck.baddie_roster,
            self.selected_baddie_slot,
        );
    }

    fn draw_roster_column(
        &self,
        state: &AppState,
        is_magical_girl_side: bool,
        roster: &[String],
        selected_slot: Option<usize>,
    ) {
        let ui = UiLayout::current();
        let title_x = if is_magical_girl_side {
            ui.x(570.0)
        } else {
            ui.x(1320.0)
        };
        draw_text(
            if is_magical_girl_side {
                state.ui_text.get("deck_builder_roster_magical_girls")
            } else {
                state.ui_text.get("deck_builder_roster_baddies")
            },
            title_x,
            ui.y(278.0),
            ui.font(28.0),
            WHITE,
        );

        let definitions = if is_magical_girl_side {
            &state.content.magical_girls
        } else {
            &state.content.baddies
        };

        for (slot_index, character_id) in roster.iter().enumerate() {
            let rect = roster_slot_rects(is_magical_girl_side)[slot_index];
            let name = definitions
                .iter()
                .find(|entry| entry.id == *character_id)
                .map(|entry| entry.name.as_str())
                .unwrap_or(character_id.as_str());
            draw_soft_panel(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if selected_slot == Some(slot_index) {
                    GOLD
                } else {
                    DARKGRAY
                },
            );
            draw_text(
                &format!("{} {}", slot_index + 1, name),
                rect.x + ui.w(16.0),
                rect.y + ui.h(30.0),
                ui.font(22.0),
                WHITE,
            );
        }

        for (index, character) in definitions.iter().enumerate() {
            let rect = roster_pool_rect(is_magical_girl_side, index);
            let is_in_roster = roster.iter().any(|entry| entry == &character.id);
            draw_soft_panel(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if is_in_roster { SKYBLUE } else { GRAY },
            );
            draw_text(
                &character.name,
                rect.x + ui.w(14.0),
                rect.y + ui.h(32.0),
                ui.font(20.0),
                WHITE,
            );
            draw_text(
                &format!(
                    "{} / {} / {}",
                    character.base_power, character.transformed_power, character.final_power
                ),
                rect.x + ui.w(14.0),
                rect.y + ui.h(58.0),
                ui.font(16.0),
                TEXT_MUTED,
            );
        }
    }

    fn draw_roster_preview(&self, state: &AppState) {
        let Some(active_deck) = state.saves.decks.active_support_deck() else {
            return;
        };
        let ui = UiLayout::current();
        let preview_rect = ui.rect(2120.0, 136.0, 330.0, 1120.0);

        if let Some(slot_index) = self.selected_magical_girl_slot {
            if let Some(character_id) = active_deck.magical_girl_roster.get(slot_index) {
                if let Some(character) = state
                    .content
                    .magical_girls
                    .iter()
                    .find(|entry| &entry.id == character_id)
                {
                    self.draw_character_preview(
                        preview_rect,
                        state.ui_text.get("deck_builder_kind_magical_girl"),
                        character,
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::MagicalGirl, character_id),
                    );
                    return;
                }
            }
        }

        if let Some(slot_index) = self.selected_baddie_slot {
            if let Some(character_id) = active_deck.baddie_roster.get(slot_index) {
                if let Some(character) = state
                    .content
                    .baddies
                    .iter()
                    .find(|entry| &entry.id == character_id)
                {
                    self.draw_character_preview(
                        preview_rect,
                        state.ui_text.get("deck_builder_kind_baddie"),
                        character,
                        state
                            .saves
                            .collection
                            .owned_count(CollectionCardKind::Baddie, character_id),
                    );
                }
            }
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum DeckBuilderLayer {
    SupportCards,
    Roster,
}

fn collection_kind_label<'a>(state: &'a AppState, kind: CollectionCardKind) -> &'a str {
    match kind {
        CollectionCardKind::MagicalGirl => state.ui_text.get("deck_builder_kind_magical_girl"),
        CollectionCardKind::Baddie => state.ui_text.get("deck_builder_kind_baddie"),
        CollectionCardKind::StoryCard => state.ui_text.get("deck_builder_kind_story"),
    }
}

fn starter_row_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(96.0),
        ui.y(216.0 + index as f32 * 72.0),
        ui.w(288.0),
        ui.h(56.0),
    )
}

fn starter_edit_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(396.0),
        ui.y(216.0 + index as f32 * 72.0),
        ui.w(72.0),
        ui.h(56.0),
    )
}

fn deck_builder_tab_rect(layer: DeckBuilderLayer) -> Rect {
    let ui = UiLayout::current();
    match layer {
        DeckBuilderLayer::SupportCards => {
            Rect::new(ui.x(560.0), ui.y(194.0), ui.w(250.0), ui.h(44.0))
        }
        DeckBuilderLayer::Roster => Rect::new(ui.x(826.0), ui.y(194.0), ui.w(210.0), ui.h(44.0)),
    }
}

fn roster_slot_rects(is_magical_girl_side: bool) -> Vec<Rect> {
    let ui = UiLayout::current();
    let start_x = if is_magical_girl_side {
        ui.x(570.0)
    } else {
        ui.x(1320.0)
    };
    (0..5)
        .map(|index| {
            Rect::new(
                start_x,
                ui.y(306.0 + index as f32 * 72.0),
                ui.w(300.0),
                ui.h(56.0),
            )
        })
        .collect()
}

fn roster_pool_rect(is_magical_girl_side: bool, index: usize) -> Rect {
    let ui = UiLayout::current();
    let start_x = if is_magical_girl_side {
        ui.x(900.0)
    } else {
        ui.x(1650.0)
    };
    Rect::new(
        start_x,
        ui.y(306.0 + index as f32 * 92.0),
        ui.w(300.0),
        ui.h(74.0),
    )
}
