#[derive(Debug, PartialEq)]
pub struct Queen { position: (i8, i8) }

impl Queen {
    pub fn new(position: (i8, i8)) -> Queen {
        Queen { position: position }
    }

    pub fn can_attack(&self, piece: Queen) -> Result<bool, ()> {
        if self.position_valid() && piece.position_valid() {
            Ok(self.rank() == piece.rank() ||
               self.file() == piece.file() ||
               self.sum() == piece.sum() ||
               self.difference() == piece.difference())
        } else {
            Err(())
        }
    }

    fn position_valid(&self) -> bool {
        (self.rank() >= 0 && self.rank() <= 7) &&
        (self.file() >= 0 && self.file() <= 7)
    }

    fn rank(&self) -> i8 {
        self.position.0
    }

    fn file(&self) -> i8 {
        self.position.1
    }

    fn sum(&self) -> i8 {
        self.rank() + self.file()
    }

    fn difference(&self) -> i8 {
        self.rank() - self.file()
    }
}
