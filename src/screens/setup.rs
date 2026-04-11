//! Match setup screen.

use macroquad::prelude::*;

use crate::data::CharacterDefinition;
use crate::screens::ScreenAction;
use crate::state::{AppState, PlayerId};
use crate::ui::card_widgets::{action_button, point_in_rect, section_panel};
use crate::ui::core::{draw_background_texture, draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

pub struct SetupScreen;

impl SetupScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, state: &AppState) -> ScreenAction {
        let ui = UiLayout::current();
        let mouse = mouse_position();

        for target in setup_panel_targets(state) {
            for main_target in target.main_targets {
                if point_in_rect(main_target.rect, mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    return ScreenAction::SetupSelectMain {
                        player: target.player,
                        is_magical_girl_side: target.is_magical_girl_side,
                        main_index: main_target.main_index,
                    };
                }
            }

            for pair_target in target.support_pair_targets {
                if point_in_rect(pair_target.rect, mouse)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    return ScreenAction::SetupSelectSupportPair {
                        player: target.player,
                        is_magical_girl_side: target.is_magical_girl_side,
                        pair_index: pair_target.pair_index,
                    };
                }
            }
        }

        if action_button(
            ui.rect(2080.0, 1328.0, 400.0, 70.0),
            state.ui_text.get("setup_start_match"),
        ) {
            return ScreenAction::StartConfiguredBattle;
        }

        if action_button(
            ui.rect(80.0, 1328.0, 360.0, 70.0),
            state.ui_text.get("battle_back_to_menu"),
        ) {
            return ScreenAction::BackToMenu;
        }

        if action_button(
            ui.rect(700.0, 1328.0, 360.0, 70.0),
            state.ui_text.get("setup_use_selected_deck_player_a"),
        ) {
            return ScreenAction::SetupUseSelectedDeck {
                player: PlayerId::PlayerA,
            };
        }

        if action_button(
            ui.rect(1080.0, 1328.0, 280.0, 70.0),
            state.ui_text.get("setup_clear_player_a_deck"),
        ) {
            return ScreenAction::SetupClearAssignedDeck {
                player: PlayerId::PlayerA,
            };
        }

        if action_button(
            ui.rect(1380.0, 1328.0, 360.0, 70.0),
            state.ui_text.get("setup_use_selected_deck_player_b"),
        ) {
            return ScreenAction::SetupUseSelectedDeck {
                player: PlayerId::PlayerB,
            };
        }

        if action_button(
            ui.rect(1760.0, 1328.0, 280.0, 70.0),
            state.ui_text.get("setup_clear_player_b_deck"),
        ) {
            return ScreenAction::SetupClearAssignedDeck {
                player: PlayerId::PlayerB,
            };
        }

        ScreenAction::None
    }

    pub fn draw(&self, state: &AppState) {
        let ui = UiLayout::current();
        if let Some(background) = state.assets.ui_background("campaign") {
            draw_background_texture(background, Color::new(1.0, 1.0, 1.0, 0.82));
        }

        draw_text(
            state.ui_text.get("setup_title"),
            ui.x(80.0),
            ui.y(96.0),
            ui.font(68.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("setup_subtitle"),
            ui.x(80.0),
            ui.y(150.0),
            ui.font(30.0),
            TEXT_MUTED,
        );

        for target in setup_panel_targets(state) {
            self.draw_panel(state, &target);
        }

        draw_text(
            state.ui_text.get("setup_hidden_support_note"),
            ui.x(80.0),
            ui.y(1252.0),
            ui.font(20.0),
            GOLD,
        );
        self.draw_assigned_deck_summary(
            state,
            PlayerId::PlayerA,
            ui.rect(700.0, 1242.0, 660.0, 72.0),
        );
        self.draw_assigned_deck_summary(
            state,
            PlayerId::PlayerB,
            ui.rect(1380.0, 1242.0, 660.0, 72.0),
        );
    }

    fn draw_panel(&self, state: &AppState, target: &SetupPanelTarget<'_>) {
        let ui = UiLayout::current();
        section_panel(target.panel_rect, target.label, GRAY);

        draw_text(
            state.ui_text.get("setup_main_options_label"),
            target.panel_rect.x + ui.w(20.0),
            target.panel_rect.y + ui.h(74.0),
            ui.font(22.0),
            GOLD,
        );

        let mouse = mouse_position();
        for main_target in &target.main_targets {
            let hovered = point_in_rect(main_target.rect, mouse);
            self.draw_character_tile(
                state,
                main_target.rect,
                main_target.character,
                main_target.selected,
                hovered,
                target.is_magical_girl_side,
            );
        }

        draw_text(
            state.ui_text.get("setup_support_pair_options_label"),
            target.panel_rect.x + ui.w(20.0),
            target.panel_rect.y + ui.h(246.0),
            ui.font(22.0),
            GOLD,
        );

        for pair_target in &target.support_pair_targets {
            let hovered = point_in_rect(pair_target.rect, mouse);
            self.draw_support_pair_tile(state, pair_target.rect, pair_target, hovered);
        }
    }

    fn draw_character_tile(
        &self,
        state: &AppState,
        rect: Rect,
        character: &CharacterDefinition,
        selected: bool,
        hovered: bool,
        is_magical_girl_side: bool,
    ) {
        let ui = UiLayout::current();
        let outline = if selected {
            GOLD
        } else if hovered {
            WHITE
        } else if is_magical_girl_side {
            SKYBLUE
        } else {
            PINK
        };

        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
        if let Some(texture) = state.assets.portrait(&character.id) {
            draw_texture_ex(
                texture,
                rect.x + ui.w(6.0),
                rect.y + ui.h(6.0),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(ui.w(76.0), rect.h - ui.h(12.0))),
                    ..Default::default()
                },
            );
        }
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, outline);
        draw_text(
            &character.name,
            rect.x + ui.w(92.0),
            rect.y + ui.h(32.0),
            ui.font(22.0),
            WHITE,
        );
        draw_text(
            &format!(
                "{} / {} / {}",
                character.base_power, character.transformed_power, character.final_power
            ),
            rect.x + ui.w(92.0),
            rect.y + ui.h(66.0),
            ui.font(18.0),
            TEXT_MUTED,
        );
        draw_text(
            &format!(
                "{} {} / {}",
                state.ui_text.get("battle_growth_label"),
                character.first_threshold,
                character.second_threshold
            ),
            rect.x + ui.w(92.0),
            rect.y + ui.h(96.0),
            ui.font(18.0),
            TEXT_MUTED,
        );
    }

    fn draw_support_pair_tile(
        &self,
        state: &AppState,
        rect: Rect,
        target: &SupportPairTarget,
        hovered: bool,
    ) {
        let ui = UiLayout::current();
        let outline = if target.selected {
            GOLD
        } else if hovered {
            WHITE
        } else {
            GRAY
        };

        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, outline);
        draw_text(
            &target.names.join(" + "),
            rect.x + ui.w(14.0),
            rect.y + ui.h(30.0),
            ui.font(20.0),
            WHITE,
        );
        draw_text(
            state.ui_text.get("setup_hidden_supports_label"),
            rect.x + ui.w(14.0),
            rect.y + ui.h(56.0),
            ui.font(16.0),
            TEXT_MUTED,
        );
    }

    fn draw_assigned_deck_summary(&self, state: &AppState, player: PlayerId, rect: Rect) {
        let ui = UiLayout::current();
        let assigned_deck_id = match player {
            PlayerId::PlayerA => state.setup.player_a_support_deck_id.as_deref(),
            PlayerId::PlayerB => state.setup.player_b_support_deck_id.as_deref(),
        };
        let selected_deck_name = state
            .saves
            .decks
            .selected_support_deck()
            .map(|deck| deck.name.as_str())
            .unwrap_or(state.ui_text.get("setup_no_saved_deck_selected"));
        let assigned_deck_name = assigned_deck_id
            .and_then(|deck_id| {
                state
                    .saves
                    .decks
                    .support_decks
                    .iter()
                    .find(|deck| deck.id == deck_id)
                    .map(|deck| deck.name.as_str())
            })
            .unwrap_or(state.ui_text.get("setup_default_support_deck"));

        draw_soft_panel(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
        draw_text(
            match player {
                PlayerId::PlayerA => state.ui_text.get("setup_player_a_assigned_deck"),
                PlayerId::PlayerB => state.ui_text.get("setup_player_b_assigned_deck"),
            },
            rect.x + ui.w(14.0),
            rect.y + ui.h(28.0),
            ui.font(18.0),
            WHITE,
        );
        draw_text(
            assigned_deck_name,
            rect.x + ui.w(14.0),
            rect.y + ui.h(54.0),
            ui.font(18.0),
            GOLD,
        );
        draw_text(
            &format!(
                "{}: {}",
                state.ui_text.get("setup_current_selected_deck"),
                selected_deck_name
            ),
            rect.x + ui.w(300.0),
            rect.y + ui.h(54.0),
            ui.font(16.0),
            TEXT_MUTED,
        );
    }
}

