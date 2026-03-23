//! Character and story-card definitions.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterDefinition {
    pub id: String,
    pub name: String,
    pub kind: CharacterKindDef,
    pub base_power: i32,
    pub transformed_power: i32,
    pub final_power: i32,
    pub first_threshold: i32,
    pub second_threshold: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharacterKindDef {
    MagicalGirl,
    Baddie,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryCardDefinition {
    pub id: String,
    pub name: String,
    pub card_type: String,
    pub speed: CardSpeed,
    pub alignment: CardAlignment,
    #[serde(default)]
    pub playable_in_daily_life: bool,
    pub effects: Vec<CardEffect>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardSpeed {
    DailyLife,
    Reaction,
    Encounter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardAlignment {
    MagicalGirl,
    Baddie,
    Neutral,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CardEffect {
    GainMainRadiance { amount: i32 },
    GainRevealedSupportRadiance { amount: i32 },
    ReduceOpponentMainRadiance { amount: i32 },
    GainPrimeDread { amount: i32 },
    GainRevealedSupportDread { amount: i32 },
    ReduceOpponentPrimeDread { amount: i32 },
    GainMainPowerThisEncounter { amount: i32 },
    GainMainPowerNextEncounter { amount: i32 },
    ReduceOpponentMainPowerThisEncounter { amount: i32 },
    GainPrimePowerThisEncounter { amount: i32 },
    GainRevealedSupportPowerThisEncounter { amount: i32 },
    GainFirstRevealedSupportRadiance { amount: i32 },
    ExhaustFirstRevealedOpponentSupport,
    RevealFirstHiddenOwnSupport,
}
