use super::castling::CastlingRights;
use super::color::{Color, WHITE};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BoardState {
    pub key: u64,
    pub pc_key: u64,
    pub color: Color,
    pub castling: CastlingRights,
    pub ep: Option<u8>,
    pub half_move: u8,
    pub full_move: u16,
    pub phase: isize,
}

impl BoardState {
    pub fn init() -> Self {
        Self {
            color: WHITE,
            castling: CastlingRights::NONE,
            ep: None,
            half_move: 0,
            full_move: 1,
            key: 0,
            pc_key: 0,
            phase: 0,
        }
    }
}
