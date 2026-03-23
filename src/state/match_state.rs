//! Deterministic runtime state for a match.

use std::collections::HashMap;

use serde::Serialize;

use crate::data::{
    CardSpeed, CharacterDefinition, GameContent, MatchRules, StarterLoadout, StoryCardDefinition,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchPhase {
    DailyLife,
    Encounter,
    FinalClimax,
    Finished,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharacterStage {
    Base,
    Transformed,
    Radiant,
    Awakened,
    Catastrophe,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceKind {
    Radiance,
    Dread,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum PlayerId {
    PlayerA,
    PlayerB,
}

#[derive(Clone, Debug)]
pub enum StackItemKind {
    PlayCard {
        card_id: String,
        card_name: String,
    },
    RevealSupport {
        is_magical_girl_side: bool,
        support_index: usize,
    },
}

#[derive(Clone, Debug)]
pub struct StackItem {
    pub player: PlayerId,
    pub is_reaction: bool,
    pub resolves_in_phase: MatchPhase,
    pub kind: StackItemKind,
}

#[derive(Clone, Debug)]
pub struct ReactionState {
    pub priority_player: PlayerId,
    pub passes_in_row: u8,
}

#[derive(Clone, Debug)]
pub struct CharacterRuntimeState {
    pub name: String,
    pub stage: CharacterStage,
    pub base_power: i32,
    pub transformed_power: i32,
    pub final_power: i32,
    pub temporary_power_bonus: i32,
    pub next_encounter_power_bonus: i32,
    pub failed_final_climax_power_bonus: i32,
    pub radiance: i32,
    pub dread: i32,
    pub first_threshold: i32,
    pub second_threshold: i32,
    pub exhausted_until_next_encounter: bool,
    pub abilities_blocked_until_next_encounter: bool,
}

impl CharacterRuntimeState {
    pub fn from_definition(definition: &CharacterDefinition) -> Self {
        Self {
            name: definition.name.clone(),
            stage: CharacterStage::Base,
            base_power: definition.base_power,
            transformed_power: definition.transformed_power,
            final_power: definition.final_power,
            temporary_power_bonus: 0,
            next_encounter_power_bonus: 0,
            failed_final_climax_power_bonus: 0,
            radiance: 0,
            dread: 0,
            first_threshold: definition.first_threshold,
            second_threshold: definition.second_threshold,
            exhausted_until_next_encounter: false,
            abilities_blocked_until_next_encounter: false,
        }
    }

    pub fn current_power(&self) -> i32 {
        if self.exhausted_until_next_encounter {
            return 0;
        }

        let base = match self.stage {
            CharacterStage::Base => self.base_power,
            CharacterStage::Transformed | CharacterStage::Awakened => self.transformed_power,
            CharacterStage::Radiant | CharacterStage::Catastrophe => self.final_power,
        };

        base + self.temporary_power_bonus + self.failed_final_climax_power_bonus
    }

    pub fn gain(&mut self, resource: ResourceKind, amount: i32) {
        match resource {
            ResourceKind::Radiance => {
                self.radiance = (self.radiance + amount).max(0);
                self.check_upgrade();
            }
            ResourceKind::Dread => {
                self.dread = (self.dread + amount).max(0);
                self.check_upgrade();
            }
        }
    }

    fn check_upgrade(&mut self) {
        match self.stage {
            CharacterStage::Base => {
                if self.radiance >= self.first_threshold {
                    self.stage = CharacterStage::Transformed;
                    self.radiance = 0;
                }
                if self.dread >= self.first_threshold {
                    self.stage = CharacterStage::Awakened;
                    self.dread = 0;
                }
            }
            CharacterStage::Transformed => {
                if self.radiance >= self.second_threshold {
                    self.stage = CharacterStage::Radiant;
                    self.radiance = 0;
                }
            }
            CharacterStage::Awakened => {
                if self.dread >= self.second_threshold {
                    self.stage = CharacterStage::Catastrophe;
                    self.dread = 0;
                }
            }
            CharacterStage::Radiant | CharacterStage::Catastrophe => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct SupportState {
    pub runtime: CharacterRuntimeState,
    pub revealed: bool,
}

impl SupportState {
    pub fn gain(&mut self, resource: ResourceKind, amount: i32) {
        self.runtime.gain(resource, amount);
    }
}

#[derive(Clone, Debug)]
pub struct SideState {
    pub main_character_id: String,
    pub main: CharacterRuntimeState,
    pub supports: Vec<SupportState>,
}

impl SideState {
    pub fn total_power(&self) -> i32 {
        let mut power = self.main.current_power();

        for support in &self.supports {
            if support.revealed && !support.runtime.exhausted_until_next_encounter {
                power += support.runtime.current_power();
            }
        }

        power
    }
}

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub magical_girls: SideState,
    pub baddies: SideState,
    pub deck: Vec<String>,
    pub hand: Vec<String>,
    pub discard: Vec<String>,
    pub encounter_card_played: bool,
    pub supports_revealed_this_round: u8,
}

#[derive(Clone, Debug)]
pub struct MatchState {
    pub rules: MatchRules,
    pub story_cards: HashMap<String, StoryCardDefinition>,
    pub phase: MatchPhase,
    pub round: u32,
    pub final_climax_active: bool,
    pub prime_baddie_defeated: bool,
    pub defeated_prime_owner: Option<PlayerId>,
    pub winner: Option<PlayerId>,
    pub player_a: PlayerState,
    pub player_b: PlayerState,
    pub active_player: PlayerId,
    pub priority_player: PlayerId,
    pub phase_passes: u8,
    pub last_played_card_name: Option<String>,
    pub last_outcome: Option<crate::engine::EncounterOutcome>,
    pub reaction_stack: Vec<StackItem>,
    pub reaction_state: Option<ReactionState>,
    pub event_log: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct MatchSetup {
    pub player_a_mg_main_index: usize,
    pub player_a_mg_support_pair_index: usize,
    pub player_a_baddie_main_index: usize,
    pub player_a_baddie_support_pair_index: usize,
    pub player_b_mg_main_index: usize,
    pub player_b_mg_support_pair_index: usize,
    pub player_b_baddie_main_index: usize,
    pub player_b_baddie_support_pair_index: usize,
}

impl MatchState {
    pub fn from_setup(content: &GameContent, setup: &MatchSetup) -> Self {
        Self::from_setup_with_options(
            content,
            setup,
            default_player_a_deck(),
            default_player_b_deck(),
            PlayerId::PlayerA,
        )
    }

    pub fn from_starter_loadouts(
        content: &GameContent,
        player_a: &StarterLoadout,
        player_b: &StarterLoadout,
        starting_player: PlayerId,
    ) -> Self {
        let setup = MatchSetup::from_starter_loadouts(content, player_a, player_b);
        Self::from_setup_with_options(
            content,
            &setup,
            player_a.support_deck.clone(),
            player_b.support_deck.clone(),
            starting_player,
        )
    }

    pub fn from_setup_with_options(
        content: &GameContent,
        setup: &MatchSetup,
        mut player_a_deck: Vec<String>,
        mut player_b_deck: Vec<String>,
        starting_player: PlayerId,
    ) -> Self {
        let story_cards = content
            .story_cards
            .iter()
            .cloned()
            .map(|card| (card.id.clone(), card))
            .collect::<HashMap<_, _>>();

        let player_a_hand = draw_opening_hand(&mut player_a_deck, 4);
        let player_b_hand = draw_opening_hand(&mut player_b_deck, 4);

        let mut state = Self {
            rules: content.rules.clone(),
            story_cards,
            phase: MatchPhase::DailyLife,
            round: 1,
            final_climax_active: false,
            prime_baddie_defeated: false,
            defeated_prime_owner: None,
            winner: None,
            player_a: PlayerState {
                magical_girls: build_side(
                    &content.magical_girls,
                    setup.player_a_mg_main_index,
                    setup.player_a_mg_support_pair_index,
                ),
                baddies: build_side(
                    &content.baddies,
                    setup.player_a_baddie_main_index,
                    setup.player_a_baddie_support_pair_index,
                ),
                deck: player_a_deck,
                hand: player_a_hand,
                discard: Vec::new(),
                encounter_card_played: false,
                supports_revealed_this_round: 0,
            },
            player_b: PlayerState {
                magical_girls: build_side(
                    &content.magical_girls,
                    setup.player_b_mg_main_index,
                    setup.player_b_mg_support_pair_index,
                ),
                baddies: build_side(
                    &content.baddies,
                    setup.player_b_baddie_main_index,
                    setup.player_b_baddie_support_pair_index,
                ),
                deck: player_b_deck,
                hand: player_b_hand,
                discard: Vec::new(),
                encounter_card_played: false,
                supports_revealed_this_round: 0,
            },
            active_player: starting_player,
            priority_player: starting_player,
            phase_passes: 0,
            last_played_card_name: None,
            last_outcome: None,
            reaction_stack: Vec::new(),
            reaction_state: None,
            event_log: Vec::new(),
        };

        state.draw_round_start_cards();
        state
    }

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

    pub fn ready_end_of_round(&mut self) {
        self.discard_down_to_hand_limit(7);

        for player in [PlayerId::PlayerA, PlayerId::PlayerB] {
            ready_side_for_next_round(&mut self.player_for_mut(player).magical_girls);
            ready_side_for_next_round(&mut self.player_for_mut(player).baddies);
            self.player_for_mut(player).encounter_card_played = false;
            self.player_for_mut(player).supports_revealed_this_round = 0;
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
            clear_side_encounter_bonuses(&mut self.player_for_mut(player).magical_girls);
            clear_side_encounter_bonuses(&mut self.player_for_mut(player).baddies);
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
        let player_a_drew = self.draw_cards(PlayerId::PlayerA, 1);
        let player_b_drew = self.draw_cards(PlayerId::PlayerB, 1);
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
            let overflow = self
                .player_for(player)
                .hand
                .len()
                .saturating_sub(max_hand_size);
            if overflow == 0 {
                continue;
            }

            let discarded = self
                .hand_for_mut(player)
                .drain(0..overflow)
                .collect::<Vec<_>>();
            self.discard_for_mut(player).extend(discarded);
            self.push_event(format!(
                "{player:?} discarded {} card(s) down to {}.",
                overflow, max_hand_size
            ));
        }
    }
}

impl MatchSetup {
    pub fn default_for_content(content: &GameContent) -> Self {
        let player_a_mg_main_index = preferred_index(&content.magical_girls, "yuki");
        let player_a_baddie_main_index = preferred_index(&content.baddies, "noctra");
        let player_b_mg_main_index = preferred_index(&content.magical_girls, "hana");
        let player_b_baddie_main_index = preferred_index(&content.baddies, "velvet_hex");

        Self {
            player_a_mg_main_index,
            player_a_mg_support_pair_index: support_pair_index_for_main(
                &content.magical_girls,
                player_a_mg_main_index,
                ["hana", "riri"],
            ),
            player_a_baddie_main_index,
            player_a_baddie_support_pair_index: support_pair_index_for_main(
                &content.baddies,
                player_a_baddie_main_index,
                ["glass_crow", "thorn_waltz"],
            ),
            player_b_mg_main_index,
            player_b_mg_support_pair_index: support_pair_index_for_main(
                &content.magical_girls,
                player_b_mg_main_index,
                ["momo", "kiko"],
            ),
            player_b_baddie_main_index,
            player_b_baddie_support_pair_index: support_pair_index_for_main(
                &content.baddies,
                player_b_baddie_main_index,
                ["noctra", "hollow_marionette"],
            ),
        }
    }

    pub fn from_starter_loadouts(
        content: &GameContent,
        player_a: &StarterLoadout,
        player_b: &StarterLoadout,
    ) -> Self {
        Self {
            player_a_mg_main_index: required_index(
                &content.magical_girls,
                &player_a.magical_girl_main,
            ),
            player_a_mg_support_pair_index: support_pair_index_for_ids(
                &content.magical_girls,
                &player_a.magical_girl_main,
                &player_a.magical_girl_supports,
            ),
            player_a_baddie_main_index: required_index(&content.baddies, &player_a.prime_baddie),
            player_a_baddie_support_pair_index: support_pair_index_for_ids(
                &content.baddies,
                &player_a.prime_baddie,
                &player_a.baddie_supports,
            ),
            player_b_mg_main_index: required_index(
                &content.magical_girls,
                &player_b.magical_girl_main,
            ),
            player_b_mg_support_pair_index: support_pair_index_for_ids(
                &content.magical_girls,
                &player_b.magical_girl_main,
                &player_b.magical_girl_supports,
            ),
            player_b_baddie_main_index: required_index(&content.baddies, &player_b.prime_baddie),
            player_b_baddie_support_pair_index: support_pair_index_for_ids(
                &content.baddies,
                &player_b.prime_baddie,
                &player_b.baddie_supports,
            ),
        }
    }

    pub fn cycle_player_a_mg_main(&mut self, content: &GameContent) {
        self.player_a_mg_main_index =
            cycle_index(content.magical_girls.len(), self.player_a_mg_main_index);
        self.player_a_mg_support_pair_index = reset_pair_index_if_needed(
            &content.magical_girls,
            self.player_a_mg_main_index,
            self.player_a_mg_support_pair_index,
        );
    }

    pub fn cycle_player_a_mg_supports(&mut self, content: &GameContent) {
        self.player_a_mg_support_pair_index = cycle_pair_index(
            &content.magical_girls,
            self.player_a_mg_main_index,
            self.player_a_mg_support_pair_index,
        );
    }

    pub fn cycle_player_a_baddie_main(&mut self, content: &GameContent) {
        self.player_a_baddie_main_index =
            cycle_index(content.baddies.len(), self.player_a_baddie_main_index);
        self.player_a_baddie_support_pair_index = reset_pair_index_if_needed(
            &content.baddies,
            self.player_a_baddie_main_index,
            self.player_a_baddie_support_pair_index,
        );
    }

    pub fn cycle_player_a_baddie_supports(&mut self, content: &GameContent) {
        self.player_a_baddie_support_pair_index = cycle_pair_index(
            &content.baddies,
            self.player_a_baddie_main_index,
            self.player_a_baddie_support_pair_index,
        );
    }

    pub fn cycle_player_b_mg_main(&mut self, content: &GameContent) {
        self.player_b_mg_main_index =
            cycle_index(content.magical_girls.len(), self.player_b_mg_main_index);
        self.player_b_mg_support_pair_index = reset_pair_index_if_needed(
            &content.magical_girls,
            self.player_b_mg_main_index,
            self.player_b_mg_support_pair_index,
        );
    }

    pub fn cycle_player_b_mg_supports(&mut self, content: &GameContent) {
        self.player_b_mg_support_pair_index = cycle_pair_index(
            &content.magical_girls,
            self.player_b_mg_main_index,
            self.player_b_mg_support_pair_index,
        );
    }

    pub fn cycle_player_b_baddie_main(&mut self, content: &GameContent) {
        self.player_b_baddie_main_index =
            cycle_index(content.baddies.len(), self.player_b_baddie_main_index);
        self.player_b_baddie_support_pair_index = reset_pair_index_if_needed(
            &content.baddies,
            self.player_b_baddie_main_index,
            self.player_b_baddie_support_pair_index,
        );
    }

    pub fn cycle_player_b_baddie_supports(&mut self, content: &GameContent) {
        self.player_b_baddie_support_pair_index = cycle_pair_index(
            &content.baddies,
            self.player_b_baddie_main_index,
            self.player_b_baddie_support_pair_index,
        );
    }

    pub fn player_a_mg_main_name<'a>(&self, content: &'a GameContent) -> &'a str {
        lookup_name(&content.magical_girls, self.player_a_mg_main_index)
    }

    pub fn player_a_mg_support_names(&self, content: &GameContent) -> Vec<String> {
        support_names(
            &content.magical_girls,
            self.player_a_mg_main_index,
            self.player_a_mg_support_pair_index,
        )
    }

    pub fn player_a_baddie_main_name<'a>(&self, content: &'a GameContent) -> &'a str {
        lookup_name(&content.baddies, self.player_a_baddie_main_index)
    }

    pub fn player_a_baddie_support_names(&self, content: &GameContent) -> Vec<String> {
        support_names(
            &content.baddies,
            self.player_a_baddie_main_index,
            self.player_a_baddie_support_pair_index,
        )
    }

    pub fn player_b_mg_main_name<'a>(&self, content: &'a GameContent) -> &'a str {
        lookup_name(&content.magical_girls, self.player_b_mg_main_index)
    }

    pub fn player_b_mg_support_names(&self, content: &GameContent) -> Vec<String> {
        support_names(
            &content.magical_girls,
            self.player_b_mg_main_index,
            self.player_b_mg_support_pair_index,
        )
    }

    pub fn player_b_baddie_main_name<'a>(&self, content: &'a GameContent) -> &'a str {
        lookup_name(&content.baddies, self.player_b_baddie_main_index)
    }

    pub fn player_b_baddie_support_names(&self, content: &GameContent) -> Vec<String> {
        support_names(
            &content.baddies,
            self.player_b_baddie_main_index,
            self.player_b_baddie_support_pair_index,
        )
    }
}

