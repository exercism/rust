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
        let mut names = ROBOT_NAMES.lock().unwrap();
        let mut name = generate_robot_name();
        
        while names.contains(&name)  {
            name = generate_robot_name();
        }

        names.insert(name.clone());

        Self { 
            name 
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let mut names = ROBOT_NAMES.lock().unwrap();
        let mut name = generate_robot_name();

        while names.contains(&name) {
            name = generate_robot_name();
        }

        names.remove(&name);

        self.name = name;
    }
}
