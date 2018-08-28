extern crate leap;

#[test]
fn test_vanilla_leap_year() {
    assert_eq!(leap::is_leap_year(1996), true);
}

#[test]
#[ignore]
fn test_any_old_year() {
    assert_eq!(leap::is_leap_year(1997), false);
}

#[test]
#[ignore]
fn test_century() {
    assert_eq!(leap::is_leap_year(1700), false);
    assert_eq!(leap::is_leap_year(1800), false);
    assert_eq!(leap::is_leap_year(1900), false);
}

#[test]
#[ignore]
fn test_exceptional_centuries() {
    assert_eq!(leap::is_leap_year(1600), true);
    assert_eq!(leap::is_leap_year(2000), true);
    assert_eq!(leap::is_leap_year(2400), true);
}

#[test]
#[ignore]
fn test_years_1600_to_1699() {
    let incorrect_years = (1600..1700)
        .filter(|&year| leap::is_leap_year(year) != (year % 4 == 0))
        .collect::<Vec<_>>();

    if !incorrect_years.is_empty() {
        panic!("incorrect result for years: {:?}", incorrect_years);
    }
}
