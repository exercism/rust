use collatz_conjecture::*;

#[test]
fn one() {
    assert_eq!(Some(0), collatz(1));
}

#[test]
#[ignore]
fn sixteen() {
    assert_eq!(Some(4), collatz(16));
}

#[test]
#[ignore]
fn twelve() {
    assert_eq!(Some(9), collatz(12));
}

#[test]
#[ignore]
fn one_million() {
    assert_eq!(Some(152), collatz(1_000_000));
}

#[test]
#[ignore]
fn zero() {
    assert_eq!(None, collatz(0));
}

#[test]
#[ignore]
fn test_110243094271() {
    let val = 110243094271;
    let collatz_length = collatz(val);
    if collatz_length.is_some() {
        assert_eq!(Some(572), collatz_length);
    }
}

#[test]
#[ignore]
fn max_div_3() {
    let max = u64::MAX / 3;
    let collatz_length = collatz(max);
    dbg!(max, collatz_length);
    if collatz_length.is_some() {
        assert_eq!(Some(65), collatz_length);
    }
}

#[test]
#[ignore]
fn max_minus_1() {
    let max = u64::MAX - 1;
    let collatz_length = collatz(max);
    if collatz_length.is_some() {
        assert_eq!(Some(863), collatz_length);
    }
}
