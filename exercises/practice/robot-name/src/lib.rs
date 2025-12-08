use rand::Rng;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory;

pub struct Robot;

impl RobotFactory {
    pub fn new() -> Self {
        todo!("Create a new robot factory")
    }

    pub fn new_robot<R: Rng>(&mut self, _rng: &mut R) -> Robot {
        todo!("Create a new robot with a unique name")
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        todo!("Return a reference to the robot's name");
    }

    pub fn reset<R: Rng>(&mut self, _rng: &mut R) {
        todo!("Assign a new unique name to the robot");
    }
}
