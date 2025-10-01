use super::utility::pext;
use crate::engine::board::color::Color;
use crate::engine::generated::bishop::{BISHOP_BASE, BISHOP_LOOKUP, BISHOP_MASKS};

pub const WHITE_SQUARES: u64 = 0x55AA55AA55AA55AA;
pub const BLACK_SQUARES: u64 = 0xAA55AA55AA55AA55;

#[inline(always)]
/// Gets Bishop moves considering other pieces on the board and excluding own pieces
pub fn get_bishop_mv(sq: usize, own: u64, enemy: u64, _clr: Color) -> u64 {
    get_bishop_mask(sq, own, enemy, _clr) & !own
}

#[inline(always)]
/// Gets Bishop moves considering other pieces on the board
pub fn get_bishop_mask(sq: usize, own: u64, enemy: u64, _: Color) -> u64 {
    let key = pext(own | enemy, BISHOP_MASKS[sq]) as usize;

    BISHOP_LOOKUP[BISHOP_BASE[sq] * 32 + key]
}

#[inline(always)]
/// Gets only the mask of possible moves, ignoring other pieces on the board
pub const fn get_bishop_lookup(sq: usize, _: u64, _: u64, _: Color) -> u64 {
    BISHOP_MASKS[sq]
}

/// Returns `true` if there is at least one bishop on a
/// white square and at least one bishop on a black square.
#[inline(always)]
pub const fn has_bishop_pair(bb: u64) -> bool {
    (bb & WHITE_SQUARES != 0) && (bb & BLACK_SQUARES != 0)
}

#[cfg(test)]
mod tests {}
