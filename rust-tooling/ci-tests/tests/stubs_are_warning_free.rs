use std::process::{Command, Stdio};

use glob::glob;

#[test]
fn stubs_are_warning_free() {
    utils::fs::cd_into_repo_root();

    let temp_dir = tempfile::tempdir().unwrap();

    let mut handles = vec![];

    for manifest in glob("exercises/*/*/Cargo.toml")
        .unwrap()
        .map(Result::unwrap)
    {
        if std::fs::read_to_string(&manifest.parent().unwrap().join(".meta").join("config.json"))
            .unwrap()
            .contains("allowed-to-not-compile")
        {
            continue;
        }
        let slug = manifest
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let handle = Command::new("cargo")
            .args([
                "clippy",
                "--quiet",
                "--manifest-path",
                &manifest.display().to_string(),
                "--target-dir",
                &temp_dir.path().join(slug).display().to_string(),
                "--",
                // necessary for clippy to return a non-zero exit code
                // if it finds warnings
                "--deny",
                "warnings",
            ])
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        handles.push((slug.to_string(), handle));
    }

    let mut log = String::new();

    for (slug, handle) in handles {
        let output = handle.wait_with_output().unwrap();

        if output.status.success() {
            continue;
        }
        let stderr = String::from_utf8(output.stderr).unwrap();
        log.push_str(&format!(
            "\
################################################################
################
################    {slug}

{stderr}

"
        ));
    }

    if !log.is_empty() {
        std::fs::write("clippy.log", &log).expect("should write clippy.log");
    }
    assert!(
        log.is_empty(),
        "
    ╔═════════════════════════════════════════╗
    ║ clippy found warnings, check clippy.log ║
    ╚═════════════════════════════════════════╝
    "
    );
}
