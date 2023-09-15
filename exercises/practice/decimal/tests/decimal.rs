use decimal::Decimal;

/// Create a Decimal from a string literal
///
/// Use only when you _know_ that your value is valid.
fn decimal(input: &str) -> Decimal {
    Decimal::try_from(input).expect("That was supposed to be a valid value")
}

/// Some big and precise values we can use for testing. [0] + [1] == [2]
const BIGS: [&str; 3] = [
    "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000001",
    "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000002",
    "200000000000000000000000000000000000000000000.00000000000000000000000000000000000000003",
];

// test simple properties of required operations
#[test]
fn eq() {
    assert!(decimal("0.0") == decimal("0.0"));
    assert!(decimal("1.0") == decimal("1.0"));
    for big in BIGS.iter() {
        assert!(decimal(big) == decimal(big));
    }
}

#[test]
#[ignore]
fn ne() {
    assert!(decimal("0.0") != decimal("1.0"));
    assert!(decimal(BIGS[0]) != decimal(BIGS[1]));
}

#[test]
#[ignore]
fn gt() {
    for slice_2 in BIGS.windows(2) {
        assert!(decimal(slice_2[1]) > decimal(slice_2[0]));
    }
}

#[test]
#[ignore]
fn lt() {
    for slice_2 in BIGS.windows(2) {
        assert!(decimal(slice_2[0]) < decimal(slice_2[1]));
    }
}

#[test]
#[ignore]
fn add() {
    assert_eq!(decimal("0.1") + decimal("0.2"), decimal("0.3"));
    assert_eq!(decimal(BIGS[0]) + decimal(BIGS[1]), decimal(BIGS[2]));
    assert_eq!(decimal(BIGS[1]) + decimal(BIGS[0]), decimal(BIGS[2]));
}

#[test]
#[ignore]
fn sub() {
    assert_eq!(decimal(BIGS[2]) - decimal(BIGS[1]), decimal(BIGS[0]));
    assert_eq!(decimal(BIGS[2]) - decimal(BIGS[0]), decimal(BIGS[1]));
}

#[test]
#[ignore]
fn mul() {
    for big in BIGS.iter() {
        assert_eq!(decimal(big) * decimal("2"), decimal(big) + decimal(big));
    }
}

// test identities
#[test]
#[ignore]
fn add_id() {
    assert_eq!(decimal("1.0") + decimal("0.0"), decimal("1.0"));
    assert_eq!(decimal("0.1") + decimal("0.0"), decimal("0.1"));
    assert_eq!(decimal("0.0") + decimal("1.0"), decimal("1.0"));
    assert_eq!(decimal("0.0") + decimal("0.1"), decimal("0.1"));
}

#[test]
#[ignore]
fn sub_id() {
    assert_eq!(decimal("1.0") - decimal("0.0"), decimal("1.0"));
    assert_eq!(decimal("0.1") - decimal("0.0"), decimal("0.1"));
}

#[test]
#[ignore]
fn mul_id() {
    assert_eq!(decimal("2.1") * decimal("1.0"), decimal("2.1"));
    assert_eq!(decimal("1.0") * decimal("2.1"), decimal("2.1"));
}

#[test]
#[ignore]
fn gt_positive_and_zero() {
    assert!(decimal("1.0") > decimal("0.0"));
    assert!(decimal("0.1") > decimal("0.0"));
}

#[test]
#[ignore]
fn gt_negative_and_zero() {
    assert!(decimal("0.0") > decimal("-0.1"));
    assert!(decimal("0.0") > decimal("-1.0"));
}

#[test]
#[ignore]
fn unequal_number_of_decimal_places() {
    assert!(decimal("3.14") > decimal("3.13"));
    assert!(decimal("3.14") > decimal("3.131"));
    assert!(decimal("3.14") > decimal("3.1"));
    assert!(decimal("3.13") < decimal("3.14"));
    assert!(decimal("3.131") < decimal("3.14"));
    assert!(decimal("3.1") < decimal("3.14"));
}

// tests of arbitrary precision behavior
#[test]
#[ignore]
fn add_uneven_position() {
    assert_eq!(decimal("0.1") + decimal("0.02"), decimal("0.12"));
}

#[test]
#[ignore]
fn eq_vary_sig_digits() {
    assert!(decimal("0") == decimal("0000000000000.0000000000000000000000"));
    assert!(decimal("1") == decimal("00000000000000001.000000000000000000"));
}

#[test]
#[ignore]
fn add_vary_precision() {
    assert_eq!(
        decimal("100000000000000000000000000000000000000000000")
            + decimal("0.00000000000000000000000000000000000000001"),
        decimal(BIGS[0])
    )
}

#[test]
#[ignore]
fn cleanup_precision() {
    assert_eq!(
        decimal("10000000000000000000000000000000000000000000000.999999999999999999999999998",)
            + decimal(
                "10000000000000000000000000000000000000000000000.000000000000000000000000002",
            ),
        decimal("20000000000000000000000000000000000000000000001")
    )
}

#[test]
#[ignore]
fn gt_varying_positive_precisions() {
    assert!(decimal("1.1") > decimal("1.01"));
    assert!(decimal("1.01") > decimal("1.0"));
    assert!(decimal("1.0") > decimal("0.1"));
    assert!(decimal("0.1") > decimal("0.01"));
}

