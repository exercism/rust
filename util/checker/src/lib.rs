use std::process::Command;

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
    vec![]
}
