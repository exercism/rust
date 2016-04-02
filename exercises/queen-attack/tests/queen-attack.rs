extern crate queen_attack;

use queen_attack::*;

#[test]
fn test_queen_creation_with_valid_position() {
    let white_queen = Queen::new((2,4));
    assert!(white_queen.is_ok());
}

#[test]
#[ignore]
fn test_queen_creation_with_incorrect_positions() {
    let white_queen = Queen::new((-1,2));
    assert!(white_queen.is_err());

    let white_queen = Queen::new((8,2));
    assert!(white_queen.is_err());

    let white_queen = Queen::new((5,-1));
    assert!(white_queen.is_err());

    let white_queen = Queen::new((5,8));
    assert!(white_queen.is_err());
}

#[test]
#[ignore]
fn test_can_not_attack() {
    let white_queen = Queen::new((2,4)).unwrap();
    let black_queen = Queen::new((6,6)).unwrap();
    assert_eq!(false, white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn test_can_attack_on_same_rank() {
    let white_queen = Queen::new((2,4)).unwrap();
    let black_queen = Queen::new((2,6)).unwrap();
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn test_can_attack_on_same_file() {
    let white_queen = Queen::new((4,5)).unwrap();
    let black_queen = Queen::new((3,5)).unwrap();
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn test_can_attack_on_first_diagonal() {
    let white_queen = Queen::new((2,2)).unwrap();
    let black_queen = Queen::new((0,4)).unwrap();
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn test_can_attack_on_second_diagonal() {
    let white_queen = Queen::new((2,2)).unwrap();
    let black_queen = Queen::new((3,1)).unwrap();
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn test_can_attack_on_third_diagonal() {
    let white_queen = Queen::new((2,2)).unwrap();
    let black_queen = Queen::new((1,1)).unwrap();
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn test_can_attack_on_fourth_diagonal() {
    let white_queen = Queen::new((2,2)).unwrap();
    let black_queen = Queen::new((5,5)).unwrap();
    assert!(white_queen.can_attack(&black_queen));
}