#[test]
#[ignore]
fn gt_positive_and_negative() {
    assert!(decimal("1.0") > decimal("-1.0"));
    assert!(decimal("1.1") > decimal("-1.1"));
    assert!(decimal("0.1") > decimal("-0.1"));
}

#[test]
#[ignore]
fn gt_varying_negative_precisions() {
    assert!(decimal("-0.01") > decimal("-0.1"));
    assert!(decimal("-0.1") > decimal("-1.0"));
    assert!(decimal("-1.0") > decimal("-1.01"));
    assert!(decimal("-1.01") > decimal("-1.1"));
}

// test signed properties
#[test]
#[ignore]
fn negatives() {
    assert!(Decimal::try_from("-1").is_some());
    assert_eq!(decimal("0") - decimal("1"), decimal("-1"));
    assert_eq!(decimal("5.5") + decimal("-6.5"), decimal("-1"));
}

#[test]
#[ignore]
fn explicit_positive() {
    assert_eq!(decimal("+1"), decimal("1"));
    assert_eq!(decimal("+2.0") - decimal("-0002.0"), decimal("4"));
}

#[test]
#[ignore]
fn multiply_by_negative() {
    assert_eq!(decimal("5") * decimal("-0.2"), decimal("-1"));
    assert_eq!(decimal("-20") * decimal("-0.2"), decimal("4"));
}

#[test]
#[ignore]
fn simple_partial_cmp() {
    assert!(decimal("1.0") < decimal("1.1"));
    assert!(decimal("0.00000000000000000000001") > decimal("-20000000000000000000000000000"));
}

// test carrying rules
// these tests are designed to ensure correctness of implementations for which the
// integer and fractional parts of the number are stored separately
#[test]
#[ignore]
fn carry_into_integer() {
    assert_eq!(decimal("0.901") + decimal("0.1"), decimal("1.001"))
}

#[test]
#[ignore]
fn carry_into_fractional_with_digits_to_right() {
    assert_eq!(decimal("0.0901") + decimal("0.01"), decimal("0.1001"))
}

#[test]
#[ignore]
fn add_carry_over_negative() {
    assert_eq!(decimal("-1.99") + decimal("-0.01"), decimal("-2.0"))
}

#[test]
#[ignore]
fn sub_carry_over_negative() {
    assert_eq!(decimal("-1.99") - decimal("0.01"), decimal("-2.0"))
}

#[test]
#[ignore]
fn add_carry_over_negative_with_fractional() {
    assert_eq!(decimal("-1.99") + decimal("-0.02"), decimal("-2.01"))
}

#[test]
#[ignore]
fn sub_carry_over_negative_with_fractional() {
    assert_eq!(decimal("-1.99") - decimal("0.02"), decimal("-2.01"))
}

#[test]
#[ignore]
fn carry_from_rightmost_one() {
    assert_eq!(decimal("0.09") + decimal("0.01"), decimal("0.1"))
}

#[test]
#[ignore]
fn carry_from_rightmost_more() {
    assert_eq!(decimal("0.099") + decimal("0.001"), decimal("0.1"))
}

#[test]
#[ignore]
fn carry_from_rightmost_into_integer() {
    assert_eq!(decimal("0.999") + decimal("0.001"), decimal("1.0"))
}

// test arithmetic borrow rules
#[test]
#[ignore]
fn add_borrow() {
    assert_eq!(decimal("0.01") + decimal("-0.0001"), decimal("0.0099"))
}

#[test]
#[ignore]
fn sub_borrow() {
    assert_eq!(decimal("0.01") - decimal("0.0001"), decimal("0.0099"))
}

#[test]
#[ignore]
fn add_borrow_integral() {
    assert_eq!(decimal("1.0") + decimal("-0.01"), decimal("0.99"))
}

#[test]
#[ignore]
fn sub_borrow_integral() {
    assert_eq!(decimal("1.0") - decimal("0.01"), decimal("0.99"))
}

#[test]
#[ignore]
fn add_borrow_integral_zeroes() {
    assert_eq!(decimal("1.0") + decimal("-0.99"), decimal("0.01"))
}

#[test]
#[ignore]
fn sub_borrow_integral_zeroes() {
    assert_eq!(decimal("1.0") - decimal("0.99"), decimal("0.01"))
}

#[test]
#[ignore]
fn borrow_from_negative() {
    assert_eq!(decimal("-1.0") + decimal("0.01"), decimal("-0.99"))
}

#[test]
#[ignore]
fn add_into_fewer_digits() {
    assert_eq!(decimal("0.011") + decimal("-0.001"), decimal("0.01"))
}

// misc tests of arithmetic properties
#[test]
#[ignore]
fn sub_into_fewer_digits() {
    assert_eq!(decimal("0.011") - decimal("0.001"), decimal("0.01"))
}

#[test]
#[ignore]
fn add_away_decimal() {
    assert_eq!(decimal("1.1") + decimal("-0.1"), decimal("1.0"))
}

#[test]
#[ignore]
fn sub_away_decimal() {
    assert_eq!(decimal("1.1") - decimal("0.1"), decimal("1.0"))
}
