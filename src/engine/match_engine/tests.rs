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
fn declaring_final_climax_updates_phase_and_event_log() {
    let mut state = sample_state();
    state.phase = MatchPhase::Encounter;
    state.player_a.magical_girls.main.stage = CharacterStage::Radiant;

    MatchEngine::apply_action(&mut state, MatchAction::DeclareFinalClimax);

    assert_eq!(state.phase, MatchPhase::FinalClimax);
    assert!(state.final_climax_active);
    assert!(state
        .event_log
        .iter()
        .any(|event| event.contains("declared Final Climax")));
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
    state.priority_player = PlayerId::PlayerB;
    MatchEngine::apply_action(
        &mut state,
        MatchAction::PassEncounter {
            player: PlayerId::PlayerB,
        },
    );
    assert!(state.player_b.encounter_card_played);
}

#[test]
fn pass_daily_life_ignores_non_priority_player() {
    let mut state = sample_state();
    state.phase = MatchPhase::DailyLife;
    state.priority_player = PlayerId::PlayerA;

    MatchEngine::apply_action(
        &mut state,
        MatchAction::PassDailyLife {
            player: PlayerId::PlayerB,
        },
    );

    assert_eq!(state.phase, MatchPhase::DailyLife);
    assert_eq!(state.phase_passes, 0);
    assert_eq!(state.priority_player, PlayerId::PlayerA);
}

#[test]
fn round_start_draws_two_for_both_players_after_encounter() {
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
    assert_eq!(state.player_a.hand.len(), player_a_hand_before + 2);
    assert_eq!(state.player_b.hand.len(), player_b_hand_before + 2);
    assert_eq!(state.player_a.deck.len(), player_a_deck_before - 2);
    assert_eq!(state.player_b.deck.len(), player_b_deck_before - 2);
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
fn reveal_requires_player_priority_when_not_in_reaction_window() {
    let mut state = sample_state();
    state.phase = MatchPhase::DailyLife;
    state.priority_player = PlayerId::PlayerA;
    state.player_b.magical_girls.supports[1].revealed = false;

    MatchEngine::apply_action(
        &mut state,
        MatchAction::RevealFirstHiddenSupport {
            player: PlayerId::PlayerB,
            is_magical_girl_side: true,
        },
    );

    assert!(state.reaction_stack.is_empty());
    assert_eq!(state.player_b.supports_revealed_this_round, 0);
    assert!(!state.player_b.magical_girls.supports[1].revealed);
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
