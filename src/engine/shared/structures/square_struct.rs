use super::piece_struct::Piece;

/** Determines if the square is empty or occupied. */
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Square {
    Empty,
    Occupied(Piece),
}
