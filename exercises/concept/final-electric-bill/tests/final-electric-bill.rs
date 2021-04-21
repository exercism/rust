use final_electric_bill::*;

#[test]
fn test_small_bill() {
    let summary = vec![100, 200];
    let fixed_summary = fix_billing_summary(summary);
    assert_eq!(fixed_summary, vec![300]);
}

#[test]
#[ignore]
fn test_large_bill() {
    let summary = vec![100; 37];
    let mut fixed_summary = fix_billing_summary(summary);
    assert_eq!(fixed_summary.pop(), Some(200));
}

#[test]
#[cfg(feature = "generic")]
#[ignore]
fn test_generic() {
    #[derive(PartialEq)]
    struct Bill(i64);

    impl std::ops::Add for Bill {
        fn add(self, rhs: Bill) -> Bill {
            Bill(self.0 + rhs.0)
        }
    }

    let bills: Vec<_> = vec![100, 200, 300].into_iter().map(Bill).collect();
    let mut fixed_bills = fix_billing_summary(bills);

    assert_eq!(bills.pop(), Some(Bill(500)));
}
