#[derive(Debug, PartialEq)]
pub struct Queen { position: ChessPosition }

impl Queen {
    pub fn new(position: (i8, i8)) -> Result<Queen, ()> {
        let position = ChessPosition::new(position);
        if position.valid() {
            Ok(Queen { position: position })
        } else {
            Err(())
        }
    }

    pub fn can_attack(&self, piece: Queen) -> bool {
        self.horizontal_from(&piece) ||
            self.vertical_from(&piece) ||
            self.diagonal_from(&piece)
    }

    fn horizontal_from(&self, piece: &Queen) -> bool {
        self.rank() == piece.rank()
    }

    fn vertical_from(&self, piece: &Queen) -> bool {
        self.file() == piece.file()
    }

    fn diagonal_from(&self, piece: &Queen) -> bool {
        self.sum() == piece.sum() ||
            self.difference() == piece.difference()
    }

    fn rank(&self) -> i8 {
        self.position.rank()
    }

    fn file(&self) -> i8 {
        self.position.file()
    }

    fn sum(&self) -> i8 {
        self.position.sum()
    }

    fn difference(&self) -> i8 {
        self.position.difference()
    }
}

#[derive(Debug, PartialEq)]
struct ChessPosition { coordinates: (i8, i8) }

impl ChessPosition {
    fn new(coordinates: (i8, i8)) -> ChessPosition {
        ChessPosition {coordinates: coordinates}
    }

    fn valid(&self) -> bool {
        (self.rank() >= 0 && self.rank() <= 7) &&
        (self.file() >= 0 && self.file() <= 7)
    }

    fn rank(&self) -> i8 {
        self.coordinates.0
    }

    fn file(&self) -> i8 {
        self.coordinates.1
    }

    fn sum(&self) -> i8 {
        self.rank() + self.file()
    }

    fn difference(&self) -> i8 {
        self.rank() - self.file()
    }
}
