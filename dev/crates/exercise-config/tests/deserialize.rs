use exercise_config::{
    get_all_concept_exercise_paths, get_all_practice_exercise_paths, ConceptExerciseConfig,
    PracticeExerciseConfig,
};

#[test]
fn deserialize_all() {
    for path in get_all_concept_exercise_paths() {
        let config_path = format!("{path}/.meta/config.json");
        let config_contents = std::fs::read_to_string(config_path).unwrap();
        let _: ConceptExerciseConfig = serde_json::from_str(config_contents.as_str())
            .expect("should deserialize concept exercise config");
    }
    for path in get_all_practice_exercise_paths() {
        let config_path = format!("{path}/.meta/config.json");
        let config_contents = std::fs::read_to_string(config_path).unwrap();
        let _: PracticeExerciseConfig = serde_json::from_str(config_contents.as_str())
            .expect("should deserialize practice exercise config");
    }
}
