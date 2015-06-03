#![allow(unused_must_use)]

extern crate forth;

use forth::{Forth, Error};

#[test]
fn no_input_no_stack() {
    assert_eq!("", Forth::new().format_stack());
}

#[test]
#[ignore]
fn numbers_just_get_pushed_onto_the_stack() {
    let mut f = Forth::new();
    f.eval("1 2 3 4 5 -1");
    assert_eq!("1 2 3 4 5 -1", f.format_stack());
}

#[test]
#[ignore]
fn non_word_characters_are_separators() {
    let mut f = Forth::new();
    // Note the Ogham Space Mark ( ), this is a spacing character.
    f.eval("1\u{0000}2\u{0001}3\n4\r5 6\t7");
    assert_eq!("1 2 3 4 5 6 7", f.format_stack());
}

#[test]
#[ignore]
fn basic_arithmetic_1() {
    let mut f = Forth::new();
    f.eval("1 2 + 4 -");
    assert_eq!("-1", f.format_stack());
}

#[test]
#[ignore]
fn basic_arithmetic_2() {
    let mut f = Forth::new();
    f.eval("2 4 * 3 /");
    assert_eq!("2", f.format_stack());
}

#[test]
#[ignore]
fn addition_error() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("+")
    );
}

#[test]
#[ignore]
fn subtraction_error() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("-")
    );
}

#[test]
#[ignore]
fn multiplication_error() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("*")
    );
}

#[test]
#[ignore]
fn division_error() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("/")
    );
}

#[test]
#[ignore]
fn division_by_zero() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::DivisionByZero),
        f.eval("4 2 2 - /")
    );
}

#[test]
#[ignore]
fn dup() {
    let mut f = Forth::new();
    f.eval("1 DUP");
    assert_eq!("1 1", f.format_stack());
}

#[test]
#[ignore]
fn dup_case_insensitive() {
    let mut f = Forth::new();
    f.eval("1 Dup");
    assert_eq!("1 1", f.format_stack());
}

#[test]
#[ignore]
fn dup_error() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("dup")
    );
}

#[test]
#[ignore]
fn drop() {
    let mut f = Forth::new();
    f.eval("1 drop");
    assert_eq!("", f.format_stack());
}

#[test]
#[ignore]
fn drop_with_two() {
    let mut f = Forth::new();
    f.eval("1 2 drop");
    assert_eq!("1", f.format_stack());
}

#[test]
#[ignore]
fn drop_error() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("drop")
    );
}

#[test]
#[ignore]
fn swap() {
    let mut f = Forth::new();
    f.eval("1 2 swap");
    assert_eq!("2 1", f.format_stack());
}

#[test]
#[ignore]
fn swap_with_three() {
    let mut f = Forth::new();
    f.eval("1 2 3 swap");
    assert_eq!("1 3 2", f.format_stack());
}

#[test]
#[ignore]
fn swap_error() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("1 swap")
    );
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("swap")
    );
}

#[test]
#[ignore]
fn over() {
    let mut f = Forth::new();
    f.eval("1 2 over");
    assert_eq!("1 2 1", f.format_stack());
}

#[test]
#[ignore]
fn over_with_three() {
    let mut f = Forth::new();
    f.eval("1 2 3 over");
    assert_eq!("1 2 3 2", f.format_stack());
}

#[test]
#[ignore]
fn over_error() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("1 over")
    );
    assert_eq!(
        Err(Error::StackUnderflow),
        f.eval("over")
    );
}

#[test]
#[ignore]
fn defining_a_new_word() {
    let mut f = Forth::new();
    f.eval(": CoUnT 1 2 3 ;");
    f.eval("count COUNT");
    assert_eq!("1 2 3 1 2 3", f.format_stack());
}

#[test]
#[ignore]
fn redefining_an_existing_word() {
    let mut f = Forth::new();
    f.eval(": foo dup ;");
    f.eval(": foo dup dup ;");
    f.eval("1 foo");
    assert_eq!("1 1 1", f.format_stack());
}

#[test]
#[ignore]
fn redefining_an_existing_built_in_word() {
    let mut f = Forth::new();
    f.eval(": swap dup ;");
    f.eval("1 swap");
    assert_eq!("1 1", f.format_stack());
}

#[test]
#[ignore]
fn defining_words_with_odd_characters() {
    let mut f = Forth::new();
    f.eval(": € 220371 ; €");
    assert_eq!("220371", f.format_stack());
}

#[test]
#[ignore]
fn defining_a_number() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::InvalidWord),
        f.eval(": 1 2 ;")
    );
}

#[test]
#[ignore]
fn malformed_word_definition() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::InvalidWord),
        f.eval(":")
    );
    assert_eq!(
        Err(Error::InvalidWord),
        f.eval(": foo")
    );
    assert_eq!(
        Err(Error::InvalidWord),
        f.eval(": foo 1")
    );
}

#[test]
#[ignore]
fn calling_non_existing_word() {
    let mut f = Forth::new();
    assert_eq!(
        Err(Error::UnknownWord),
        f.eval("1 foo")
    );
}
