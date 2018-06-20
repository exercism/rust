// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot;

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        unimplemented!("Create a robot at (x, y) ({}, {}) facing {:?}", x, y, d,)
    }

    pub fn turn_right(self) -> Self {
        unimplemented!()
    }

    pub fn turn_left(self) -> Self {
        unimplemented!()
    }

    pub fn advance(self) -> Self {
        unimplemented!()
    }

    pub fn instructions(self, instructions: &str) -> Self {
        unimplemented!(
            "Follow the given sequence of instructions: {}",
            instructions
        )
    }

    pub fn position(&self) -> (isize, isize) {
        unimplemented!()
    }

    pub fn direction(&self) -> &Direction {
        unimplemented!()
    }
}
