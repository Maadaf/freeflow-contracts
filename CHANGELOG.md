# Changelog

## 0.1.2 — 2026-11-27

- Contratos `passage.evidence_package` e `passage.lost_package` (schema + fixtures + Python/Rust).
- Módulo partilhado `confidence` (fórmula de correlação VT / twin replay).

## 0.1.1 — 2026-11-26

- Novo evento `wim.axles_counted` (`sensor_type: wim`): contagem de eixos por piezo.
- **Piezo = WIM sem pesagem** — contrato sem campos de peso/massa; `sensing: "piezo"`.
- Subject: `freeflow.events.<gantry>.wim.axles_counted`.
- Modelos Python/Rust, schema dedicado, fixture e CI de sync actualizados.

## 0.1.0 — 2026-11-25

- Fonte única inicial: JSON Schema Draft 2020-12 em `schemas/v1`.
- Eventos: envelope + `radar.vehicle_detected`, `lidar.vehicle_profiled`, `camera.plate_read`, `antenna.tag_read`.
- Passagens: `passage.finalized` (`complete` / `expired`) com `schema_version: "1.0"`.
- Convenção NATS com gantry no subject; URIs de evidência `sim://`, `s3://`, `file://`.
- `passage_id` nos eventos é nullable (alinhado ao vehicle-tracker / gateways).
- Pacote Python `freeflow-contracts` e crate Rust `freeflow-contracts` espelhados (sem codegen).
- Fixtures v1 + CI de sync (schema ↔ Python ↔ Rust).
