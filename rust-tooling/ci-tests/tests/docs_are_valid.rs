use glob::glob;
use utils::fs::cd_into_repo_root;

// https://exercism.org/docs/building/tracks/practice-exercises#h-file-docs-hints-md
#[test]
fn hints_have_correct_heading() {
    cd_into_repo_root();
    for entry in glob("exercises/practice/*/.docs/hints.md").unwrap() {
        let path = entry.unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(
            content.starts_with("## General"),
            "incorrect heading in {}",
            path.display()
        )
    }
}

// https://exercism.org/docs/building/tracks/practice-exercises#h-file-docs-introduction-append-md
#[test]
fn instructions_append_have_correct_heading() {
    cd_into_repo_root();
    for entry in glob("exercises/*/*/.docs/instructions.append.md").unwrap() {
        let path = entry.unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(
            content.starts_with("# Instructions append"),
            "incorrect heading in {}",
            path.display()
        )
    }
}
