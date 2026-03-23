use super::*;

impl MatchState {
    pub fn ready_end_of_round(&mut self) {
        self.discard_down_to_hand_limit(7);

        for player in [PlayerId::PlayerA, PlayerId::PlayerB] {
            let player_state = self.player_for_mut(player);
            ready_side_for_next_round(&mut player_state.magical_girls);
            ready_side_for_next_round(&mut player_state.baddies);
            player_state.encounter_card_played = false;
            player_state.supports_revealed_this_round = 0;
        }

        self.active_player = opposing(self.active_player);
        self.priority_player = self.active_player;
        self.phase_passes = 0;
        self.reaction_stack.clear();
        self.reaction_state = None;

        if self.phase == MatchPhase::DailyLife {
            self.draw_round_start_cards();
        }
    }

    pub fn clear_encounter_bonuses(&mut self) {
        for player in [PlayerId::PlayerA, PlayerId::PlayerB] {
            let player_state = self.player_for_mut(player);
            clear_side_encounter_bonuses(&mut player_state.magical_girls);
            clear_side_encounter_bonuses(&mut player_state.baddies);
        }
    }

    pub fn push_event(&mut self, event: impl Into<String>) {
        self.event_log.push(event.into());
        if self.event_log.len() > 8 {
            let overflow = self.event_log.len() - 8;
            self.event_log.drain(0..overflow);
        }
    }

    fn draw_round_start_cards(&mut self) {
        let player_a_drew = self.draw_cards(PlayerId::PlayerA, 2);
        let player_b_drew = self.draw_cards(PlayerId::PlayerB, 2);
        self.push_event(format!(
            "Round {} draw: PlayerA +{}, PlayerB +{}.",
            self.round, player_a_drew, player_b_drew
        ));
    }

    fn draw_cards(&mut self, player: PlayerId, count: usize) -> usize {
        let draw_count = count.min(self.player_for(player).deck.len());
        if draw_count == 0 {
            return 0;
        }

        let drawn_cards = self
            .deck_for_mut(player)
            .drain(0..draw_count)
            .collect::<Vec<_>>();
        self.hand_for_mut(player).extend(drawn_cards);
        draw_count
    }

    fn discard_down_to_hand_limit(&mut self, max_hand_size: usize) {
        for player in [PlayerId::PlayerA, PlayerId::PlayerB] {
            let discarded_count = {
                let player_state = self.player_for_mut(player);
                discard_player_down_to_hand_limit(
                    &mut player_state.hand,
                    &mut player_state.discard,
                    max_hand_size,
                )
            };

            if discarded_count > 0 {
                self.push_event(format!(
                    "{player:?} discarded {} card(s) down to {}.",
                    discarded_count, max_hand_size
                ));
            }
        }
    }
}

fn ready_side_for_next_round(side: &mut SideState) {
    side.main.temporary_power_bonus = side.main.next_encounter_power_bonus;
    side.main.next_encounter_power_bonus = 0;
    side.main.exhausted_until_next_encounter = false;
    side.main.abilities_blocked_until_next_encounter = false;

    for support in &mut side.supports {
        support.runtime.temporary_power_bonus = support.runtime.next_encounter_power_bonus;
        support.runtime.next_encounter_power_bonus = 0;
        support.runtime.exhausted_until_next_encounter = false;
        support.runtime.abilities_blocked_until_next_encounter = false;
    }
}

fn clear_side_encounter_bonuses(side: &mut SideState) {
    side.main.temporary_power_bonus = 0;

    for support in &mut side.supports {
        support.runtime.temporary_power_bonus = 0;
    }
}

pub(super) fn discard_player_down_to_hand_limit(
    hand: &mut Vec<String>,
    discard: &mut Vec<String>,
    max_hand_size: usize,
) -> usize {
    let overflow = hand.len().saturating_sub(max_hand_size);
    if overflow == 0 {
        return 0;
    }

    let discarded = hand.drain(0..overflow).collect::<Vec<_>>();
    discard.extend(discarded);
    overflow
}
