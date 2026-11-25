from __future__ import annotations

from datetime import datetime
from enum import StrEnum
from typing import Annotated, Literal
from uuid import UUID

from pydantic import BaseModel, ConfigDict, Field

SCHEMA_VERSION = "1.0"
EVIDENCE_URI_PATTERN = r"^(sim|s3|file)://"


class StrictModel(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True)


class SensorType(StrEnum):
    RADAR = "radar"
    LIDAR = "lidar"
    CAMERA = "camera"
    ANTENNA = "antenna"


class VehicleClass(StrEnum):
    LIGHT = "light"
    HEAVY = "heavy"
    MOTORCYCLE = "motorcycle"


class Weather(StrEnum):
    CLEAR = "clear"
    RAIN = "rain"
    FOG = "fog"


class Lighting(StrEnum):
    DAY = "day"
    DUSK = "dusk"
    NIGHT = "night"


class SensorObservation(StrictModel):
    passage_id: UUID | None = None
    sensor_id: str
    sensor_type: SensorType
    lane: int = Field(ge=1)
    speed_kmh: float = Field(gt=0)
    vehicle_class: VehicleClass
    confidence: float = Field(ge=0, le=1)
    detected_position_m: float = 0
    lateral_offset_m: float = 0
    sensor_timestamp_offset_ms: float = 0


class RadarObservation(SensorObservation):
    event_type: Literal["radar.vehicle_detected"] = "radar.vehicle_detected"
    sensor_type: Literal[SensorType.RADAR] = SensorType.RADAR
    heading_degrees: float = Field(ge=0, lt=360)
    is_ghost: bool = False


class VehicleDimensions(StrictModel):
    length_m: float = Field(gt=0)
    width_m: float = Field(gt=0)
    height_m: float = Field(gt=0)


class LidarObservation(SensorObservation):
    event_type: Literal["lidar.vehicle_profiled"] = "lidar.vehicle_profiled"
    sensor_type: Literal[SensorType.LIDAR] = SensorType.LIDAR
    dimensions: VehicleDimensions
    axle_count: int = Field(ge=2, le=16)
    point_count: int = Field(gt=0)
    profile_quality: float = Field(ge=0, le=1)
    has_trailer: bool = False


class EvidenceReference(StrictModel):
    uri: str = Field(pattern=EVIDENCE_URI_PATTERN)
    media_type: Literal["image/jpeg"] = "image/jpeg"
    sha256: str = Field(pattern=r"^[0-9a-f]{64}$")
    size_bytes: int = Field(gt=0)
    width_px: int | None = Field(default=None, gt=0)
    height_px: int | None = Field(default=None, gt=0)


class CameraObservation(SensorObservation):
    event_type: Literal["camera.plate_read"] = "camera.plate_read"
    sensor_type: Literal[SensorType.CAMERA] = SensorType.CAMERA
    plate: str | None = None
    plate_country: str | None = None
    plate_confidence: float = Field(ge=0, le=1)
    evidence: EvidenceReference
    weather: Weather = Weather.CLEAR
    lighting: Lighting = Lighting.DAY
    blur: float = Field(default=0, ge=0, le=1)
    plate_occluded: bool = False


class AntennaObservation(SensorObservation):
    event_type: Literal["antenna.tag_read"] = "antenna.tag_read"
    sensor_type: Literal[SensorType.ANTENNA] = SensorType.ANTENNA
    tag_id: str | None = None
    protocol: Literal["ISO-18000-6C"] = "ISO-18000-6C"
    read_lane: int | None = Field(default=None, ge=1)
    cross_read: bool = False


Observation = RadarObservation | LidarObservation | CameraObservation | AntennaObservation


class EventEnvelope(StrictModel):
    schema_version: Literal["1.0"] = "1.0"
    event_id: UUID
    occurred_at: datetime
    source: str
    gantry_id: str
    correlation_id: UUID
    data: Annotated[Observation, Field(discriminator="event_type")]
