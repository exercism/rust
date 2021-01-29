#[derive(Debug)]
pub struct ChessPosition;

#[derive(Debug)]
pub struct Queen;

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        unimplemented!(
            "Construct a ChessPosition struct, given the following rank, file: ({}, {}). If the position is invalid return None.",
            rank,
            file
        );
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        unimplemented!(
            "Given the chess position {:?}, construct a Queen struct.",
            position
        );
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        unimplemented!(
            "Determine if this Queen can attack the other Queen {:?}",
            other
        );
    }
}
