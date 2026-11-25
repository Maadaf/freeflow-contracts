use std::{fs, path::PathBuf};

use freeflow_contracts::{
    event_subject, passage_subject, Observation, PassageFinalized, PassageStatus, VehicleEvent,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn read(rel: &str) -> Vec<u8> {
    fs::read(repo_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn parses_all_event_fixtures() {
    for name in [
        "radar.vehicle_detected",
        "lidar.vehicle_profiled",
        "camera.plate_read",
        "antenna.tag_read",
    ] {
        let raw = read(&format!("fixtures/v1/events/{name}.json"));
        let event = VehicleEvent::from_json(&raw).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(event.schema_version, "1.0");
        assert_eq!(event.data.event_type(), name);
    }
}

#[test]
fn camera_fixture_allows_file_uri_and_null_passage_id() {
    let event = VehicleEvent::from_json(&read("fixtures/v1/events/camera.plate_read.json")).unwrap();
    match event.data {
        Observation::Camera(data) => {
            assert!(data.evidence.uri.starts_with("file://"));
            assert!(data.common.passage_id.is_none());
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn parses_passage_fixtures() {
    let complete =
        PassageFinalized::from_json(&read("fixtures/v1/passages/complete.json")).unwrap();
    assert!(matches!(complete.status, PassageStatus::Complete));
    let expired = PassageFinalized::from_json(&read("fixtures/v1/passages/expired.json")).unwrap();
    assert!(matches!(expired.status, PassageStatus::Expired));
}

#[test]
fn subjects_include_gantry() {
    assert_eq!(
        event_subject("A1-KM10-N", "camera", "plate_read"),
        "freeflow.events.A1-KM10-N.camera.plate_read"
    );
    assert_eq!(passage_subject("complete"), "freeflow.passages.completed");
}
