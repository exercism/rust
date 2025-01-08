#[test]
pub fn analyze_non_fake_non_newyork() {
    assert_eq!(
        (false, false, "1234"),
        phone_number_analysis::analyze("631-502-1234")
    );
}

#[test]
#[ignore]
pub fn analyze_fake_non_newyork() {
    assert_eq!(
        (false, true, "1234"),
        phone_number_analysis::analyze("631-555-1234")
    );
}

#[test]
#[ignore]
pub fn analyze_non_fake_newyork() {
    assert_eq!(
        (true, false, "1234"),
        phone_number_analysis::analyze("212-502-1234")
    );
}

#[test]
#[ignore]
pub fn analyze_fake_newyork() {
    assert_eq!(
        (true, true, "1234"),
        phone_number_analysis::analyze("212-555-1234")
    );
}

#[test]
#[ignore]
pub fn analyze_fake_fake() {
    assert_eq!(
        (false, false, "1234"),
        phone_number_analysis::analyze("515-212-1234")
    );
}

#[test]
#[ignore]
pub fn is_fake_fake() {
    assert!(phone_number_analysis::is_fake(
        phone_number_analysis::analyze("212-555-1234")
    ));
}

#[test]
#[ignore]
pub fn is_fake_non_fake() {
    assert!(!phone_number_analysis::is_fake(
        phone_number_analysis::analyze("555-212-1234")
    ));
}
