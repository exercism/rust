use robot_simulator::*;

#[test]
fn at_origin_facing_north() {
    let robot = Robot::new(0, 0, Direction::North);
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::North);
}

#[test]
#[ignore]
fn at_negative_position_facing_south() {
    let robot = Robot::new(-1, -1, Direction::South);
    assert_eq!(robot.position(), (-1, -1));
    assert_eq!(robot.direction(), &Direction::South);
}

#[test]
#[ignore]
fn changes_north_to_east() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.turn_right();
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::East);
}

#[test]
#[ignore]
fn changes_east_to_south() {
    let robot = Robot::new(0, 0, Direction::East);
    let robot = robot.turn_right();
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::South);
}

#[test]
#[ignore]
fn changes_south_to_west() {
    let robot = Robot::new(0, 0, Direction::South);
    let robot = robot.turn_right();
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::West);
}

#[test]
#[ignore]
fn changes_west_to_north() {
    let robot = Robot::new(0, 0, Direction::West);
    let robot = robot.turn_right();
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::North);
}

#[test]
#[ignore]
fn changes_north_to_west() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.turn_left();
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::West);
}

#[test]
#[ignore]
fn changes_west_to_south() {
    let robot = Robot::new(0, 0, Direction::West);
    let robot = robot.turn_left();
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::South);
}

#[test]
#[ignore]
fn changes_south_to_east() {
    let robot = Robot::new(0, 0, Direction::South);
    let robot = robot.turn_left();
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::East);
}

#[test]
#[ignore]
fn changes_east_to_north() {
    let robot = Robot::new(0, 0, Direction::East);
    let robot = robot.turn_left();
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::North);
}

#[test]
#[ignore]
fn facing_north_increments_y() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.advance();
    assert_eq!(robot.position(), (0, 1));
    assert_eq!(robot.direction(), &Direction::North);
}

#[test]
#[ignore]
fn facing_south_decrements_y() {
    let robot = Robot::new(0, 0, Direction::South);
    let robot = robot.advance();
    assert_eq!(robot.position(), (0, -1));
    assert_eq!(robot.direction(), &Direction::South);
}

#[test]
#[ignore]
fn facing_east_increments_x() {
    let robot = Robot::new(0, 0, Direction::East);
    let robot = robot.advance();
    assert_eq!(robot.position(), (1, 0));
    assert_eq!(robot.direction(), &Direction::East);
}

#[test]
#[ignore]
fn facing_west_decrements_x() {
    let robot = Robot::new(0, 0, Direction::West);
    let robot = robot.advance();
    assert_eq!(robot.position(), (-1, 0));
    assert_eq!(robot.direction(), &Direction::West);
}

#[test]
#[ignore]
fn moving_east_and_north_from_readme() {
    let robot = Robot::new(7, 3, Direction::North);
    let robot = robot.instructions("RAALAL");
    assert_eq!(robot.position(), (9, 4));
    assert_eq!(robot.direction(), &Direction::West);
}

#[test]
#[ignore]
fn moving_west_and_north() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.instructions("LAAARALA");
    assert_eq!(robot.position(), (-4, 1));
    assert_eq!(robot.direction(), &Direction::West);
}

#[test]
#[ignore]
fn moving_west_and_south() {
    let robot = Robot::new(2, -7, Direction::East);
    let robot = robot.instructions("RRAAAAALA");
    assert_eq!(robot.position(), (-3, -8));
    assert_eq!(robot.direction(), &Direction::South);
}

#[test]
#[ignore]
fn moving_east_and_north() {
    let robot = Robot::new(8, 4, Direction::South);
    let robot = robot.instructions("LAAARRRALLLL");
    assert_eq!(robot.position(), (11, 5));
    assert_eq!(robot.direction(), &Direction::North);
}
