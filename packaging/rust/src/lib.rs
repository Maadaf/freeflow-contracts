//! Contratos FreeFlow v1 — modelos Serde espelhados dos JSON Schema.

mod events;
mod passages;
mod subjects;

pub use events::*;
pub use passages::*;
pub use subjects::*;

pub const SCHEMA_VERSION: &str = "1.0";
