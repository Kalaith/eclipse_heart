//! Match rules and threshold definitions.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterRules {
    pub first_threshold: i32,
    pub second_threshold: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchRules {
    pub encounter_win_gain: i32,
    pub encounter_loss_gain: i32,
    pub encounter_tie_gain: i32,
    pub default_magical_girl_rules: CharacterRules,
    pub default_baddie_rules: CharacterRules,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeckRules {
    pub support_deck_size: usize,
    pub max_copies_per_story_card: usize,
    pub universal_copy_limit: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressionRules {
    pub overflow_is_lost: bool,
    pub upgrades_happen_immediately: bool,
    pub upgrades_do_not_revert_by_default: bool,
    pub tie_points_stop_at_final_form: bool,
}

impl Default for MatchRules {
    fn default() -> Self {
        Self {
            encounter_win_gain: 3,
            encounter_loss_gain: 1,
            encounter_tie_gain: 1,
            default_magical_girl_rules: CharacterRules {
                first_threshold: 3,
                second_threshold: 6,
            },
            default_baddie_rules: CharacterRules {
                first_threshold: 3,
                second_threshold: 6,
            },
        }
    }
}

impl Default for DeckRules {
    fn default() -> Self {
        Self {
            support_deck_size: 40,
            max_copies_per_story_card: 3,
            universal_copy_limit: true,
        }
    }
}

impl Default for ProgressionRules {
    fn default() -> Self {
        Self {
            overflow_is_lost: true,
            upgrades_happen_immediately: true,
            upgrades_do_not_revert_by_default: true,
            tie_points_stop_at_final_form: true,
        }
    }
}
