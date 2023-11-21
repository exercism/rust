use std::path::Path;

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
fn no_trailing_whitespace() {
    utils::fs::cd_into_repo_root();

    for entry in ignore::Walk::new("./") {
        let entry = entry.unwrap();
        if !entry.file_type().is_some_and(|t| t.is_file()) {
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
