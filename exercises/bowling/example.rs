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
        self.rolls.iter().fold(0, |acc, r| acc + r)
    }

    fn bonus_score(&self) -> u16 {
        self.bonus.iter().fold(0, |acc, r| acc + r)
    }

    fn is_complete(&self) -> bool {
        if self.is_spare() {
            self.bonus.len() == 1
        } else if self.is_strike() {
            self.bonus.len() == 2
        } else {
            self.rolls_done()
        }
    }

    fn rolls_done(&self) -> bool {
        self.is_spare() || self.is_strike() || self.rolls.len() == 2
    }

    fn is_spare(&self) -> bool {
        self.rolls.len() == 2 && self.roll_score() == 10
    }

    fn is_strike(&self) -> bool {
        self.rolls.len() == 1 && self.roll_score() == 10
    }

    fn add_roll(&mut self, roll: u16) {
        if self.is_spare() {
            if self.bonus.len() < 1 {
                self.bonus.push(roll)
            }
        } else if self.is_strike() {
            if self.bonus.len() < 2 {
                self.bonus.push(roll)
            }
        } else {
            if !self.rolls_done() {
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
            Err("Greate than 10")
        } else {
            if self.score().is_ok() {
                return Err("Game Finished. No more rolls.");
            }

            for mut frame in self.frames.iter_mut() {
                frame.add_roll(pins)
            }

            if self.frames.iter().last().unwrap().roll_score() > 10 {
                return Err("Invalid Roll");
            }

            if self.frames.iter().last().unwrap().bonus_score() > 10 {
                return Err("Invalid Roll");
            }

            if self.frames.iter().last().unwrap().rolls_done() && self.frames.len() < 10 {
                self.frames.push(Frame::new());
            }

            Ok(())
        }
    }

    pub fn score(&self) -> Result<u16, &'static str> {
        if self.frames.len() != 10 || self.frames.iter().any(|f| !f.is_complete()) {
            Err("Game Incomplete")
        } else {
            Ok(self.frames.iter().fold(0, |acc, r| acc + r.score()))
        }
    }
}
