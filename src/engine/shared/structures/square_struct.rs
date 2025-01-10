use super::piece_struct::Piece;

/** Determines if the square is empty or occupied. */
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Square {
    Empty,
    Occupied(Piece),
}

// NOTE: If Performance is better without enum and struct
// Remove Square and instead use u8 (6 bits for the peaces and 2 for black or white)