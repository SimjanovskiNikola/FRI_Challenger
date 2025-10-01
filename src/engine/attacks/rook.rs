use super::utility::pext;
use crate::engine::board::color::Color;
use crate::engine::generated::rook::{ROOK_BASE, ROOK_LOOKUP, ROOK_MASKS};

#[inline(always)]
/// Gets Queen moves considering other pieces on the board and excluding own pieces
pub fn get_rook_mv(sq: usize, own: u64, enemy: u64, _clr: Color) -> u64 {
    get_rook_mask(sq, own, enemy, _clr) & !own
}

#[inline(always)]
/// Gets Queen moves considering other pieces on the board
pub fn get_rook_mask(sq: usize, own: u64, enemy: u64, _: Color) -> u64 {
    let key = pext(own | enemy, ROOK_MASKS[sq]) as usize;

    ROOK_LOOKUP[ROOK_BASE[sq] * 1024 + key]
}

#[inline(always)]
/// Gets only the mask of possible moves, ignoring other pieces on the board
pub const fn get_rook_lookup(sq: usize, _: u64, _: u64, _: Color) -> u64 {
    ROOK_MASKS[sq]
}

#[cfg(test)]
mod tests {}
