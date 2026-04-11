//! Battle screen for the rules-engine shell.

use macroquad::prelude::*;

use crate::engine::MatchAction;
use crate::screens::ScreenAction;
use crate::state::{
    opposing, AppState, BattleContext, CharacterStage, MatchPhase, MatchState, PlayerId, SideState,
    SupportState,
};
use crate::ui::card_widgets::{
    action_button, draw_story_card_preview, draw_story_card_tile, point_in_rect, section_panel,
};
use crate::ui::core::{draw_background_texture, draw_panel, draw_soft_panel, TEXT_MUTED};
use crate::ui::layout::UiLayout;

pub struct BattleScreen;

impl BattleScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, state: &AppState) -> ScreenAction {
        let Some(match_state) = state.match_state.as_ref() else {
            return ScreenAction::BackToMenu;
        };

        let ui = UiLayout::current();
        let player = PlayerId::PlayerA;

        let mouse = mouse_position();
        let hand_rects = hand_card_rects(match_state.hand_for(player).len());

        for (hand_index, (card_id, rect)) in match_state
            .hand_for(player)
            .iter()
            .zip(hand_rects.iter())
            .enumerate()
        {
            let Some(_card) = match_state.story_cards.get(card_id) else {
                continue;
            };
            let enabled = match_state.can_play_hand_card(player, hand_index);

            if point_in_rect(*rect, mouse) && is_mouse_button_pressed(MouseButton::Left) && enabled
            {
                return ScreenAction::ApplyMatchAction(MatchAction::PlayCardFromHand {
                    player,
                    hand_index,
                });
            }
        }

        let side_x = ui.x(56.0);
        let side_width = ui.w(410.0);
        let mut y = ui.y(820.0);

        if action_button(
            Rect::new(side_x, y, side_width, ui.h(66.0)),
            state.ui_text.get("battle_back_to_menu"),
        ) {
            return ScreenAction::BackToMenu;
        }
        y += ui.h(80.0);

        if match_state.reaction_priority_player() == Some(player)
            && action_button(
                Rect::new(side_x, y, side_width, ui.h(66.0)),
                state.ui_text.get("battle_pass_reaction"),
            )
        {
            return ScreenAction::ApplyMatchAction(MatchAction::PassReaction { player });
        }
        y += ui.h(80.0);

        if can_reveal_side(match_state, player, true)
            && action_button(
                Rect::new(side_x, y, side_width, ui.h(66.0)),
                state.ui_text.get("battle_reveal_mg_support"),
            )
        {
            return ScreenAction::ApplyMatchAction(MatchAction::RevealFirstHiddenSupport {
                player,
                is_magical_girl_side: true,
            });
        }
        y += ui.h(80.0);

        if !matches!(state.battle_context, BattleContext::Campaign { .. })
            && can_reveal_side(match_state, player, false)
            && action_button(
                Rect::new(side_x, y, side_width, ui.h(66.0)),
                state.ui_text.get("battle_reveal_baddie_support"),
            )
        {
            return ScreenAction::ApplyMatchAction(MatchAction::RevealFirstHiddenSupport {
                player,
                is_magical_girl_side: false,
            });
        }
        y += ui.h(80.0);

        if match_state.phase == MatchPhase::DailyLife
            && match_state.proactive_priority_player() == Some(player)
            && match_state.reaction_state.is_none()
            && action_button(
                Rect::new(side_x, y, side_width, ui.h(66.0)),
                state.ui_text.get("battle_pass_daily_life"),
            )
        {
            return ScreenAction::ApplyMatchAction(MatchAction::PassDailyLife { player });
        }
        y += ui.h(80.0);

        if (match_state.phase == MatchPhase::Encounter
            || match_state.phase == MatchPhase::FinalClimax)
            && match_state.proactive_priority_player() == Some(player)
            && match_state.reaction_state.is_none()
            && action_button(
                Rect::new(side_x, y, side_width, ui.h(66.0)),
                state.ui_text.get("battle_pass_encounter"),
            )
        {
            return ScreenAction::ApplyMatchAction(MatchAction::PassEncounter { player });
        }
        y += ui.h(80.0);

