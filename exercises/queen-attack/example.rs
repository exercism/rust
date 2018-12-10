#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

pub trait ChessPiece {
    fn position(&self) -> &ChessPosition;
    fn can_attack<T: ChessPiece>(&self, other: &T) -> bool;
}

impl ChessPiece for Queen {
    fn position(&self) -> &ChessPosition {
        &self.position
    }

    fn can_attack<T: ChessPiece>(&self, piece: &T) -> bool {
        self.position.horizontal_from(&piece.position())
            || self.position.vertical_from(&piece.position())
            || self.position.diagonal_from(&piece.position())
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Queen {
        Queen { position }
    }
}

#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i8,
    pub file: i8,
}

impl ChessPosition {
    pub fn new(rank: i8, file: i8) -> Option<Self> {
        let position = ChessPosition { rank, file };

        if position.is_valid() {
            Some(position)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        self.rank >= 0 && self.rank <= 7 && self.file >= 0 && self.file <= 7
    }

    fn horizontal_from(&self, other: &ChessPosition) -> bool {
        self.rank == other.rank
    }

    fn vertical_from(&self, other: &ChessPosition) -> bool {
        self.file == other.file
    }

    fn diagonal_from(&self, other: &ChessPosition) -> bool {
        self.sum() == other.sum() || self.difference() == other.difference()
    }

    fn sum(&self) -> i8 {
        self.rank + self.file
    }

    fn difference(&self) -> i8 {
        self.rank - self.file
    }
}
