use roman_numerals::*;

#[test]
fn test_one() {
    assert_eq!("I", Roman::from(1).to_string());
}

#[test]
#[ignore]
fn test_two() {
    assert_eq!("II", Roman::from(2).to_string());
}

#[test]
#[ignore]
fn test_three() {
    assert_eq!("III", Roman::from(3).to_string());
}

#[test]
#[ignore]
fn test_four() {
    assert_eq!("IV", Roman::from(4).to_string());
}

#[test]
#[ignore]
fn test_five() {
    assert_eq!("V", Roman::from(5).to_string());
}

#[test]
#[ignore]
fn test_six() {
    assert_eq!("VI", Roman::from(6).to_string());
}

#[test]
#[ignore]
fn test_nine() {
    assert_eq!("IX", Roman::from(9).to_string());
}

#[test]
#[ignore]
fn test_twenty_seven() {
    assert_eq!("XXVII", Roman::from(27).to_string());
}

#[test]
#[ignore]
fn test_forty_eight() {
    assert_eq!("XLVIII", Roman::from(48).to_string());
}

#[test]
#[ignore]
fn test_fifty_nine() {
    assert_eq!("LIX", Roman::from(59).to_string());
}

#[test]
#[ignore]
fn test_ninety_three() {
    assert_eq!("XCIII", Roman::from(93).to_string());
}

#[test]
#[ignore]
fn test_141() {
    assert_eq!("CXLI", Roman::from(141).to_string());
}

#[test]
#[ignore]
fn test_163() {
    assert_eq!("CLXIII", Roman::from(163).to_string());
}

#[test]
#[ignore]
fn test_402() {
    assert_eq!("CDII", Roman::from(402).to_string());
}

#[test]
#[ignore]
fn test_575() {
    assert_eq!("DLXXV", Roman::from(575).to_string());
}

#[test]
#[ignore]
fn test_911() {
    assert_eq!("CMXI", Roman::from(911).to_string());
}

#[test]
#[ignore]
fn test_1024() {
    assert_eq!("MXXIV", Roman::from(1024).to_string());
}

#[test]
#[ignore]
fn test_3000() {
    assert_eq!("MMM", Roman::from(3000).to_string());
}
