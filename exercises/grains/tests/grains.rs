extern crate grains;

#[test]
fn square_one() {
    assert_eq!(grains::square(1), 1);
}

#[test]
fn square_two() {
    assert_eq!(grains::square(2), 2);
}

#[test]
fn square_three() {
    assert_eq!(grains::square(3), 4);
}

#[test]
fn square_four() {
    assert_eq!(grains::square(4), 8);
}

#[test]
fn square_sixteen() {
    assert_eq!(grains::square(16), 32_768);
}

#[test]
fn square_thirty_two() {
    assert_eq!(grains::square(32), 2_147_483_648);
}

#[test]
fn square_sixty_four() {
    assert_eq!(grains::square(64), 9_223_372_036_854_775_808);
}

#[test]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_zero_panics() {
    grains::square(0);
}

#[test]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_sixty_five_panics() {
    grains::square(65);
}

#[test]
fn total_sums_all_squares() {
    assert_eq!(grains::total(), 18_446_744_073_709_551_615);
    grains::total();
}
