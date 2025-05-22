use crate::engine::board::structures::color::Color;
use crate::engine::board::structures::square::get_file;
use crate::engine::misc::const_utility::FILE_BITBOARD;

use super::generated::rook::{ROOK_BASE, ROOK_LOOKUP, ROOK_MASKS};
use super::utility::pext;

#[inline(always)]
pub fn get_rook_mv(sq: usize, own: u64, enemy: u64, _: Color) -> u64 {
    let occupancy = own | enemy;
    let key = pext(occupancy, ROOK_MASKS[sq]) as usize;

    ROOK_LOOKUP[ROOK_BASE[sq] * 1024 + key] & !own
}

// TODO: TEST ME
#[inline(always)]
pub fn is_rook_on_open_file(sq: usize, own_pawns: u64) -> bool {
    FILE_BITBOARD[get_file(sq)] & (own_pawns) != 0
}

// TODO: TEST ME
#[inline(always)]
pub fn is_rook_on_semi_open_file(sq: usize, own_pawns: u64, enemy_pawns: u64) -> bool {
    FILE_BITBOARD[get_file(sq)] & (own_pawns | enemy_pawns) != 0
}

#[cfg(test)]
mod tests {}
