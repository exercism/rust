#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn previous_clockwise(&self) -> Self {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn next_clockwise(&self) -> Self {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn advance(&self, direction: &Direction) -> Self {
        match *direction {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Clone)]
pub struct Robot {
    position: Position,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot::build(Position::new(x, y), d)
    }

    fn build(position: Position, direction: Direction) -> Self {
        Robot {
            position,
            direction,
        }
    }

    #[must_use]
    pub fn turn_right(&self) -> Self {
        Self::build(self.position, self.direction.next_clockwise())
    }

    #[must_use]
    pub fn turn_left(&self) -> Self {
        Self::build(self.position, self.direction.previous_clockwise())
    }

    #[must_use]
    pub fn advance(&self) -> Self {
        Self::build(self.position.advance(&self.direction), self.direction)
    }

    #[must_use]
    pub fn instructions(&self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self.clone(), |robot, instruction| {
                robot.execute(instruction)
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    #[must_use]
    fn execute(self, command: char) -> Self {
        match command {
            'R' => self.turn_right(),
            'L' => self.turn_left(),
            'A' => self.advance(),
            _ => self,
        }
    }
}
