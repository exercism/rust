extern crate decimal;
use decimal::Decimal;

/// Create a Decimal from a string literal
///
/// Use only when you _know_ that your value is valid.
fn decimal(input: &str) -> Decimal {
    Decimal::try_from(input).expect("That was supposed to be a valid value")
}

/// Some big and precise values we can use for testing. [0] + [1] == [2]
const BIGS: [&'static str; 3] = [
    "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000001",
    "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000002",
    "200000000000000000000000000000000000000000000.00000000000000000000000000000000000000003",
];

#[test]
fn test_eq() {
    assert!(decimal("0.0") == decimal("0.0"));
    assert!(decimal("1.0") == decimal("1.0"));
    for big in BIGS.iter() {
        assert!(decimal(big) == decimal(big));
    }
}

#[test]
fn test_ne() {
    assert!(decimal("0.0") != decimal("1.0"));
    assert!(decimal(BIGS[0]) != decimal(BIGS[1]));
}

#[test]
fn test_gt() {
    for slice_2 in BIGS.windows(2) {
        assert!(decimal(slice_2[1]) > decimal(slice_2[0]));
        assert!(!(decimal(slice_2[0]) > decimal(slice_2[1])));
    }
}

#[test]
fn test_lt() {
    for slice_2 in BIGS.windows(2) {
        assert!(decimal(slice_2[0]) < decimal(slice_2[1]));
        assert!(!(decimal(slice_2[1]) < decimal(slice_2[0])));
    }
}

#[test]
fn test_add() {
    assert_eq!(decimal("0.1") + decimal("0.2"), decimal("0.3"));
    assert_eq!(decimal(BIGS[0]) + decimal(BIGS[1]), decimal(BIGS[2]));
    assert_eq!(decimal(BIGS[1]) + decimal(BIGS[0]), decimal(BIGS[2]));
}

#[test]
fn test_sub() {
    assert_eq!(decimal(BIGS[2]) - decimal(BIGS[1]), decimal(BIGS[0]));
    assert_eq!(decimal(BIGS[2]) - decimal(BIGS[0]), decimal(BIGS[1]));
}

#[test]
fn test_mul() {
    for big in BIGS.iter() {
        assert_eq!(decimal(big) * decimal("2"), decimal(big) + decimal(big));
    }
}

#[test]
fn test_eq_vary_sig_digits() {
    assert!(decimal("0") == decimal("0000000000000.0000000000000000000000"));
    assert!(decimal("1") == decimal("00000000000000001.000000000000000000"));
}

#[test]
fn test_add_vary_precision() {
    assert_eq!(
        decimal("100000000000000000000000000000000000000000000") +
            decimal("0.00000000000000000000000000000000000000001"),
        decimal(BIGS[0])
    )
}

#[test]
fn test_cleanup_precision() {
    assert_eq!(
        decimal(
            "10000000000000000000000000000000000000000000000.999999999999999999999999998",
        ) +
            decimal(
                "10000000000000000000000000000000000000000000000.000000000000000000000000002",
            ),
        decimal("20000000000000000000000000000000000000000000001")
    )
}

#[test]
fn test_negatives() {
    assert!(Decimal::try_from("-1").is_some());
    assert_eq!(decimal("0") - decimal("1"), decimal("-1"));
    assert_eq!(decimal("5.5") + decimal("-6.5"), decimal("-1"));
}

#[test]
fn test_explicit_positive() {
    assert_eq!(decimal("+1"), decimal("1"));
    assert_eq!(decimal("+2.0") - decimal("-0002.0"), decimal("4"));
}
