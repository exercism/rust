pub fn grep(pattern: &str, flags: &[&str], files: &[&str]) -> Vec<String> {
    unimplemented!(
        "Search the files '{:?}' for '{}' pattern and save the matches in a vector. Your search logic should be aware of the given flags '{:?}'",
        files,
        pattern,
        flags
    );
}
