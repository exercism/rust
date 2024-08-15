use roman_numerals::Roman;

#[test]
fn test_1_is_i() {
    let input = 1;
    let output = Roman::from(input).to_string();
    let expected = "I";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_2_is_ii() {
    let input = 2;
    let output = Roman::from(input).to_string();
    let expected = "II";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_3_is_iii() {
    let input = 3;
    let output = Roman::from(input).to_string();
    let expected = "III";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_4_is_iv() {
    let input = 4;
    let output = Roman::from(input).to_string();
    let expected = "IV";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_5_is_v() {
    let input = 5;
    let output = Roman::from(input).to_string();
    let expected = "V";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_6_is_vi() {
    let input = 6;
    let output = Roman::from(input).to_string();
    let expected = "VI";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_9_is_ix() {
    let input = 9;
    let output = Roman::from(input).to_string();
    let expected = "IX";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_16_is_xvi() {
    let input = 16;
    let output = Roman::from(input).to_string();
    let expected = "XVI";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_27_is_xxvii() {
    let input = 27;
    let output = Roman::from(input).to_string();
    let expected = "XXVII";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_48_is_xlviii() {
    let input = 48;
    let output = Roman::from(input).to_string();
    let expected = "XLVIII";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_49_is_xlix() {
    let input = 49;
    let output = Roman::from(input).to_string();
    let expected = "XLIX";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_59_is_lix() {
    let input = 59;
    let output = Roman::from(input).to_string();
    let expected = "LIX";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_66_is_lxvi() {
    let input = 66;
    let output = Roman::from(input).to_string();
    let expected = "LXVI";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_93_is_xciii() {
    let input = 93;
    let output = Roman::from(input).to_string();
    let expected = "XCIII";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_141_is_cxli() {
    let input = 141;
    let output = Roman::from(input).to_string();
    let expected = "CXLI";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_163_is_clxiii() {
    let input = 163;
    let output = Roman::from(input).to_string();
    let expected = "CLXIII";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_166_is_clxvi() {
    let input = 166;
    let output = Roman::from(input).to_string();
    let expected = "CLXVI";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_402_is_cdii() {
    let input = 402;
    let output = Roman::from(input).to_string();
    let expected = "CDII";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_575_is_dlxxv() {
    let input = 575;
    let output = Roman::from(input).to_string();
    let expected = "DLXXV";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_666_is_dclxvi() {
    let input = 666;
    let output = Roman::from(input).to_string();
    let expected = "DCLXVI";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_911_is_cmxi() {
    let input = 911;
    let output = Roman::from(input).to_string();
    let expected = "CMXI";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_1024_is_mxxiv() {
    let input = 1024;
    let output = Roman::from(input).to_string();
    let expected = "MXXIV";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_1666_is_mdclxvi() {
    let input = 1666;
    let output = Roman::from(input).to_string();
    let expected = "MDCLXVI";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_3000_is_mmm() {
    let input = 3000;
    let output = Roman::from(input).to_string();
    let expected = "MMM";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_3001_is_mmmi() {
    let input = 3001;
    let output = Roman::from(input).to_string();
    let expected = "MMMI";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_3888_is_mmmdccclxxxviii() {
    let input = 3888;
    let output = Roman::from(input).to_string();
    let expected = "MMMDCCCLXXXVIII";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_3999_is_mmmcmxcix() {
    let input = 3999;
    let output = Roman::from(input).to_string();
    let expected = "MMMCMXCIX";
    assert_eq!(output, expected);
}
