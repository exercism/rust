extern crate perfect_numbers;

use perfect_numbers::{Classification, classify};

#[test]
fn basic() {
    assert_eq!(classify(0).unwrap_err(), "Number must be positive");
}

#[test]
#[ignore]
fn test_all() {
    struct TestClassification {
        num: u64,
        result: perfect_numbers::Classification
    }
    let test_table: Vec<TestClassification> = vec![
        TestClassification { num: 1, result: Classification::Deficient },
        TestClassification { num: 13, result: Classification::Deficient },
        TestClassification { num: 12, result: Classification::Abundant },
        TestClassification { num: 6, result: Classification::Perfect },
        TestClassification { num: 28, result: Classification::Perfect },
        TestClassification { num: 496, result: Classification::Perfect },
        TestClassification { num: 8128, result: Classification::Perfect }
    ];
    for t in test_table {
        assert_eq!(classify(t.num).unwrap(), t.result);
    }
}

