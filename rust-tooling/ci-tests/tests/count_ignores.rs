use models::exercise_config::get_all_exercise_paths;

#[test]
fn count_ignores() {
    for path in get_all_exercise_paths() {
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
}
