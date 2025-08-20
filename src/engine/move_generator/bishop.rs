use crate::engine::board::structures::color::Color;

use super::generated::bishop::{BISHOP_BASE, BISHOP_LOOKUP, BISHOP_MASKS};
use super::utility::pext;

pub const WHITE_SQUARES: u64 = 0b0101010110101010010101011010101001010101101010100101010110101010;
pub const BLACK_SQUARES: u64 = 0b1010101001010101101010100101010110101010010101011010101001010101;

#[inline(always)]
pub fn get_bishop_mv(sq: usize, own: u64, enemy: u64, _clr: Color) -> u64 {
    get_bishop_mask(sq, own, enemy, _clr) & !own
}

#[inline(always)]
pub fn get_bishop_mask(sq: usize, own: u64, enemy: u64, _: Color) -> u64 {
    let occupancy = own | enemy;
    let key = pext(occupancy, BISHOP_MASKS[sq]) as usize;

    BISHOP_LOOKUP[BISHOP_BASE[sq] * 32 + key]
}

/// Returns `true` if there is at least one bishop on a white square
/// and at least one bishop on a black square.
#[inline(always)]
pub fn has_bishop_pair(bb: u64) -> bool {
    (bb & WHITE_SQUARES != 0) && (bb & BLACK_SQUARES != 0)
}

#[cfg(test)]
mod tests {}
