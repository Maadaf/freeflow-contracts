pub const EVENT_PREFIX: &str = "freeflow.events";
pub const PASSAGE_PREFIX: &str = "freeflow.passages";

pub fn event_subject(gantry_id: &str, sensor_type: &str, event_name: &str) -> String {
    format!("{EVENT_PREFIX}.{gantry_id}.{sensor_type}.{event_name}")
}

pub fn passage_subject(status: &str) -> String {
    let suffix = if status == "complete" {
        "completed"
    } else {
        status
    };
    format!("{PASSAGE_PREFIX}.{suffix}")
}
