//! Make sure exercise difficulties are set correctly

use track_config::TRACK_CONFIG;

#[test]
fn test_difficulties_are_valid() {
    let mut difficulties = TRACK_CONFIG
        .exercises
        .concept
        .iter()
        .map(|e| e.difficulty)
        .chain(TRACK_CONFIG.exercises.practice.iter().map(|e| e.difficulty));

    assert!(
        difficulties.all(|d| matches!(d, 1 | 4 | 7 | 10)),
        "exercises must have a difficulty of 1, 4, 7, or 10"
    )
}
