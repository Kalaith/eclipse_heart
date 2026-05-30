//! Persistent campaign run data.

use serde::{Deserialize, Serialize};

use crate::data::CampaignDefinition;

use super::{timestamp::current_unix_timestamp, DeckPreset};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CampaignRunStatus {
    InProgress,
    Won,
    Lost,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CampaignBattleRecord {
    pub node_id: String,
    pub encounter_id: String,
    pub won: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignRunSave {
    pub id: String,
    pub name: String,
    pub campaign_id: String,
    pub status: CampaignRunStatus,
    pub current_node_id: String,
    #[serde(default)]
    pub completed_node_ids: Vec<String>,
    pub player_deck: DeckPreset,
    #[serde(default)]
    pub selected_magical_girl_support_ids: Vec<String>,
    #[serde(default)]
    pub battle_history: Vec<CampaignBattleRecord>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignSaveBundle {
    pub version: u32,
    #[serde(default)]
    pub runs: Vec<CampaignRunSave>,
    #[serde(default)]
    pub selected_run_id: Option<String>,
}

impl Default for CampaignSaveBundle {
    fn default() -> Self {
        Self {
            version: 2,
            runs: Vec::new(),
            selected_run_id: None,
        }
    }
}

impl CampaignSaveBundle {
    pub fn create_run(
        &mut self,
        campaign: &CampaignDefinition,
        source_deck: &DeckPreset,
        default_name: &str,
    ) -> Option<String> {
        let first_node = campaign.first_node()?;
        let run_id = format!("campaign_run_{}", self.runs.len() + 1);
        let now = current_unix_timestamp();
        let run_name = if source_deck.name.trim().is_empty() {
            default_name.to_owned()
        } else {
            format!("{} Run", source_deck.name)
        };
        self.runs.push(CampaignRunSave {
            id: run_id.clone(),
            name: run_name,
            campaign_id: campaign.id.clone(),
            status: CampaignRunStatus::InProgress,
            current_node_id: first_node.id.clone(),
            completed_node_ids: Vec::new(),
            player_deck: source_deck.clone(),
            selected_magical_girl_support_ids: Vec::new(),
            battle_history: Vec::new(),
            created_at_unix: now,
            updated_at_unix: now,
        });
        self.selected_run_id = Some(run_id.clone());
        Some(run_id)
    }

    pub fn selected_run(&self) -> Option<&CampaignRunSave> {
        let selected_id = self.selected_run_id.as_deref().or_else(|| {
            self.runs
                .iter()
                .rev()
                .find(|run| run.status == CampaignRunStatus::InProgress)
                .or_else(|| self.runs.last())
                .map(|run| run.id.as_str())
        })?;
        self.runs.iter().find(|run| run.id == selected_id)
    }

    pub fn selected_run_mut(&mut self) -> Option<&mut CampaignRunSave> {
        let selected_id = self.selected_run_id.clone().or_else(|| {
            self.runs
                .iter()
                .rev()
                .find(|run| run.status == CampaignRunStatus::InProgress)
                .or_else(|| self.runs.last())
                .map(|run| run.id.clone())
        })?;
        self.runs.iter_mut().find(|run| run.id == selected_id)
    }

    pub fn select_run(&mut self, run_id: &str) -> bool {
        if self.runs.iter().any(|run| run.id == run_id) {
            self.selected_run_id = Some(run_id.to_owned());
            return true;
        }
        false
    }

    pub fn selected_run_is_in_progress(&self) -> bool {
        self.selected_run()
            .map(|run| run.status == CampaignRunStatus::InProgress)
            .unwrap_or(false)
    }

    pub fn selected_run_has_valid_support_pair(&self) -> bool {
        self.selected_run()
            .map(|run| run.selected_magical_girl_support_ids.len() == 2)
            .unwrap_or(false)
    }

    pub fn update_selected_magical_girl_supports(&mut self, support_ids: &[String]) -> bool {
        let Some(run) = self.selected_run_mut() else {
            return false;
        };
        if support_ids.len() != 2 {
            return false;
        }
        run.selected_magical_girl_support_ids = support_ids.to_vec();
        run.updated_at_unix = current_unix_timestamp();
        true
    }

    pub fn toggle_selected_magical_girl_support(&mut self, character_id: &str) -> bool {
        let Some(run) = self.selected_run_mut() else {
            return false;
        };
        if run
            .player_deck
            .magical_girl_roster
            .first()
            .is_some_and(|main_id| main_id == character_id)
        {
            return false;
        }
        if !run
            .player_deck
            .magical_girl_roster
            .iter()
            .any(|entry| entry == character_id)
        {
            return false;
        }

        if let Some(index) = run
            .selected_magical_girl_support_ids
            .iter()
            .position(|entry| entry == character_id)
        {
            run.selected_magical_girl_support_ids.remove(index);
        } else if run.selected_magical_girl_support_ids.len() < 2 {
            run.selected_magical_girl_support_ids
                .push(character_id.to_owned());
        } else {
            return false;
        }

        run.updated_at_unix = current_unix_timestamp();
        true
    }

    pub fn run_by_id(&self, run_id: &str) -> Option<&CampaignRunSave> {
        self.runs.iter().find(|run| run.id == run_id)
    }

    fn run_by_id_mut(&mut self, run_id: &str) -> Option<&mut CampaignRunSave> {
        self.runs.iter_mut().find(|run| run.id == run_id)
    }

    pub fn abandon_selected_run(&mut self) -> bool {
        let Some(run) = self.selected_run_mut() else {
            return false;
        };
        run.status = CampaignRunStatus::Lost;
        run.updated_at_unix = current_unix_timestamp();
        true
    }

    pub fn record_victory_for_run(
        &mut self,
        run_id: &str,
        campaign: &CampaignDefinition,
        node_id: &str,
        reward_card_id: Option<&str>,
    ) -> Option<bool> {
        let run = self.run_by_id_mut(run_id)?;
        let node = campaign.node(node_id)?;
        let encounter = campaign.encounter(&node.encounter_id)?;
        run.completed_node_ids.push(node.id.clone());
        if let Some(card_id) = reward_card_id {
            run.player_deck.story_cards.push(card_id.to_owned());
            run.player_deck.recent_story_cards.push(card_id.to_owned());
            if run.player_deck.recent_story_cards.len() > 5 {
                run.player_deck.recent_story_cards.remove(0);
            }
        }
        run.battle_history.push(CampaignBattleRecord {
            node_id: node.id.clone(),
            encounter_id: encounter.id.clone(),
            won: true,
        });
        run.updated_at_unix = current_unix_timestamp();
        if let Some(next_node_id) = node.next_node_ids.first() {
            run.current_node_id = next_node_id.clone();
            return Some(false);
        }
        run.status = CampaignRunStatus::Won;
        Some(true)
    }

    pub fn record_defeat_for_run(
        &mut self,
        run_id: &str,
        campaign: &CampaignDefinition,
        node_id: &str,
    ) -> bool {
        let Some(run) = self.run_by_id_mut(run_id) else {
            return false;
        };
        let Some(node) = campaign.node(node_id) else {
            return false;
        };
        let Some(encounter) = campaign.encounter(&node.encounter_id) else {
            return false;
        };
        run.battle_history.push(CampaignBattleRecord {
            node_id: node.id.clone(),
            encounter_id: encounter.id.clone(),
            won: false,
        });
        run.status = CampaignRunStatus::Lost;
        run.updated_at_unix = current_unix_timestamp();
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{CampaignDefinition, CampaignEncounterDefinition, CampaignNodeDefinition};
    use crate::state::{CampaignRunStatus, DeckPreset};

    use super::CampaignSaveBundle;

    fn sample_campaign() -> CampaignDefinition {
        CampaignDefinition {
            id: "campaign_alpha".to_owned(),
            name: "Campaign Alpha".to_owned(),
            description: String::new(),
            nodes: vec![
                CampaignNodeDefinition {
                    id: "node_1".to_owned(),
                    encounter_id: "encounter_1".to_owned(),
                    next_node_ids: vec!["node_2".to_owned()],
                    boss: false,
                },
                CampaignNodeDefinition {
                    id: "node_2".to_owned(),
                    encounter_id: "encounter_2".to_owned(),
                    next_node_ids: Vec::new(),
                    boss: true,
                },
            ],
            encounters: vec![
                CampaignEncounterDefinition {
                    id: "encounter_1".to_owned(),
                    name: "First".to_owned(),
                    enemy_loadout_id: "starter_a".to_owned(),
                    intro_text: String::new(),
                    reward_story_card_ids: vec!["reward_a".to_owned()],
                    boss: false,
                },
                CampaignEncounterDefinition {
                    id: "encounter_2".to_owned(),
                    name: "Second".to_owned(),
                    enemy_loadout_id: "starter_b".to_owned(),
                    intro_text: String::new(),
                    reward_story_card_ids: vec!["reward_b".to_owned()],
                    boss: true,
                },
            ],
        }
    }

    fn sample_deck() -> DeckPreset {
        DeckPreset {
            id: "deck_1".to_owned(),
            name: "Moonlight".to_owned(),
            story_cards: vec!["quiet_lunch_on_the_rooftop".to_owned()],
            magical_girl_roster: vec!["yuki".to_owned(), "hana".to_owned(), "riri".to_owned()],
            baddie_roster: vec![
                "noctra".to_owned(),
                "glass_crow".to_owned(),
                "thorn_waltz".to_owned(),
            ],
            ..DeckPreset::default()
        }
    }

    #[test]
    fn create_run_selects_new_slot_without_closing_existing_runs() {
        let campaign = sample_campaign();
        let mut saves = CampaignSaveBundle::default();

        saves.create_run(&campaign, &sample_deck(), "New Run");
        saves.create_run(&campaign, &sample_deck(), "New Run");

        assert_eq!(saves.runs.len(), 2);
        assert_eq!(saves.runs[0].status, CampaignRunStatus::InProgress);
        assert_eq!(
            saves.selected_run().map(|run| run.id.as_str()),
            Some("campaign_run_2")
        );
        assert_eq!(
            saves
                .selected_run()
                .map(|run| run.selected_magical_girl_support_ids.clone()),
            Some(Vec::new())
        );
    }

    #[test]
    fn record_victory_advances_and_then_completes_selected_run() {
        let campaign = sample_campaign();
        let mut saves = CampaignSaveBundle::default();
        let run_id = saves
            .create_run(&campaign, &sample_deck(), "New Run")
            .expect("run");

        assert_eq!(
            saves.record_victory_for_run(&run_id, &campaign, "node_1", Some("reward_a")),
            Some(false)
        );
        assert_eq!(
            saves.selected_run().map(|run| run.current_node_id.as_str()),
            Some("node_2")
        );

        assert_eq!(
            saves.record_victory_for_run(&run_id, &campaign, "node_2", Some("reward_b")),
            Some(true)
        );
        assert_eq!(saves.runs[0].status, CampaignRunStatus::Won);
        assert!(saves.runs[0]
            .player_deck
            .story_cards
            .iter()
            .any(|card| card == "reward_b"));
    }

    #[test]
    fn select_run_switches_current_slot() {
        let campaign = sample_campaign();
        let mut saves = CampaignSaveBundle::default();
        let first_run_id = saves
            .create_run(&campaign, &sample_deck(), "New Run")
            .expect("first run");
        let second_run_id = saves
            .create_run(&campaign, &sample_deck(), "New Run")
            .expect("second run");

        assert!(saves.select_run(&first_run_id));
        assert_eq!(
            saves.selected_run().map(|run| run.id.as_str()),
            Some(first_run_id.as_str())
        );
        assert_ne!(first_run_id, second_run_id);
    }

    #[test]
    fn selected_support_pair_can_be_updated_for_selected_run() {
        let campaign = sample_campaign();
        let mut saves = CampaignSaveBundle::default();
        saves.create_run(&campaign, &sample_deck(), "New Run");

        assert!(
            saves.update_selected_magical_girl_supports(&["riri".to_owned(), "momo".to_owned()])
        );
        assert_eq!(
            saves
                .selected_run()
                .map(|run| run.selected_magical_girl_support_ids.clone()),
            Some(vec!["riri".to_owned(), "momo".to_owned()])
        );
    }

    #[test]
    fn toggled_supports_can_be_cleared_and_do_not_autofill() {
        let campaign = sample_campaign();
        let mut saves = CampaignSaveBundle::default();
        saves.create_run(&campaign, &sample_deck(), "New Run");

        assert!(saves.toggle_selected_magical_girl_support("hana"));
        assert_eq!(
            saves
                .selected_run()
                .map(|run| run.selected_magical_girl_support_ids.clone()),
            Some(vec!["hana".to_owned()])
        );
        assert!(saves.toggle_selected_magical_girl_support("hana"));
        assert_eq!(
            saves
                .selected_run()
                .map(|run| run.selected_magical_girl_support_ids.clone()),
            Some(Vec::new())
        );
    }
}
