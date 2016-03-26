#[derive(Debug, PartialEq)]
pub struct Queen {
    position: (i8, i8)
}

#[derive(Debug, PartialEq)]
pub struct InvalidQueen;

impl Queen {
    pub fn new(position: (i8, i8)) -> Result<Queen, InvalidQueen> {
        if (position.0 < 0 || position.0 > 7) || (position.1 < 0 || position.1 >7 ) {
            Err(InvalidQueen)
        } else {
            Ok(Queen {position: position})
        }
    }
}

pub struct Queens {
    first: Queen,
    second: Queen
}

impl Queens {
    pub fn new(first: Result<Queen, InvalidQueen>, second: Result<Queen, InvalidQueen>) -> Queens {
        Queens {
            first: first.unwrap(),
            second: second.unwrap()
        }
    }

    pub fn can_attack(&self) -> bool {
        self.first.position.0 == self.second.position.0 ||
            self.first.position.1 == self.second.position.1 ||
            (
                (self.first.position.0 + self.first.position.1)
                ==
                (self.second.position.0 + self.second.position.1)
            )
            ||
            (
                (self.first.position.0 - self.first.position.1)
                ==
                (self.second.position.0 - self.second.position.1)
            )

    }
}