struct SetupPanelTarget<'a> {
    player: PlayerId,
    is_magical_girl_side: bool,
    label: &'a str,
    panel_rect: Rect,
    main_targets: Vec<MainTarget<'a>>,
    support_pair_targets: Vec<SupportPairTarget>,
}

struct MainTarget<'a> {
    rect: Rect,
    main_index: usize,
    character: &'a CharacterDefinition,
    selected: bool,
}

struct SupportPairTarget {
    rect: Rect,
    pair_index: usize,
    names: Vec<String>,
    selected: bool,
}

fn setup_panel_targets<'a>(state: &'a AppState) -> Vec<SetupPanelTarget<'a>> {
    let ui = UiLayout::current();
    let setup = &state.setup;

    vec![
        build_panel_target(
            state,
            PlayerId::PlayerA,
            true,
            state.ui_text.get("setup_player_a_mg_side"),
            ui.rect(80.0, 220.0, 1120.0, 500.0),
            &state.content.magical_girls,
            setup.player_a_mg_main_index,
            setup.player_a_mg_support_pair_index,
        ),
        build_panel_target(
            state,
            PlayerId::PlayerB,
            true,
            state.ui_text.get("setup_player_b_mg_side"),
            ui.rect(1360.0, 220.0, 1120.0, 500.0),
            &state.content.magical_girls,
            setup.player_b_mg_main_index,
            setup.player_b_mg_support_pair_index,
        ),
        build_panel_target(
            state,
            PlayerId::PlayerA,
            false,
            state.ui_text.get("setup_player_a_baddie_side"),
            ui.rect(80.0, 748.0, 1120.0, 500.0),
            &state.content.baddies,
            setup.player_a_baddie_main_index,
            setup.player_a_baddie_support_pair_index,
        ),
        build_panel_target(
            state,
            PlayerId::PlayerB,
            false,
            state.ui_text.get("setup_player_b_baddie_side"),
            ui.rect(1360.0, 748.0, 1120.0, 500.0),
            &state.content.baddies,
            setup.player_b_baddie_main_index,
            setup.player_b_baddie_support_pair_index,
        ),
    ]
}

