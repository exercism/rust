use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use rand::Rng;

fn generate_name<R: Rng>(rng: &mut R, used_names: &mut HashSet<String>) -> String {
    loop {
        let mut s = String::with_capacity(5);
        for _ in 0..2 {
            s.push(rng.random_range('A'..='Z'));
        }
        for _ in 0..3 {
            s.push(rng.random_range('0'..='9'));
        }
        if used_names.insert(s.clone()) {
            return s;
        }
    }
}

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    used_names: Arc<Mutex<HashSet<String>>>,
}

pub struct Robot {
    used_names: Arc<Mutex<HashSet<String>>>,
    name: String,
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            used_names: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        let mut guard = self.used_names.lock().unwrap();
        Robot {
            used_names: Arc::clone(&self.used_names),
            name: generate_name(rng, &mut guard),
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    // When a robot is reset, its factory must still ensure that there are no
    // name conflicts with other robots from the same factory.
    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        let mut used_names = self.used_names.lock().unwrap();
        let new_name = generate_name(rng, &mut used_names);
        used_names.remove(&self.name);
        self.name = new_name
    }
}
