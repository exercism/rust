use reqwest::{self, StatusCode};
use serde_json::Value;
use std::{
    fs, io,
    path::Path,
    process::{Command, Stdio},
};

pub fn get_track_root() -> String {
    let rev_parse_output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .expect("Failed to get the path to the track repo.");

    let track_root = String::from_utf8(rev_parse_output.stdout).unwrap();

    track_root.trim().to_string()
}

pub fn run_configlet_command(command: &str, args: &[&str]) {
    let track_root = get_track_root();

    let bin_path = Path::new(&track_root).join("bin");

    let configlet_name_unix = "configlet";

    let configlet_name_windows = "configlet.exe";

    let configlet_name = if bin_path.join(configlet_name_unix).exists() {
        configlet_name_unix
    } else if bin_path.join(configlet_name_windows).exists() {
        configlet_name_windows
    } else {
        println!("Configlet not found in the bin directory. Running bin/fetch-configlet.");

        // FIXME: Uses bash script that would not work on Windows.
        // RIIR is preferred.
        Command::new("bash")
            .current_dir(&track_root)
            .stdout(Stdio::inherit())
            .arg(bin_path.join("fetch-configlet"))
            .output()
            .expect("Failed to run fetch-configlet script");

        if bin_path.join(configlet_name_unix).exists() {
            configlet_name_unix
        } else if bin_path.join(configlet_name_windows).exists() {
            configlet_name_windows
        } else {
            panic!("Could not locate configlet after running bin/fetch-configlet. Aborting");
        }
    };

    Command::new(&bin_path.join(configlet_name))
        .current_dir(&track_root)
        .stdout(Stdio::inherit())
        .arg(command)
        .args(args)
        .output()
        .expect("Failed to run configlet generate command");
}

// Try to get the canonical data for the exercise of the given name
pub fn get_canonical_data(exercise_name: &str) -> Option<Value> {
    let url = format!("https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/{}/canonical-data.json", exercise_name);

    let mut response =
        reqwest::get(&url).expect("Failed to make HTTP request for the canonical data.");

    if response.status() != StatusCode::Ok {
        None
    } else {
        Some(
            response
                .json()
                .expect("Failed to parse the JSON canonical-data response"),
        )
    }
}

pub fn get_tests_content(exercise_name: &str) -> io::Result<String> {
    let track_root = get_track_root();

    let tests_path = Path::new(&track_root)
        .join("exercises")
        .join(exercise_name)
        .join("tests")
        .join(format!("{}.rs", exercise_name));

    fs::read_to_string(tests_path)
}

pub fn format_exercise_description(description: &str) -> String {
    description
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect::<String>()
        .replace(" ", "_")
        .to_lowercase()
}

pub fn format_exercise_property(property: &str) -> String {
    property.replace(" ", "_").to_lowercase()
}

pub fn generate_property_body(property: &str) -> String {
    format!(
        "\
         /// Process a single test case for the property `{property}`\n\
         ///\n\
         /// All cases for the `{property}` property are implemented\n\
         /// in terms of this function.\n\
         /// \n\
         /// Note that you'll need to both name the expected transform which\n\
         /// the student needs to write, and name the types of the inputs and outputs.\n\
         /// While rustc _may_ be able to handle things properly given a working example,\n\
         /// students will face confusing errors if the `I` and `O` types are not concrete.\n\
         /// \n\
         fn process_{property_formatted}_case<I, O>(input: I, expected: O) {{\n\
         //  typical implementation:\n\
         //  assert_eq!(\n\
         //      student_{property_formatted}_func(input),\n\
         //      expected\n\
         //  )\n    unimplemented!()\n\
         }}\n\
         \n\
         ",
        property = property,
        property_formatted = format_exercise_property(property),
    )
}

// Depending on the type of the item variable,
// transform item into corresponding Rust literal
fn into_literal(item: &Value, use_maplit: bool) -> String {
    if item.is_string() {
        format!("\"{}\"", item.as_str().unwrap())
    } else if item.is_array() {
        format!(
            "vec![{}]",
            item.as_array()
                .unwrap()
                .iter()
                .map(|item| into_literal(item, use_maplit))
                .collect::<Vec<String>>()
                .join(", ")
        )
    } else if item.is_number() || item.is_boolean() || item.is_null() {
        format!("{}", item)
    } else if !use_maplit {
        let key_values = item
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| {
                format!(
                    "hm.insert(\"{}\", {});",
                    key,
                    into_literal(value, use_maplit)
                )
            }).collect::<String>();

        format!(
            "{{let mut hm = ::std::collections::HashMap::new(); {} hm}}",
            key_values
        )
    } else {
        let key_values = item
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| format!("\"{}\"=>{}", key, into_literal(value, use_maplit)))
            .collect::<Vec<String>>()
            .join(",");

        format!("hashmap!{{{}}}", key_values)
    }
}

pub fn generate_test_function(case: &Value, use_maplit: bool) -> String {
    let description = case.get("description").unwrap().as_str().unwrap();

    let property = case.get("property").unwrap().as_str().unwrap();

    let comments = if let Some(comments) = case.get("comments") {
        if comments.is_array() {
            comments
                .as_array()
                .unwrap()
                .iter()
                .map(|line| format!("/// {}", line))
                .collect::<String>()
        } else {
            format!("/// {}\n", comments.as_str().unwrap())
        }
    } else {
        "".to_string()
    };

    let input = into_literal(case.get("input").unwrap(), use_maplit);

    let expected = into_literal(case.get("expected").unwrap(), use_maplit);

    format!(
        "#[test]\n\
         #[ignore]\n\
         /// {description}\n\
         {comments}
         fn test_{description_formatted}() {{\n\
         process_{property}_case({input}, {expected});\n\
         }}\n\
         \n\
         ",
        description = description,
        description_formatted = format_exercise_description(description),
        property = format_exercise_property(property),
        comments = comments,
        input = input,
        expected = expected
    )
}

// FIXME: The algorithm is Unix-specific and will always fail on Windows. General solution required
pub fn rustfmt(file_path: &Path) {
    if let Ok(which_output) = Command::new("which").arg("rustfmt").output() {
        if !String::from_utf8_lossy(&which_output.stdout)
            .trim()
            .is_empty()
        {
            Command::new("rustfmt")
                .arg(file_path)
                .output()
                .expect("Failed to run rustfmt command on the test suite file");
        }
    }
}

pub fn exercise_exists(exercise_name: &str) -> bool {
    let track_root = get_track_root();

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
