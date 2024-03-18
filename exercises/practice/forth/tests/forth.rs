use forth::{Error, Forth, Value};

#[test]
fn parsing_and_numbers_numbers_just_get_pushed_onto_the_stack() {
    let mut f = Forth::new();
    f.eval("1 2 3 4 5").unwrap();
    assert_eq!(f.stack(), [1, 2, 3, 4, 5]);
}

#[test]
#[ignore]
fn parsing_and_numbers_pushes_negative_numbers_onto_the_stack() {
    let mut f = Forth::new();
    f.eval("-1 -2 -3 -4 -5").unwrap();
    assert_eq!(f.stack(), [-1, -2, -3, -4, -5]);
}

#[test]
#[ignore]
fn addition_can_add_two_numbers() {
    let mut f = Forth::new();
    f.eval("1 2 +").unwrap();
    assert_eq!(f.stack(), [3]);
}

#[test]
#[ignore]
fn addition_errors_if_there_is_nothing_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("+"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn addition_errors_if_there_is_only_one_value_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("1 +"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn subtraction_can_subtract_two_numbers() {
    let mut f = Forth::new();
    f.eval("3 4 -").unwrap();
    assert_eq!(f.stack(), [-1]);
}

#[test]
#[ignore]
fn subtraction_errors_if_there_is_nothing_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("-"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn subtraction_errors_if_there_is_only_one_value_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("1 -"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn multiplication_can_multiply_two_numbers() {
    let mut f = Forth::new();
    f.eval("2 4 *").unwrap();
    assert_eq!(f.stack(), [8]);
}

#[test]
#[ignore]
fn multiplication_errors_if_there_is_nothing_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("*"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn multiplication_errors_if_there_is_only_one_value_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("1 *"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn division_can_divide_two_numbers() {
    let mut f = Forth::new();
    f.eval("12 3 /").unwrap();
    assert_eq!(f.stack(), [4]);
}

#[test]
#[ignore]
fn division_performs_integer_division() {
    let mut f = Forth::new();
    f.eval("8 3 /").unwrap();
    assert_eq!(f.stack(), [2]);
}

#[test]
#[ignore]
fn division_errors_if_dividing_by_zero() {
    let mut f = Forth::new();
    assert_eq!(f.eval("4 0 /"), Err(Error::DivisionByZero));
}

#[test]
#[ignore]
fn division_errors_if_there_is_nothing_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("/"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn division_errors_if_there_is_only_one_value_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("1 /"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn combined_arithmetic_addition_and_subtraction() {
    let mut f = Forth::new();
    f.eval("1 2 + 4 -").unwrap();
    assert_eq!(f.stack(), [-1]);
}

#[test]
#[ignore]
fn combined_arithmetic_multiplication_and_division() {
    let mut f = Forth::new();
    f.eval("2 4 * 3 /").unwrap();
    assert_eq!(f.stack(), [2]);
}

#[test]
#[ignore]
fn dup_copies_a_value_on_the_stack() {
    let mut f = Forth::new();
    f.eval("1 dup").unwrap();
    assert_eq!(f.stack(), [1, 1]);
}

#[test]
#[ignore]
fn dup_copies_the_top_value_on_the_stack() {
    let mut f = Forth::new();
    f.eval("1 2 dup").unwrap();
    assert_eq!(f.stack(), [1, 2, 2]);
}

#[test]
#[ignore]
fn dup_errors_if_there_is_nothing_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("dup"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn drop_removes_the_top_value_on_the_stack_if_it_is_the_only_one() {
    let mut f = Forth::new();
    f.eval("1 drop").unwrap();
    assert_eq!(f.stack(), []);
}

#[test]
#[ignore]
fn drop_removes_the_top_value_on_the_stack_if_it_is_not_the_only_one() {
    let mut f = Forth::new();
    f.eval("1 2 drop").unwrap();
    assert_eq!(f.stack(), [1]);
}

#[test]
#[ignore]
fn drop_errors_if_there_is_nothing_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("drop"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn swap_swaps_the_top_two_values_on_the_stack_if_they_are_the_only_ones() {
    let mut f = Forth::new();
    f.eval("1 2 swap").unwrap();
    assert_eq!(f.stack(), [2, 1]);
}

#[test]
#[ignore]
fn swap_swaps_the_top_two_values_on_the_stack_if_they_are_not_the_only_ones() {
    let mut f = Forth::new();
    f.eval("1 2 3 swap").unwrap();
    assert_eq!(f.stack(), [1, 3, 2]);
}

#[test]
#[ignore]
fn swap_errors_if_there_is_nothing_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("swap"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn swap_errors_if_there_is_only_one_value_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("1 swap"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn over_copies_the_second_element_if_there_are_only_two() {
    let mut f = Forth::new();
    f.eval("1 2 over").unwrap();
    assert_eq!(f.stack(), [1, 2, 1]);
}

#[test]
#[ignore]
fn over_copies_the_second_element_if_there_are_more_than_two() {
    let mut f = Forth::new();
    f.eval("1 2 3 over").unwrap();
    assert_eq!(f.stack(), [1, 2, 3, 2]);
}

#[test]
#[ignore]
fn over_errors_if_there_is_nothing_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("over"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn over_errors_if_there_is_only_one_value_on_the_stack() {
    let mut f = Forth::new();
    assert_eq!(f.eval("1 over"), Err(Error::StackUnderflow));
}

#[test]
#[ignore]
fn user_defined_words_can_consist_of_built_in_words() {
    let mut f = Forth::new();
    f.eval(": dup-twice dup dup ;").unwrap();
    f.eval("1 dup-twice").unwrap();
    assert_eq!(f.stack(), [1, 1, 1]);
}

#[test]
#[ignore]
fn user_defined_words_execute_in_the_right_order() {
    let mut f = Forth::new();
    f.eval(": countup 1 2 3 ;").unwrap();
    f.eval("countup").unwrap();
    assert_eq!(f.stack(), [1, 2, 3]);
}

#[test]
#[ignore]
fn user_defined_words_can_override_other_user_defined_words() {
    let mut f = Forth::new();
    f.eval(": foo dup ;").unwrap();
    f.eval(": foo dup dup ;").unwrap();
    f.eval("1 foo").unwrap();
    assert_eq!(f.stack(), [1, 1, 1]);
}

#[test]
#[ignore]
fn user_defined_words_can_override_built_in_words() {
    let mut f = Forth::new();
    f.eval(": swap dup ;").unwrap();
    f.eval("1 swap").unwrap();
    assert_eq!(f.stack(), [1, 1]);
}

#[test]
#[ignore]
fn user_defined_words_can_override_built_in_operators() {
    let mut f = Forth::new();
    f.eval(": + * ;").unwrap();
    f.eval("3 4 +").unwrap();
    assert_eq!(f.stack(), [12]);
}

#[test]
#[ignore]
fn user_defined_words_can_use_different_words_with_the_same_name() {
    let mut f = Forth::new();
    f.eval(": foo 5 ;").unwrap();
    f.eval(": bar foo ;").unwrap();
    f.eval(": foo 6 ;").unwrap();
    f.eval("bar foo").unwrap();
    assert_eq!(f.stack(), [5, 6]);
}

#[test]
#[ignore]
fn user_defined_words_can_define_word_that_uses_word_with_the_same_name() {
    let mut f = Forth::new();
    f.eval(": foo 10 ;").unwrap();
    f.eval(": foo foo 1 + ;").unwrap();
    f.eval("foo").unwrap();
    assert_eq!(f.stack(), [11]);
}

#[test]
#[ignore]
fn user_defined_words_cannot_redefine_non_negative_numbers() {
    let mut f = Forth::new();
    assert_eq!(f.eval(": 1 2 ;"), Err(Error::InvalidWord));
}

#[test]
#[ignore]
fn user_defined_words_cannot_redefine_negative_numbers() {
    let mut f = Forth::new();
    assert_eq!(f.eval(": -1 2 ;"), Err(Error::InvalidWord));
}

#[test]
#[ignore]
fn user_defined_words_errors_if_executing_a_non_existent_word() {
    let mut f = Forth::new();
    assert_eq!(f.eval("foo"), Err(Error::UnknownWord));
}

#[test]
#[ignore]
fn user_defined_words_only_defines_locally() {
    let mut f = Forth::new();
    f.eval(": + - ;").unwrap();
    f.eval("1 1 +").unwrap();
    assert_eq!(f.stack(), [0]);
    let mut f = Forth::new();
    f.eval("1 1 +").unwrap();
    assert_eq!(f.stack(), [2]);
}

#[test]
#[ignore]
fn case_insensitivity_dup_is_case_insensitive() {
    let mut f = Forth::new();
    f.eval("1 DUP Dup dup").unwrap();
    assert_eq!(f.stack(), [1, 1, 1, 1]);
}

#[test]
#[ignore]
fn case_insensitivity_drop_is_case_insensitive() {
    let mut f = Forth::new();
    f.eval("1 2 3 4 DROP Drop drop").unwrap();
    assert_eq!(f.stack(), [1]);
}

#[test]
#[ignore]
fn case_insensitivity_swap_is_case_insensitive() {
    let mut f = Forth::new();
    f.eval("1 2 SWAP 3 Swap 4 swap").unwrap();
    assert_eq!(f.stack(), [2, 3, 4, 1]);
}

#[test]
#[ignore]
fn case_insensitivity_over_is_case_insensitive() {
    let mut f = Forth::new();
    f.eval("1 2 OVER Over over").unwrap();
    assert_eq!(f.stack(), [1, 2, 1, 2, 1]);
}

#[test]
#[ignore]
fn case_insensitivity_user_defined_words_are_case_insensitive() {
    let mut f = Forth::new();
    f.eval(": foo dup ;").unwrap();
    f.eval("1 FOO Foo foo").unwrap();
    assert_eq!(f.stack(), [1, 1, 1, 1]);
}

#[test]
#[ignore]
fn case_insensitivity_definitions_are_case_insensitive() {
    let mut f = Forth::new();
    f.eval(": SWAP DUP Dup dup ;").unwrap();
    f.eval("1 swap").unwrap();
    assert_eq!(f.stack(), [1, 1, 1, 1]);
}
