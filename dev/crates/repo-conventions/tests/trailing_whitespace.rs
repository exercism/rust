use std::path::Path;

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
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let repo_dir = format!("{crate_dir}/../../..");

    for entry in WalkDir::new(repo_dir) {
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