        if match_state.proactive_priority_player() == Some(player)
            && match_state.active_player == player
            && match_state.active_magical_girls().main.stage == CharacterStage::Radiant
            && (match_state.phase == MatchPhase::Encounter
                || match_state.phase == MatchPhase::FinalClimax)
            && !match_state.final_climax_active
            && action_button(
                Rect::new(side_x, y, side_width, ui.h(66.0)),
                state.ui_text.get("battle_declare_final_climax"),
            )
        {
            return ScreenAction::ApplyMatchAction(MatchAction::DeclareFinalClimax);
        }

        ScreenAction::None
    }

    pub fn draw(&self, state: &AppState) {
        let ui = UiLayout::current();
        let Some(match_state) = state.match_state.as_ref() else {
            draw_text(
                state.ui_text.get("battle_missing_match"),
                ui.x(56.0),
                ui.y(84.0),
                ui.font(48.0),
                WHITE,
            );
            return;
        };

        if let Some(background) = state.assets.ui_background("battle") {
            draw_background_texture(background, Color::new(1.0, 1.0, 1.0, 0.34));
        }

        section_panel(
            ui.rect(40.0, 36.0, 430.0, 744.0),
            state.ui_text.get("battle_event_log"),
            GRAY,
        );
        section_panel(
            ui.rect(40.0, 800.0, 430.0, 596.0),
            state.ui_text.get("battle_actions_label"),
            SKYBLUE,
        );
        draw_panel(ui.x(500.0), ui.y(36.0), ui.w(2020.0), ui.h(836.0), SKYBLUE);
        section_panel(
            ui.rect(500.0, 890.0, 2020.0, 506.0),
            state.ui_text.get("battle_your_hand_label"),
            SKYBLUE,
        );

        if matches!(state.battle_context, BattleContext::Campaign { .. }) {
            self.draw_campaign_battle(state, match_state);
        } else {
            self.draw_skirmish_battle(state, match_state);
        }

        self.draw_status_strip(state, match_state);

        let action_hint = battle_action_hint(state, match_state, PlayerId::PlayerA);
        draw_soft_panel(ui.x(540.0), ui.y(792.0), ui.w(820.0), ui.h(52.0), SKYBLUE);
        draw_text(
            action_hint,
            ui.x(562.0),
            ui.y(826.0),
            ui.font(24.0),
            TEXT_MUTED,
        );

        let mut line_y = ui.y(102.0);
        let wrapped_lines = wrap_event_lines(
            &match_state
                .event_log
                .iter()
                .rev()
                .take(6)
                .cloned()
                .collect::<Vec<_>>(),
            ui.w(360.0),
            ui.font(24.0),
        );
        for line in wrapped_lines {
            draw_text(&line, ui.x(68.0), line_y, ui.font(24.0), TEXT_MUTED);
            line_y += ui.h(34.0);
            if line_y > ui.y(740.0) {
                break;
            }
        }

        if let Some(card_name) = &match_state.last_played_card_name {
            let played_line = format!("{}: {}", state.ui_text.get("last_card_label"), card_name);
            draw_soft_panel(ui.x(1706.0), ui.y(792.0), ui.w(774.0), ui.h(52.0), PINK);
            draw_text(
                &played_line,
                ui.x(1730.0),
                ui.y(826.0),
                ui.font(22.0),
                TEXT_MUTED,
            );
        }

        self.draw_hand_cards(state, match_state, PlayerId::PlayerA);
    }

    fn draw_skirmish_battle(&self, state: &AppState, match_state: &MatchState) {
        let ui = UiLayout::current();
        draw_text(
            state.ui_text.get("battle_title"),
            ui.x(540.0),
            ui.y(86.0),
            ui.font(58.0),
            WHITE,
        );

        let lane_line = format!(
            "{}: {:?} MG -> {:?} Baddie",
            state.ui_text.get("battle_engagement_label"),
            match_state.active_player,
            opposing(match_state.active_player)
        );
        draw_text(&lane_line, ui.x(540.0), ui.y(128.0), ui.font(30.0), GOLD);

        if match_state.phase == MatchPhase::Finished {
            let winner_line = format!(
                "{}: {}",
                state.ui_text.get("battle_winner_label"),
                winner_label(state, match_state.winner)
            );
            draw_text(
                &winner_line,
                ui.x(540.0),
                ui.y(170.0),
                ui.font(30.0),
                ORANGE,
            );
        }

        self.draw_player_column(
            state,
            ui.x(540.0),
            ui.y(180.0),
            state.ui_text.get("battle_player_a_label"),
            state.ui_text.get("battle_player_a_identity"),
            &match_state.player_a.magical_girls,
            &match_state.player_a.baddies,
            match_state.player_a.hand.len(),
            match_state.player_a.deck.len(),
            player_status(state, match_state, PlayerId::PlayerA),
            match_state.defeated_prime_owner == Some(PlayerId::PlayerA),
        );

        self.draw_player_column(
            state,
            ui.x(1500.0),
            ui.y(180.0),
            state.ui_text.get("battle_player_b_label"),
            state.ui_text.get("battle_player_b_identity"),
            &match_state.player_b.magical_girls,
            &match_state.player_b.baddies,
            match_state.player_b.hand.len(),
            match_state.player_b.deck.len(),
            player_status(state, match_state, PlayerId::PlayerB),
            match_state.defeated_prime_owner == Some(PlayerId::PlayerB),
        );
    }

    fn draw_campaign_battle(&self, state: &AppState, match_state: &MatchState) {
        let ui = UiLayout::current();
        draw_text(
            state.ui_text.get("campaign_battle_title"),
            ui.x(540.0),
            ui.y(88.0),
            ui.font(54.0),
            WHITE,
        );

        let encounter_name = campaign_encounter_name(state);
        draw_text(
            &encounter_name,
            ui.x(540.0),
            ui.y(128.0),
            ui.font(30.0),
            GOLD,
        );

        let lane_line = if match_state.active_player == PlayerId::PlayerA {
            state.ui_text.get("campaign_battle_lane_player_attack")
        } else {
            state.ui_text.get("campaign_battle_lane_enemy_attack")
        };
        draw_text(lane_line, ui.x(540.0), ui.y(166.0), ui.font(24.0), ORANGE);

        if match_state.phase == MatchPhase::Finished {
            let winner_line = format!(
                "{}: {}",
                state.ui_text.get("battle_winner_label"),
                campaign_winner_label(state, match_state.winner)
            );
            draw_text(
                &winner_line,
                ui.x(540.0),
                ui.y(198.0),
                ui.font(28.0),
                ORANGE,
            );
        }

        self.draw_side_box(
            state,
            ui.x(540.0),
            ui.y(210.0),
            ui.w(900.0),
            ui.h(224.0),
            state.ui_text.get("campaign_battle_player_mg_label"),
            &match_state.player_a.magical_girls,
            true,
            false,
        );
        self.draw_side_box(
            state,
            ui.x(1540.0),
            ui.y(210.0),
            ui.w(900.0),
            ui.h(224.0),
            state.ui_text.get("campaign_battle_enemy_baddie_label"),
            &match_state.player_b.baddies,
            false,
            match_state.defeated_prime_owner == Some(PlayerId::PlayerB),
        );

        draw_soft_panel(ui.x(540.0), ui.y(462.0), ui.w(1900.0), ui.h(58.0), GRAY);
        draw_text(
            state.ui_text.get("campaign_battle_focus_note"),
            ui.x(564.0),
            ui.y(498.0),
            ui.font(22.0),
            TEXT_MUTED,
        );

        draw_stat_chip(
            ui.rect(540.0, 542.0, 220.0, 52.0),
            state.ui_text.get("battle_hand_count_label"),
            &match_state.player_a.hand.len().to_string(),
            SKYBLUE,
        );
        draw_stat_chip(
            ui.rect(776.0, 542.0, 220.0, 52.0),
            state.ui_text.get("battle_draw_pile_label"),
            &match_state.player_a.deck.len().to_string(),
            SKYBLUE,
        );
        draw_stat_chip(
            ui.rect(1540.0, 542.0, 220.0, 52.0),
            state.ui_text.get("campaign_battle_enemy_hand_label"),
            &match_state.player_b.hand.len().to_string(),
            PINK,
        );
        draw_stat_chip(
            ui.rect(1776.0, 542.0, 220.0, 52.0),
            state.ui_text.get("battle_draw_pile_label"),
            &match_state.player_b.deck.len().to_string(),
            PINK,
        );
    }

    fn draw_player_column(
        &self,
        state: &AppState,
        x: f32,
        y: f32,
        player_label: &str,
        identity_label: &str,
        magical_girls: &SideState,
        baddies: &SideState,
        hand_size: usize,
        deck_size: usize,
        status_label: &str,
        prime_defeated: bool,
    ) {
        let ui = UiLayout::current();
        draw_text(player_label, x, y, ui.font(30.0), WHITE);
        draw_text(identity_label, x + ui.w(210.0), y, ui.font(24.0), GRAY);
        draw_text(status_label, x + ui.w(500.0), y, ui.font(24.0), GOLD);

        self.draw_side_box(
            state,
            x,
            y + ui.h(28.0),
            ui.w(860.0),
            ui.h(180.0),
            state.ui_text.get("battle_magical_girls_label"),
            magical_girls,
            true,
            false,
        );
        self.draw_side_box(
            state,
            x,
            y + ui.h(236.0),
            ui.w(860.0),
            ui.h(180.0),
            state.ui_text.get("battle_baddies_label"),
            baddies,
            false,
            prime_defeated,
        );
        draw_text(
            &format!(
                "{} {}  {} {}",
                state.ui_text.get("battle_hand_count_label"),
                hand_size,
                state.ui_text.get("battle_draw_pile_label"),
                deck_size
            ),
            x,
            y + ui.h(452.0),
            ui.font(24.0),
            TEXT_MUTED,
        );
        draw_soft_panel(
            x + ui.w(582.0),
            y + ui.h(390.0),
            ui.w(208.0),
            ui.h(70.0),
            LIGHTGRAY,
        );
        draw_text(
            &format!(
                "{} {}",
                state.ui_text.get("battle_draw_pile_label"),
                deck_size
            ),
            x + ui.w(610.0),
            y + ui.h(434.0),
            ui.font(20.0),
            TEXT_MUTED,
        );
    }

    fn draw_side_box(
        &self,
        state: &AppState,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        label: &str,
        side: &SideState,
        is_magical_girl_side: bool,
        prime_defeated: bool,
    ) {
        let outline = if prime_defeated {
            RED
        } else if is_magical_girl_side {
            SKYBLUE
        } else {
            PINK
        };
        let ui = UiLayout::current();
        section_panel(Rect::new(x, y, width, height), label, outline);
        if prime_defeated {
            draw_text(
                state.ui_text.get("battle_prime_defeated_label"),
                x + ui.w(640.0),
                y + ui.h(34.0),
                ui.font(18.0),
                RED,
            );
        }

        if let Some(texture) = state.assets.portrait(&side.main_character_id) {
            draw_texture_ex(
                texture,
                x + ui.w(18.0),
                y + ui.h(54.0),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(ui.w(132.0), ui.h(132.0))),
                    ..Default::default()
                },
            );
        }
        let growth = if is_magical_girl_side {
            side.main.radiance
        } else {
            side.main.dread
        };
        draw_text(
            &side.main.name,
            x + ui.w(170.0),
            y + ui.h(84.0),
            ui.font(28.0),
            outline,
        );
        draw_stat_chip(
            Rect::new(x + ui.w(170.0), y + ui.h(104.0), ui.w(170.0), ui.h(38.0)),
            state.ui_text.get("battle_stage_label"),
            &format!("{:?}", side.main.stage),
            outline,
        );
        draw_stat_chip(
            Rect::new(x + ui.w(356.0), y + ui.h(104.0), ui.w(150.0), ui.h(38.0)),
            state.ui_text.get("battle_main_power_label"),
            &side.main.current_power().to_string(),
            outline,
        );
        draw_stat_chip(
            Rect::new(x + ui.w(522.0), y + ui.h(104.0), ui.w(156.0), ui.h(38.0)),
            state.ui_text.get("battle_total_power_label"),
            &side.total_power().to_string(),
            outline,
        );
        draw_stat_chip(
            Rect::new(x + ui.w(694.0), y + ui.h(104.0), ui.w(170.0), ui.h(38.0)),
            state.ui_text.get("battle_growth_label"),
            &growth.to_string(),
            outline,
        );
        draw_soft_panel(
            x + ui.w(170.0),
            y + ui.h(154.0),
            ui.w(694.0),
            ui.h(42.0),
            outline,
        );
        draw_text(
            &format_supports(state, &side.supports),
            x + ui.w(186.0),
            y + ui.h(182.0),
            ui.font(17.0),
            TEXT_MUTED,
        );
    }
}

