use say;

// Note: No tests created using 'and' with numbers.
// Apparently Most American English does not use the 'and' with numbers,
// where it is common in British English to use the 'and'.

#[test]
fn test_zero() {
    assert_eq!(String::from("zero"), say::encode(0));
}

//
// If the below test is uncommented, it should not compile.
//
/*
#[test]
#[ignore]
fn test_negative() {
    assert_eq!(String::from("won't compile"), say::encode(-1));
}
*/

#[test]
#[ignore]
fn test_one() {
    assert_eq!(String::from("one"), say::encode(1));
}

#[test]
#[ignore]
fn test_fourteen() {
    assert_eq!(String::from("fourteen"), say::encode(14));
}

#[test]
#[ignore]
fn test_twenty() {
    assert_eq!(String::from("twenty"), say::encode(20));
}

#[test]
#[ignore]
fn test_twenty_two() {
    assert_eq!(String::from("twenty-two"), say::encode(22));
}

#[test]
#[ignore]
fn test_one_hundred() {
    assert_eq!(String::from("one hundred"), say::encode(100));
}

// note, using American style with no and
#[test]
#[ignore]
fn test_one_hundred_twenty() {
    assert_eq!(String::from("one hundred twenty"), say::encode(120));
}

#[test]
#[ignore]
fn test_one_hundred_twenty_three() {
    assert_eq!(String::from("one hundred twenty-three"), say::encode(123));
}

#[test]
#[ignore]
fn test_one_thousand() {
    assert_eq!(String::from("one thousand"), say::encode(1000));
}

#[test]
#[ignore]
fn test_one_thousand_two_hundred_thirty_four() {
    assert_eq!(String::from("one thousand two hundred thirty-four"), 
        say::encode(1234));
}

// note, using American style with no and
#[test]
#[ignore]
fn test_eight_hundred_and_ten_thousand() {
    assert_eq!(String::from("eight hundred ten thousand"), 
        say::encode(810_000));
}

#[test]
#[ignore]
fn test_one_million() {
    assert_eq!(String::from("one million"), say::encode(1_000_000));
}

// note, using American style with no and
#[test]
#[ignore]
fn test_one_million_two() {
    assert_eq!(String::from("one million two"), say::encode(1_000_002));
}

#[test]
#[ignore]
fn test_1002345() {
    assert_eq!(String::from("one million two thousand three hundred forty-five"), 
        say::encode(1_002_345));
}

#[test]
#[ignore]
fn test_one_billion() {
    assert_eq!(String::from("one billion"), say::encode(1_000_000_000));
}

#[test]
#[ignore]
fn test_987654321123() {
    assert_eq!(String::from(
            "nine hundred eighty-seven billion \
             six hundred fifty-four million \
             three hundred twenty-one thousand \
             one hundred twenty-three"
        ), 
        say::encode(987_654_321_123));
}

/*
  These tests are only if you implemented full parsing for u64 type.
*/
#[test]
#[ignore]
fn test_max_i64() {
    assert_eq!(String::from(
            "nine quintillion two hundred twenty-three \
             quadrillion three hundred seventy-two trillion \
             thirty-six billion eight hundred fifty-four million \
             seven hundred seventy-five thousand eight hundred seven"
        ), 
        say::encode(9_223_372_036_854_775_807));
}

#[test]
#[ignore]
fn test_max_u64() {
    assert_eq!(String::from(
            "eighteen quintillion four hundred forty-six \
             quadrillion seven hundred forty-four trillion \
             seventy-three billion seven hundred nine million \
             five hundred fifty-one thousand six hundred fifteen"
        ), 
        say::encode(18_446_744_073_709_551_615));
}
