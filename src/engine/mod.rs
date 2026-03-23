//! Deterministic match engine.

mod ai;
mod match_engine;
mod simulation;

pub use ai::AiController;
pub use match_engine::{EncounterOutcome, MatchAction, MatchEngine};
pub use simulation::{BatchReport, MatchReport, SimulationRunner};
