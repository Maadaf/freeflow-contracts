"""Motor de correlação/confiança partilhado (pórtico VT + twin replay).

Fórmula: c = 1 - (0.7 * Δt/W + 0.3 * Δv/V).
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class CorrelationConfig:
    window_seconds: float = 10.0
    speed_tolerance_kmh: float = 20.0
    min_confidence: float = 0.5
    ambiguity_margin: float = 0.1

    @classmethod
    def from_overrides(
        cls,
        *,
        window_seconds: float | None = None,
        speed_tolerance_kmh: float | None = None,
        min_confidence: float | None = None,
        ambiguity_margin: float | None = None,
    ) -> CorrelationConfig:
        base = cls()
        return cls(
            window_seconds=max(0.001, window_seconds)
            if window_seconds is not None
            else base.window_seconds,
            speed_tolerance_kmh=max(0.001, speed_tolerance_kmh)
            if speed_tolerance_kmh is not None
            else base.speed_tolerance_kmh,
            min_confidence=min(1.0, max(0.0, min_confidence))
            if min_confidence is not None
            else base.min_confidence,
            ambiguity_margin=min(1.0, max(0.0, ambiguity_margin))
            if ambiguity_margin is not None
            else base.ambiguity_margin,
        )


@dataclass(frozen=True, slots=True)
class ScoredCandidate:
    passage_id: str
    confidence: float


def pair_confidence(
    delta_seconds: float, speed_delta_kmh: float, cfg: CorrelationConfig
) -> float:
    score = 1.0 - (
        0.7 * abs(delta_seconds) / cfg.window_seconds
        + 0.3 * abs(speed_delta_kmh) / cfg.speed_tolerance_kmh
    )
    return round(score, 4)


def select_best(
    matches: list[ScoredCandidate], cfg: CorrelationConfig
) -> ScoredCandidate | None:
    eligible = [m for m in matches if m.confidence >= cfg.min_confidence]
    eligible.sort(key=lambda m: (-m.confidence, m.passage_id))
    if not eligible:
        return None
    best = eligible[0]
    if len(eligible) > 1 and best.confidence - eligible[1].confidence < cfg.ambiguity_margin:
        return None
    return best


def replay_overall_confidence(
    correlation: float,
    plate_confidence: float | None,
    sensors_present: int,
    sensors_expected: int = 5,
) -> float:
    sensor_fill = 1.0 if sensors_expected <= 0 else max(0.0, min(1.0, sensors_present / sensors_expected))
    plate = plate_confidence if plate_confidence is not None else correlation
    overall = 0.5 * max(0.0, min(1.0, correlation)) + 0.3 * max(0.0, min(1.0, plate)) + 0.2 * sensor_fill
    return round(overall, 4)
