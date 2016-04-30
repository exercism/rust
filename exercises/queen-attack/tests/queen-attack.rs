extern crate queen_attack;

use queen_attack::*;

#[test]
fn chess_position_on_the_board_is_ok() {
    assert!(ChessPosition::new(2, 4).is_ok());
}

#[test]
#[ignore]
fn chess_position_off_the_board_is_err() {
    assert!(ChessPosition::new(-1, 2).is_err());
    assert!(ChessPosition::new(8, 2).is_err());
    assert!(ChessPosition::new(5, -1).is_err());
    assert!(ChessPosition::new(5, 8).is_err());
}

#[test]
#[ignore]
fn queen_is_created_with_a_ok_position() {
    Queen::new(ChessPosition::new(2, 4).unwrap());
}

#[test]
#[ignore]
#[should_panic(expected = "Invalid Position")]
fn queen_panics_with_err_position() {
    Queen::new(ChessPosition::new(-1, 4).unwrap());
}

#[test]
#[ignore]
fn queens_that_can_not_attack() {
    let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
    let black_queen = Queen::new(ChessPosition::new(6, 6).unwrap());
    assert!(!white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn queens_on_the_same_rank_can_attack() {
    let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
    let black_queen = Queen::new(ChessPosition::new(2, 6).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn queens_on_the_same_file_can_attack() {
    let white_queen = Queen::new(ChessPosition::new(4, 5).unwrap());
    let black_queen = Queen::new(ChessPosition::new(3, 5).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn queens_on_the_same_diagonal_can_attack_one() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn queens_on_the_same_diagonal_can_attack_two() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(3, 1).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn queens_on_the_same_diagonal_can_attack_three() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(1, 1).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn queens_on_the_same_diagonal_can_attack_four() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(5, 5).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}
