//! Make sure exercise difficulties are set correctly

use track_config::TrackConfig;

static CONFIG: &str = include_str!("../../../../config.json");

#[test]
fn test_difficulties_are_valid() {
    let config: TrackConfig = serde_json::from_str(CONFIG).unwrap();

    let mut difficulties = config
        .exercises
        .concept
        .iter()
        .map(|e| e.difficulty)
        .chain(config.exercises.practice.iter().map(|e| e.difficulty));

    if difficulties.any(|d| !matches!(d, 1 | 4 | 7 | 10)) {
        panic!("Exercises with invalid difficulty (must be in {{1, 4, 7, 10}})")
    }
}
