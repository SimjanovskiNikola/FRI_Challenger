use super::generated::bishop::{BISHOP_BASE, BISHOP_LOOKUP, BISHOP_MASKS};
use super::utility::pext;

const WHITE_SQUARES: u64 = 0b1010101010101010101010101010101010101010101010101010101010101010;
const BLACK_SQUARES: u64 = 0b0101010101010101010101010101010101010101010101010101010101010101;

#[inline(always)]
pub fn get_bishop_mv(sq: usize, own: u64, enemy: u64) -> u64 {
    let occupancy = own | enemy;
    let key = pext(occupancy, BISHOP_MASKS[sq]) as usize;

    BISHOP_LOOKUP[BISHOP_BASE[sq] * 32 + key] & !own
}

/// Returns `true` if there is at least one bishop on a white square
/// and at least one bishop on a black square.
#[inline(always)]
pub fn has_bishop_pair(bb: u64) -> bool {
    bb & WHITE_SQUARES != 0 && bb & BLACK_SQUARES != 0
}

#[cfg(test)]
mod tests {}
