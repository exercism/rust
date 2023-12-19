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
    let robot_start = Robot::new(0, 0, Direction::North);
    let robot_end = robot_start.turn_right();
    assert_eq!(robot_end.position(), (0, 0));
    assert_eq!(robot_end.direction(), &Direction::East);
}

#[test]
#[ignore]
fn changes_east_to_south() {
    let robot_start = Robot::new(0, 0, Direction::East);
    let robot_end = robot_start.turn_right();
    assert_eq!(robot_end.position(), (0, 0));
    assert_eq!(robot_end.direction(), &Direction::South);
}

#[test]
#[ignore]
fn changes_south_to_west() {
    let robot_start = Robot::new(0, 0, Direction::South);
    let robot_end = robot_start.turn_right();
    assert_eq!(robot_end.position(), (0, 0));
    assert_eq!(robot_end.direction(), &Direction::West);
}

#[test]
#[ignore]
fn changes_west_to_north() {
    let robot_start = Robot::new(0, 0, Direction::West);
    let robot_end = robot_start.turn_right();
    assert_eq!(robot_end.position(), (0, 0));
    assert_eq!(robot_end.direction(), &Direction::North);
}

#[test]
#[ignore]
fn changes_north_to_west() {
    let robot_start = Robot::new(0, 0, Direction::North);
    let robot_end = robot_start.turn_left();
    assert_eq!(robot_end.position(), (0, 0));
    assert_eq!(robot_end.direction(), &Direction::West);
}

#[test]
#[ignore]
fn changes_west_to_south() {
    let robot_start = Robot::new(0, 0, Direction::West);
    let robot_end = robot_start.turn_left();
    assert_eq!(robot_end.position(), (0, 0));
    assert_eq!(robot_end.direction(), &Direction::South);
}

#[test]
#[ignore]
fn changes_south_to_east() {
    let robot_start = Robot::new(0, 0, Direction::South);
    let robot_end = robot_start.turn_left();
    assert_eq!(robot_end.position(), (0, 0));
    assert_eq!(robot_end.direction(), &Direction::East);
}

#[test]
#[ignore]
fn changes_east_to_north() {
    let robot_start = Robot::new(0, 0, Direction::East);
    let robot_end = robot_start.turn_left();
    assert_eq!(robot_end.position(), (0, 0));
    assert_eq!(robot_end.direction(), &Direction::North);
}

#[test]
#[ignore]
fn facing_north_increments_y() {
    let robot_start = Robot::new(0, 0, Direction::North);
    let robot_end = robot_start.advance();
    assert_eq!(robot_end.position(), (0, 1));
    assert_eq!(robot_end.direction(), &Direction::North);
}

#[test]
#[ignore]
fn facing_south_decrements_y() {
    let robot_start = Robot::new(0, 0, Direction::South);
    let robot_end = robot_start.advance();
    assert_eq!(robot_end.position(), (0, -1));
    assert_eq!(robot_end.direction(), &Direction::South);
}

#[test]
#[ignore]
fn facing_east_increments_x() {
    let robot_start = Robot::new(0, 0, Direction::East);
    let robot_end = robot_start.advance();
    assert_eq!(robot_end.position(), (1, 0));
    assert_eq!(robot_end.direction(), &Direction::East);
}

#[test]
#[ignore]
fn facing_west_decrements_x() {
    let robot_start = Robot::new(0, 0, Direction::West);
    let robot_end = robot_start.advance();
    assert_eq!(robot_end.position(), (-1, 0));
    assert_eq!(robot_end.direction(), &Direction::West);
}

#[test]
#[ignore]
fn moving_east_and_north_from_readme() {
    let robot_start = Robot::new(7, 3, Direction::North);
    let robot_end = robot_start.instructions("RAALAL");
    assert_eq!(robot_end.position(), (9, 4));
    assert_eq!(robot_end.direction(), &Direction::West);
}

#[test]
#[ignore]
fn moving_west_and_north() {
    let robot_start = Robot::new(0, 0, Direction::North);
    let robot_end = robot_start.instructions("LAAARALA");
    assert_eq!(robot_end.position(), (-4, 1));
    assert_eq!(robot_end.direction(), &Direction::West);
}

#[test]
#[ignore]
fn moving_west_and_south() {
    let robot_start = Robot::new(2, -7, Direction::East);
    let robot_end = robot_start.instructions("RRAAAAALA");
    assert_eq!(robot_end.position(), (-3, -8));
    assert_eq!(robot_end.direction(), &Direction::South);
}

#[test]
#[ignore]
fn moving_east_and_north() {
    let robot_start = Robot::new(8, 4, Direction::South);
    let robot_end = robot_start.instructions("LAAARRRALLLL");
    assert_eq!(robot_end.position(), (11, 5));
    assert_eq!(robot_end.direction(), &Direction::North);
}
