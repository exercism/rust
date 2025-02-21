use perfect_numbers::*;

#[test]
fn smallest_perfect_number_is_classified_correctly() {
    let input = 6;
    let output = classify(input);
    let expected = Some(Classification::Perfect);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn medium_perfect_number_is_classified_correctly() {
    let input = 28;
    let output = classify(input);
    let expected = Some(Classification::Perfect);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn large_perfect_number_is_classified_correctly() {
    let input = 33_550_336;
    let output = classify(input);
    let expected = Some(Classification::Perfect);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_abundant_number_is_classified_correctly() {
    let input = 12;
    let output = classify(input);
    let expected = Some(Classification::Abundant);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn medium_abundant_number_is_classified_correctly() {
    let input = 30;
    let output = classify(input);
    let expected = Some(Classification::Abundant);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn large_abundant_number_is_classified_correctly() {
    let input = 33_550_335;
    let output = classify(input);
    let expected = Some(Classification::Abundant);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_prime_deficient_number_is_classified_correctly() {
    let input = 2;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_non_prime_deficient_number_is_classified_correctly() {
    let input = 4;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn medium_deficient_number_is_classified_correctly() {
    let input = 32;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn large_deficient_number_is_classified_correctly() {
    let input = 33_550_337;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn edge_case_no_factors_other_than_itself_is_classified_correctly() {
    let input = 1;
    let output = classify(input);
    let expected = Some(Classification::Deficient);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn zero_is_rejected_as_it_is_not_a_positive_integer() {
    let input = 0;
    let output = classify(input);
    assert!(output.is_none());
}
