use models::exercise_config::{
    get_all_concept_exercise_paths, get_all_practice_exercise_paths, PracticeExercise,
};

fn assert_one_less_ignore_than_tests(path: &str) {
    let slug = path.split('/').last().unwrap();
    let test_path = format!("{path}/tests/{slug}.rs");
    let test_contents = std::fs::read_to_string(test_path).unwrap();
    let num_tests = test_contents.matches("#[test]").count();
    let num_ignores = test_contents.matches("#[ignore]").count();
    assert_eq!(
        num_tests,
        num_ignores + 1,
        "should have one more test than ignore in {slug}"
    )
}

#[test]
fn count_ignores() {
    for path in get_all_concept_exercise_paths() {
        assert_one_less_ignore_than_tests(&path);
    }
    for path in get_all_practice_exercise_paths() {
        let config_path = format!("{path}/.meta/config.json");
        let config_contents = std::fs::read_to_string(config_path).unwrap();
        let config: PracticeExercise = serde_json::from_str(config_contents.as_str()).unwrap();
        if let Some(custom) = config.custom {
            if custom.ignore_count_ignores.unwrap_or_default() {
                continue;
            }
        }
        assert_one_less_ignore_than_tests(&path);
    }
}
