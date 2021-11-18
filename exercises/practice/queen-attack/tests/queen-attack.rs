use queen_attack::*;

#[test]
fn chess_position_on_the_board_is_some() {
    assert!(ChessPosition::new(2, 4).is_some());
}

#[test]
#[ignore]
fn chess_position_off_the_board_is_none() {
    assert!(ChessPosition::new(-1, 2).is_none());

    assert!(ChessPosition::new(8, 2).is_none());

    assert!(ChessPosition::new(5, -1).is_none());

    assert!(ChessPosition::new(5, 8).is_none());
}

#[test]
#[ignore]
fn queen_is_created_with_a_valid_position() {
    Queen::new(ChessPosition::new(2, 4).unwrap());
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

#[test]
#[ignore]
fn queens_that_cannot_attack_with_equal_difference() {
    let white_queen = Queen::new(ChessPosition::new(4, 1).unwrap());
    let black_queen = Queen::new(ChessPosition::new(2, 5).unwrap());

    assert!(!white_queen.can_attack(&black_queen));
}
