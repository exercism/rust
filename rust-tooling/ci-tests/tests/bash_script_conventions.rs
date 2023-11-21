use std::path::PathBuf;

use convert_case::{Case, Casing};

/// Runs a function for each bash script in the bin directory.
/// The function is passed the path of the script.
fn for_all_scripts(f: fn(&str)) {
    utils::fs::cd_into_repo_root();

    for entry in std::fs::read_dir("bin").unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // exceptions:
        // configlet is not a bash script, but it must be in `bin`.
        // fetch-configlet comes from upstream, we don't control it.
        if file_name == "configlet" || file_name == "fetch-configlet" {
            continue;
        }

        f(file_name)
    }
}

#[test]
fn file_extension() {
    for_all_scripts(|file_name| {
        assert!(
            file_name.ends_with(".sh"),
            "name of '{file_name}' should end with .sh"
        );
    })
}

#[test]
fn snake_case_name() {
    for_all_scripts(|file_name| {
        let file_name = file_name.trim_end_matches(".sh");

        assert!(
            file_name.is_case(Case::Snake),
            "name of '{file_name}' should be snake_case"
        );
    })
}

/// Notably on nixOS and macOS, bash is not installed in `/bin/bash`.
#[test]
fn portable_shebang() {
    for_all_scripts(|file_name| {
        let contents = std::fs::read_to_string(PathBuf::from("bin").join(file_name)).unwrap();
        assert!(
            contents.starts_with("#!/usr/bin/env bash"),
            "'{file_name}' should start with the shebang '#!/usr/bin/env bash'"
        );
    })
}

#[test]
fn error_handling_flags() {
    for_all_scripts(|file_name| {
        let contents = std::fs::read_to_string(PathBuf::from("bin").join(file_name)).unwrap();
        assert!(
            contents.contains("set -eo pipefail"),
            "'{file_name}' should set error handling flags 'set -eo pipefail'"
        );
    })
}
