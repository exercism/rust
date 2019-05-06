use space_age::*;

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!(
            "Your result of {} should be within {} of the expected result {}",
            actual, delta, expected
        )
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
    let duration = Duration::from(2_329_871_239);
    assert_in_delta(39.25, Mars::years_during(&duration));
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
    let duration = Duration::from(3_000_000_000);
    assert_in_delta(3.23, Saturn::years_during(&duration));
}

#[test]
#[ignore]
fn uranus_age() {
    let duration = Duration::from(3_210_123_456);
    assert_in_delta(1.21, Uranus::years_during(&duration));
}

#[test]
#[ignore]
fn neptune_age() {
    let duration = Duration::from(8_210_123_456);
    assert_in_delta(1.58, Neptune::years_during(&duration));
}
