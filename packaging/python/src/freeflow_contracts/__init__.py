"""Contratos FreeFlow v1 — modelos espelhados dos JSON Schema."""

from freeflow_contracts.confidence import (
    CorrelationConfig,
    ScoredCandidate,
    pair_confidence,
    replay_overall_confidence,
    select_best,
)
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
from freeflow_contracts.packages import PassageEvidencePackage, PassageLostPackage
from freeflow_contracts.passages import PassageFinalized, PassageStatus
from freeflow_contracts.subjects import event_subject, passage_subject

__all__ = [
    "SCHEMA_VERSION",
    "AntennaObservation",
    "CameraObservation",
    "CorrelationConfig",
    "EventEnvelope",
    "EvidenceReference",
    "LidarObservation",
    "Observation",
    "PassageEvidencePackage",
    "PassageFinalized",
    "PassageLostPackage",
    "PassageStatus",
    "RadarObservation",
    "ScoredCandidate",
    "WimObservation",
    "event_subject",
    "pair_confidence",
    "passage_subject",
    "replay_overall_confidence",
    "select_best",
]

__version__ = "0.1.2"
