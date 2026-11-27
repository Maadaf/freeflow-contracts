from __future__ import annotations

EVENT_PREFIX = "freeflow.events"
PASSAGE_PREFIX = "freeflow.passages"

SENSOR_EVENT_NAMES = {
    "radar": "vehicle_detected",
    "lidar": "vehicle_profiled",
    "camera": "plate_read",
    "antenna": "tag_read",
    "wim": "axles_counted",
}


def event_subject(gantry_id: str, sensor_type: str, event_name: str | None = None) -> str:
    """Subject v1: freeflow.events.<gantry>.<sensor_type>.<event_name>."""
    name = event_name or SENSOR_EVENT_NAMES[sensor_type]
    return f"{EVENT_PREFIX}.{gantry_id}.{sensor_type}.{name}"


def passage_subject(status: str, *, prefix: str = PASSAGE_PREFIX) -> str:
    suffix = "completed" if status == "complete" else status
    return f"{prefix}.{suffix}"
