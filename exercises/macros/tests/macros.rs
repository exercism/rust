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
fn test_invalid_syntax() {
    // These tests check for syntax that is invalid and should not compile.
    // If the test fail, then that means the hashmap! macro is allowing
    // invalid syntax

    const TRYBUILD: &str = "TRYBUILD";
    let cleanup = if std::env::var(TRYBUILD).is_ok() {
        false
    } else {
        std::env::set_var(TRYBUILD, "overwrite");
        true
    };

    let result = std::panic::catch_unwind(|| {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/invalid/*.rs");
    });

    if cleanup {
        std::env::remove_var(TRYBUILD);
    }

    assert!(result.is_ok());
}

mod test {
    use macros::hashmap;
    #[test]
    #[ignore]
    fn type_not_in_scope() {
        let _expected: ::std::collections::HashMap<(), ()> = hashmap!();
    }
}
