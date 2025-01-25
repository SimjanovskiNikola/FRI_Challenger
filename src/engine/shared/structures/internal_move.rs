use super::castling_struct::*;
use super::piece::*;
use super::color::*;

// Check about BigPawn Flag and what it does
// DEPRECATE:
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flag {
    Normal = 1,
    Capture = 2,
    EP = 4,
    Promotion = 8,
    KingSideCastle = 16,
    QueenSideCastle = 32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InternalMove {
    pub position_key: u64,
    pub active_color: Color,
    pub from: usize,
    pub to: usize,
    pub piece: Piece,
    pub captured: Option<Piece>,
    pub promotion: Option<Piece>,
    pub ep: Option<u64>,
    pub castle: CastlingRights,
    pub flag: Flag,
    pub half_move: usize,
    //TODO: Add Score
}

impl InternalMove {}

// NOTE: If Performance is better without enum and struct
// Change InternalMove from struct to u128 where you will store all relevant information as one integer
// 64 bits -> position key (IF NECESSARY)
// 8 bits -> color and piece
// 7 bits -> from position
// 7 bits -> to position
// 8 bits -> color and piece captured
// 4 bits -> promotion piece (knight, bishop, rook, queen)
// 4 bits -> Castling Rights (wKingSIde, wQueenSide, bKingSIde, bQueenSide)
// 7 bits -> ep position
// other bits -> maybe for score

// TODO: Generate position key every time a move is created
