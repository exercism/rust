use leap;

fn process_leapyear_case(year: u64, expected: bool) {
    assert_eq!(leap::is_leap_year(year), expected);
}

#[test]
fn test_year_divisible_by_4_not_divisible_by_100_leap_year() {
    process_leapyear_case(1996, true);
}

#[test]
#[ignore]
fn test_year_not_divisible_by_4_common_year() {
    process_leapyear_case(2015, false);
}

#[test]
#[ignore]
fn test_year_divisible_by_200_not_divisible_by_400_common_year() {
    process_leapyear_case(1800, false);
}

#[test]
#[ignore]
fn test_year_divisible_by_100_not_divisible_by_400_common_year() {
    process_leapyear_case(2100, false);
}

#[test]
#[ignore]
fn test_year_divisible_by_400_leap_year() {
    process_leapyear_case(2000, true);
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
