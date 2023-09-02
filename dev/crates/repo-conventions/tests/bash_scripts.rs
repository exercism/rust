use std::path::PathBuf;

/// Runs a function for each bash script in the scripts directory.
/// The function is passed the path of the script.
fn for_all_scripts(f: fn(PathBuf)) {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let scripts_dir = format!("{crate_dir}/../../scripts");

    for entry in std::fs::read_dir(scripts_dir).unwrap() {
        f(entry.unwrap().path())
    }
}

#[test]
fn test_file_extension() {
    for_all_scripts(|path| {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        assert!(
            file_name.ends_with(".sh"),
            "name of '{file_name}' should end with .sh"
        );
    })
}

#[test]
fn test_snake_case_name() {
    for_all_scripts(|path| {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        assert!(
            file_name.to_lowercase() == file_name && !file_name.contains('-'),
            "name of '{file_name}' should be snake_case"
        );
    })
}

/// Notably on nixOS and macOS, bash is not installed in `/bin/bash`.
#[test]
fn test_portable_shebang() {
    for_all_scripts(|path| {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(
            contents.starts_with("#!/usr/bin/env bash"),
            "'{file_name}' should start with the shebang '#!/usr/bin/env bash'"
        );
    })
}
