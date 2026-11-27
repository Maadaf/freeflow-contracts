from __future__ import annotations

from datetime import datetime
from enum import StrEnum
from typing import Any, Literal
from uuid import UUID

from pydantic import BaseModel, ConfigDict, Field

from freeflow_contracts.events import VehicleClass


class StrictModel(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True)


class PackageAckState(StrEnum):
    PENDING = "pending"
    ACKED = "acked"
    LOST = "lost"


class LostPackageReason(StrEnum):
    DISK_PRESSURE = "disk_pressure"
    OVERWRITE = "overwrite"
    OTHER = "other"


class PackageMediaType(StrEnum):
    JPEG = "image/jpeg"
    MP4 = "video/mp4"
    OCTET = "application/octet-stream"


class PackageMediaRole(StrEnum):
    ANPR_FRONT = "anpr_front"
    ANPR_REAR = "anpr_rear"
    VIDEO_LATERAL = "video_lateral"
    VIDEO_OVERVIEW = "video_overview"
    OTHER = "other"


class PackageMediaRef(StrictModel):
    evidence_id: UUID
    uri: str
    media_type: PackageMediaType
    sha256: str = Field(pattern=r"^[0-9a-f]{64}$")
    size_bytes: int = Field(gt=0)
    captured_at: datetime | None = None
    role: PackageMediaRole | None = None


class PackageConfidence(StrictModel):
    correlation: float = Field(ge=0, le=1)
    plate: float | None = Field(default=None, ge=0, le=1)
    overall: float = Field(ge=0, le=1)


class PackageMedia(StrictModel):
    video_lateral: PackageMediaRef | None = None
    video_overview: PackageMediaRef | None = None
    anpr: tuple[PackageMediaRef, ...] = ()


class PackageStubs(StrictModel):
    videos_pending: bool | None = None
    notes: str | None = None


class PassageEvidencePackage(StrictModel):
    schema_version: Literal["1.0"] = "1.0"
    package_id: UUID
    passage_id: UUID
    gantry_id: str
    lane: int | None = Field(default=None, ge=1)
    vehicle_class: VehicleClass | None = None
    occurred_at: datetime
    assembled_at: datetime
    ack_state: PackageAckState
    confidence: PackageConfidence
    media: PackageMedia
    events: tuple[dict[str, Any], ...]
    passage: dict[str, Any] | None = None
    stubs: PackageStubs | None = None


class PassageLostPackage(StrictModel):
    schema_version: Literal["1.0"] = "1.0"
    lost_id: UUID
    package_id: UUID | None = None
    passage_id: UUID | None = None
    gantry_id: str
    lane: int | None = Field(default=None, ge=1)
    vehicle_class: VehicleClass | None = None
    occurred_at: datetime
    lost_at: datetime
    reason: LostPackageReason
