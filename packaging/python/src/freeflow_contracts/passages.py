from __future__ import annotations

from datetime import datetime
from enum import StrEnum
from typing import Literal
from uuid import UUID

from pydantic import BaseModel, ConfigDict, Field

from freeflow_contracts.events import EvidenceReference, VehicleClass, VehicleDimensions


class StrictModel(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True)


class PassageStatus(StrEnum):
    COMPLETE = "complete"
    EXPIRED = "expired"


class PassageIdSource(StrEnum):
    PROVIDED = "provided"
    INFERRED = "inferred"
    GENERATED = "generated"


class PassageEvidence(EvidenceReference):
    evidence_id: UUID
    captured_at: datetime | None = None


class PassageFinalized(StrictModel):
    schema_version: Literal["1.0"] = "1.0"
    passage_id: UUID
    gantry_id: str
    status: PassageStatus
    first_seen_at: datetime
    last_seen_at: datetime
    finalized_at: datetime
    observation_count: int = Field(ge=1)
    passage_id_source: PassageIdSource
    correlation_confidence: float = Field(ge=0, le=1)
    speed_kmh: float | None = Field(default=None, gt=0)
    vehicle_class: VehicleClass | None = None
    plate: str | None = None
    plate_confidence: float | None = Field(default=None, ge=0, le=1)
    tag_id: str | None = None
    dimensions: VehicleDimensions | None = None
    sensors_present: tuple[str, ...]
    sensors_missing: tuple[str, ...]
    evidence: tuple[PassageEvidence, ...]
    observation_event_ids: tuple[UUID, ...]
