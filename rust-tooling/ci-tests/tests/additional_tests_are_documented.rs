use std::path::Path;

use glob::glob;
use models::problem_spec::TestCase;
use utils::fs::cd_into_repo_root;

#[test]
fn additional_tests_are_documented() {
    cd_into_repo_root();
    for entry in glob("exercises/*/*/.meta/additional-tests.json").unwrap() {
        let path = entry.unwrap();
        let f = std::fs::File::open(&path).unwrap();
        let reader = std::io::BufReader::new(f);
        let test_cases: Vec<TestCase> = serde_json::from_reader(reader).unwrap();

        fn rec(case: TestCase, path: &Path) {
            match case {
                TestCase::Single { case } => assert!(
                    !case.comments.unwrap_or_default().is_empty(),
                    "missing documentation for additional tests in {}",
                    path.display()
                ),
                TestCase::Group { cases, .. } => cases.into_iter().for_each(|c| rec(c, path)),
            }
        }
        for case in test_cases {
            rec(case, &path);
        }
    }
}
