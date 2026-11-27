//! Motor de correlação/confiança partilhado (pórtico VT + twin replay).
//!
//! Fórmula entre dois candidatos no mesmo lane dentro da janela:
//! `c = 1 - (0.7 * Δt/W + 0.3 * Δv/V)`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrelationConfig {
    pub window_seconds: f64,
    pub speed_tolerance_kmh: f64,
    pub min_confidence: f64,
    pub ambiguity_margin: f64,
}

impl Default for CorrelationConfig {
    fn default() -> Self {
        Self {
            window_seconds: 10.0,
            speed_tolerance_kmh: 20.0,
            min_confidence: 0.5,
            ambiguity_margin: 0.1,
        }
    }
}

impl CorrelationConfig {
    pub fn from_overrides(
        window_seconds: Option<f64>,
        speed_tolerance_kmh: Option<f64>,
        min_confidence: Option<f64>,
        ambiguity_margin: Option<f64>,
    ) -> Self {
        let mut cfg = Self::default();
        if let Some(v) = window_seconds {
            cfg.window_seconds = v.max(0.001);
        }
        if let Some(v) = speed_tolerance_kmh {
            cfg.speed_tolerance_kmh = v.max(0.001);
        }
        if let Some(v) = min_confidence {
            cfg.min_confidence = v.clamp(0.0, 1.0);
        }
        if let Some(v) = ambiguity_margin {
            cfg.ambiguity_margin = v.clamp(0.0, 1.0);
        }
        cfg
    }
}

/// Confiança de um par (delta temporal + delta velocidade).
pub fn pair_confidence(delta_seconds: f64, speed_delta_kmh: f64, cfg: &CorrelationConfig) -> f64 {
    let score = 1.0
        - (0.7 * delta_seconds.abs() / cfg.window_seconds
            + 0.3 * speed_delta_kmh.abs() / cfg.speed_tolerance_kmh);
    (score * 10_000.0).round() / 10_000.0
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoredCandidate {
    pub passage_id: String,
    pub confidence: f64,
}

/// Escolhe o melhor candidato se passar min_confidence e margem de ambiguidade.
pub fn select_best(
    mut matches: Vec<ScoredCandidate>,
    cfg: &CorrelationConfig,
) -> Option<ScoredCandidate> {
    matches.retain(|m| m.confidence >= cfg.min_confidence);
    matches.sort_by(|a, b| {
        b.confidence
            .total_cmp(&a.confidence)
            .then_with(|| a.passage_id.cmp(&b.passage_id))
    });
    let best = matches.first()?;
    if matches
        .get(1)
        .is_none_or(|second| best.confidence - second.confidence >= cfg.ambiguity_margin)
    {
        Some(best.clone())
    } else {
        None
    }
}

/// Replay v1: confiança overall a partir de eventos estruturados + config.
/// Não reprocessa vídeo/OCR — usa plate_confidence já presente nos eventos.
pub fn replay_overall_confidence(
    correlation: f64,
    plate_confidence: Option<f64>,
    sensors_present: usize,
    sensors_expected: usize,
) -> f64 {
    let sensor_fill = if sensors_expected == 0 {
        1.0
    } else {
        (sensors_present as f64 / sensors_expected as f64).clamp(0.0, 1.0)
    };
    let plate = plate_confidence.unwrap_or(correlation);
    let overall = 0.5 * correlation.clamp(0.0, 1.0) + 0.3 * plate.clamp(0.0, 1.0) + 0.2 * sensor_fill;
    (overall * 10_000.0).round() / 10_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_matches_vehicle_tracker_formula() {
        let cfg = CorrelationConfig::default();
        let c = pair_confidence(0.0, 0.0, &cfg);
        assert!((c - 1.0).abs() < 1e-9);
        let mid = pair_confidence(5.0, 10.0, &cfg);
        assert!((mid - 0.5).abs() < 1e-4);
    }

    #[test]
    fn select_rejects_ambiguous() {
        let cfg = CorrelationConfig::default();
        let chosen = select_best(
            vec![
                ScoredCandidate {
                    passage_id: "a".into(),
                    confidence: 0.8,
                },
                ScoredCandidate {
                    passage_id: "b".into(),
                    confidence: 0.75,
                },
            ],
            &cfg,
        );
        assert!(chosen.is_none());
    }
}