impl BattleScreen {
    fn draw_status_strip(&self, state: &AppState, match_state: &MatchState) {
        let ui = UiLayout::current();
        draw_stat_chip(
            ui.rect(540.0, 688.0, 250.0, 56.0),
            state.ui_text.get("phase_label"),
            match_state.current_phase_label(),
            SKYBLUE,
        );
        draw_stat_chip(
            ui.rect(808.0, 688.0, 296.0, 56.0),
            state.ui_text.get("controller_label"),
            &format!(
                "{:?}",
                match_state
                    .reaction_priority_player()
                    .unwrap_or(match_state.active_player)
            ),
            GOLD,
        );
        draw_stat_chip(
            ui.rect(1122.0, 688.0, 352.0, 56.0),
            state.ui_text.get("battle_reaction_window_label"),
            if match_state.reaction_state.is_some() {
                state.ui_text.get("battle_open_short")
            } else {
                state.ui_text.get("battle_closed_short")
            },
            PINK,
        );
    }

    fn draw_hand_cards(&self, state: &AppState, match_state: &MatchState, player: PlayerId) {
        let ui = UiLayout::current();
        let mouse = mouse_position();
        let mut hovered_card = None;
        let hand_rects = hand_card_rects(match_state.hand_for(player).len());

        for (hand_index, (card_id, rect)) in match_state
            .hand_for(player)
            .iter()
            .zip(hand_rects.iter())
            .enumerate()
        {
            let Some(card) = match_state.story_cards.get(card_id) else {
                continue;
            };
            let enabled = match_state.can_play_hand_card(player, hand_index);
            let hovered = point_in_rect(*rect, mouse);
            if hovered {
                hovered_card = Some((card, enabled));
            }
            draw_story_card_tile(
                state,
                *rect,
                card,
                hand_card_status_label(state, enabled),
                enabled,
                hovered,
            );
        }

        if let Some((card, enabled)) = hovered_card {
            let preview_rect = Rect::new(ui.x(1760.0), ui.y(312.0), ui.w(420.0), ui.h(590.0));
            let footer = vec![
                format!("{}: {}", state.ui_text.get("phase_label"), card.card_type),
                hand_card_status_label(state, enabled).to_owned(),
            ];
            draw_story_card_preview(state, preview_rect, card, &footer);
        }
    }
}