fn build_side(
    definitions: &[CharacterDefinition],
    main_index: usize,
    support_pair_index: usize,
) -> SideState {
    let main_definition = &definitions[main_index];
    let support_indices = support_pairs(definitions.len(), main_index)
        .get(support_pair_index)
        .cloned()
        .unwrap_or_default();

    SideState {
        main_character_id: main_definition.id.clone(),
        main: CharacterRuntimeState::from_definition(main_definition),
        supports: support_indices
            .into_iter()
            .enumerate()
            .map(|(index, support_index)| {
                let definition = &definitions[support_index];
                SupportState {
                    runtime: CharacterRuntimeState::from_definition(definition),
                    revealed: index == 0,
                }
            })
            .collect(),
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

fn preferred_index(definitions: &[CharacterDefinition], preferred_id: &str) -> usize {
    definitions
        .iter()
        .position(|character| character.id == preferred_id)
        .unwrap_or(0)
}

fn required_index(definitions: &[CharacterDefinition], character_id: &str) -> usize {
    definitions
        .iter()
        .position(|character| character.id == character_id)
        .expect("starter loadout references missing character")
}

fn support_pair_index_for_main(
    definitions: &[CharacterDefinition],
    main_index: usize,
    preferred_ids: [&str; 2],
) -> usize {
    let pairs = support_pairs(definitions.len(), main_index);
    let preferred = preferred_ids
        .iter()
        .filter_map(|preferred_id| {
            definitions
                .iter()
                .position(|character| character.id == *preferred_id)
        })
        .collect::<Vec<_>>();

    pairs
        .iter()
        .position(|pair| pair == &preferred)
        .unwrap_or(0)
}

fn support_pair_index_for_ids(
    definitions: &[CharacterDefinition],
    main_id: &str,
    support_ids: &[String],
) -> usize {
    let main_index = required_index(definitions, main_id);
    let required_pair = support_ids
        .iter()
        .map(|support_id| required_index(definitions, support_id))
        .collect::<Vec<_>>();
    let pairs = support_pairs(definitions.len(), main_index);

    pairs
        .iter()
        .position(|pair| pair == &required_pair)
        .expect("starter loadout references invalid support pair")
}

fn cycle_index(len: usize, current: usize) -> usize {
    if len == 0 {
        0
    } else {
        (current + 1) % len
    }
}

fn cycle_pair_index(
    definitions: &[CharacterDefinition],
    main_index: usize,
    current: usize,
) -> usize {
    let pair_count = support_pairs(definitions.len(), main_index).len();
    if pair_count == 0 {
        0
    } else {
        (current + 1) % pair_count
    }
}

fn reset_pair_index_if_needed(
    definitions: &[CharacterDefinition],
    main_index: usize,
    current: usize,
) -> usize {
    let pair_count = support_pairs(definitions.len(), main_index).len();
    if current < pair_count {
        current
    } else {
        0
    }
}

fn support_names(
    definitions: &[CharacterDefinition],
    main_index: usize,
    support_pair_index: usize,
) -> Vec<String> {
    let pair = support_pairs(definitions.len(), main_index)
        .get(support_pair_index)
        .cloned()
        .unwrap_or_default();

    pair.into_iter()
        .filter_map(|index| definitions.get(index))
        .map(|definition| definition.name.clone())
        .collect()
}

fn lookup_name(definitions: &[CharacterDefinition], index: usize) -> &str {
    definitions
        .get(index)
        .map(|character| character.name.as_str())
        .unwrap_or("Unknown")
}

fn support_pairs(len: usize, main_index: usize) -> Vec<Vec<usize>> {
    let candidates = (0..len)
        .filter(|index| *index != main_index)
        .collect::<Vec<_>>();
    let mut pairs = Vec::new();

    for first in 0..candidates.len() {
        for second in (first + 1)..candidates.len() {
            pairs.push(vec![candidates[first], candidates[second]]);
        }
    }

    pairs
}

fn draw_opening_hand(deck: &mut Vec<String>, count: usize) -> Vec<String> {
    let draw_count = count.min(deck.len());
    deck.drain(0..draw_count).collect()
}

fn default_player_a_deck() -> Vec<String> {
    vec![
        "quiet_lunch_on_the_rooftop",
        "false_sense_of_calm",
        "secret_rehearsal",
        "not_on_my_watch",
        "shared_umbrella",
        "emergency_costume_repair",
        "crescent_counterpose",
        "moonlit_rescue_chain",
        "break_the_formation",
        "identity_reveal",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn default_player_b_deck() -> Vec<String> {
    vec![
        "ominous_graffiti",
        "whisper_campaign",
        "twisted_headline",
        "expose_the_accomplice",
        "curtain_call_denied",
        "last_minute_cover_story",
        "panic_spiral",
        "panic_in_the_parade_route",
        "forbidden_bloom_of_calamity",
        "break_the_formation",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

pub fn opposing(player: PlayerId) -> PlayerId {
    match player {
        PlayerId::PlayerA => PlayerId::PlayerB,
        PlayerId::PlayerB => PlayerId::PlayerA,
    }
}

#[cfg(test)]
mod tests {
    use crate::data::GameContent;

    use super::{support_pairs, MatchSetup, MatchState, PlayerId};

    #[test]
    fn support_pairs_exclude_the_selected_main() {
        let pairs = support_pairs(5, 1);
        assert!(!pairs.iter().any(|pair| pair.contains(&1)));
        assert_eq!(pairs.len(), 6);
    }

    #[test]
    fn setup_builds_hidden_supports_for_both_players() {
        let content = GameContent::load().unwrap_or_default();
        let setup = MatchSetup::default_for_content(&content);
        let state = MatchState::from_setup(&content, &setup);

        assert_eq!(state.player_a.magical_girls.main_character_id, "yuki");
        assert_eq!(state.player_b.baddies.main_character_id, "velvet_hex");
        assert_eq!(state.player_a.magical_girls.supports.len(), 2);
        assert!(state.player_a.magical_girls.supports[0].revealed);
        assert!(!state.player_a.magical_girls.supports[1].revealed);
        assert_eq!(state.active_player, PlayerId::PlayerA);
        assert_eq!(state.player_a.hand.len(), 5);
        assert_eq!(state.player_b.hand.len(), 5);
    }

    #[test]
    fn end_of_round_discards_down_to_seven_cards() {
        let content = GameContent::load().unwrap_or_default();
        let setup = MatchSetup::default_for_content(&content);
        let mut state = MatchState::from_setup(&content, &setup);

        state.player_a.hand.extend([
            "identity_reveal".to_string(),
            "break_the_formation".to_string(),
            "secret_rehearsal".to_string(),
            "crescent_counterpose".to_string(),
        ]);
        state.player_b.hand.extend([
            "ominous_graffiti".to_string(),
            "break_the_formation".to_string(),
            "panic_spiral".to_string(),
            "panic_in_the_parade_route".to_string(),
        ]);
        state.ready_end_of_round();

        assert_eq!(state.player_a.hand.len(), 8);
        assert_eq!(state.player_b.hand.len(), 8);
        assert!(!state.player_a.discard.is_empty());
        assert!(!state.player_b.discard.is_empty());
    }
}
