extern crate leap;

#[test]
fn test_vanilla_leap_year() {
    assert_eq!(leap::is_leap_year(1996), true);
}

#[test]

fn test_any_old_year() {
    assert_eq!(leap::is_leap_year(1997), false);
}

#[test]

fn test_century() {
    assert_eq!(leap::is_leap_year(1900), false);
}

#[test]

fn test_exceptional_centuries() {
    assert_eq!(leap::is_leap_year(2000), true);
    assert_eq!(leap::is_leap_year(2400), true);
}
