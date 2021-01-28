pub mod cmd;
pub mod errors;
pub mod structs;

use errors::Result;
use failure::format_err;
use lazy_static::lazy_static;
use reqwest;
use serde_json::Value;
use std::{
    collections::HashMap,
    env, fs, io,
    path::Path,
    process::{Command, Stdio},
};
use structs::{CanonicalData, LabeledTest};
use tera::{Context, Tera};
use toml;
use toml::Value as TomlValue;

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

// Create a static `Tera` struct so we can access the templates from anywhere.
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let templates = Path::new(&*TRACK_ROOT)
            .join("util")
            .join("exercise")
            .join("src")
            .join("cmd")
            .join("templates")
            .join("**")
            .join("*.rs");

        // Since `TRACK_ROOT` already checks for UTF-8 and nothing added is not
        // UTF-8, unwrapping is fine.
        let mut tera = match Tera::new(templates.to_str().unwrap()) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        // Build wrappers around the formatting functions.
        let format_description = |args: &HashMap<String, Value>|
            args.get("description")
                .and_then(Value::as_str)
                .map(format_exercise_description)
                .map(Value::from)
                .ok_or(tera::Error::from("Problem formatting the description."))
        ;

        let format_property = |args: &HashMap<String, Value>|
            args.get("property")
                .and_then(Value::as_str)
                .map(format_exercise_property)
                .map(Value::from)
                .ok_or(tera::Error::from("Problem formatting the property."))
        ;

        tera.register_function("format_description", format_description);
        tera.register_function("format_property", format_property);
        tera
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

        let bin_path = crate::cmd::fetch_configlet::download()?;

        if bin_path.join(configlet_name_unix).exists() {
            configlet_name_unix
        } else if bin_path.join(configlet_name_windows).exists() {
            configlet_name_windows
        } else {
            return Err(format_err!(
                "could not locate configlet after running bin/fetch-configlet"
            )
            .into());
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
        "https://raw.githubusercontent.com/exercism/problem-specifications/main/exercises/{}/{}",
        exercise, file,
    )
}

fn get_canonical(exercise: &str, file: &str) -> Result<reqwest::Response> {
    reqwest::get(&url_for(exercise, file)).map_err(|e| e.into())
}

// Try to get the canonical data for the exercise of the given name
pub fn get_canonical_data(exercise_name: &str) -> Result<CanonicalData> {
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

pub fn generate_property_body(property: &str) -> Result<String> {
    let mut context = Context::new();
    context.insert("property", property);
    TEMPLATES
        .render("property_fn.rs", &context)
        .map_err(|e| e.into())
}

pub fn generate_test_function(case: &LabeledTest, use_maplit: bool) -> Result<String> {
    let mut context = Context::from_serialize(case)?;
    context.insert("use_maplit", &use_maplit);
    TEMPLATES
        .render("test_fn.rs", &context)
        .map_err(|e| e.into())
}

pub fn rustfmt(file_path: &Path) -> Result<()> {
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
pub fn update_cargo_toml_version(
    exercise_name: &str,
    canonical_data: &CanonicalData,
) -> Result<()> {
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
            TomlValue::String(canonical_data.version.to_string()),
        );
    }

    fs::write(&cargo_toml_path, cargo_toml.to_string())?;

    Ok(())
}
