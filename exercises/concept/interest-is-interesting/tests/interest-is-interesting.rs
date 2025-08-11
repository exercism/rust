fn round_to_precision(input: f64, digits: u32) -> f64 {
    let precision = (10_i32.pow(digits)) as f64;
    let mut result = input * precision;
    result = result.round();
    result /= precision;
    result
}

#[test]
pub fn minimal_first_interest_rate() {
    assert_eq!(0.5, interest_is_interesting::interest_rate(0.0));
}

#[test]
#[ignore]
pub fn tiny_first_interest_rate() {
    assert_eq!(0.5, interest_is_interesting::interest_rate(0.000_001));
}

#[test]
#[ignore]
pub fn maximum_first_interest_rate() {
    assert_eq!(0.5, interest_is_interesting::interest_rate(999.9999));
}

#[test]
#[ignore]
pub fn minimal_second_interest_rate() {
    assert_eq!(1.621, interest_is_interesting::interest_rate(1_000.0));
}

#[test]
#[ignore]
pub fn tiny_second_interest_rate() {
    assert_eq!(1.621, interest_is_interesting::interest_rate(1_000.000_1));
}

#[test]
#[ignore]
pub fn maximum_second_interest_rate() {
    assert_eq!(1.621, interest_is_interesting::interest_rate(4_999.999_0));
}

#[test]
#[ignore]
pub fn minimal_third_interest_rate() {
    assert_eq!(2.475, interest_is_interesting::interest_rate(5_000.000_0));
}

#[test]
#[ignore]
pub fn tiny_third_interest_rate() {
    assert_eq!(2.475, interest_is_interesting::interest_rate(5_000.000_1));
}

#[test]
#[ignore]
pub fn large_third_interest_rate() {
    assert_eq!(
        2.475,
        interest_is_interesting::interest_rate(5_639_998.742_909)
    );
}

#[test]
#[ignore]
pub fn rate_on_minimal_negative_balance() {
    assert_eq!(3.213, interest_is_interesting::interest_rate(-0.000_001));
}

#[test]
#[ignore]
pub fn rate_on_small_negative_balance() {
    assert_eq!(3.213, interest_is_interesting::interest_rate(-0.123));
}

#[test]
#[ignore]
pub fn rate_on_regular_negative_balance() {
    assert_eq!(3.213, interest_is_interesting::interest_rate(-300.0));
}

#[test]
#[ignore]
pub fn rate_on_large_negative_balance() {
    assert_eq!(3.213, interest_is_interesting::interest_rate(-152_964.231));
}

#[test]
#[ignore]
pub fn interest_on_negative_balance() {
    assert_eq!(-321.3, interest_is_interesting::interest(-10_000.0));
}

#[test]
#[ignore]
pub fn interest_on_small_balance() {
    let actual = interest_is_interesting::interest(555.55);
    assert_eq!(2.777_75, round_to_precision(actual, 5));
}

#[test]
#[ignore]
pub fn interest_on_medium_balance() {
    let actual = interest_is_interesting::interest(4999.99);
    assert_eq!(81.049_84, round_to_precision(actual, 5));
}

#[test]
#[ignore]
pub fn interest_on_large_balance() {
    let actual = interest_is_interesting::interest(34600.80);
    assert_eq!(856.3698, round_to_precision(actual, 4));
}

#[test]
#[ignore]
pub fn annual_balance_update_for_empty_start_balance() {
    assert_eq!(0.0000, interest_is_interesting::annual_balance_update(0.0));
}

#[test]
#[ignore]
pub fn annual_balance_update_for_small_positive_start_balance() {
    assert_eq!(
        0.000_001_005,
        interest_is_interesting::annual_balance_update(0.000_001)
    );
}

#[test]
#[ignore]
pub fn annual_balance_update_for_average_positive_start_balance() {
    assert_eq!(
        1_016.210_000,
        interest_is_interesting::annual_balance_update(1_000.0)
    );
}

#[test]
#[ignore]
pub fn annual_balance_update_for_large_positive_start_balance() {
    let actual = interest_is_interesting::annual_balance_update(1_000.000_1);
    assert_eq!(1_016.210_1, round_to_precision(actual, 4));
}

#[test]
#[ignore]
pub fn annual_balance_update_for_huge_positive_start_balance() {
    let actual = interest_is_interesting::annual_balance_update(898_124_017.826_243_4);
    assert_eq!(920_352_587.267_443, actual);
}

#[test]
#[ignore]
pub fn annual_balance_update_for_small_negative_start_balance() {
    assert_eq!(
        -0.126_951_99,
        interest_is_interesting::annual_balance_update(-0.123)
    );
}

#[test]
#[ignore]
pub fn annual_balance_update_for_large_negative_start_balance() {
    assert_eq!(
        -157_878.971_742_03,
        interest_is_interesting::annual_balance_update(-152_964.231)
    );
}

#[test]
#[ignore]
pub fn years_before_desired_balance_for_small_start_balance() {
    assert_eq!(
        47,
        interest_is_interesting::years_before_desired_balance(100.0, 125.80)
    );
}

#[test]
#[ignore]
pub fn years_before_desired_balance_for_average_start_balance() {
    assert_eq!(
        6,
        interest_is_interesting::years_before_desired_balance(1_000.0, 1_100.0)
    );
}

#[test]
#[ignore]
pub fn years_before_desired_balance_for_large_start_balance() {
    assert_eq!(
        5,
        interest_is_interesting::years_before_desired_balance(8_080.80, 9_090.90)
    );
}

#[test]
#[ignore]
pub fn years_before_desired_balance_for_large_different_between_start_and_target_balance() {
    assert_eq!(
        85,
        interest_is_interesting::years_before_desired_balance(2_345.67, 12_345.678_9)
    );
}