fn can_reveal_side(match_state: &MatchState, player: PlayerId, is_magical_girl_side: bool) -> bool {
    ((match_state.reaction_priority_player() == Some(player))
        || (match_state.reaction_state.is_none()
            && match_state.proactive_priority_player() == Some(player)))
        && match_state.can_reveal_support(player, is_magical_girl_side)
}

fn format_supports(state: &AppState, supports: &[SupportState]) -> String {
    let labels = supports
        .iter()
        .enumerate()
        .map(|(index, support)| {
            if support.revealed {
                format!("S{} {}", index + 1, support.runtime.name)
            } else {
                format!(
                    "S{} {}",
                    index + 1,
                    state.ui_text.get("battle_hidden_support_short")
                )
            }
        })
        .collect::<Vec<_>>();

    format!(
        "{}: {}",
        state.ui_text.get("battle_supports_label"),
        labels.join(" | ")
    )
}

fn player_status<'a>(
    state: &'a AppState,
    match_state: &'a MatchState,
    player: PlayerId,
) -> &'a str {
    if match_state.phase == MatchPhase::Finished {
        state.ui_text.get("battle_finished_label")
    } else if match_state.active_player == player {
        state.ui_text.get("battle_attacking_label")
    } else if opposing(match_state.active_player) == player {
        state.ui_text.get("battle_defending_label")
    } else {
        state.ui_text.get("battle_idle_label")
    }
}

