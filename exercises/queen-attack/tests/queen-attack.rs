extern crate queen_attack;

use queen_attack::*;

#[test]
fn test_can_not_attack() {
    let white_queen = Queen::new((2,4));
    let black_queen = Queen::new((6,6));
    let queens = Queens::new(white_queen, black_queen);
    assert_eq!(false, queens.can_attack());
}

#[test]
#[ignore]
fn test_can_attack_on_same_rank() {
    let white_queen = Queen::new((2,4));
    let black_queen = Queen::new((2,6));
    let queens = Queens::new(white_queen, black_queen);
    assert!(queens.can_attack());
}

#[test]
#[ignore]
fn test_can_attack_on_same_file() {
    let white_queen = Queen::new((4,5));
    let black_queen = Queen::new((3,5));
    let queens = Queens::new(white_queen, black_queen);
    assert!(queens.can_attack());
}

#[test]
#[ignore]
fn test_can_attack_on_first_diagonal() {
    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((0,4));
    let queens = Queens::new(white_queen, black_queen);
    assert!(queens.can_attack());
}

#[test]
#[ignore]
fn test_can_attack_on_second_diagonal() {
    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((3,1));
    let queens = Queens::new(white_queen, black_queen);
    assert!(queens.can_attack());
}

#[test]
#[ignore]
fn test_can_attack_on_third_diagonal() {
    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((1,1));
    let queens = Queens::new(white_queen, black_queen);
    assert!(queens.can_attack());
}

#[test]
#[ignore]
fn test_can_attack_on_fourth_diagonal() {
    let white_queen = Queen::new((2,2));
    let black_queen = Queen::new((5,5));
    let queens = Queens::new(white_queen, black_queen);
    assert!(queens.can_attack());
}

#[test]
#[ignore]
fn test_queen_with_invalid_position() {
    let white_queen = Queen::new((-1,2));
    match white_queen {
        Ok(_) => panic!("This queen should be invalid"),
        Err(_) => assert!(true)
    }

    let white_queen = Queen::new((8,2));
    match white_queen {
        Ok(_) => panic!("This queen should be invalid"),
        Err(_) => assert!(true)
    }

    let white_queen = Queen::new((2,-1));
    match white_queen {
        Ok(_) => panic!("This queen should be invalid"),
        Err(_) => assert!(true)
    }

    let white_queen = Queen::new((2,8));
    match white_queen {
        Ok(_) => panic!("This queen should be invalid"),
        Err(_) => assert!(true)
    }
}
