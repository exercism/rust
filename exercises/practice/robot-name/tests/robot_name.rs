use std::collections::HashSet;

use rand::SeedableRng as _;
use rand::rngs::SmallRng;
use robot_name::*;

fn deterministic_rng() -> SmallRng {
    SmallRng::seed_from_u64(0)
}

fn assert_name_matches_pattern(n: &str) {
    assert!(n.len() == 5, "name is exactly 5 characters long");
    assert!(
        n[0..2].chars().all(|c| c.is_ascii_uppercase()),
        "name starts with 2 uppercase letters"
    );
    assert!(
        n[2..].chars().all(|c| c.is_ascii_digit()),
        "name ends with 3 numbers"
    );
}

#[test]
fn name_should_match_expected_pattern() {
    let mut rng = deterministic_rng();
    let mut factory = RobotFactory::new();
    let robot = factory.new_robot(&mut rng);
    assert_name_matches_pattern(robot.name());
}

#[test]
#[ignore]
fn factory_prevents_name_collisions() {
    let mut factory = RobotFactory::new();
    let robot_1 = factory.new_robot(&mut deterministic_rng());
    let robot_2 = factory.new_robot(&mut deterministic_rng());
    assert_ne!(robot_1.name(), robot_2.name());
}

#[test]
#[ignore]
fn robot_name_depends_on_rng() {
    let mut rng = deterministic_rng();
    let robot_1 = RobotFactory::new().new_robot(&mut rng);
    let robot_2 = RobotFactory::new().new_robot(&mut rng);
    assert_ne!(robot_1.name(), robot_2.name());
}

#[test]
#[ignore]
fn robot_name_only_depends_on_rng() {
    let robot_1 = RobotFactory::new().new_robot(&mut deterministic_rng());
    let robot_2 = RobotFactory::new().new_robot(&mut deterministic_rng());
    assert_eq!(robot_1.name(), robot_2.name());
}

#[test]
#[ignore]
fn many_different_robots_have_different_names() {
    // In 3,529 random robot names, there is ~99.99% chance of a name collision
    let mut rng = deterministic_rng();
    let mut factory = RobotFactory::new();
    let robots: Vec<_> = (0..3529).map(|_| factory.new_robot(&mut rng)).collect();
    let mut set = HashSet::new();
    assert!(robots.iter().all(|robot| set.insert(robot.name())));
}

#[test]
#[ignore]
fn new_name_should_match_expected_pattern() {
    let mut rng = deterministic_rng();
    let mut factory = RobotFactory::new();
    let mut robot = factory.new_robot(&mut rng);
    assert_name_matches_pattern(robot.name());
    robot.reset(&mut rng);
    assert_name_matches_pattern(robot.name());
}

#[test]
#[ignore]
fn new_name_is_different_from_old_name() {
    let mut rng = deterministic_rng();
    let mut factory = RobotFactory::new();
    let mut robot = factory.new_robot(&mut rng);
    let name_1 = robot.name().to_string();
    robot.reset(&mut rng);
    let name_2 = robot.name().to_string();
    assert_ne!(name_1, name_2, "Robot name should change when reset");
}

#[test]
#[ignore]
fn factory_prevents_name_collision_despite_reset() {
    // To keep the same probablity as the first test with many robots, we
    // generate 3,529 robots and reset their names, then generate another 3,529
    // robots and check if there are collisions across these two groups.
    let mut rng = deterministic_rng();
    let mut factory = RobotFactory::new();
    let mut reset_robots: Vec<_> = (0..3529).map(|_| factory.new_robot(&mut rng)).collect();
    for robot in &mut reset_robots {
        robot.reset(&mut rng);
    }
    let mut set = HashSet::new();
    assert!(reset_robots.iter().all(|robot| set.insert(robot.name())));

    assert!((0..3529).all(|_| !set.contains(factory.new_robot(&mut rng).name())));
}
