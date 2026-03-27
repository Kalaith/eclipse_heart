//! Campaign content definitions.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CampaignDefinition {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub nodes: Vec<CampaignNodeDefinition>,
    #[serde(default)]
    pub encounters: Vec<CampaignEncounterDefinition>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CampaignNodeDefinition {
    pub id: String,
    pub encounter_id: String,
    #[serde(default)]
    pub next_node_ids: Vec<String>,
    #[serde(default)]
    pub boss: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CampaignEncounterDefinition {
    pub id: String,
    pub name: String,
    pub enemy_loadout_id: String,
    #[serde(default)]
    pub intro_text: String,
    #[serde(default)]
    pub reward_story_card_ids: Vec<String>,
    #[serde(default)]
    pub boss: bool,
}

impl CampaignDefinition {
    pub fn node(&self, node_id: &str) -> Option<&CampaignNodeDefinition> {
        self.nodes.iter().find(|node| node.id == node_id)
    }

    pub fn encounter(&self, encounter_id: &str) -> Option<&CampaignEncounterDefinition> {
        self.encounters
            .iter()
            .find(|encounter| encounter.id == encounter_id)
    }

    pub fn first_node(&self) -> Option<&CampaignNodeDefinition> {
        self.nodes.first()
    }
}
