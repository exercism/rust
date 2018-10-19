use serde_json::Value;
use std::{collections::HashSet, fs, path::Path};
use utils;

enum DiffType {
    NEW,
    UPDATED,
}

fn generate_diff_test(case: &Value, diff_type: &DiffType, use_maplit: bool) -> String {
    format!(
        "//{}\n{}",
        match diff_type {
            DiffType::NEW => "NEW",
            DiffType::UPDATED => "UPDATED",
        },
        utils::generate_test_function(case, use_maplit)
    )
}

fn generate_diff_property(property: &str) -> String {
    format!("//{}\n{}", "NEW", utils::generate_property_body(property))
}

fn generate_diffs(
    case: &Value,
    tests_content: &str,
    diffs: &mut HashSet<String>,
    use_maplit: bool,
) {
    let description = case["description"].as_str().unwrap();

    let description_formatted = utils::format_exercise_description(description);

    let diff_type = if !tests_content.contains(&format!("test_{}", description_formatted)) {
        DiffType::NEW
    } else {
        DiffType::UPDATED
    };

    if diffs.insert(generate_diff_test(&case, &diff_type, use_maplit)) {
        match diff_type {
            DiffType::NEW => println!("New test case detected: {}.", description_formatted),
            DiffType::UPDATED => println!("Updated test case: {}.", description_formatted),
        }
    }

    let property = case["property"].as_str().unwrap();

    let property_formatted = utils::format_exercise_property(property);

    if !tests_content.contains(&format!("process_{}_case", property_formatted))
        && diffs.insert(generate_diff_property(property))
    {
        println!("New property detected: {}.", property);
    }
}

fn get_diffs(exercise_name: &str, tests_content: &str, use_maplit: bool) -> HashSet<String> {
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

    let mut diffs: HashSet<String> = HashSet::new();

    for case in cases.as_array().unwrap().iter() {
        if let Some(sub_cases) = case.get("cases") {
            for sub_case in sub_cases.as_array().unwrap().iter() {
                generate_diffs(&sub_case, &tests_content, &mut diffs, use_maplit);
            }
        } else {
            generate_diffs(&case, &tests_content, &mut diffs, use_maplit);
        }
    }

    diffs
}

fn apply_diffs(exercise_name: &str, diffs: &HashSet<String>, tests_content: &str) {
    let updated_tests_content = format!(
        "{}\n{}",
        tests_content,
        diffs
            .iter()
            .map(|diff| format!("\n{}", diff))
            .collect::<String>()
    );

    let tests_path = Path::new(&utils::get_track_root())
        .join("exercises")
        .join(exercise_name)
        .join("tests")
        .join(format!("{}.rs", exercise_name));

    fs::write(&tests_path, updated_tests_content.as_bytes()).unwrap_or_else(|_| {
        panic!(
            "Failed to update tests file for the '{}' exercise. Aborting.",
            exercise_name,
        )
    });

    utils::rustfmt(&tests_path);
}

pub fn update_exercise(exercise_name: &str, use_maplit: bool) {
    if !utils::exercise_exists(exercise_name) {
        panic!(
            "Exercise with the name '{}' does not exists. Aborting",
            exercise_name
        );
    }

    let tests_content = utils::get_tests_content(exercise_name).unwrap_or_else(|_| {
        panic!(
            "Failed to get test content for the '{}' exercise",
            exercise_name
        )
    });

    let diffs = get_diffs(exercise_name, &tests_content, use_maplit);

    apply_diffs(exercise_name, &diffs, &tests_content);
}
