//! Make sure we can actually deserialize the track config

use track_config::TrackConfig;

static CONFIG: &str = include_str!("../../../../config.json");

#[test]
fn test_deserialize() {
    let _: TrackConfig = serde_json::from_str(CONFIG)
        .expect("should be able to deserialize the actual track config");
}
