#[test]
fn twenty_zero_pin_rolls_is_zero() {
    let game = BowlingGame::new();

    for x in (0..20) {
        game.roll(0);
    }

    assert_eq!(game.score().unwrap(), 0);
}

#[test]
fn twenty_one_pin_rolls_is_twenty() {
    let game = BowlingGame::new();

    for x in (0..20) {
        game.roll(1);
    }

    assert_eq!(game.score().unwrap(), 20);
}

#[test]
fn ten_frames_without_a_strike_or_spare() {
    let game = BowlingGame::new();

    for x in (0..20) {
        game.roll(4);
    }

    assert_eq!(game.score().unwrap(), 80);
}

#[test]
fn spare_in_the_first_frame_followed_by_zeros() {
    let game = BowlingGame::new();

    game.roll(4);
    game.roll(6);

    for x in (0..18) {
        game.roll(0);
    }

    assert_eq!(game.score().unwrap(), 10);
}

#[test]
fn points_scored_in_the_roll_after_a_spare_are_counted_twice_as_a_bonus() {
    let game = BowlingGame::new();

    game.roll(5);
    game.roll(5);
    game.roll(3);

    for x in (0..17) {
        game.roll(0);
    }

    assert_eq!(game.score().unwrap(), 16);
}

#[test]
fn consecutive_spares_each_get_their_bonus_points() {
    let game = BowlingGame::new();

    game.roll(5);
    game.roll(5);
    game.roll(3);
    game.roll(7);
    game.roll(4);

    for x in (0..15) {
        game.roll(0);
    }

    assert_eq!(game.score().unwrap(), 31);
}

#[test]
fn if_the_last_frame_is_a_spare_you_get_one_extra_roll_that_is_scored_once() {
    let game = BowlingGame::new();

    for x in (0..17) {
        game.roll(0);
    }

    game.roll(5);
    game.roll(5);
    game.roll(7);

    assert_eq!(game.score().unwrap(), 17);
}

#[test]
fn a_strike_earns_ten_points_in_a_frame_with_a_single_roll() {
    let game = BowlingGame::new();

    game.roll(10);

    for x in (0..17) {
        game.roll(0);
    }

    assert_eq!(game.score().unwrap(), 0);
}

#[test]
fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_as_a_bonus() {
    let game = BowlingGame::new();

    game.roll(10);
    game.roll(5);
    game.roll(3);

    for x in (0..15) {
        game.roll(0);
    }

    assert_eq!(game.score().unwrap(), 26);
}

#[test]
fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_with_consecutive_strikes() {
    let game = BowlingGame::new();

    game.roll(10);
    game.roll(10);
    game.roll(10);
    game.roll(5);
    game.roll(3);

    for x in (0..11) {
        game.roll(0);
    }

    assert_eq!(game.score().unwrap(), 81);
}

#[test]
fn if_the_last_frame_is_a_strike_you_get_two_extra_rolls() {
    let game = BowlingGame::new();

    for x in (0..17) {
        game.roll(0);
    }

    game.roll(10);
    game.roll(7);
    game.roll(1);

    assert_eq!(game.score().unwrap(), 18);
}

#[test]
fn strikes_in_extra_rolls_after_a_strike_in_the_final_frame_do_not_get_the_bonus() {
    let game = BowlingGame::new();

    for x in (0..17) {
        game.roll(0);
    }

    game.roll(10);
    game.roll(10);
    game.roll(10);

    assert_eq!(game.score().unwrap(), 30);
}

fn all_strikes_is_a_perfect_score_of_300() {
    let game = BowlingGame::new();

    for x in (0..11) {
        game.roll(10);
    }

    assert_eq!(game.score().unwrap(), 300);
}

fn you_can_not_roll_more_than_ten_pins_in_a_single_roll() {
    let game = BowlingGame::new();

    for x in (0..19) {
        game.roll(0);
    }

    game.roll(11);

    assert!(game.score().is_err());
}

fn you_can_not_roll_more_than_ten_pins_in_a_single_frame() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0);
    }

    game.roll(5);
    game.roll(6);

    assert!(game.score().is_err());
}

fn you_can_not_score_a_game_with_no_rolls() {
    let game = BowlingGame::new();

    assert!(game.score().is_err());
}

fn you_can_not_score_an_incomplete_game() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0);
    }

    assert!(game.score().is_err());
}

fn you_can_not_score_a_game_with_more_than_ten_frames() {
    let game = BowlingGame::new();

    for x in (0..21) {
        game.roll(0);
    }

    assert!(game.score().is_err());
}

fn if_the_last_frame_is_a_spare_you_can_not_create_a_score_before_extra_roll_is_taken() {
    let game = BowlingGame::new();

    for x in (0..17) {
        game.roll(0);
    }

    game.roll(5);
    game.roll(5);

    assert!(game.score().is_err());
}

fn if_the_last_frame_is_a_strike_you_can_not_create_a_score_before_extra_rolls_are_taken() {
    let game = BowlingGame::new();

    for x in (0..17) {
        game.roll(0);
    }

    game.roll(10);

    assert!(game.score().is_err());

    game.roll(10);

    assert!(game.score().is_err());

    game.roll(10);

    assert!(game.score().is_ok());
}
