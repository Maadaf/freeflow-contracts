use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::events::{valid_evidence_uri, ContractError, VehicleClass};

fn default_schema_version() -> String {
    "1.0".to_owned()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageAckState {
    Pending,
    Acked,
    Lost,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LostPackageReason {
    DiskPressure,
    Overwrite,
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PackageMediaType {
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "video/mp4")]
    Mp4,
    #[serde(rename = "application/octet-stream")]
    OctetStream,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageMediaRole {
    AnprFront,
    AnprRear,
    VideoLateral,
    VideoOverview,
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageMediaRef {
    pub evidence_id: Uuid,
    pub uri: String,
    pub media_type: PackageMediaType,
    pub sha256: String,
    pub size_bytes: u64,
    #[serde(default)]
    pub captured_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub role: Option<PackageMediaRole>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageConfidence {
    pub correlation: f64,
    #[serde(default)]
    pub plate: Option<f64>,
    pub overall: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageMedia {
    #[serde(default)]
    pub video_lateral: Option<PackageMediaRef>,
    #[serde(default)]
    pub video_overview: Option<PackageMediaRef>,
    pub anpr: Vec<PackageMediaRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageStubs {
    #[serde(default)]
    pub videos_pending: Option<bool>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PassageEvidencePackage {
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    pub package_id: Uuid,
    pub passage_id: Uuid,
    pub gantry_id: String,
    #[serde(default)]
    pub lane: Option<u32>,
    #[serde(default)]
    pub vehicle_class: Option<VehicleClass>,
    pub occurred_at: DateTime<Utc>,
    pub assembled_at: DateTime<Utc>,
    pub ack_state: PackageAckState,
    pub confidence: PackageConfidence,
    pub media: PackageMedia,
    pub events: Vec<Value>,
    #[serde(default)]
    pub passage: Option<Value>,
    #[serde(default)]
    pub stubs: Option<PackageStubs>,
}

impl PassageEvidencePackage {
    pub fn from_json(raw: &[u8]) -> Result<Self, ContractError> {
        let package: Self = serde_json::from_slice(raw)?;
        package.validate()?;
        Ok(package)
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema_version != "1.0" {
            return Err(ContractError::UnsupportedSchema(self.schema_version.clone()));
        }
        if self.gantry_id.is_empty() {
            return Err(ContractError::Field("gantry_id"));
        }
        if !(0.0..=1.0).contains(&self.confidence.correlation)
            || !(0.0..=1.0).contains(&self.confidence.overall)
        {
            return Err(ContractError::Field("confidence"));
        }
        if let Some(p) = self.confidence.plate {
            if !(0.0..=1.0).contains(&p) {
                return Err(ContractError::Field("confidence.plate"));
            }
        }
        for item in &self.media.anpr {
            validate_media(item)?;
        }
        if let Some(v) = &self.media.video_lateral {
            validate_media(v)?;
        }
        if let Some(v) = &self.media.video_overview {
            validate_media(v)?;
        }
        Ok(())
    }
}

fn validate_media(item: &PackageMediaRef) -> Result<(), ContractError> {
    if item.size_bytes == 0 || !valid_evidence_uri(&item.uri) {
        return Err(ContractError::Field("media"));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PassageLostPackage {
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    pub lost_id: Uuid,
    #[serde(default)]
    pub package_id: Option<Uuid>,
    #[serde(default)]
    pub passage_id: Option<Uuid>,
    pub gantry_id: String,
    #[serde(default)]
    pub lane: Option<u32>,
    #[serde(default)]
    pub vehicle_class: Option<VehicleClass>,
    pub occurred_at: DateTime<Utc>,
    pub lost_at: DateTime<Utc>,
    pub reason: LostPackageReason,
}

impl PassageLostPackage {
    pub fn from_json(raw: &[u8]) -> Result<Self, ContractError> {
        let lost: Self = serde_json::from_slice(raw)?;
        lost.validate()?;
        Ok(lost)
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema_version != "1.0" {
            return Err(ContractError::UnsupportedSchema(self.schema_version.clone()));
        }
        if self.gantry_id.is_empty() {
            return Err(ContractError::Field("gantry_id"));
        }
        Ok(())
    }
}
