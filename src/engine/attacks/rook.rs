use std::array;
use crate::engine::shared::helper_func::const_utility::*;
use crate::engine::shared::structures::directions::*;
use lazy_static::lazy_static;

use super::rays::{blocked_ray_att, RAYS};
use super::utility::{insert_bits, pext};

lazy_static! {
    #[rustfmt::skip]
    pub static ref ROOK_BASE: [usize; 64] = [
        0, 4, 6, 8, 10, 12, 14, 16,
        20, 22, 23, 24, 25, 26, 27, 28,
        30, 32, 33, 34, 35, 36, 37, 38,
        40, 42, 43, 44, 45, 46, 47, 48,
        50, 52, 53, 54, 55, 56, 57, 58,
        60, 62, 63, 64, 65, 66, 67, 68,
        70, 72, 73, 74, 75, 76, 77, 78,
        80, 84, 86, 88, 90, 92, 94, 96,
    ];
    pub static ref ROOK_MASKS: [u64; 64] = array::from_fn(|sq| gen_rook_mask(sq));
    pub static ref ROOK_LOOKUP: [u64; 102400] = gen_lookup_rook();
}

pub fn gen_lookup_rook() -> [u64; 102400] {
    let mut lookup_table = [0u64; 102400];
    for sq in 0..64 {
        for occ in 0..((2 as u64).pow(ROOK_MASKS[sq].count_ones())) {
            let extracted = insert_bits(ROOK_MASKS[sq], occ as u64);
            let mut moves = 0u64;

            for dir in DIRECTIONS {
                if dir.is_orthogonal() {
                    let ray = RAYS[dir.idx()][sq];
                    moves |=
                        blocked_ray_att(DIRECTIONS[dir.idx()], &RAYS[dir.idx()], ray, 0, extracted);
                }
            }
            lookup_table[ROOK_BASE[sq] * 1024 + (occ as usize)] = moves;
        }
    }

    return lookup_table;
}

pub fn gen_rook_mask(pos: usize) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= RAYS[Dir::NORTH.idx()][pos] & !RANK_BITBOARD[7];
    attacks |= RAYS[Dir::SOUTH.idx()][pos] & !RANK_BITBOARD[0];
    attacks |= RAYS[Dir::EAST.idx()][pos] & !FILE_BITBOARD[7];
    attacks |= RAYS[Dir::WEST.idx()][pos] & !FILE_BITBOARD[0];

    return attacks;
}

pub fn get_rook_mv(sq: usize, own: u64, enemy: u64) -> u64 {
    let occupancy = own | enemy;
    let key = pext(occupancy, ROOK_MASKS[sq]) as usize;

    return ROOK_LOOKUP[ROOK_BASE[sq] * 1024 + key] & !own;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_king_knight_attacks_init1() {}
}
