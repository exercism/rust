// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { 
            x, 
            y, 
            direction: d
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            direction: match self.direction {
                Direction::East => Direction::South,
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North
            }
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self { 
            x: self.x, 
            y: self.y, 
            direction: match self.direction {
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::South
            }
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (newx, newy) = match self.direction {
            Direction::East => (self.x + 1, self.y),
            Direction::North => (self.x, self.y + 1),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y)
        };
        Self { 
            x: newx, 
            y: newy, 
            direction: self.direction 
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
        .chars()
        .fold(self, |robot, instruction| {
            match instruction {
                'A' => robot.advance(),
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                _ => robot
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
