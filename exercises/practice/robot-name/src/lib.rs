use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::sync::Mutex;

pub struct Robot {
    name: String
}

static ROBOT_NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {
    Mutex::new(HashSet::new())
});

fn generate_robot_name() -> String {
    let mut rng = thread_rng();

    let letters: String = (0..2)
    .map(|_| rng.gen_range(b'A'..=b'Z') as char)
    .collect();

    let digits: String = (0..3)
    .map(|_| rng.gen_range(0..=9).to_string())
    .collect();

    format!("{letters}{digits}")
}

impl Robot {
    pub fn new() -> Self {
        loop {
            let name = generate_robot_name();
            if ROBOT_NAMES.lock().unwrap().insert(name.clone()) {
                return Self {name}
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        // The generation of the new name, the removing of the old name and the 
        // affectation of the new one are perform during the lock 
        let mut names = ROBOT_NAMES.lock().unwrap();

        loop {
            let name = generate_robot_name();
            if names.insert(name.clone()) {
                names.remove(self.name());
                self.name = name;
                break;
            }
        }
    }
}
