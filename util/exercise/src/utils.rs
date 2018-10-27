use fetch_configlet;
use reqwest::{self, StatusCode};
use serde_json::Value;
use std::{
    env, fs, io,
    path::Path,
    process::{Command, Stdio},
};
use toml::Value as TomlValue;

lazy_static! {
    pub static ref TRACK_ROOT: String = {
        let rev_parse_output = Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()
            .expect("Failed to get the path to the track repo.");

        String::from_utf8(rev_parse_output.stdout)
            .unwrap()
            .trim()
            .to_string()
    };
}

pub fn run_configlet_command(command: &str, args: &[&str]) {
    let track_root = &*TRACK_ROOT;

    let mut bin_path = Path::new(track_root).join("bin");

    let configlet_name_unix = "configlet";

    let configlet_name_windows = "configlet.exe";

    let configlet_name = if bin_path.join(configlet_name_unix).exists() {
        configlet_name_unix
    } else if bin_path.join(configlet_name_windows).exists() {
        configlet_name_windows
    } else {
        println!("Configlet not found in the bin directory. Running bin/fetch-configlet.");

        if let Ok(path) = fetch_configlet::download() {
            bin_path = path;
        } else {
            panic!("Could not fetch configlet. Aborting");
        }

        if bin_path.join(configlet_name_unix).exists() {
            configlet_name_unix
        } else if bin_path.join(configlet_name_windows).exists() {
            configlet_name_windows
        } else {
            panic!("Could not locate configlet after running bin/fetch-configlet. Aborting");
        }
    };

    Command::new(&bin_path.join(configlet_name))
        .current_dir(track_root)
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
    let tests_path = Path::new(&*TRACK_ROOT)
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
            })
            .collect::<String>();

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
            let comments_string = comments
                .as_array()
                .unwrap()
                .iter()
                .map(|line| format!("/// {}", line))
                .collect::<String>();

            format!("\n{}", comments_string)
        } else {
            format!("\n/// {}", comments.as_str().unwrap())
        }
    } else {
        "".to_string()
    };

    let input = into_literal(case.get("input").unwrap(), use_maplit);

    let expected = into_literal(case.get("expected").unwrap(), use_maplit);

    format!(
        "#[test]\n\
         #[ignore]\n\
         /// {description}{comments}\n\
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

pub fn rustfmt(file_path: &Path) {
    if let Ok(which_output) = Command::new("which").arg("rustfmt").output() {
        if !String::from_utf8_lossy(&which_output.stdout)
            .trim()
            .is_empty()
        {}
    }

    let rustfmt_is_available = {
        if let Some(path_var) = env::var_os("PATH") {
            env::split_paths(&path_var)
                .any(|path| path.join("rustfmt").exists())
        } else {
            false
        }
    };

    if rustfmt_is_available {
        Command::new("rustfmt")
            .arg(file_path)
            .output()
            .expect("Failed to run rustfmt command on the test suite file");
    }
}

pub fn exercise_exists(exercise_name: &str) -> bool {
    Path::new(&*TRACK_ROOT)
        .join("exercises")
        .join(exercise_name)
        .exists()
}

// Update the version of the specified exercise in the Cargo.toml file according to the passed canonical data
pub fn update_cargo_toml_version(exercise_name: &str, canonical_data: &Value) {
    let cargo_toml_path = Path::new(&*TRACK_ROOT)
        .join("exercises")
        .join(exercise_name)
        .join("Cargo.toml");

    let cargo_toml_content = fs::read_to_string(&cargo_toml_path).unwrap_or_else(|_| {
        panic!(
            "Failed to read the contents of the {} file",
            cargo_toml_path.to_str().unwrap()
        )
    });

    let mut cargo_toml: TomlValue = cargo_toml_content.parse().unwrap();

    {
        let package_table = cargo_toml["package"].as_table_mut().unwrap();

        package_table.insert(
            "version".to_string(),
            TomlValue::String(canonical_data["version"].as_str().unwrap().to_string()),
        );
    }

    fs::write(&cargo_toml_path, cargo_toml.to_string()).unwrap_or_else(|_| {
        panic!(
            "Failed to update the contents of the {} file",
            cargo_toml_path.to_str().unwrap()
        );
    });
}
