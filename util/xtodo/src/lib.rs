use std::process::Command;

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
