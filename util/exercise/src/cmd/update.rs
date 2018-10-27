use exercise::Result;
use serde_json::Value;
use std::{collections::HashSet, fs, path::Path};

enum DiffType {
    NEW,
    UPDATED,
}

fn generate_diff_test(case: &Value, diff_type: &DiffType, use_maplit: bool) -> Result<String> {
    Ok(format!(
        "//{}\n{}",
        match diff_type {
            DiffType::NEW => "NEW",
            DiffType::UPDATED => "UPDATED",
        },
        exercise::generate_test_function(case, use_maplit)?
    ))
}

fn generate_diff_property(property: &str) -> String {
    format!(
        "//{}\n{}",
        "NEW",
        exercise::generate_property_body(property)
    )
}

fn generate_diffs(
    case: &Value,
    tests_content: &str,
    diffs: &mut HashSet<String>,
    use_maplit: bool,
) -> Result<()> {
    let description = case["description"]
        .as_str()
        .ok_or(format_err!("description property not string"))?;

    let description_formatted = exercise::format_exercise_description(description);

    let diff_type = if !tests_content.contains(&format!("test_{}", description_formatted)) {
        DiffType::NEW
    } else {
        DiffType::UPDATED
    };

    if diffs.insert(generate_diff_test(&case, &diff_type, use_maplit)?) {
        match diff_type {
            DiffType::NEW => println!("New test case detected: {}.", description_formatted),
            DiffType::UPDATED => println!("Updated test case: {}.", description_formatted),
        }
    }

    let property = case["property"]
        .as_str()
        .ok_or(format_err!("case property not string"))?;

    let property_formatted = exercise::format_exercise_property(property);

    if !tests_content.contains(&format!("process_{}_case", property_formatted))
        && diffs.insert(generate_diff_property(property))
    {
        println!("New property detected: {}.", property);
    }

    Ok(())
}

fn get_diffs(
    exercise_name: &str,
    canonical_data: &Value,
    tests_content: &str,
    use_maplit: bool,
) -> Result<HashSet<String>> {
    let cases = canonical_data.get("cases").ok_or(format_err!(
        "Failed to get 'cases' field from the canonical data of the '{}' exercise",
        exercise_name
    ))?;

    let mut diffs: HashSet<String> = HashSet::new();

    for case in cases
        .as_array()
        .ok_or(format_err!("case property not array"))?
        .iter()
    {
        if let Some(sub_cases) = case.get("cases") {
            for sub_case in sub_cases
                .as_array()
                .ok_or(format_err!("subcase property not array"))?
                .iter()
            {
                generate_diffs(&sub_case, &tests_content, &mut diffs, use_maplit)?;
            }
        } else {
            generate_diffs(&case, &tests_content, &mut diffs, use_maplit)?;
        }
    }

    Ok(diffs)
}

fn apply_diffs(exercise_name: &str, diffs: &HashSet<String>, tests_content: &str) -> Result<()> {
    let updated_tests_content = format!(
        "{}\n{}",
        tests_content,
        diffs
            .iter()
            .map(|diff| format!("\n{}", diff))
            .collect::<String>()
    );

    let tests_path = Path::new(&*exercise::TRACK_ROOT)
        .join("exercises")
        .join(exercise_name)
        .join("tests")
        .join(format!("{}.rs", exercise_name));

    fs::write(&tests_path, updated_tests_content.as_bytes())?;
    exercise::rustfmt(&tests_path)?;

    Ok(())
}

pub fn update_exercise(exercise_name: &str, use_maplit: bool) -> Result<()> {
    if !exercise::exercise_exists(exercise_name) {
        return Err(format_err!("exercise with the name '{}' does not exist", exercise_name).into());
    }

    let tests_content = exercise::get_tests_content(exercise_name)?;
    let canonical_data = exercise::get_canonical_data(exercise_name)?;
    let diffs = get_diffs(exercise_name, &canonical_data, &tests_content, use_maplit)?;

    apply_diffs(exercise_name, &diffs, &tests_content)?;
    exercise::update_cargo_toml_version(exercise_name, &canonical_data)?;

    Ok(())
}
