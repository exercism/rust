extern crate twofer;
use twofer::twofer;

#[test]
fn empty_string() {
    assert_eq!(twofer(""), "One for you, one for me.");
}

#[test]
#[ignore]
fn alice() {
    assert_eq!(twofer("Alice"), "One for Alice, one for me.");
}

#[test]
#[ignore]
fn bob() {
    assert_eq!(twofer("Bob"), "One for Bob, one for me.");
}

#[test]
#[ignore]
fn escape_characters_test() {
    assert_eq!(twofer(" "), "One for you, one for me.");

    assert_eq!(twofer("\n"), "One for you, one for me.");

    assert_eq!(twofer("\n\r"), "One for you, one for me.");

    assert_eq!(twofer("\t"), "One for you, one for me.");
}
