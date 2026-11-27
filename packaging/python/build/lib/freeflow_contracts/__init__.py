"""Contratos FreeFlow v1 — modelos espelhados dos JSON Schema."""

from freeflow_contracts.events import (
    AntennaObservation,
    CameraObservation,
    EventEnvelope,
    EvidenceReference,
    LidarObservation,
    Observation,
    RadarObservation,
    SCHEMA_VERSION,
    WimObservation,
)
from freeflow_contracts.passages import PassageFinalized, PassageStatus
from freeflow_contracts.subjects import event_subject, passage_subject

__all__ = [
    "SCHEMA_VERSION",
    "AntennaObservation",
    "CameraObservation",
    "EventEnvelope",
    "EvidenceReference",
    "LidarObservation",
    "Observation",
    "PassageFinalized",
    "PassageStatus",
    "RadarObservation",
    "WimObservation",
    "event_subject",
    "passage_subject",
]

__version__ = "0.1.1"
