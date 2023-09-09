use std::path::PathBuf;

use exercism_tooling::bin_utils;

/// Runs a function for each bash script in the bin directory.
/// The function is passed the path of the script.
fn for_all_scripts(f: fn(PathBuf)) {
    bin_utils::cd_into_repo_root();

    for entry in std::fs::read_dir("bin").unwrap() {
        f(entry.unwrap().path())
    }
}

#[test]
fn test_file_extension() {
    for_all_scripts(|path| {
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // exceptions
        if file_name == "fetch-configlet" || file_name == "configlet" {
            return;
        }

        assert!(
            file_name.ends_with(".sh"),
            "name of '{file_name}' should end with .sh"
        );
    })
}

#[test]
fn test_snake_case_name() {
    for_all_scripts(|path| {
        let file_name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".sh");

        // fetch-configlet comes from upstream, we don't control its name
        if file_name == "fetch-configlet" {
            return;
        }

        assert!(
            file_name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '_'),
            "name of '{file_name}' should be snake_case"
        );
    })
}

/// Notably on nixOS and macOS, bash is not installed in `/bin/bash`.
#[test]
fn test_portable_shebang() {
    for_all_scripts(|path| {
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // not a bash script, but it must be in `bin`
        if file_name == "configlet" {
            return;
        }

        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(
            contents.starts_with("#!/usr/bin/env bash"),
            "'{file_name}' should start with the shebang '#!/usr/bin/env bash'"
        );
    })
}
