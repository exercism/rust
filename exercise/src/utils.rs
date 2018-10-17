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
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .replace(" ", "_")
        .to_lowercase()
}

pub fn format_exercise_property(property: &str) -> String {
    property.replace(" ", "_").to_lowercase()
}
