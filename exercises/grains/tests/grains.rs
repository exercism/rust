extern crate grains;

#[test]
fn square_one() {
    assert_eq!(grains::square(1), 1);
}

#[test]
#[ignore]
fn square_two() {
    assert_eq!(grains::square(2), 2);
}

#[test]
#[ignore]
fn square_three() {
    assert_eq!(grains::square(3), 4);
}

#[test]
#[ignore]
fn square_four() {
    assert_eq!(grains::square(4), 8);
}

#[test]
#[ignore]
fn square_sixteen() {
    assert_eq!(grains::square(16), 32_768);
}

#[test]
#[ignore]
fn square_thirty_two() {
    assert_eq!(grains::square(32), 2_147_483_648);
}

#[test]
#[ignore]
fn square_sixty_four() {
    assert_eq!(grains::square(64), 9_223_372_036_854_775_808);
}

#[test]
#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_zero_panics() {
    grains::square(0);
}

#[test]
#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_sixty_five_panics() {
    grains::square(65);
}

#[test]
#[ignore]
fn total_sums_all_squares() {
    assert_eq!(grains::total(), 18_446_744_073_709_551_615);
    grains::total();
}
