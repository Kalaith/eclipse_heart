//! Core rules application isolated from UI.

use crate::data::{CardEffect, CardSpeed};
use crate::state::{
    opposing, CharacterStage, MatchPhase, MatchState, PlayerId, ReactionState, ResourceKind,
    SideState, StackItem, StackItemKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchAction {
    ResolveEncounter,
    DeclareFinalClimax,
    PlayCardFromHand {
        player: PlayerId,
        hand_index: usize,
    },
    RevealFirstHiddenSupport {
        player: PlayerId,
        is_magical_girl_side: bool,
    },
    PassDailyLife {
        player: PlayerId,
    },
    PassEncounter {
        player: PlayerId,
    },
    PassReaction {
        player: PlayerId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncounterOutcome {
    ActivePlayerWins,
    DefendingPlayerWins,
    Tie,
}

pub struct MatchEngine;

impl MatchEngine {
    pub fn apply_action(state: &mut MatchState, action: MatchAction) {
        match action {
            MatchAction::ResolveEncounter => {
                if state.reaction_state.is_none()
                    && (state.phase == MatchPhase::Encounter
                        || state.phase == MatchPhase::FinalClimax)
                {
                    let outcome = Self::resolve_encounter(state);
                    state.last_outcome = Some(outcome);
                    state.round += 1;
                    if state.phase != MatchPhase::Finished {
                        state.phase = if state.final_climax_active {
                            MatchPhase::FinalClimax
                        } else {
                            MatchPhase::DailyLife
                        };
                    }
                    state.clear_encounter_bonuses();
                    state.ready_end_of_round();
                }
            }
            MatchAction::DeclareFinalClimax => {
                if Self::can_declare_final_climax(state) {
                    state.final_climax_active = true;
                    state.phase = MatchPhase::FinalClimax;
                    state.push_event(format!("{:?} declared Final Climax.", state.active_player));
                }
            }
            MatchAction::PlayCardFromHand { player, hand_index } => {
                Self::queue_selected_card(state, player, hand_index);
            }
            MatchAction::RevealFirstHiddenSupport {
                player,
                is_magical_girl_side,
            } => Self::queue_support_reveal(state, player, is_magical_girl_side),
            MatchAction::PassDailyLife { player } => Self::pass_daily_life(state, player),
            MatchAction::PassEncounter { player } => Self::pass_encounter(state, player),
            MatchAction::PassReaction { player } => Self::pass_reaction(state, player),
        }
    }

    pub fn can_declare_final_climax(state: &MatchState) -> bool {
        state.reaction_state.is_none()
            && (state.phase == MatchPhase::Encounter || state.phase == MatchPhase::FinalClimax)
            && state.active_magical_girls().main.stage == CharacterStage::Radiant
            && !state.final_climax_active
    }

    pub fn resolve_encounter(state: &mut MatchState) -> EncounterOutcome {
        let attacker = state.active_player;
        let defender = opposing(attacker);
        let win_gain = state.rules.encounter_win_gain;
        let loss_gain = state.rules.encounter_loss_gain;
        let tie_gain = state.rules.encounter_tie_gain;
        let magical_girl_power = state.active_magical_girls().total_power();
        let baddie_power = state.defending_baddies().total_power();

        let outcome = if magical_girl_power > baddie_power {
            EncounterOutcome::ActivePlayerWins
        } else if baddie_power > magical_girl_power {
            EncounterOutcome::DefendingPlayerWins
        } else {
            EncounterOutcome::Tie
        };

        match outcome {
            EncounterOutcome::ActivePlayerWins => {
                Self::grant_growth(
                    &mut state.player_for_mut(attacker).magical_girls,
                    ResourceKind::Radiance,
                    win_gain,
                );
                Self::grant_growth(
                    &mut state.player_for_mut(defender).baddies,
                    ResourceKind::Dread,
                    loss_gain,
                );
                state.push_event(format!(
                    "{attacker:?} Magical Girls beat {defender:?} Baddies."
                ));
            }
            EncounterOutcome::DefendingPlayerWins => {
                Self::grant_growth(
                    &mut state.player_for_mut(attacker).magical_girls,
                    ResourceKind::Radiance,
                    loss_gain,
                );
                Self::grant_growth(
                    &mut state.player_for_mut(defender).baddies,
                    ResourceKind::Dread,
                    win_gain,
                );
                state.push_event(format!("{defender:?} Baddies hold against {attacker:?}."));
            }
            EncounterOutcome::Tie => {
                if state.active_magical_girls().main.stage != CharacterStage::Radiant {
                    Self::grant_growth(
                        &mut state.player_for_mut(attacker).magical_girls,
                        ResourceKind::Radiance,
                        tie_gain,
                    );
                }

                if state.defending_baddies().main.stage != CharacterStage::Catastrophe {
                    Self::grant_growth(
                        &mut state.player_for_mut(defender).baddies,
                        ResourceKind::Dread,
                        tie_gain,
                    );
                }
                state.push_event("Encounter resolved as a tie.");
            }
        }

        if state.final_climax_active && baddie_power < magical_girl_power {
            state.phase = MatchPhase::Finished;
            state.prime_baddie_defeated = true;
            state.defeated_prime_owner = Some(defender);
            state.winner = Some(attacker);
            state.push_event(format!("{defender:?} Prime Baddie was defeated."));
        } else if state.final_climax_active {
            Self::grant_failed_final_climax_power(state, attacker);

            if magical_girl_power < baddie_power {
                if state.player_for(defender).magical_girls.main.stage != CharacterStage::Radiant {
                    state
                        .active_magical_girls_mut()
                        .main
                        .exhausted_until_next_encounter = true;
                    state
                        .active_magical_girls_mut()
                        .main
                        .abilities_blocked_until_next_encounter = true;
                    state.push_event(format!(
                        "{attacker:?} lost Final Climax and its Main Magical Girl is exhausted."
                    ));
                } else {
                    state.push_event(format!(
                        "{attacker:?} lost Final Climax, but no exhaustion is applied because both Main Magical Girls are Radiant."
                    ));
                }
            } else {
                state.push_event(format!(
                    "{attacker:?} drew Final Climax and gains +1 power for the next attempt."
                ));
            }
        }

        outcome
    }

    fn pass_daily_life(state: &mut MatchState, player: PlayerId) {
        if state.reaction_state.is_none()
            && state.phase == MatchPhase::DailyLife
            && state.active_player == player
        {
            state.daily_life_passes += 1;
            if state.daily_life_passes >= 2 {
                state.phase = MatchPhase::Encounter;
                state.daily_life_passes = 0;
                state.push_event(format!("{player:?} finished Daily Life. Encounter begins."));
            } else {
                state.push_event(format!("{player:?} passed Daily Life."));
            }
        }
    }

    fn pass_encounter(state: &mut MatchState, player: PlayerId) {
        if state.reaction_state.is_some()
            || (state.phase != MatchPhase::Encounter && state.phase != MatchPhase::FinalClimax)
            || state.active_player != player
        {
            return;
        }

        if !state.encounter_card_played(player) {
            state.set_encounter_card_played(player, true);
            state.push_event(format!("{player:?} passed encounter priority."));
        }
    }

    fn queue_selected_card(state: &mut MatchState, player: PlayerId, hand_index: usize) {
        let Some(card) = state.card_in_hand(player, hand_index).cloned() else {
            return;
        };

        let Some(expected_speed) = state.expected_hand_speed(player) else {
            return;
        };

        if card.speed != expected_speed {
            return;
        }

        Self::queue_card_from_hand(
            state,
            player,
            hand_index,
            expected_speed == CardSpeed::Reaction,
        );
    }

    fn queue_card_from_hand(
        state: &mut MatchState,
        player: PlayerId,
        card_index: usize,
        is_reaction: bool,
    ) {
        let Some(card) = state.card_in_hand(player, card_index).cloned() else {
            return;
        };
        let card_id = state.hand_for_mut(player).remove(card_index);

        state.discard_for_mut(player).push(card_id.clone());
        state.reaction_stack.push(StackItem {
            player,
            is_reaction,
            resolves_in_phase: state.phase,
            kind: StackItemKind::PlayCard {
                card_id,
                card_name: card.name.clone(),
            },
        });
        state.reaction_state = Some(ReactionState {
            priority_player: opposing(player),
            passes_in_row: 0,
        });
        state.push_event(format!("{player:?} queued {}.", card.name));
    }

    fn queue_support_reveal(state: &mut MatchState, player: PlayerId, is_magical_girl_side: bool) {
        if state.phase == MatchPhase::Finished {
            return;
        }

        let is_reaction = state.reaction_priority_player() == Some(player);
        if !is_reaction && state.active_player != player {
            return;
        }

        if !state.can_reveal_support(player, is_magical_girl_side) {
            return;
        }

        let Some(support_index) = state
            .side_for(player, is_magical_girl_side)
            .supports
            .iter()
            .position(|support| !support.revealed)
        else {
            return;
        };

        state.mark_support_revealed_this_round(player);
        state.reaction_stack.push(StackItem {
            player,
            is_reaction,
            resolves_in_phase: state.phase,
            kind: StackItemKind::RevealSupport {
                is_magical_girl_side,
                support_index,
            },
        });
        state.reaction_state = Some(ReactionState {
            priority_player: opposing(player),
            passes_in_row: 0,
        });
        state.push_event(format!("{player:?} queued a support reveal."));
    }

    fn pass_reaction(state: &mut MatchState, player: PlayerId) {
        let Some(priority_player) = state.reaction_priority_player() else {
            return;
        };
        if priority_player != player {
            return;
        }

        let passes_in_row = {
            let reaction_state = state
                .reaction_state
                .as_mut()
                .expect("reaction state exists");
            reaction_state.passes_in_row += 1;
            reaction_state.passes_in_row
        };
        state.push_event(format!("{player:?} passed priority."));

        if passes_in_row >= 2 {
            Self::resolve_stack(state);
        } else {
            state
                .reaction_state
                .as_mut()
                .expect("reaction state exists")
                .priority_player = opposing(player);
        }
    }

    fn resolve_stack(state: &mut MatchState) {
        let Some(root_item) = state.reaction_stack.first().cloned() else {
            state.reaction_state = None;
            return;
        };

        state.reaction_state = None;
        while let Some(item) = state.reaction_stack.pop() {
            match item.kind {
                StackItemKind::PlayCard { card_id, card_name } => {
                    let effects = state
                        .story_cards
                        .get(&card_id)
                        .map(|card| card.effects.clone())
                        .unwrap_or_default();

                    for effect in &effects {
                        Self::apply_card_effect(state, item.player, effect);
                    }

                    state.last_played_card_name = Some(card_name.clone());
                    state.push_event(format!("{:?} resolved {}.", item.player, card_name));
                }
                StackItemKind::RevealSupport {
                    is_magical_girl_side,
                    support_index,
                } => {
                    let revealed_name = {
                        let side = state.side_for_mut(item.player, is_magical_girl_side);
                        side.supports.get_mut(support_index).and_then(|support| {
                            if support.revealed {
                                None
                            } else {
                                support.revealed = true;
                                Some(support.runtime.name.clone())
                            }
                        })
                    };
                    if let Some(name) = revealed_name {
                        state.push_event(format!("{:?} revealed {}.", item.player, name));
                    }
                }
            }
        }

        if !root_item.is_reaction {
            Self::finish_root_action(state, &root_item);
        }
    }

    fn finish_root_action(state: &mut MatchState, root_item: &StackItem) {
        match root_item.resolves_in_phase {
            MatchPhase::DailyLife => {
                state.daily_life_passes = 0;
            }
            MatchPhase::Encounter | MatchPhase::FinalClimax => {
                state.set_encounter_card_played(root_item.player, true);
            }
            MatchPhase::Finished => {}
        }
    }

    fn apply_card_effect(state: &mut MatchState, player: PlayerId, effect: &CardEffect) {
        match effect {
            CardEffect::GainMainRadiance { amount } => {
                Self::apply_main_growth(
                    state,
                    player,
                    true,
                    ResourceKind::Radiance,
                    *amount,
                    "Radiance",
                );
            }
            CardEffect::GainRevealedSupportRadiance { amount } => {
                Self::apply_to_revealed_supports(
                    state,
                    player,
                    true,
                    ResourceKind::Radiance,
                    *amount,
                    "Radiance",
                );
            }
            CardEffect::ReduceOpponentMainRadiance { amount } => {
                Self::apply_main_growth(
                    state,
                    opposing(player),
                    true,
                    ResourceKind::Radiance,
                    -*amount,
                    "Radiance",
                );
            }
            CardEffect::GainPrimeDread { amount } => {
                Self::apply_main_growth(
                    state,
                    player,
                    false,
                    ResourceKind::Dread,
                    *amount,
                    "Dread",
                );
            }
            CardEffect::GainRevealedSupportDread { amount } => {
                Self::apply_to_revealed_supports(
                    state,
                    player,
                    false,
                    ResourceKind::Dread,
                    *amount,
                    "Dread",
                );
            }
            CardEffect::ReduceOpponentPrimeDread { amount } => {
                Self::apply_main_growth(
                    state,
                    opposing(player),
                    false,
                    ResourceKind::Dread,
                    -*amount,
                    "Dread",
                );
            }
            CardEffect::GainMainPowerThisEncounter { amount } => {
                let name = state.player_for(player).magical_girls.main.name.clone();
                state
                    .player_for_mut(player)
                    .magical_girls
                    .main
                    .temporary_power_bonus += amount;
                state.push_event(format!("{name} gains {amount} power this encounter."));
            }
            CardEffect::GainMainPowerNextEncounter { amount } => {
                let name = state.player_for(player).magical_girls.main.name.clone();
                state
                    .player_for_mut(player)
                    .magical_girls
                    .main
                    .next_encounter_power_bonus += amount;
                state.push_event(format!("{name} gains {amount} power next encounter."));
            }
            CardEffect::ReduceOpponentMainPowerThisEncounter { amount } => {
                let target = opposing(player);
                let name = state.player_for(target).magical_girls.main.name.clone();
                state
                    .player_for_mut(target)
                    .magical_girls
                    .main
                    .temporary_power_bonus -= amount;
                state.push_event(format!("{name} loses {amount} power this encounter."));
            }
            CardEffect::GainPrimePowerThisEncounter { amount } => {
                let name = state.player_for(player).baddies.main.name.clone();
                state
                    .player_for_mut(player)
                    .baddies
                    .main
                    .temporary_power_bonus += amount;
                state.push_event(format!("{name} gains {amount} power this encounter."));
            }
            CardEffect::GainRevealedSupportPowerThisEncounter { amount } => {
                let names = state
                    .player_for_mut(player)
                    .magical_girls
                    .supports
                    .iter_mut()
                    .filter(|support| support.revealed)
                    .map(|support| {
                        support.runtime.temporary_power_bonus += amount;
                        support.runtime.name.clone()
                    })
                    .collect::<Vec<_>>();
                for name in names {
                    state.push_event(format!("{name} gains {amount} power this encounter."));
                }
            }
            CardEffect::GainFirstRevealedSupportRadiance { amount } => {
                let log = {
                    state
                        .player_for_mut(player)
                        .magical_girls
                        .supports
                        .iter_mut()
                        .find(|support| support.revealed)
                        .map(|support| {
                            let name = support.runtime.name.clone();
                            let before = support.runtime.radiance;
                            let stage_before = support.runtime.stage;
                            support.gain(ResourceKind::Radiance, *amount);
                            (
                                name,
                                before,
                                support.runtime.radiance,
                                stage_before,
                                support.runtime.stage,
                            )
                        })
                };
                if let Some((name, before, after, stage_before, stage_after)) = log {
                    Self::log_resource_change(state, &name, "Radiance", before, after);
                    Self::log_stage_change(state, &name, stage_before, stage_after);
                }
            }
            CardEffect::ExhaustFirstRevealedOpponentSupport => {
                let target = opposing(player);
                let exhausted_name = {
                    state
                        .player_for_mut(target)
                        .magical_girls
                        .supports
                        .iter_mut()
                        .find(|support| support.revealed)
                        .map(|support| {
                            support.runtime.exhausted_until_next_encounter = true;
                            support.runtime.abilities_blocked_until_next_encounter = true;
                            support.runtime.name.clone()
                        })
                };
                if let Some(name) = exhausted_name {
                    state.push_event(format!("{name} becomes exhausted."));
                }
            }
            CardEffect::RevealFirstHiddenOwnSupport => {
                if state.player_for(player).supports_revealed_this_round > 0 {
                    return;
                }

                let revealed_name = {
                    state
                        .player_for_mut(player)
                        .magical_girls
                        .supports
                        .iter_mut()
                        .find(|support| !support.revealed)
                        .map(|support| {
                            support.revealed = true;
                            support.runtime.name.clone()
                        })
                };
                if let Some(name) = revealed_name {
                    state.mark_support_revealed_this_round(player);
                    state.push_event(format!("{name} is revealed."));
                }
            }
        }
    }

    fn apply_main_growth(
        state: &mut MatchState,
        player: PlayerId,
        is_magical_girl_side: bool,
        resource: ResourceKind,
        amount: i32,
        label: &str,
    ) {
        let (name, before, after, stage_before, stage_after) = {
            let side = state.side_for_mut(player, is_magical_girl_side);
            let name = side.main.name.clone();
            let before = match resource {
                ResourceKind::Radiance => side.main.radiance,
                ResourceKind::Dread => side.main.dread,
            };
            let stage_before = side.main.stage;
            side.main.gain(resource, amount);
            let after = match resource {
                ResourceKind::Radiance => side.main.radiance,
                ResourceKind::Dread => side.main.dread,
            };
            (name, before, after, stage_before, side.main.stage)
        };

        Self::log_resource_change(state, &name, label, before, after);
        Self::log_stage_change(state, &name, stage_before, stage_after);
    }

    fn apply_to_revealed_supports(
        state: &mut MatchState,
        player: PlayerId,
        is_magical_girl_side: bool,
        resource: ResourceKind,
        amount: i32,
        label: &str,
    ) {
        let mut logs = Vec::new();
        for support in &mut state.side_for_mut(player, is_magical_girl_side).supports {
            if support.revealed {
                let before = match resource {
                    ResourceKind::Radiance => support.runtime.radiance,
                    ResourceKind::Dread => support.runtime.dread,
                };
                let stage_before = support.runtime.stage;
                let name = support.runtime.name.clone();
                support.gain(resource, amount);
                let after = match resource {
                    ResourceKind::Radiance => support.runtime.radiance,
                    ResourceKind::Dread => support.runtime.dread,
                };
                logs.push((name, before, after, stage_before, support.runtime.stage));
            }
        }

        for (name, before, after, stage_before, stage_after) in logs {
            Self::log_resource_change(state, &name, label, before, after);
            Self::log_stage_change(state, &name, stage_before, stage_after);
        }
    }

    fn log_resource_change(
        state: &mut MatchState,
        name: &str,
        label: &str,
        before: i32,
        after: i32,
    ) {
        if before != after {
            state.push_event(format!("{name} {label}: {before} -> {after}."));
        }
    }

    fn log_stage_change(
        state: &mut MatchState,
        name: &str,
        before: CharacterStage,
        after: CharacterStage,
    ) {
        if before != after {
            state.push_event(format!("{name} advanced from {before:?} to {after:?}."));
        }
    }

    fn grant_growth(side: &mut SideState, resource: ResourceKind, amount: i32) {
        side.main.gain(resource, amount);
        for support in &mut side.supports {
            if support.revealed {
                support.gain(resource, amount);
            }
        }
    }

    fn grant_failed_final_climax_power(state: &mut MatchState, player: PlayerId) {
        let main = &mut state.player_for_mut(player).magical_girls.main;
        main.failed_final_climax_power_bonus += 1;
        let name = main.name.clone();
        state.push_event(format!(
            "{name} gains +1 power for the next Final Climax attempt."
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{CardSpeed, GameContent};
    use crate::state::{CharacterStage, MatchPhase, MatchSetup, MatchState, PlayerId};

    use super::{EncounterOutcome, MatchAction, MatchEngine};

    fn sample_state() -> MatchState {
        let content = GameContent::load().unwrap_or_default();
        let setup = MatchSetup::default_for_content(&content);
        MatchState::from_setup(&content, &setup)
    }

    #[test]
    fn sums_only_revealed_non_exhausted_support_power() {
        let mut state = sample_state();
        state.player_a.magical_girls.supports[0].revealed = true;
        state.player_a.magical_girls.supports[1].revealed = true;
        state.player_a.magical_girls.supports[1]
            .runtime
            .exhausted_until_next_encounter = true;
        assert_eq!(state.player_a.magical_girls.total_power(), 3);
    }

    #[test]
    fn upgrades_to_radiant_on_threshold_cross() {
        let mut state = sample_state();
        state.player_a.magical_girls.main.stage = CharacterStage::Transformed;
        state.player_a.magical_girls.main.radiance = 5;
        state
            .player_a
            .magical_girls
            .main
            .gain(crate::state::ResourceKind::Radiance, 1);
        assert_eq!(
            state.player_a.magical_girls.main.stage,
            CharacterStage::Radiant
        );
        assert_eq!(state.player_a.magical_girls.main.radiance, 0);
    }

    #[test]
    fn final_climax_can_only_be_declared_with_active_radiant_main() {
        let mut state = sample_state();
        state.phase = MatchPhase::Encounter;
        assert!(!MatchEngine::can_declare_final_climax(&state));
        state.player_a.magical_girls.main.stage = CharacterStage::Radiant;
        assert!(MatchEngine::can_declare_final_climax(&state));
    }

    #[test]
    fn player_a_final_climax_win_defeats_player_b_prime_baddie() {
        let mut state = sample_state();
        state.phase = MatchPhase::Encounter;
        state.final_climax_active = true;
        state.player_a.magical_girls.main.stage = CharacterStage::Radiant;
        state.player_a.magical_girls.main.base_power = 4;
        state.player_b.baddies.main.base_power = 1;
        let outcome = MatchEngine::resolve_encounter(&mut state);
        assert_eq!(outcome, EncounterOutcome::ActivePlayerWins);
        assert_eq!(state.defeated_prime_owner, Some(PlayerId::PlayerB));
        assert_eq!(state.winner, Some(PlayerId::PlayerA));
    }

    #[test]
    fn reaction_stack_resolves_newest_to_oldest() {
        let mut state = sample_state();
        let player_a_daily = state
            .first_playable_hand_index(PlayerId::PlayerA, CardSpeed::DailyLife)
            .expect("player a daily card");
        MatchEngine::apply_action(
            &mut state,
            MatchAction::PlayCardFromHand {
                player: PlayerId::PlayerA,
                hand_index: player_a_daily,
            },
        );
        let player_b_reaction = state
            .first_playable_hand_index(PlayerId::PlayerB, CardSpeed::Reaction)
            .expect("player b reaction");
        MatchEngine::apply_action(
            &mut state,
            MatchAction::PlayCardFromHand {
                player: PlayerId::PlayerB,
                hand_index: player_b_reaction,
            },
        );
        let player_a_reaction = state
            .first_playable_hand_index(PlayerId::PlayerA, CardSpeed::Reaction)
            .expect("player a reaction");
        MatchEngine::apply_action(
            &mut state,
            MatchAction::PlayCardFromHand {
                player: PlayerId::PlayerA,
                hand_index: player_a_reaction,
            },
        );
        MatchEngine::apply_action(
            &mut state,
            MatchAction::PassReaction {
                player: PlayerId::PlayerB,
            },
        );
        MatchEngine::apply_action(
            &mut state,
            MatchAction::PassReaction {
                player: PlayerId::PlayerA,
            },
        );
        assert!(state.reaction_stack.is_empty());
        assert!(state.reaction_state.is_none());
    }

    #[test]
    fn pass_encounter_marks_player_done_when_out_of_actions() {
        let mut state = sample_state();
        state.phase = MatchPhase::Encounter;
        state.player_b.hand.clear();
        state
            .player_b
            .magical_girls
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = true);
        state
            .player_b
            .baddies
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = true);
        state.active_player = PlayerId::PlayerB;
        MatchEngine::apply_action(
            &mut state,
            MatchAction::PassEncounter {
                player: PlayerId::PlayerB,
            },
        );
        assert!(state.player_b.encounter_card_played);
    }

    #[test]
    fn round_start_draws_one_for_both_players_after_encounter() {
        let mut state = sample_state();
        state.phase = MatchPhase::Encounter;
        state.player_a.encounter_card_played = true;
        state.player_b.encounter_card_played = true;
        let player_a_hand_before = state.player_a.hand.len();
        let player_b_hand_before = state.player_b.hand.len();
        let player_a_deck_before = state.player_a.deck.len();
        let player_b_deck_before = state.player_b.deck.len();

        MatchEngine::apply_action(&mut state, MatchAction::ResolveEncounter);

        assert_eq!(state.phase, MatchPhase::DailyLife);
        assert_eq!(state.player_a.hand.len(), player_a_hand_before + 1);
        assert_eq!(state.player_b.hand.len(), player_b_hand_before + 1);
        assert_eq!(state.player_a.deck.len(), player_a_deck_before - 1);
        assert_eq!(state.player_b.deck.len(), player_b_deck_before - 1);
    }

    #[test]
    fn player_can_only_reveal_one_support_per_round() {
        let mut state = sample_state();
        state.phase = MatchPhase::DailyLife;
        state.player_a.magical_girls.supports[0].revealed = false;
        state.player_a.magical_girls.supports[1].revealed = false;
        state.player_a.baddies.supports[1].revealed = false;

        MatchEngine::apply_action(
            &mut state,
            MatchAction::RevealFirstHiddenSupport {
                player: PlayerId::PlayerA,
                is_magical_girl_side: true,
            },
        );

        assert_eq!(state.player_a.supports_revealed_this_round, 1);
        assert!(!state.can_reveal_support(PlayerId::PlayerA, false));
    }

    #[test]
    fn failed_final_climax_draw_grants_next_attempt_power() {
        let mut state = sample_state();
        state.phase = MatchPhase::FinalClimax;
        state.final_climax_active = true;
        state.player_a.magical_girls.main.stage = CharacterStage::Radiant;
        state
            .player_a
            .magical_girls
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = false);
        state
            .player_b
            .baddies
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = false);
        state.player_a.magical_girls.main.final_power = 3;
        state.player_b.baddies.main.base_power = 3;

        let outcome = MatchEngine::resolve_encounter(&mut state);

        assert_eq!(outcome, EncounterOutcome::Tie);
        assert_eq!(
            state
                .player_a
                .magical_girls
                .main
                .failed_final_climax_power_bonus,
            1
        );
        assert_eq!(state.player_a.magical_girls.main.current_power(), 4);
    }

    #[test]
    fn failed_final_climax_loss_grants_next_attempt_power() {
        let mut state = sample_state();
        state.phase = MatchPhase::FinalClimax;
        state.final_climax_active = true;
        state.player_a.magical_girls.main.stage = CharacterStage::Radiant;
        state
            .player_a
            .magical_girls
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = false);
        state
            .player_b
            .baddies
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = false);
        state.player_a.magical_girls.main.final_power = 2;
        state.player_b.baddies.main.base_power = 3;

        let outcome = MatchEngine::resolve_encounter(&mut state);

        assert_eq!(outcome, EncounterOutcome::DefendingPlayerWins);
        assert_eq!(
            state
                .player_a
                .magical_girls
                .main
                .failed_final_climax_power_bonus,
            1
        );
        assert_eq!(state.player_a.magical_girls.main.current_power(), 0);
        assert!(
            state
                .player_a
                .magical_girls
                .main
                .exhausted_until_next_encounter
        );
    }

    #[test]
    fn failed_final_climax_loss_does_not_exhaust_when_both_mains_are_radiant() {
        let mut state = sample_state();
        state.phase = MatchPhase::FinalClimax;
        state.final_climax_active = true;
        state.player_a.magical_girls.main.stage = CharacterStage::Radiant;
        state.player_b.magical_girls.main.stage = CharacterStage::Radiant;
        state
            .player_a
            .magical_girls
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = false);
        state
            .player_b
            .baddies
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = false);
        state.player_a.magical_girls.main.final_power = 2;
        state.player_b.baddies.main.base_power = 4;

        let outcome = MatchEngine::resolve_encounter(&mut state);

        assert_eq!(outcome, EncounterOutcome::DefendingPlayerWins);
        assert_eq!(
            state
                .player_a
                .magical_girls
                .main
                .failed_final_climax_power_bonus,
            1
        );
        assert_eq!(state.player_a.magical_girls.main.current_power(), 3);
        assert!(
            !state
                .player_a
                .magical_girls
                .main
                .exhausted_until_next_encounter
        );
    }

    #[test]
    fn failed_final_climax_power_stacks_across_attempts() {
        let mut state = sample_state();
        state.phase = MatchPhase::FinalClimax;
        state.final_climax_active = true;
        state.player_a.magical_girls.main.stage = CharacterStage::Radiant;
        state.player_b.magical_girls.main.stage = CharacterStage::Radiant;
        state
            .player_a
            .magical_girls
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = false);
        state
            .player_b
            .baddies
            .supports
            .iter_mut()
            .for_each(|support| support.revealed = false);
        state.player_a.magical_girls.main.final_power = 2;
        state.player_b.baddies.main.base_power = 4;

        let first_outcome = MatchEngine::resolve_encounter(&mut state);
        MatchEngine::resolve_encounter(&mut state);

        assert_eq!(first_outcome, EncounterOutcome::DefendingPlayerWins);
        assert_eq!(
            state
                .player_a
                .magical_girls
                .main
                .failed_final_climax_power_bonus,
            2
        );
        assert_eq!(state.player_a.magical_girls.main.current_power(), 4);
    }
}
