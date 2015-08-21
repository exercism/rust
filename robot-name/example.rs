extern crate rand;
use rand::{thread_rng, Rng};

pub struct Robot {
    name: String
}

fn generate_name() -> String {
    let mut s = String::with_capacity(5);
    static LETTERS: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    static NUMBERS: &'static [u8] = b"0123456789";
    for _ in 0..2 {
        s.push(thread_rng().choose(LETTERS).unwrap().clone() as char);
    }
    for _ in 0..3 {
        s.push(thread_rng().choose(NUMBERS).unwrap().clone() as char);
    }
    s
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: generate_name() }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name[..]
    }

    pub fn reset_name(&mut self) {
        self.name = generate_name();
    }

}
