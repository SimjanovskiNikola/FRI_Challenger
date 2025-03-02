use super::castling_struct::*;
use super::color::*;
use super::piece::*;

// Check about BigPawn Flag and what it does
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flag {
    Quiet,
    Capture(Piece),
    EP(usize, Piece),
    Promotion(Piece, Option<Piece>),
    KingSideCastle(usize, usize),
    QueenSideCastle(usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InternalMove {
    pub position_key: u64,
    pub active_color: Color,
    pub from: usize,
    pub to: usize,
    pub piece: Piece,
    pub ep: Option<usize>,
    pub castle: CastlingRights,
    pub flag: Flag,
    pub half_move: usize,
    //TODO: Add Score
}

impl InternalMove {
    pub fn init() -> Self {
        Self {
            position_key: 0u64,
            active_color: WHITE,
            from: 0,
            to: 0,
            piece: 0,
            ep: None,
            castle: CastlingRights::NONE,
            flag: Flag::Quiet,
            half_move: 0,
        }
    }
}
