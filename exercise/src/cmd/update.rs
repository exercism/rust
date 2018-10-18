use serde_json::Value;
use std::{collections::HashSet, path::Path};
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

fn generate_diff_test(case: &Value, diff_prefix: &str) -> String {
    // FIXME: Add use_maplit arg
    format!(
        "//{}\n{}",
        diff_prefix,
        utils::generate_test_function(case, false)
    )
}

fn generate_diff_property(property: &str, diff_prefix: &str) -> String {
    format!(
        "//{}\n{}",
        diff_prefix,
        utils::generate_property_body(property)
    )
}

fn generate_diffs(case: &Value, tests_content: &str, diffs: &mut HashSet<String>) {
    let description = case["description"].as_str().unwrap();

    let description_formatted = utils::format_exercise_description(description);

    let diff_prefix = if !tests_content.contains(&format!("test_{}", description_formatted)) {
        "NEW"
    } else {
        "UPDATED"
    };

    if diffs.insert(generate_diff_test(&case, diff_prefix)) {
        if diff_prefix == "NEW" {
            println!("New test case detected: {}.", description_formatted);
        } else {
            println!("Updated test case: {}.", description_formatted);
        }
    }

    let property = case["property"].as_str().unwrap();

    let property_formatted = utils::format_exercise_property(property);

    if !tests_content.contains(&format!("process_{}_case", property_formatted))
        && diffs.insert(generate_diff_property(property, "NEW"))
    {
        println!("New property detected: {}.", property);
    }
}

fn get_diffs(exercise_name: &str) -> HashSet<String> {
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

    let mut diffs: HashSet<String> = HashSet::new();

    for case in cases.as_array().unwrap().iter() {
        if let Some(sub_cases) = case.get("cases") {
            for sub_case in sub_cases.as_array().unwrap().iter() {
                generate_diffs(&sub_case, &tests_content, &mut diffs);
            }
        } else {
            generate_diffs(&case, &tests_content, &mut diffs);
        }
    }

    diffs
}

pub fn update_exercise(exercise_name: &str) {
    if !exercise_exists(exercise_name) {
        panic!(
            "Exercise with the name '{}' does not exists. Aborting",
            exercise_name
        );
    }

    let _diffs = get_diffs(exercise_name);
}