fn winner_label<'a>(state: &'a AppState, winner: Option<PlayerId>) -> &'a str {
    match winner {
        Some(PlayerId::PlayerA) => state.ui_text.get("battle_result_player_a"),
        Some(PlayerId::PlayerB) => state.ui_text.get("battle_result_player_b"),
        None => state.ui_text.get("battle_result_unknown"),
    }
}

fn campaign_winner_label<'a>(state: &'a AppState, winner: Option<PlayerId>) -> &'a str {
    match winner {
        Some(PlayerId::PlayerA) => state.ui_text.get("campaign_battle_player_wins"),
        Some(PlayerId::PlayerB) => state.ui_text.get("campaign_battle_enemy_wins"),
        None => state.ui_text.get("battle_result_unknown"),
    }
}

fn hand_card_status_label<'a>(state: &'a AppState, enabled: bool) -> &'a str {
    if enabled {
        state.ui_text.get("battle_card_ready")
    } else {
        state.ui_text.get("battle_card_hold")
    }
}

fn battle_action_hint<'a>(
    state: &'a AppState,
    match_state: &'a MatchState,
    player: PlayerId,
) -> &'a str {
    if match_state.reaction_priority_player() == Some(player) {
        state.ui_text.get("battle_hint_reaction")
    } else if match_state.proactive_priority_player() == Some(player)
        && match_state.phase == MatchPhase::DailyLife
    {
        state.ui_text.get("battle_hint_daily_life")
    } else if match_state.proactive_priority_player() == Some(player)
        && (match_state.phase == MatchPhase::Encounter
            || match_state.phase == MatchPhase::FinalClimax)
    {
        state.ui_text.get("battle_hint_encounter")
    } else if match_state.phase == MatchPhase::Finished {
        state.ui_text.get("battle_hint_finished")
    } else {
        state.ui_text.get("battle_hint_waiting")
    }
}

