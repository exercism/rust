
pub struct BowlingGame {
    frames: Vec<Frame>,
}

struct Frame {
    rolls: Vec<u16>,
    bonus: Vec<u16>,
}

impl Frame {
    fn score(&self) -> u16 {
        self.roll_score() + self.bonus_score()
    }

    fn roll_score(&self) -> u16 {
        self.rolls.iter().sum()
    }

    fn bonus_score(&self) -> u16 {
        self.bonus.iter().sum()
    }

    fn is_valid(&self) -> bool {
        self.rolls_valid() && self.bonus_valid()
    }

    fn rolls_valid(&self) -> bool {
        self.roll_score() <= 10
    }

    fn bonus_valid(&self) -> bool {
        if self.is_open() || !self.bonus_done() {
            return true;
        }

        if self.is_spare() {
            return self.bonus_score() <= 10;
        }

        if let Some(first) = self.bonus.iter().next() {
            if *first == 10 {
                self.bonus_score() <= 20
            } else {
                self.bonus_score() <= 10
            }
        } else {
            unreachable!();
        }
    }

    fn is_complete(&self) -> bool {
        self.is_open() || self.bonus_done()
    }

    fn rolls_done(&self) -> bool {
        self.rolls.len() == 2 || self.is_strike()
    }

    fn bonus_done(&self) -> bool {
        (self.is_spare() && self.bonus.len() == 1) || (self.is_strike() && self.bonus.len() == 2)
    }

    fn is_open(&self) -> bool {
        self.rolls.len() == 2 && self.roll_score() < 10
    }

    fn is_spare(&self) -> bool {
        self.rolls.len() == 2 && self.roll_score() == 10
    }

    fn is_strike(&self) -> bool {
        self.rolls.len() == 1 && self.roll_score() == 10
    }

    fn add_roll(&mut self, roll: u16) {
        if !self.is_complete() {
            if self.is_spare() || self.is_strike() {
                self.bonus.push(roll)
            } else {
                self.rolls.push(roll)
            }
        }
    }

    fn new() -> Self {
        Frame {
            rolls: vec![],
            bonus: vec![],
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: vec![Frame::new()] }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), &'static str> {
        if pins > 10 {
            Err("Greater than 10 pins")
        } else {
            if self.score().is_ok() {
                return Err("Game Finished. No more rolls.");
            }

            for frame in self.frames.iter_mut() {
                frame.add_roll(pins)
            }

            if self.frames.iter().any(|f| !f.is_valid()) {
                return Err("Invalid Roll");
            }

            if self.frames.iter().last().unwrap().rolls_done() && self.frames.len() < 10 {
                self.frames.push(Frame::new());
            }

            Ok(())
        }
    }

    pub fn score(&self) -> Result<u16, &'static str> {
        if !self.is_done() {
            Err("Game Incomplete")
        } else {
            Ok(self.frames.iter().fold(0, |acc, r| acc + r.score()))
        }
    }

    fn is_done(&self) -> bool {
        self.frames.len() == 10 && self.frames.iter().all(|f| f.is_complete())
    }
}
