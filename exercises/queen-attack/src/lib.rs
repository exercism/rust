pub struct ChessPosition;

pub struct Queen;

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Result<Self, i32> {
        unimplemented!("Construct a ChessPosition struct, given the following rank, file: ({}, {}). If the position is invalid return error that contains wrong rank/file.", rank, file);
    }
}

impl Queen {
    pub fn new(_position: ChessPosition) -> Self {
        unimplemented!("Given the chess position, construct a Queen struct.");
    }

    pub fn can_attack(&self, _other: &Queen) -> bool {
        unimplemented!("Determine if this Queen can attack the other Queen");
    }
}
