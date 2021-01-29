use sum_of_multiples::*;

#[test]
fn no_multiples_within_limit() {
    assert_eq!(0, sum_of_multiples(1, &[3, 5]))
}

#[test]
#[ignore]
fn one_factor_has_multiples_within_limit() {
    assert_eq!(3, sum_of_multiples(4, &[3, 5]))
}

#[test]
#[ignore]
fn more_than_one_multiple_within_limit() {
    assert_eq!(9, sum_of_multiples(7, &[3]))
}

#[test]
#[ignore]
fn more_than_one_factor_with_multiples_within_limit() {
    assert_eq!(23, sum_of_multiples(10, &[3, 5]))
}

#[test]
#[ignore]
fn each_multiple_is_only_counted_once() {
    assert_eq!(2318, sum_of_multiples(100, &[3, 5]))
}

#[test]
#[ignore]
fn a_much_larger_limit() {
    assert_eq!(233_168, sum_of_multiples(1000, &[3, 5]))
}

#[test]
#[ignore]
fn three_factors() {
    assert_eq!(51, sum_of_multiples(20, &[7, 13, 17]))
}

#[test]
#[ignore]
fn factors_not_relatively_prime() {
    assert_eq!(30, sum_of_multiples(15, &[4, 6]))
}

#[test]
#[ignore]
fn some_pairs_of_factors_relatively_prime_and_some_not() {
    assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]))
}

#[test]
#[ignore]
fn one_factor_is_a_multiple_of_another() {
    assert_eq!(275, sum_of_multiples(51, &[5, 25]))
}

#[test]
#[ignore]
fn much_larger_factors() {
    assert_eq!(2_203_160, sum_of_multiples(10_000, &[43, 47]))
}

#[test]
#[ignore]
fn all_numbers_are_multiples_of_1() {
    assert_eq!(4950, sum_of_multiples(100, &[1]))
}

#[test]
#[ignore]
fn no_factors_means_an_empty_sum() {
    assert_eq!(0, sum_of_multiples(10_000, &[]))
}

#[test]
#[ignore]
fn the_only_multiple_of_0_is_0() {
    assert_eq!(0, sum_of_multiples(1, &[0]))
}

#[test]
#[ignore]
fn the_factor_0_does_not_affect_the_sum_of_multiples_of_other_factors() {
    assert_eq!(3, sum_of_multiples(4, &[3, 0]))
}

#[test]
#[ignore]
fn solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3() {
    assert_eq!(39_614_537, sum_of_multiples(10_000, &[2, 3, 5, 7, 11]))
}
