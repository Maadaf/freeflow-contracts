#!/usr/bin/env python3
"""CI de sync: fixtures validam contra JSON Schema e contra modelos Python."""
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMAS = ROOT / "schemas" / "v1"
EVENTS = ROOT / "fixtures" / "v1" / "events"
PASSAGES = ROOT / "fixtures" / "v1" / "passages"

EVENT_FILES = {
    "radar.vehicle_detected": "radar.vehicle_detected.schema.json",
    "lidar.vehicle_profiled": "lidar.vehicle_profiled.schema.json",
    "camera.plate_read": "camera.plate_read.schema.json",
    "antenna.tag_read": "antenna.tag_read.schema.json",
    "wim.axles_counted": "wim.axles_counted.schema.json",
}


def fail(message: str) -> None:
    print(f"ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    try:
        import jsonschema
    except ImportError:
        fail("instalar jsonschema: pip install jsonschema")

    sys.path.insert(0, str(ROOT / "packaging" / "python" / "src"))
    from freeflow_contracts.events import EventEnvelope
    from freeflow_contracts.passages import PassageFinalized

    for name, schema_name in EVENT_FILES.items():
        payload = json.loads((EVENTS / f"{name}.json").read_text(encoding="utf-8"))
        schema = json.loads((SCHEMAS / schema_name).read_text(encoding="utf-8"))
        jsonschema.Draft202012Validator(schema).validate(payload)
        event = EventEnvelope.model_validate(payload)
        if event.data.event_type != name:
            fail(f"{name}: event_type mismatch")
        print(f"OK event {name}")

    passage_schema = json.loads(
        (SCHEMAS / "passage.finalized.schema.json").read_text(encoding="utf-8")
    )
    for name in ("complete", "expired"):
        payload = json.loads((PASSAGES / f"{name}.json").read_text(encoding="utf-8"))
        jsonschema.Draft202012Validator(passage_schema).validate(payload)
        PassageFinalized.model_validate(payload)
        print(f"OK passage {name}")

    expected = {
        "antenna.tag_read.schema.json",
        "camera.plate_read.schema.json",
        "event-envelope.schema.json",
        "lidar.vehicle_profiled.schema.json",
        "passage.finalized.schema.json",
        "radar.vehicle_detected.schema.json",
        "subjects.md",
        "wim.axles_counted.schema.json",
    }
    present = {p.name for p in SCHEMAS.iterdir() if p.is_file()}
    missing = expected - present
    if missing:
        fail(f"schemas em falta: {sorted(missing)}")
    print("OK schema inventory")
    print("sync check passed")


if __name__ == "__main__":
    main()
