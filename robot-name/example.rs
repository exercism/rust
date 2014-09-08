use std::rand::{task_rng, Rng};

pub struct Robot {
    name: String
}

fn generate_name() -> String {
    let mut rng = task_rng();
    let mut s = String::with_capacity(6);
    static LETTERS: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    static NUMBERS: &'static [u8] = b"0123456789";
    for _ in range(0u, 3) {
        s.push_char(rng.choose(LETTERS).unwrap().clone() as char);
    }
    for _ in range(0u, 3) {
        s.push_char(rng.choose(NUMBERS).unwrap().clone() as char);
    }
    s
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: generate_name() }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        self.name.as_slice()
    }

    pub fn reset_name(&mut self) {
        self.name = generate_name();
    }

}
