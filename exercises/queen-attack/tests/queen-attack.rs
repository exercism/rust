extern crate queen_attack;

use queen_attack::*;

#[test]
fn test_can_not_attack() {
    let white_queen = Queen::new((2,4));
    let black_queen = Queen::new((6,6));
    assert_eq!(false, white_queen.can_attack(black_queen).unwrap());
}

#[test]
#[ignore]
fn test_can_attack_on_same_rank() {
    let white_queen = Queen::new((2,4));
    let black_queen = Queen::new((2,6));
    assert!(white_queen.can_attack(black_queen).unwrap());
}

#[test]
#[ignore]
fn test_can_attack_on_same_file() {
    let white_queen = Queen::new((4,5));
    let black_queen = Queen::new((3,5));
    assert!(white_queen.can_attack(black_queen).unwrap());
}

#[test]
#[ignore]
fn test_can_attack_on_first_diagonal() {
    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((0,4));
    assert!(white_queen.can_attack(black_queen).unwrap());
}

#[test]
#[ignore]
fn test_can_attack_on_second_diagonal() {
    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((3,1));
    assert!(white_queen.can_attack(black_queen).unwrap());
}

#[test]
#[ignore]
fn test_can_attack_on_third_diagonal() {
    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((1,1));
    assert!(white_queen.can_attack(black_queen).unwrap());
}

#[test]
#[ignore]
fn test_can_attack_on_fourth_diagonal() {
    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((5,5));
    assert!(white_queen.can_attack(black_queen).unwrap());
}

#[test]
#[ignore]
fn test_queen_with_invalid_position() {
    let white_queen = Queen::new((-1,2));
    let black_queen = Queen::new((5,5));
    assert!(white_queen.can_attack(black_queen).is_err());

    let white_queen = Queen::new((8,2));
    let black_queen = Queen::new((5,5));
    assert!(white_queen.can_attack(black_queen).is_err());

    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((5,-1));
    assert!(white_queen.can_attack(black_queen).is_err());

    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((5,8));
    assert!(white_queen.can_attack(black_queen).is_err());
}
