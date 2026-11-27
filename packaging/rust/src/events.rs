use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn default_schema_version() -> String {
    "1.0".to_owned()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VehicleEvent {
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    pub event_id: Uuid,
    pub correlation_id: Uuid,
    pub occurred_at: DateTime<FixedOffset>,
    pub gantry_id: String,
    pub source: String,
    pub data: Observation,
}

impl VehicleEvent {
    pub fn from_json(raw: &[u8]) -> Result<Self, ContractError> {
        let event: Self = serde_json::from_slice(raw)?;
        event.validate()?;
        Ok(event)
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema_version != "1.0" {
            return Err(ContractError::UnsupportedSchema(self.schema_version.clone()));
        }
        if self.gantry_id.is_empty() || self.source.is_empty() {
            return Err(ContractError::Field("identifier"));
        }
        self.data.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum Observation {
    #[serde(rename = "radar.vehicle_detected")]
    Radar(RadarData),
    #[serde(rename = "lidar.vehicle_profiled")]
    Lidar(LidarData),
    #[serde(rename = "camera.plate_read")]
    Camera(CameraData),
    #[serde(rename = "antenna.tag_read")]
    Antenna(AntennaData),
    #[serde(rename = "wim.axles_counted")]
    Wim(WimData),
}

impl Observation {
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::Radar(_) => "radar.vehicle_detected",
            Self::Lidar(_) => "lidar.vehicle_profiled",
            Self::Camera(_) => "camera.plate_read",
            Self::Antenna(_) => "antenna.tag_read",
            Self::Wim(_) => "wim.axles_counted",
        }
    }

    pub fn common(&self) -> &ObservationBase {
        match self {
            Self::Radar(v) => &v.common,
            Self::Lidar(v) => &v.common,
            Self::Camera(v) => &v.common,
            Self::Antenna(v) => &v.common,
            Self::Wim(v) => &v.common,
        }
    }

    fn validate(&self) -> Result<(), ContractError> {
        self.common().validate()?;
        match self {
            Self::Radar(v) => {
                if v.common.sensor_type != SensorType::Radar {
                    return Err(ContractError::Field("sensor_type"));
                }
                if !(0.0..360.0).contains(&v.heading_degrees) {
                    return Err(ContractError::Field("heading_degrees"));
                }
            }
            Self::Lidar(v) => {
                if v.common.sensor_type != SensorType::Lidar {
                    return Err(ContractError::Field("sensor_type"));
                }
                if v.dimensions.length_m <= 0.0
                    || v.dimensions.width_m <= 0.0
                    || v.dimensions.height_m <= 0.0
                    || !(2..=16).contains(&v.axle_count)
                    || v.point_count == 0
                    || !unit(v.profile_quality)
                {
                    return Err(ContractError::Field("lidar"));
                }
            }
            Self::Camera(v) => {
                if v.common.sensor_type != SensorType::Camera {
                    return Err(ContractError::Field("sensor_type"));
                }
                if !unit(v.plate_confidence)
                    || !unit(v.blur)
                    || !is_lower_hex_sha256(&v.evidence.sha256)
                    || v.evidence.size_bytes == 0
                    || !valid_evidence_uri(&v.evidence.uri)
                {
                    return Err(ContractError::Field("camera"));
                }
            }
            Self::Antenna(v) => {
                if v.common.sensor_type != SensorType::Antenna {
                    return Err(ContractError::Field("sensor_type"));
                }
                if v.read_lane == Some(0) {
                    return Err(ContractError::Field("read_lane"));
                }
            }
            Self::Wim(v) => {
                if v.common.sensor_type != SensorType::Wim {
                    return Err(ContractError::Field("sensor_type"));
                }
                if !(2..=16).contains(&v.axle_count) {
                    return Err(ContractError::Field("axle_count"));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservationBase {
    pub passage_id: Option<Uuid>,
    pub sensor_id: String,
    pub sensor_type: SensorType,
    pub lane: u16,
    pub speed_kmh: f64,
    pub vehicle_class: VehicleClass,
    pub confidence: f64,
    #[serde(default)]
    pub detected_position_m: f64,
    #[serde(default)]
    pub lateral_offset_m: f64,
    #[serde(default)]
    pub sensor_timestamp_offset_ms: f64,
}

impl ObservationBase {
    fn validate(&self) -> Result<(), ContractError> {
        if self.sensor_id.is_empty() || self.lane == 0 || self.speed_kmh <= 0.0 || !unit(self.confidence)
        {
            return Err(ContractError::Field("observation"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SensorType {
    Radar,
    Lidar,
    Camera,
    Antenna,
    Wim,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VehicleClass {
    Light,
    Heavy,
    Motorcycle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarData {
    #[serde(flatten)]
    pub common: ObservationBase,
    pub heading_degrees: f64,
    #[serde(default)]
    pub is_ghost: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VehicleDimensions {
    pub length_m: f64,
    pub width_m: f64,
    pub height_m: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LidarData {
    #[serde(flatten)]
    pub common: ObservationBase,
    pub dimensions: VehicleDimensions,
    pub axle_count: u16,
    pub point_count: u64,
    pub profile_quality: f64,
    #[serde(default)]
    pub has_trailer: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceReference {
    pub uri: String,
    pub media_type: ImageMediaType,
    pub sha256: String,
    pub size_bytes: u64,
    pub width_px: Option<u32>,
    pub height_px: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageMediaType {
    #[serde(rename = "image/jpeg")]
    Jpeg,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Weather {
    Clear,
    Rain,
    Fog,
}

fn clear() -> Weather {
    Weather::Clear
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lighting {
    Day,
    Dusk,
    Night,
}

fn day() -> Lighting {
    Lighting::Day
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraData {
    #[serde(flatten)]
    pub common: ObservationBase,
    pub plate: Option<String>,
    pub plate_country: Option<String>,
    pub plate_confidence: f64,
    pub evidence: EvidenceReference,
    #[serde(default = "clear")]
    pub weather: Weather,
    #[serde(default = "day")]
    pub lighting: Lighting,
    #[serde(default)]
    pub blur: f64,
    #[serde(default)]
    pub plate_occluded: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "ISO-18000-6C")]
    Iso180006c,
}

fn protocol() -> Protocol {
    Protocol::Iso180006c
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntennaData {
    #[serde(flatten)]
    pub common: ObservationBase,
    pub tag_id: Option<String>,
    #[serde(default = "protocol")]
    pub protocol: Protocol,
    pub read_lane: Option<u16>,
    #[serde(default)]
    pub cross_read: bool,
}

/// Piezo/WIM: axle count only — no weight/mass fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WimData {
    #[serde(flatten)]
    pub common: ObservationBase,
    pub axle_count: u16,
    #[serde(default = "piezo")]
    pub sensing: Sensing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sensing {
    Piezo,
}

fn piezo() -> Sensing {
    Sensing::Piezo
}

fn unit(value: f64) -> bool {
    (0.0..=1.0).contains(&value)
}

fn is_lower_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub fn valid_evidence_uri(uri: &str) -> bool {
    !uri.is_empty()
        && uri.len() <= 2048
        && (uri.starts_with("sim://") || uri.starts_with("s3://") || uri.starts_with("file://"))
}

#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("invalid JSON contract: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported schema version: {0}")]
    UnsupportedSchema(String),
    #[error("invalid field: {0}")]
    Field(&'static str),
}
