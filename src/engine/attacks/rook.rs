use super::generated::rook::{ROOK_BASE, ROOK_LOOKUP, ROOK_MASKS};
use super::utility::pext;

#[inline(always)]
pub fn get_rook_mv(sq: usize, own: u64, enemy: u64) -> u64 {
    let occupancy = own | enemy;
    let key = pext(occupancy, ROOK_MASKS[sq]) as usize;

    ROOK_LOOKUP[ROOK_BASE[sq] * 1024 + key] & !own
}

#[cfg(test)]
mod tests {}
