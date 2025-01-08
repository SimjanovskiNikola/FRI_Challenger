use super::piece_struct::{Piece, Color};

// Check about BigPawn Flag and what it does
// DEPRECATE:
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flags {
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
    //TODO: Add Score
    //TODO: Add Pawn Start
}

pub fn generate_unique_key() {}

// TODO: Generate position key every time a move is created
