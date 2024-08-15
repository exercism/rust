use raindrops::*;

#[test]
fn the_sound_for_1_is_1() {
    let input = 1;
    let output = raindrops(input);
    let expected = "1";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_3_is_pling() {
    let input = 3;
    let output = raindrops(input);
    let expected = "Pling";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_5_is_plang() {
    let input = 5;
    let output = raindrops(input);
    let expected = "Plang";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_7_is_plong() {
    let input = 7;
    let output = raindrops(input);
    let expected = "Plong";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_6_is_pling_as_it_has_a_factor_3() {
    let input = 6;
    let output = raindrops(input);
    let expected = "Pling";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_2_to_the_power_3_does_not_make_a_raindrop_sound_as_3_is_the_exponent_not_the_base() {
    let input = 8;
    let output = raindrops(input);
    let expected = "8";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_9_is_pling_as_it_has_a_factor_3() {
    let input = 9;
    let output = raindrops(input);
    let expected = "Pling";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_10_is_plang_as_it_has_a_factor_5() {
    let input = 10;
    let output = raindrops(input);
    let expected = "Plang";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_14_is_plong_as_it_has_a_factor_of_7() {
    let input = 14;
    let output = raindrops(input);
    let expected = "Plong";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_15_is_plingplang_as_it_has_factors_3_and_5() {
    let input = 15;
    let output = raindrops(input);
    let expected = "PlingPlang";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_21_is_plingplong_as_it_has_factors_3_and_7() {
    let input = 21;
    let output = raindrops(input);
    let expected = "PlingPlong";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_25_is_plang_as_it_has_a_factor_5() {
    let input = 25;
    let output = raindrops(input);
    let expected = "Plang";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_27_is_pling_as_it_has_a_factor_3() {
    let input = 27;
    let output = raindrops(input);
    let expected = "Pling";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_35_is_plangplong_as_it_has_factors_5_and_7() {
    let input = 35;
    let output = raindrops(input);
    let expected = "PlangPlong";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_49_is_plong_as_it_has_a_factor_7() {
    let input = 49;
    let output = raindrops(input);
    let expected = "Plong";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_52_is_52() {
    let input = 52;
    let output = raindrops(input);
    let expected = "52";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_105_is_plingplangplong_as_it_has_factors_3_5_and_7() {
    let input = 105;
    let output = raindrops(input);
    let expected = "PlingPlangPlong";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn the_sound_for_3125_is_plang_as_it_has_a_factor_5() {
    let input = 3125;
    let output = raindrops(input);
    let expected = "Plang";
    assert_eq!(output, expected);
}
