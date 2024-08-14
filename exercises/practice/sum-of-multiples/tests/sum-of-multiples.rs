use sum_of_multiples::*;

#[test]
fn no_multiples_within_limit() {
    let factors = &[3, 5];
    let limit = 1;
    let output = sum_of_multiples(limit, factors);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn one_factor_has_multiples_within_limit() {
    let factors = &[3, 5];
    let limit = 4;
    let output = sum_of_multiples(limit, factors);
    let expected = 3;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn more_than_one_multiple_within_limit() {
    let factors = &[3];
    let limit = 7;
    let output = sum_of_multiples(limit, factors);
    let expected = 9;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn more_than_one_factor_with_multiples_within_limit() {
    let factors = &[3, 5];
    let limit = 10;
    let output = sum_of_multiples(limit, factors);
    let expected = 23;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn each_multiple_is_only_counted_once() {
    let factors = &[3, 5];
    let limit = 100;
    let output = sum_of_multiples(limit, factors);
    let expected = 2_318;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_much_larger_limit() {
    let factors = &[3, 5];
    let limit = 1_000;
    let output = sum_of_multiples(limit, factors);
    let expected = 233_168;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn three_factors() {
    let factors = &[7, 13, 17];
    let limit = 20;
    let output = sum_of_multiples(limit, factors);
    let expected = 51;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn factors_not_relatively_prime() {
    let factors = &[4, 6];
    let limit = 15;
    let output = sum_of_multiples(limit, factors);
    let expected = 30;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn some_pairs_of_factors_relatively_prime_and_some_not() {
    let factors = &[5, 6, 8];
    let limit = 150;
    let output = sum_of_multiples(limit, factors);
    let expected = 4_419;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn one_factor_is_a_multiple_of_another() {
    let factors = &[5, 25];
    let limit = 51;
    let output = sum_of_multiples(limit, factors);
    let expected = 275;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn much_larger_factors() {
    let factors = &[43, 47];
    let limit = 10_000;
    let output = sum_of_multiples(limit, factors);
    let expected = 2_203_160;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn all_numbers_are_multiples_of_1() {
    let factors = &[1];
    let limit = 100;
    let output = sum_of_multiples(limit, factors);
    let expected = 4_950;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn no_factors_means_an_empty_sum() {
    let factors = &[];
    let limit = 10_000;
    let output = sum_of_multiples(limit, factors);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_only_multiple_of_0_is_0() {
    let factors = &[0];
    let limit = 1;
    let output = sum_of_multiples(limit, factors);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_factor_0_does_not_affect_the_sum_of_multiples_of_other_factors() {
    let factors = &[3, 0];
    let limit = 4;
    let output = sum_of_multiples(limit, factors);
    let expected = 3;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3() {
    let factors = &[2, 3, 5, 7, 11];
    let limit = 10_000;
    let output = sum_of_multiples(limit, factors);
    let expected = 39_614_537;
    assert_eq!(output, expected);
}
