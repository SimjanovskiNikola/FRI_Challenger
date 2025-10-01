use crate::engine::board::castling::{CASTLING_NONE, Castling};

// use super::castling::CastlingRights;
use super::color::{Color, WHITE};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BoardState {
    pub key: u64,
    pub pk_key: u64, // Pawn King Key
    pub color: Color,
    pub castling: Castling,
    pub ep: Option<u8>,
    pub half_move: u8,
    pub full_move: u16,
    pub phase: isize,
}

impl BoardState {
    pub const fn init() -> Self {
        Self {
            color: WHITE,
            castling: CASTLING_NONE,
            ep: None,
            half_move: 0,
            full_move: 1,
            key: 0,
            pk_key: 0,
            phase: 0,
        }
    }
}
