use glob::glob;
use models::problem_spec::SingleTestCase;
use utils::fs::cd_into_repo_root;

#[test]
fn additional_tests_are_documented() {
    cd_into_repo_root();
    for entry in glob("exercises/*/*/.meta/additional-tests.json").unwrap() {
        let path = entry.unwrap();
        let f = std::fs::File::open(&path).unwrap();
        let reader = std::io::BufReader::new(f);
        let test_cases: Vec<SingleTestCase> = serde_json::from_reader(reader).unwrap();

        for case in test_cases {
            assert!(
                !case.comments.unwrap_or_default().is_empty(),
                "missing documentation for additional tests in {}",
                path.display()
            );
        }
    }
}
