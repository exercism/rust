extern crate serde_json;

use serde_json::Value;
use std::{fs, path::Path, process::Command};

pub fn get_track_root() -> String {
    let rev_parse_output = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()
        .unwrap();

    String::from_utf8(rev_parse_output.stdout)
        .unwrap()
        .trim()
        .to_string()
}

pub fn get_config_value() -> Value {
    let track_root = get_track_root();

    let config_path = Path::new(&track_root).join("config.json");

    let config_content = fs::read_to_string(config_path).unwrap();

    serde_json::from_str(&config_content).unwrap()
}
