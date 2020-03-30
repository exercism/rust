use macros::hashmap;
use std::collections::HashMap;

#[test]
fn test_empty() {
    let expected: HashMap<u32, u32> = HashMap::new();
    let computed: HashMap<u32, u32> = hashmap!();
    assert_eq!(computed, expected);
}

#[test]
#[ignore]
fn test_single() {
    let mut expected = HashMap::new();
    expected.insert(1, "one");
    assert_eq!(hashmap!(1 => "one"), expected);
}

#[test]
#[ignore]
fn test_no_trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert(1, "one");
    expected.insert(2, "two");
    assert_eq!(hashmap!(1 => "one", 2 => "two"), expected);
}

#[test]
#[ignore]
fn test_trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert('h', 89);
    expected.insert('a', 1);
    expected.insert('s', 19);
    expected.insert('h', 8);
    assert_eq!(
        hashmap!(
            'h' => 89,
            'a' => 1,
            's' => 19,
            'h' => 8,
        ),
        expected
    );
}

#[test]
#[ignore]
fn test_nested() {
    let mut expected = HashMap::new();
    expected.insert("non-empty", {
        let mut subhashmap = HashMap::new();
        subhashmap.insert(23, 623);
        subhashmap.insert(34, 21);
        subhashmap
    });
    expected.insert("empty", HashMap::new());
    assert_eq!(
        hashmap!(
            "non-empty" => hashmap!(
                23 => 623,
                34 => 21
            ),
            "empty" => hashmap!()
        ),
        expected
    );
}

#[test]
#[ignore]
fn test_compile_fails_comma_sep() {
    simple_trybuild::compile_fail("comma-sep.rs");
}

#[test]
#[ignore]
fn test_compile_fails_double_commas() {
    simple_trybuild::compile_fail("double-commas.rs");
}

#[test]
#[ignore]
fn test_compile_fails_only_comma() {
    simple_trybuild::compile_fail("only-comma.rs");
}

#[test]
#[ignore]
fn test_compile_fails_single_argument() {
    simple_trybuild::compile_fail("single-argument.rs");
}

#[test]
#[ignore]
fn test_compile_fails_triple_arguments() {
    simple_trybuild::compile_fail("triple-arguments.rs");
}

#[test]
#[ignore]
fn test_compile_fails_only_arrow() {
    simple_trybuild::compile_fail("only-arrow.rs");
}

#[test]
#[ignore]
fn test_compile_fails_two_arrows() {
    simple_trybuild::compile_fail("two-arrows.rs");
}

mod test {
    use macros::hashmap;
    #[test]
    #[ignore]
    fn type_not_in_scope() {
        let _expected: ::std::collections::HashMap<(), ()> = hashmap!();
    }
}

mod simple_trybuild {
    use std::process::Command;

    pub fn compile_fail(file_name: &str) {
        const INVALID_PATH: &str = "tests/invalid";

        assert!(
            [INVALID_PATH, file_name]
                .iter()
                .collect::<std::path::PathBuf>()
                .exists(),
            "file tests/invalid/{} does not exist.",
            file_name
        );

        let test_name = file_name.replace(".", "-");

        let result = Command::new("cargo")
            .current_dir(INVALID_PATH)
            .arg("build")
            .arg("--offline")
            .arg("--target-dir=../../target/tests/macros/")
            .arg("--bin")
            .arg(test_name)
            .output();

        assert!(result.is_ok());
        if let Ok(result) = result {
            assert!(
                !result.status.success(),
                "{}/{} compiled sucessfully, but should not.",
                INVALID_PATH,
                file_name
            );
        }
    }
}
