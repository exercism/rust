use std::{path::Path, process::Command};

fn get_track_root() -> String {
    let rev_parse_output = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()
        .expect("Failed to execute 'git rev-parse --show-toplevel' command");

    String::from_utf8(rev_parse_output.stdout)
        .expect("Failed to convert 'git rev-parse' output into UTF-8 String")
        .trim()
        .to_string()
}

pub fn get_all_exercises() -> Vec<String> {
    let exercises_path = Path::new(&get_track_root()).join("exercises");

    exercises_path
        .read_dir()
        .unwrap_or_else(|_| panic!("Failed to read {:?} directory", &exercises_path))
        .filter(|entry| entry.is_ok() && entry.as_ref().unwrap().path().is_dir())
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect()
}

pub fn get_current_branch_name() -> String {
    let rev_parse_output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Failed to execute 'git rev-parse --abbrev-ref HEAD' command");

    String::from_utf8(rev_parse_output.stdout)
        .expect("Failed to convert 'git rev-parse' output into UTF-8 String")
        .trim()
        .to_string()
}

fn get_modifications() -> Vec<String> {
    let diff_output = Command::new("git")
        .args(&["diff", "--name-only", "master"])
        .output()
        .expect("Failed to execute 'git diff --name-only master' command");

    String::from_utf8(diff_output.stdout)
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| x.to_owned())
        .collect()
}

pub fn get_modified_exercises() -> Vec<String> {
    let modifications = get_modifications();

    modifications
        .iter()
        .filter(|modification| modification.contains("exercises/"))
        .map(|modified_exercise| modified_exercise.to_string())
        .collect::<Vec<String>>()
}
