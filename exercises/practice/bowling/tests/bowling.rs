use bowling::*;

#[test]
fn roll_returns_a_result() {
    let mut game = BowlingGame::new();
    assert!(game.roll(0).is_ok());
}

#[test]
#[ignore]
fn a_roll_cannot_score_more_than_10_points() {
    let mut game = BowlingGame::new();

    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
}

#[test]
#[ignore]
fn should_be_able_to_score_a_game_with_all_zeros() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }

    assert!(game.score().is_some());
}

#[test]
#[ignore]
fn an_unstarted_game_cannot_be_scored() {
    let game = BowlingGame::new();

    assert_eq!(game.score(), None);
}

#[test]
#[ignore]
fn an_incomplete_game_cannot_be_scored() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), None);
}

#[test]
#[ignore]
fn cannot_roll_if_game_already_has_ten_frames() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }

    assert_eq!(game.roll(0), Err(Error::GameComplete));
}

#[test]
#[ignore]
fn twenty_zero_pin_rolls_scores_zero() {
    let mut game = BowlingGame::new();

    for _ in 0..20 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(0));
}

#[test]
#[ignore]
fn should_be_able_to_score_a_game_with_no_strikes_or_spares() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(3);
        let _ = game.roll(6);
    }

    assert_eq!(game.score(), Some(90));
}

#[test]
#[ignore]
fn a_spare_followed_by_zeros_is_worth_ten_points() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4);

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(10));
}

#[test]
#[ignore]
fn points_scored_in_the_roll_after_a_spare_are_counted_twice() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4);
    let _ = game.roll(3);

    for _ in 0..17 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(16));
}

#[test]
#[ignore]
fn consecutive_spares_each_get_a_one_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(5);
    let _ = game.roll(5);
    let _ = game.roll(3);
    let _ = game.roll(7);
    let _ = game.roll(4);

    for _ in 0..15 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(31));
}

#[test]
#[ignore]
fn a_spare_in_the_last_frame_gets_a_one_roll_bonus_that_is_counted_once() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(5);
    let _ = game.roll(5);
    let _ = game.roll(7);

    assert_eq!(game.score(), Some(17));
}

#[test]
#[ignore]
fn a_strike_earns_ten_points_in_a_frame_with_a_single_roll() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(10));
}

#[test]
#[ignore]
fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_as_a_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);
    let _ = game.roll(5);
    let _ = game.roll(3);

    for _ in 0..16 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(26));
}

#[test]
#[ignore]
fn consecutive_strikes_each_get_the_two_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(5);
    let _ = game.roll(3);

    for _ in 0..12 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(81));
}

#[test]
#[ignore]
fn a_strike_in_the_last_frame_earns_a_two_roll_bonus_that_is_counted_once() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(7);
    let _ = game.roll(1);

    assert_eq!(game.score(), Some(18));
}

#[test]
#[ignore]
fn rolling_a_spare_with_the_two_roll_bonus_does_not_get_a_bonus_roll() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(7);
    let _ = game.roll(3);

    assert_eq!(game.score(), Some(20));
}

#[test]
#[ignore]
fn strikes_with_the_two_roll_bonus_do_not_get_a_bonus_roll() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(10);

    assert_eq!(game.score(), Some(30));
}

#[test]
#[ignore]
fn a_strike_with_the_one_roll_bonus_after_a_spare_in_the_last_frame_does_not_get_a_bonus() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(7);
    let _ = game.roll(3);
    let _ = game.roll(10);

    assert_eq!(game.score(), Some(20));
}

#[test]
#[ignore]
fn all_strikes_is_a_perfect_game() {
    let mut game = BowlingGame::new();

    for _ in 0..12 {
        let _ = game.roll(10);
    }

    assert_eq!(game.score(), Some(300));
}

#[test]
#[ignore]
fn two_rolls_in_a_frame_cannot_score_more_than_10_points() {
    let mut game = BowlingGame::new();

    assert!(game.roll(5).is_ok());
    assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
}

#[test]
#[ignore]
fn bonus_roll_after_a_strike_in_the_last_frame_cannot_score_more_than_10_points() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
}

#[test]
#[ignore]
fn two_bonus_rolls_after_a_strike_in_the_last_frame_cannot_score_more_than_10_points() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(5).is_ok());
    assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
}

#[test]
#[ignore]
fn two_bonus_rolls_after_a_strike_in_the_last_frame_can_score_more_than_10_points_if_one_is_a_strike()
 {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(10).is_ok());
    assert!(game.roll(6).is_ok());
}

#[test]
#[ignore]
fn the_second_bonus_rolls_after_a_strike_in_the_last_frame_cannot_be_a_strike_if_the_first_one_is_not_a_strike()
 {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(6).is_ok());
    assert_eq!(game.roll(10), Err(Error::NotEnoughPinsLeft));
}

#[test]
#[ignore]
fn second_bonus_roll_after_a_strike_in_the_last_frame_cannot_score_more_than_10_points() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(10).is_ok());
    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
}

#[test]
#[ignore]
fn bonus_rolls_for_a_strike_in_the_last_frame_must_be_rolled_before_score_can_be_calculated() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert_eq!(game.score(), None);
}

#[test]
#[ignore]
fn both_bonus_rolls_for_a_strike_in_the_last_frame_must_be_rolled_before_score_can_be_calculated() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(10);
    assert_eq!(game.score(), None);

    let _ = game.roll(10);
    assert!(game.score().is_some());
}

#[test]
#[ignore]
fn bonus_roll_for_a_spare_in_the_last_frame_must_be_rolled_before_score_can_be_calculated() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(5);
    let _ = game.roll(5);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert!(game.score().is_some());
}

#[test]
#[ignore]
fn cannot_roll_after_bonus_rolls_for_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }
    let _ = game.roll(7);
    let _ = game.roll(3);
    assert!(game.roll(2).is_ok());

    assert_eq!(game.roll(2), Err(Error::GameComplete));
}

#[test]
#[ignore]
fn cannot_roll_after_bonus_roll_for_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }
    let _ = game.roll(10);
    let _ = game.roll(3);
    assert!(game.roll(2).is_ok());

    assert_eq!(game.roll(2), Err(Error::GameComplete));
}

#[test]
#[ignore]
fn last_two_strikes_followed_by_only_last_bonus_with_non_strike_points() {
    let mut game = BowlingGame::new();
    for _ in 0..16 {
        let _ = game.roll(0);
    }
    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(0);
    let _ = game.roll(1);

    assert_eq!(game.score(), Some(31));
}
