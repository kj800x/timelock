pub type Hash = [u8; 32];
pub type Count = u64;
pub type Chain = (Hash, Count, Hash);
pub type Work = Vec<Chain>;

pub type PuzzlePiece = (Hash, Count);
pub type Puzzle = Vec<PuzzlePiece>;
