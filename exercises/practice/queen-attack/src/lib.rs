use std::cmp::{max, min};

#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            return None;
        }
        Some(Self { 
            rank, 
            file})
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self {
            position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank {
            return true;
        }
        if self.position.file == other.position.file {
            return true;
        }

        // lower right
        for i in 1..=(7-max(self.position.rank, self.position.file)) {
            if other.position.rank == self.position.rank + i && other.position.file == self.position.file + i {
                return true;
            }
        }

        // upper left
        for i in 1..=min(self.position.rank, self.position.file) {
            if other.position.rank == self.position.rank - i && other.position.file == self.position.file - i {
                return true;
            }
        }

        // upper right
        for i in 1..=min(self.position.rank, 7 - self.position.file) {
            if other.position.rank == self.position.rank - i && other.position.file == self.position.file + i {
                return true;
            }
        }

        // lower left
        for i in 1..=min(7 - self.position.rank, self.position.file) {
            if other.position.rank == self.position.rank + i && other.position.file == self.position.file - i {
                return true;
            }
        }

        false
    }
}
