use nth_prime::*;

#[test]
fn first_prime() {
    let output = nth(0);
    let expected = 2;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn second_prime() {
    let output = nth(1);
    let expected = 3;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn sixth_prime() {
    let output = nth(5);
    let expected = 13;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn big_prime() {
    let output = nth(10_000);
    let expected = 104_743;
    assert_eq!(output, expected);
}
