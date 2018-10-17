use serde_json::Value;
use std::path::Path;
use utils;

fn exercise_exists(exercise_name: &str) -> bool {
    let track_root = utils::get_track_root();

    let exercises_path = Path::new(&track_root).join("exercises");

    for entry in exercises_path
        .read_dir()
        .expect("Failed to read 'exercises' dir")
    {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_dir() && entry.file_name() == exercise_name {
                return true;
            }
        }
    }

    false
}

fn generate_diff_test(_case: &Value, _diff_prefix: &str) -> String {
    String::new()
}

fn generate_diff_property(_case: &Value, _diff_prefix: &str) -> String {
    String::new()
}

fn find_diffs(case: &Value, tests_content: &str, diffs: &mut Vec<String>) {
    let description = case["description"].as_str().unwrap();

    let diff_prefix = if !tests_content.contains(&format!(
        "test_{}",
        utils::format_exercise_description(description)
    )) {
        "NEW"
    } else {
        "UPDATED"
    };

    diffs.push(generate_diff_test(&case, diff_prefix));

    let property = case["property"].as_str().unwrap();

    if !tests_content.contains(&format!(
        "process_{}_case",
        utils::format_exercise_property(property)
    )) {
        diffs.push(generate_diff_property(&case, "NEW"));
    }
}

fn apply_diffs(exercise_name: &str) {
    let canonical_data = utils::get_canonical_data(exercise_name).unwrap_or_else(|| {
        panic!(
            "Failed to get canonical data for the '{}' exercise. Aborting",
            exercise_name
        )
    });

    let cases = canonical_data.get("cases").unwrap_or_else(|| {
        panic!(
            "Failed to get 'cases' field from the canonical data of the '{}' exercise",
            exercise_name
        )
    });

    let tests_content = utils::get_tests_content(exercise_name).unwrap_or_else(|_| {
        panic!(
            "Failed to get test content for the '{}' exercise",
            exercise_name
        )
    });

    let mut diffs: Vec<String> = vec![];

    for case in cases.as_array().unwrap().iter() {
        if let Some(sub_cases) = case.get("cases") {
            for sub_case in sub_cases.as_array().unwrap().iter() {
                find_diffs(&sub_case, &tests_content, &mut diffs);
            }
        } else {
            find_diffs(&case, &tests_content, &mut diffs);
        }
    }
}

pub fn update_exercise(exercise_name: &str) {
    if !exercise_exists(exercise_name) {
        panic!(
            "Exercise with the name '{}' does not exists. Aborting",
            exercise_name
        );
    }

    apply_diffs(exercise_name);
}
