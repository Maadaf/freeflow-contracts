use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::events::{valid_evidence_uri, ContractError, ImageMediaType, VehicleClass, VehicleDimensions};

fn default_schema_version() -> String {
    "1.0".to_owned()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PassageStatus {
    Complete,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PassageIdSource {
    Provided,
    Inferred,
    Generated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PassageEvidence {
    pub evidence_id: Uuid,
    pub uri: String,
    pub media_type: ImageMediaType,
    pub sha256: String,
    pub size_bytes: u64,
    #[serde(default)]
    pub width_px: Option<u32>,
    #[serde(default)]
    pub height_px: Option<u32>,
    #[serde(default)]
    pub captured_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PassageFinalized {
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    pub passage_id: Uuid,
    pub gantry_id: String,
    pub status: PassageStatus,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub finalized_at: DateTime<Utc>,
    pub observation_count: u32,
    pub passage_id_source: PassageIdSource,
    pub correlation_confidence: f64,
    #[serde(default)]
    pub speed_kmh: Option<f64>,
    #[serde(default)]
    pub vehicle_class: Option<VehicleClass>,
    #[serde(default)]
    pub plate: Option<String>,
    #[serde(default)]
    pub plate_confidence: Option<f64>,
    #[serde(default)]
    pub tag_id: Option<String>,
    #[serde(default)]
    pub dimensions: Option<VehicleDimensions>,
    pub sensors_present: Vec<String>,
    pub sensors_missing: Vec<String>,
    pub evidence: Vec<PassageEvidence>,
    pub observation_event_ids: Vec<Uuid>,
}

impl PassageFinalized {
    pub fn from_json(raw: &[u8]) -> Result<Self, ContractError> {
        let passage: Self = serde_json::from_slice(raw)?;
        passage.validate()?;
        Ok(passage)
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema_version != "1.0" {
            return Err(ContractError::UnsupportedSchema(self.schema_version.clone()));
        }
        if self.gantry_id.is_empty() || self.observation_count == 0 {
            return Err(ContractError::Field("passage"));
        }
        if !(0.0..=1.0).contains(&self.correlation_confidence) {
            return Err(ContractError::Field("correlation_confidence"));
        }
        for item in &self.evidence {
            if item.size_bytes == 0 || !valid_evidence_uri(&item.uri) {
                return Err(ContractError::Field("evidence"));
            }
        }
        Ok(())
    }
}
