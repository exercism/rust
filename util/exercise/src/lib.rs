#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate serde_json;
extern crate toml;

use failure::Error;
use reqwest::StatusCode;
use serde_json::Value;
use std::{
    env, fs, io,
    path::Path,
    process::{Command, Stdio},
    result,
};
use toml::Value as TomlValue;

pub type Result<T> = result::Result<T, Error>;

lazy_static! {
    pub static ref TRACK_ROOT: String = {
        let rev_parse_output = Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()
            .expect("Failed to get the path to the track repo.");

        String::from_utf8(rev_parse_output.stdout)
            .expect("git rev-parse produced non-utf8 output")
            .trim()
            .to_string()
    };
}

pub fn run_configlet_command(command: &str, args: &[&str]) {
    let track_root = &*TRACK_ROOT;

    let bin_path = Path::new(track_root).join("bin");

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
            .current_dir(track_root)
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
fn into_literal(item: &Value, use_maplit: bool) -> Result<String> {
    use std::string;
    use Value::*;
    Ok(match item {
        Null => string::String::from("None"),
        String(s) => format!("\"{}\"", s),
        Number(_) | Bool(_) => format!("{}", item),
        Array(vs) => {
            let mut items = Vec::with_capacity(vs.len());
            for im in vs.iter() {
                items.push(into_literal(im, use_maplit)?);
            }
            format!("vec![{}]", items.join(", "))
        }
        Object(m) => {
            let mut kvs = Vec::with_capacity(m.len());
            for (key, value) in m.iter() {
                if use_maplit {
                    kvs.push(format!("\"{}\"=>{}", key, into_literal(value, use_maplit)?));
                } else {
                    kvs.push(format!(
                        "hm.insert(\"{}\", {});",
                        key,
                        into_literal(value, use_maplit)?
                    ));
                }
            }
            if use_maplit {
                format!("hashmap!{{{}}}", kvs.join(","))
            } else {
                format!(
                    "{{let mut hm = ::std::collections::HashMap::new(); {} hm}}",
                    kvs.join(" "),
                )
            }
        }
    })
}

pub fn generate_test_function(case: &Value, use_maplit: bool) -> Result<String> {
    let description = case
        .get("description")
        .ok_or(format_err!("description is not available"))?
        .as_str()
        .ok_or(format_err!("description is not string"))?;

    let property = case
        .get("property")
        .ok_or(format_err!("property is not available"))?
        .as_str()
        .ok_or(format_err!("property is not string"))?;

    let comments = if let Some(comments) = case.get("comments") {
        if comments.is_array() {
            let comments_string = comments
                .as_array()
                .ok_or(format_err!("comments is not array"))?
                .iter()
                .map(|line| format!("/// {}", line))
                .collect::<String>();

            format!("\n{}", comments_string)
        } else {
            format!(
                "\n/// {}",
                comments
                    .as_str()
                    .ok_or(format_err!("comments is not string"))?
            )
        }
    } else {
        "".to_string()
    };

    let input = into_literal(
        case.get("input")
            .ok_or(format_err!("input is not available"))?,
        use_maplit,
    )?;

    let expected = into_literal(
        case.get("expected")
            .ok_or(format_err!("expected is not available"))?,
        use_maplit,
    )?;

    Ok(format!(
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
    ))
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
                .into_iter()
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
pub fn update_cargo_toml_version(exercise_name: &str, canonical_data: &Value) -> Result<()> {
    let cargo_toml_path = Path::new(&*TRACK_ROOT)
        .join("exercises")
        .join(exercise_name)
        .join("Cargo.toml");

    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)?;

    let mut cargo_toml: TomlValue = cargo_toml_content.parse()?;

    {
        let package_table = cargo_toml["package"]
            .as_table_mut()
            .ok_or(format_err!("package not a table"))?;

        package_table.insert(
            "version".to_string(),
            TomlValue::String(
                canonical_data["version"]
                    .as_str()
                    .ok_or(format_err!("version not a string"))?
                    .to_string(),
            ),
        );
    }

    fs::write(&cargo_toml_path, cargo_toml.to_string())?;

    Ok(())
}
