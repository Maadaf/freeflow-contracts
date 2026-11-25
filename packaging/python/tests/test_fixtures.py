from __future__ import annotations

import json
from pathlib import Path

import pytest
from freeflow_contracts.events import EventEnvelope
from freeflow_contracts.passages import PassageFinalized
from freeflow_contracts.subjects import event_subject, passage_subject

REPO_ROOT = Path(__file__).resolve().parents[3]
EVENTS = REPO_ROOT / "fixtures" / "v1" / "events"
PASSAGES = REPO_ROOT / "fixtures" / "v1" / "passages"
SCHEMAS = REPO_ROOT / "schemas" / "v1"

EVENT_SCHEMA = {
    "radar.vehicle_detected": "radar.vehicle_detected.schema.json",
    "lidar.vehicle_profiled": "lidar.vehicle_profiled.schema.json",
    "camera.plate_read": "camera.plate_read.schema.json",
    "antenna.tag_read": "antenna.tag_read.schema.json",
}


def _load(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


@pytest.mark.parametrize("name", sorted(EVENT_SCHEMA))
def test_event_fixture_parses_with_pydantic(name: str) -> None:
    payload = _load(EVENTS / f"{name}.json")
    event = EventEnvelope.model_validate(payload)
    assert event.data.event_type == name
    assert event.schema_version == "1.0"


@pytest.mark.parametrize("name", ["complete", "expired"])
def test_passage_fixture_parses_with_pydantic(name: str) -> None:
    payload = _load(PASSAGES / f"{name}.json")
    passage = PassageFinalized.model_validate(payload)
    assert passage.status.value == name


@pytest.mark.parametrize("name,schema_file", sorted(EVENT_SCHEMA.items()))
def test_event_fixture_matches_json_schema(name: str, schema_file: str) -> None:
    jsonschema = pytest.importorskip("jsonschema")
    schema = _load(SCHEMAS / schema_file)
    payload = _load(EVENTS / f"{name}.json")
    jsonschema.Draft202012Validator(schema).validate(payload)


@pytest.mark.parametrize("name", ["complete", "expired"])
def test_passage_fixture_matches_json_schema(name: str) -> None:
    jsonschema = pytest.importorskip("jsonschema")
    schema = _load(SCHEMAS / "passage.finalized.schema.json")
    payload = _load(PASSAGES / f"{name}.json")
    jsonschema.Draft202012Validator(schema).validate(payload)


def test_subjects_include_gantry() -> None:
    assert (
        event_subject("A1-KM10-N", "camera")
        == "freeflow.events.A1-KM10-N.camera.plate_read"
    )
    assert passage_subject("complete") == "freeflow.passages.completed"
    assert passage_subject("expired") == "freeflow.passages.expired"


def test_file_uri_accepted() -> None:
    payload = _load(EVENTS / "camera.plate_read.json")
    event = EventEnvelope.model_validate(payload)
    assert str(event.data.evidence.uri).startswith("file://")
