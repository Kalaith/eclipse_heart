use super::*;

impl MatchState {
    pub fn has_hidden_support(&self, player: PlayerId, is_magical_girl_side: bool) -> bool {
        self.side_for(player, is_magical_girl_side)
            .supports
            .iter()
            .any(|support| !support.revealed)
    }

    pub fn can_reveal_support(&self, player: PlayerId, is_magical_girl_side: bool) -> bool {
        self.player_for(player).supports_revealed_this_round == 0
            && !self.has_pending_support_reveal_any(player)
            && self.has_hidden_support(player, is_magical_girl_side)
    }

    pub fn has_pending_support_reveal(&self, player: PlayerId, is_magical_girl_side: bool) -> bool {
        self.reaction_stack.iter().any(|item| {
            item.player == player
                && matches!(
                    item.kind,
                    StackItemKind::RevealSupport {
                        is_magical_girl_side: queued_side,
                        ..
                    } if queued_side == is_magical_girl_side
                )
        })
    }

    pub fn has_pending_support_reveal_any(&self, player: PlayerId) -> bool {
        self.reaction_stack.iter().any(|item| {
            item.player == player && matches!(item.kind, StackItemKind::RevealSupport { .. })
        })
    }

    pub fn mark_support_revealed_this_round(&mut self, player: PlayerId) {
        self.player_for_mut(player).supports_revealed_this_round += 1;
    }

    pub fn reaction_priority_player(&self) -> Option<PlayerId> {
        self.reaction_state
            .as_ref()
            .map(|state| state.priority_player)
    }

    pub fn proactive_priority_player(&self) -> Option<PlayerId> {
        if self.reaction_state.is_none() && self.phase != MatchPhase::Finished {
            Some(self.priority_player)
        } else {
            None
        }
    }

    pub fn player_for(&self, player: PlayerId) -> &PlayerState {
        match player {
            PlayerId::PlayerA => &self.player_a,
            PlayerId::PlayerB => &self.player_b,
        }
    }

    pub fn player_for_mut(&mut self, player: PlayerId) -> &mut PlayerState {
        match player {
            PlayerId::PlayerA => &mut self.player_a,
            PlayerId::PlayerB => &mut self.player_b,
        }
    }

    pub fn encounter_card_played(&self, player: PlayerId) -> bool {
        self.player_for(player).encounter_card_played
    }

    pub fn set_encounter_card_played(&mut self, player: PlayerId, value: bool) {
        self.player_for_mut(player).encounter_card_played = value;
    }

    pub fn side_for(&self, player: PlayerId, is_magical_girl_side: bool) -> &SideState {
        let player_state = self.player_for(player);
        if is_magical_girl_side {
            &player_state.magical_girls
        } else {
            &player_state.baddies
        }
    }

    pub fn side_for_mut(&mut self, player: PlayerId, is_magical_girl_side: bool) -> &mut SideState {
        let player_state = self.player_for_mut(player);
        if is_magical_girl_side {
            &mut player_state.magical_girls
        } else {
            &mut player_state.baddies
        }
    }

    pub fn active_magical_girls(&self) -> &SideState {
        &self.player_for(self.active_player).magical_girls
    }

    pub fn active_magical_girls_mut(&mut self) -> &mut SideState {
        &mut self.player_for_mut(self.active_player).magical_girls
    }

    pub fn defending_baddies(&self) -> &SideState {
        &self.player_for(opposing(self.active_player)).baddies
    }

    pub fn hand_for(&self, player: PlayerId) -> &Vec<String> {
        &self.player_for(player).hand
    }

    pub fn hand_for_mut(&mut self, player: PlayerId) -> &mut Vec<String> {
        &mut self.player_for_mut(player).hand
    }

    pub fn deck_for_mut(&mut self, player: PlayerId) -> &mut Vec<String> {
        &mut self.player_for_mut(player).deck
    }

    pub fn discard_for_mut(&mut self, player: PlayerId) -> &mut Vec<String> {
        &mut self.player_for_mut(player).discard
    }

    pub fn card_in_hand(
        &self,
        player: PlayerId,
        hand_index: usize,
    ) -> Option<&StoryCardDefinition> {
        let card_id = self.hand_for(player).get(hand_index)?;
        self.story_cards.get(card_id)
    }

    pub fn first_playable_hand_index(
        &self,
        player: PlayerId,
        desired_speed: CardSpeed,
    ) -> Option<usize> {
        self.hand_for(player).iter().position(|card_id| {
            self.story_cards
                .get(card_id)
                .map(|card| card.speed == desired_speed)
                .unwrap_or(false)
        })
    }

    pub fn expected_hand_speed(&self, player: PlayerId) -> Option<CardSpeed> {
        if self.phase == MatchPhase::Finished {
            return None;
        }

        if self.reaction_priority_player() == Some(player) {
            return Some(CardSpeed::Reaction);
        }

        if self.reaction_state.is_some() || self.priority_player != player {
            return None;
        }

        match self.phase {
            MatchPhase::DailyLife => Some(CardSpeed::DailyLife),
            MatchPhase::Encounter | MatchPhase::FinalClimax => {
                if self.encounter_card_played(player) {
                    None
                } else {
                    Some(CardSpeed::Encounter)
                }
            }
            MatchPhase::Finished => None,
        }
    }

    pub fn can_play_hand_card(&self, player: PlayerId, hand_index: usize) -> bool {
        let Some(expected_speed) = self.expected_hand_speed(player) else {
            return false;
        };

        self.card_in_hand(player, hand_index)
            .map(|card| card.speed == expected_speed)
            .unwrap_or(false)
    }

    pub fn current_phase_label(&self) -> &'static str {
        match self.phase {
            MatchPhase::DailyLife => "DailyLife",
            MatchPhase::Encounter => "Encounter",
            MatchPhase::FinalClimax => "FinalClimax",
            MatchPhase::Finished => "Finished",
        }
    }
}
