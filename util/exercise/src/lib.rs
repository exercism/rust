#[macro_use]
extern crate failure;
extern crate failure_derive;
extern crate flate2;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate serde_json;
extern crate tar;
extern crate toml;

use serde_json::Value;
use std::{
    env, fs, io,
    path::Path,
    process::{Command, Stdio},
};
use toml::Value as TomlValue;

pub mod errors;
pub mod fetch_configlet;
pub use errors::Result;

// we look for the track root in various places, but it's never going to change
// we therefore cache the value for efficiency
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

#[macro_export]
macro_rules! val_as {
    ($value:expr, $as:ident) => {
        $value
            .$as()
            .ok_or_else(|| $crate::errors::Error::SchemaTypeError {
                file: "config.json".to_string(),
                field: stringify!($name).to_string(),
                as_type: stringify!($as)[3..].to_string(),
            })?
    };
}

#[macro_export]
macro_rules! get {
    ($value:expr, $name:expr) => {
        $value
            .get($name)
            .ok_or_else(|| $crate::errors::Error::ConfigJsonSchemaError {
                parent: stringify!($value).to_string(),
                field: stringify!($name).to_string(),
            })?
    };
    ($value:expr, $name:expr, $as:ident) => {
        val_as!(get!($value, $name), $as)
    };
}

#[macro_export]
macro_rules! get_mut {
    ($value:expr, $name:expr) => {
        $value
            .get_mut($name)
            .ok_or_else(|| $crate::errors::Error::ConfigJsonSchemaError {
                parent: stringify!($value).to_string(),
                field: stringify!($name).to_string(),
            })?
    };
    ($value:expr, $name:expr, $as:ident) => {
        val_as!(get_mut!($value, $name), $as)
    };
}

pub fn run_configlet_command(command: &str, args: &[&str]) -> Result<()> {
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

        let bin_path = fetch_configlet::download()?;

        if bin_path.join(configlet_name_unix).exists() {
            configlet_name_unix
        } else if bin_path.join(configlet_name_windows).exists() {
            configlet_name_windows
        } else {
            return Err(
                format_err!("could not locate configlet after running bin/fetch-configlet").into(),
            );
        }
    };

    Command::new(&bin_path.join(configlet_name))
        .current_dir(track_root)
        .stdout(Stdio::inherit())
        .arg(command)
        .args(args)
        .output()?;

    Ok(())
}

fn url_for(exercise: &str, file: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/{}/{}",
        exercise, file,
    )
}

fn get_canonical(exercise: &str, file: &str) -> Result<reqwest::Response> {
    reqwest::get(&url_for(exercise, file)).map_err(|e| e.into())
}

// Try to get the canonical data for the exercise of the given name
pub fn get_canonical_data(exercise_name: &str) -> Result<Value> {
    let mut response = get_canonical(exercise_name, "canonical-data.json")?.error_for_status()?;
    response.json().map_err(|e| e.into())
}

pub fn canonical_file_exists(exercise: &str, file: &str) -> Result<bool> {
    Ok(get_canonical(exercise, file)?.status().is_success())
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
    let description = get!(case, "description", as_str);
    let property = get!(case, "property", as_str);
    let comments = if let Some(comments) = case.get("comments") {
        use Value::*;
        match comments {
            Array(cs) => cs
                .iter()
                .map(|line| format!("/// {}", line))
                .collect::<Vec<_>>()
                .join("\n"),
            String(s) => format!("\n/// {}", s),
            _ => {
                return Err(errors::Error::SchemaTypeError {
                    file: "config.json".to_string(),
                    field: "comments".to_string(),
                    as_type: "string or array".to_string(),
                })
            }
        }
    } else {
        "".to_string()
    };

    let input = into_literal(get!(case, "input"), use_maplit)?;
    let expected = into_literal(get!(case, "expected"), use_maplit)?;

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

pub fn rustfmt(file_path: &Path) -> Result<()> {
    if let Ok(which_output) = Command::new("which").arg("rustfmt").output() {
        if !String::from_utf8_lossy(&which_output.stdout)
            .trim()
            .is_empty()
        {}
    }

    let rustfmt_is_available = {
        if let Some(path_var) = env::var_os("PATH") {
            env::split_paths(&path_var).any(|path| path.join("rustfmt").exists())
        } else {
            false
        }
    };

    if rustfmt_is_available {
        Command::new("rustfmt").arg(file_path).output()?;
    }

    Ok(())
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
        let package_table =
            cargo_toml["package"]
                .as_table_mut()
                .ok_or_else(|| errors::Error::SchemaTypeError {
                    file: "Config.toml".to_string(),
                    field: "package".to_string(),
                    as_type: "table".to_string(),
                })?;

        package_table.insert(
            "version".to_string(),
            TomlValue::String(get!(canonical_data, "version", as_str).to_string()),
        );
    }

    fs::write(&cargo_toml_path, cargo_toml.to_string())?;

    Ok(())
}
