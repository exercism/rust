extern crate two_fer;

use two_fer::{two_fer};

#[test]
fn no_name_given() {
    assert_eq!(two_fer(None),"One for you, one for me.".to_string());
}
#[ignore]
#[test]
fn a_name_given() {
    assert_eq!(two_fer(Some("Alice")),"One for Alice, one for me.".to_string());
}
#[ignore]
#[test]
fn another_name_given() {
    assert_eq!(two_fer(Some("Bob")),"One for Bob, one for me.".to_string());
}
