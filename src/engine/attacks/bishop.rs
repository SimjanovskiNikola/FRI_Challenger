use super::generated::bishop::{BISHOP_BASE, BISHOP_LOOKUP, BISHOP_MASKS};
use super::utility::pext;

pub fn get_bishop_mv(sq: usize, own: u64, enemy: u64) -> u64 {
    let occupancy = own | enemy;
    let key = pext(occupancy, BISHOP_MASKS[sq]) as usize;

    BISHOP_LOOKUP[BISHOP_BASE[sq] * 32 + key] & !own
}

#[cfg(test)]
mod tests {}
