fn process_leapyear_case(year: u64, expected: bool) {
    assert_eq!(leap::is_leap_year(year), expected);
}

#[test]
fn year_not_divisible_by_4_common_year() {
    process_leapyear_case(2015, false);
}

#[test]
#[ignore]
fn year_divisible_by_2_not_divisible_by_4_in_common_year() {
    process_leapyear_case(1970, false);
}

#[test]
#[ignore]
fn year_divisible_by_4_not_divisible_by_100_leap_year() {
    process_leapyear_case(1996, true);
}

#[test]
#[ignore]
fn year_divisible_by_4_and_5_is_still_a_leap_year() {
    process_leapyear_case(1960, true);
}

#[test]
#[ignore]
fn year_divisible_by_100_not_divisible_by_400_common_year() {
    process_leapyear_case(2100, false);
}

#[test]
#[ignore]
fn year_divisible_by_100_but_not_by_3_is_still_not_a_leap_year() {
    process_leapyear_case(1900, false);
}

#[test]
#[ignore]
fn year_divisible_by_400_leap_year() {
    process_leapyear_case(2000, true);
}

#[test]
#[ignore]
fn year_divisible_by_400_but_not_by_125_is_still_a_leap_year() {
    process_leapyear_case(2400, true);
}

#[test]
#[ignore]
fn year_divisible_by_200_not_divisible_by_400_common_year() {
    process_leapyear_case(1800, false);
}

#[test]
#[ignore]
fn any_old_year() {
    process_leapyear_case(1997, false);
}

#[test]
#[ignore]
fn early_years() {
    process_leapyear_case(1, false);
    process_leapyear_case(4, true);
    process_leapyear_case(100, false);
    process_leapyear_case(400, true);
    process_leapyear_case(900, false);
}

#[test]
#[ignore]
fn century() {
    process_leapyear_case(1700, false);
    process_leapyear_case(1800, false);
    process_leapyear_case(1900, false);
}

#[test]
#[ignore]
fn exceptional_centuries() {
    process_leapyear_case(1600, true);
    process_leapyear_case(2000, true);
    process_leapyear_case(2400, true);
}

#[test]
#[ignore]
fn years_1600_to_1699() {
    let incorrect_years = (1600..1700)
        .filter(|&year| leap::is_leap_year(year) != (year % 4 == 0))
        .collect::<Vec<_>>();

    if !incorrect_years.is_empty() {
        panic!("incorrect result for years: {incorrect_years:?}");
    }
}
