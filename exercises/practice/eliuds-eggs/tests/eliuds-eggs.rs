use eliuds_eggs::*;

#[test]
fn test_0_eggs() {
    let input = 0;
    let output = egg_count(input);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_1_egg() {
    let input = 16;
    let output = egg_count(input);
    let expected = 1;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_4_eggs() {
    let input = 89;
    let output = egg_count(input);
    let expected = 4;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_13_eggs() {
    let input = 2_000_000_000;
    let output = egg_count(input);
    let expected = 13;
    assert_eq!(output, expected);
}
