extern crate perfect_numbers;

use perfect_numbers::{Classification, classify};

#[test]
fn basic() {
    assert_eq!(classify(0), Err("Number must be positive"));
}

#[test]
#[ignore]
fn test_all() {
    struct TestClassification {
        num: u64,
        result: perfect_numbers::Classification
    }
    let test_table: Vec<TestClassification> = vec![
        TestClassification { num: 6, result: Classification::Perfect },
        TestClassification { num: 28, result: Classification::Perfect },
        TestClassification { num: 33550336, result: Classification::Perfect },
        TestClassification { num: 12, result: Classification::Abundant },
        TestClassification { num: 30, result: Classification::Abundant },
        TestClassification { num: 33550335, result: Classification::Abundant },
        TestClassification { num: 2, result: Classification::Deficient },
        TestClassification { num: 4, result: Classification::Deficient },
        TestClassification { num: 32, result: Classification::Deficient },
        TestClassification { num: 33550337, result: Classification::Deficient },
        TestClassification { num: 1, result: Classification::Deficient },
    ];
    for t in test_table {
        assert_eq!(classify(t.num), Ok(t.result));
    }
}
