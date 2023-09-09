use std::path::Path;

use exercism_tooling::fs_utils;
use walkdir::WalkDir;

fn contains_trailing_whitespace(p: &Path) -> bool {
    let contents = std::fs::read_to_string(p).unwrap();
    for line in contents.lines() {
        if line != line.trim_end() {
            return true;
        }
    }
    false
}

#[test]
fn test_no_trailing_whitespace() {
    fs_utils::cd_into_repo_root();

    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let ext = path.extension().unwrap_or_default().to_str().unwrap();
        if matches!(ext, "rs" | "toml" | "md" | "sh") {
            assert!(
                !contains_trailing_whitespace(path),
                "trailing whitespace in {}",
                path.display()
            );
        }
    }
}