fn campaign_encounter_name(state: &AppState) -> String {
    let BattleContext::Campaign { node_id, .. } = &state.battle_context else {
        return String::new();
    };
    state
        .content
        .campaign
        .node(node_id)
        .and_then(|node| state.content.campaign.encounter(&node.encounter_id))
        .map(|encounter| encounter.name.clone())
        .unwrap_or_else(|| state.ui_text.get("campaign_missing_encounter").to_owned())
}

fn wrap_event_lines(events: &[String], max_width: f32, font_size: f32) -> Vec<String> {
    let mut wrapped = Vec::new();

    for event in events {
        let mut current = String::new();
        for word in event.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_owned()
            } else {
                format!("{current} {word}")
            };
            if measure_text(&candidate, None, font_size as u16, 1.0).width <= max_width {
                current = candidate;
            } else {
                if !current.is_empty() {
                    wrapped.push(current);
                }
                current = word.to_owned();
            }
        }
        if !current.is_empty() {
            wrapped.push(current);
        }
    }

    wrapped
}

fn hand_card_rects(card_count: usize) -> Vec<Rect> {
    let ui = UiLayout::current();
    let start_x = ui.x(548.0);
    let y = ui.y(986.0);
    let card_width = ui.w(250.0);
    let card_height = ui.h(352.0);
    let preferred_gap = ui.w(18.0);
    let available_width = ui.w(1920.0);

    if card_count == 0 {
        return Vec::new();
    }

    let step = if card_count == 1 {
        0.0
    } else {
        let preferred_total =
            card_width * card_count as f32 + preferred_gap * (card_count.saturating_sub(1)) as f32;
        if preferred_total <= available_width {
            card_width + preferred_gap
        } else {
            (available_width - card_width) / (card_count.saturating_sub(1)) as f32
        }
    };

    (0..card_count)
        .map(|index| Rect::new(start_x + index as f32 * step, y, card_width, card_height))
        .collect()
}

fn draw_stat_chip(rect: Rect, label: &str, value: &str, accent: Color) {
    draw_soft_panel(rect.x, rect.y, rect.w, rect.h, accent);
    draw_text(label, rect.x + 14.0, rect.y + 20.0, 16.0, TEXT_MUTED);
    draw_text(value, rect.x + 14.0, rect.y + 42.0, 22.0, WHITE);
}
