//! Contratos FreeFlow v1 — modelos Serde espelhados dos JSON Schema.

pub mod confidence;
mod events;
mod packages;
mod passages;
mod subjects;

pub use confidence::{
    pair_confidence, replay_overall_confidence, select_best, CorrelationConfig, ScoredCandidate,
};
pub use events::*;
pub use packages::*;
pub use passages::*;
pub use subjects::*;

pub const SCHEMA_VERSION: &str = "1.0";
