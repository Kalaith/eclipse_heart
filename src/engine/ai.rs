//! Simple AI for the offline prototype.

use crate::data::CardSpeed;
use crate::state::{MatchPhase, MatchState, PlayerId};

use super::MatchAction;

pub struct AiController;

impl AiController {
    pub fn choose_action(state: &MatchState) -> Option<MatchAction> {
        Self::choose_action_for(state, PlayerId::PlayerB)
    }

    pub fn choose_action_for(state: &MatchState, player: PlayerId) -> Option<MatchAction> {
        if state.reaction_priority_player() == Some(player) {
            return reaction_action(state, player);
        }

        if state.proactive_priority_player() != Some(player) {
            return None;
        }

        match state.phase {
            MatchPhase::DailyLife => daily_life_action(state, player),
            MatchPhase::Encounter | MatchPhase::FinalClimax => encounter_action(state, player),
            MatchPhase::Finished => None,
        }
    }
}

fn reaction_action(state: &MatchState, player: PlayerId) -> Option<MatchAction> {
    if let Some(hand_index) = state.first_playable_hand_index(player, CardSpeed::Reaction) {
        Some(MatchAction::PlayCardFromHand { player, hand_index })
    } else if state.can_reveal_support(player, true) {
        Some(MatchAction::RevealFirstHiddenSupport {
            player,
            is_magical_girl_side: true,
        })
    } else if state.can_reveal_support(player, false) {
        Some(MatchAction::RevealFirstHiddenSupport {
            player,
            is_magical_girl_side: false,
        })
    } else {
        Some(MatchAction::PassReaction { player })
    }
}

fn daily_life_action(state: &MatchState, player: PlayerId) -> Option<MatchAction> {
    if let Some(hand_index) = state.first_playable_hand_index(player, CardSpeed::DailyLife) {
        Some(MatchAction::PlayCardFromHand { player, hand_index })
    } else if state.can_reveal_support(player, true) {
        Some(MatchAction::RevealFirstHiddenSupport {
            player,
            is_magical_girl_side: true,
        })
    } else if state.can_reveal_support(player, false) {
        Some(MatchAction::RevealFirstHiddenSupport {
            player,
            is_magical_girl_side: false,
        })
    } else {
        Some(MatchAction::PassDailyLife { player })
    }
}

fn encounter_action(state: &MatchState, player: PlayerId) -> Option<MatchAction> {
    if !state.player_for(player).encounter_card_played {
        if !state.final_climax_active && crate::engine::MatchEngine::can_declare_final_climax(state)
        {
            return Some(MatchAction::DeclareFinalClimax);
        }

        if let Some(hand_index) = state.first_playable_hand_index(player, CardSpeed::Encounter) {
            return Some(MatchAction::PlayCardFromHand { player, hand_index });
        }

        if state.can_reveal_support(player, true) {
            return Some(MatchAction::RevealFirstHiddenSupport {
                player,
                is_magical_girl_side: true,
            });
        }

        if state.can_reveal_support(player, false) {
            return Some(MatchAction::RevealFirstHiddenSupport {
                player,
                is_magical_girl_side: false,
            });
        }
    }

    Some(MatchAction::PassEncounter { player })
}
