use space_age::*;

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
    }
}

#[test]
fn earth_age() {
    let duration = Duration::from(1_000_000_000);
    assert_in_delta(31.69, Earth::years_during(&duration));
}

#[test]
#[ignore]
fn mercury_age() {
    let duration = Duration::from(2_134_835_688);
    assert_in_delta(280.88, Mercury::years_during(&duration));
}

#[test]
#[ignore]
fn venus_age() {
    let duration = Duration::from(189_839_836);
    assert_in_delta(9.78, Venus::years_during(&duration));
}

#[test]
#[ignore]
fn mars_age() {
    let duration = Duration::from(2_129_871_239);
    assert_in_delta(35.88, Mars::years_during(&duration));
}

#[test]
#[ignore]
fn jupiter_age() {
    let duration = Duration::from(901_876_382);
    assert_in_delta(2.41, Jupiter::years_during(&duration));
}

#[test]
#[ignore]
fn saturn_age() {
    let duration = Duration::from(2_000_000_000);
    assert_in_delta(2.15, Saturn::years_during(&duration));
}

#[test]
#[ignore]
fn uranus_age() {
    let duration = Duration::from(1_210_123_456);
    assert_in_delta(0.46, Uranus::years_during(&duration));
}

#[test]
#[ignore]
fn neptune_age() {
    let duration = Duration::from(1_821_023_456);
    assert_in_delta(0.35, Neptune::years_during(&duration));
}
