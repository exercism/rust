// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot;

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        todo!("Create a robot at (x, y) ({x}, {y}) facing {d:?}")
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        todo!()
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        todo!()
    }

    #[must_use]
    pub fn advance(self) -> Self {
        todo!()
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        todo!("Follow the given sequence of instructions: {instructions}")
    }

    pub fn position(&self) -> (i32, i32) {
        todo!()
    }

    pub fn direction(&self) -> &Direction {
        todo!()
    }
}