fn build_panel_target<'a>(
    _state: &'a AppState,
    player: PlayerId,
    is_magical_girl_side: bool,
    label: &'a str,
    panel_rect: Rect,
    definitions: &'a [CharacterDefinition],
    selected_main_index: usize,
    selected_pair_index: usize,
) -> SetupPanelTarget<'a> {
    let ui = UiLayout::current();
    let main_targets = definitions
        .iter()
        .enumerate()
        .map(|(main_index, character)| MainTarget {
            rect: Rect::new(
                panel_rect.x + ui.w(18.0) + main_index as f32 * ui.w(214.0),
                panel_rect.y + ui.h(92.0),
                ui.w(196.0),
                ui.h(118.0),
            ),
            main_index,
            character,
            selected: selected_main_index == main_index,
        })
        .collect::<Vec<_>>();

    let support_pair_targets = support_pair_name_options(definitions, selected_main_index)
        .into_iter()
        .enumerate()
        .map(|(pair_index, names)| {
            let row = pair_index / 3;
            let column = pair_index % 3;
            SupportPairTarget {
                rect: Rect::new(
                    panel_rect.x + ui.w(18.0) + column as f32 * ui.w(356.0),
                    panel_rect.y + ui.h(264.0) + row as f32 * ui.h(92.0),
                    ui.w(338.0),
                    ui.h(74.0),
                ),
                pair_index,
                names,
                selected: selected_pair_index == pair_index,
            }
        })
        .collect::<Vec<_>>();

    SetupPanelTarget {
        player,
        is_magical_girl_side,
        label,
        panel_rect,
        main_targets,
        support_pair_targets,
    }
}

fn support_pair_name_options(
    definitions: &[CharacterDefinition],
    main_index: usize,
) -> Vec<Vec<String>> {
    let candidates = definitions
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != main_index)
        .collect::<Vec<_>>();
    let mut pairs = Vec::new();

    for first in 0..candidates.len() {
        for second in (first + 1)..candidates.len() {
            pairs.push(vec![
                candidates[first].1.name.clone(),
                candidates[second].1.name.clone(),
            ]);
        }
    }

    pairs
}
