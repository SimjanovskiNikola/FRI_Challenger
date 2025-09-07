use super::utility::pext;
use crate::engine::board::color::Color;
use crate::engine::board::square::get_file;
use crate::engine::generated::rook::{ROOK_BASE, ROOK_LOOKUP, ROOK_MASKS};
use crate::engine::misc::const_utility::FILE_BITBOARD;

#[inline(always)]
pub fn get_rook_mv(sq: usize, own: u64, enemy: u64, _clr: Color) -> u64 {
    get_rook_mask(sq, own, enemy, _clr) & !own
}

#[inline(always)]
pub fn get_rook_mask(sq: usize, own: u64, enemy: u64, _: Color) -> u64 {
    let occupancy = own | enemy;
    let key = pext(occupancy, ROOK_MASKS[sq]) as usize;

    ROOK_LOOKUP[ROOK_BASE[sq] * 1024 + key]
}

// TODO: TEST ME
#[inline(always)]
pub fn is_rook_on_semi_open_file(sq: usize, own_pawns: u64) -> bool {
    FILE_BITBOARD[get_file(sq)] & (own_pawns) != 0
}

// TODO: TEST ME
#[inline(always)]
pub fn is_rook_on_open_file(sq: usize, own_pawns: u64, enemy_pawns: u64) -> bool {
    FILE_BITBOARD[get_file(sq)] & (own_pawns | enemy_pawns) != 0
}

#[cfg(test)]
mod tests {}
