#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    throws: Vec<u16>,
    second_throw_in_frame: bool
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            throws: Vec::new(),
            second_throw_in_frame: false
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.second_throw_in_frame && (pins + self.throws.last().unwrap()) > 10 ){
            return Result::Err(Error::NotEnoughPinsLeft);
        }
        if self.score().is_some() {
            Result::Err(Error::GameComplete)
        }
        else {
            self.throws.push(pins);
            self.second_throw_in_frame = if pins == 10 {false} else {!self.second_throw_in_frame};
            Result::Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut total: u16 = 0;
        let mut frame = 0;

        let throws = &self.throws;

        for _ in 0..10 {
            if let (Some(&first), Some(&second)) = (throws.get(frame), throws.get(frame+1)) {
                total += first + second;

                if first == 10 || first + second == 10 {
                    if let Some(&third) = throws.get(frame+2) {
                        total += third;
                    }
                    else {
                        return None;
                    }
                }
                frame += if first == 10 {1} else {2};
            }
            else {
                return None;
            }
        }
        Some(total)
    }
}
