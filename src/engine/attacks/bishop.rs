use std::array;
use super::rays::RAYS;
use super::utility::{insert_bits, pext};
use super::rays::blocked_ray_att;

use crate::engine::shared::helper_func::const_utility::*;
use crate::engine::shared::structures::directions::*;
use lazy_static::lazy_static;

// NOTE: CONSTANTS
lazy_static! {
     #[rustfmt::skip]
    pub static ref BISHOP_BASE: [usize; 64] = [
        0, 2, 3, 4, 5, 6, 7, 8,
        10, 11, 12, 13, 14, 15, 16, 17,
        18, 19, 20, 24, 28, 32, 36, 37,
        38, 39, 40, 44, 60, 76, 80, 81,
        82, 83, 84, 88, 104, 120, 124, 125,
        126, 127, 128, 132, 136, 140, 144, 145,
        146, 147, 148, 149, 150, 151, 152, 153,
        154, 156, 157, 158, 159, 160, 161, 162,
    ];
    pub static ref BISHOP_MASKS: [u64; 64] = array::from_fn(|sq| gen_bishop_mask(sq));
    pub static ref BISHOP_LOOKUP: [u64; 5248] = gen_lookup_bishop();
}

// TODO: REFACTOR #NOT CRITICAL
fn gen_lookup_bishop() -> [u64; 5248] {
    let mut lookup_table = [0u64; 5248];
    for sq in 0..64 {
        for occ in 0..((2 as u64).pow(BISHOP_MASKS[sq].count_ones())) {
            let extracted = insert_bits(BISHOP_MASKS[sq], occ as u64);
            let mut moves = 0u64;

            for dir in DIRECTIONS {
                if dir.is_diagonal() {
                    let ray = RAYS[dir.idx()][sq];
                    moves |=
                        blocked_ray_att(DIRECTIONS[dir.idx()], &RAYS[dir.idx()], ray, 0, extracted);
                }
            }
            lookup_table[BISHOP_BASE[sq] * 32 + (occ as usize)] = moves;
        }
    }
    return lookup_table;
}

pub fn gen_bishop_mask(pos: usize) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= RAYS[Dir::NORTHEAST.idx()][pos] & !RANK_BITBOARD[7] & !FILE_BITBOARD[7];
    attacks |= RAYS[Dir::NORTHWEST.idx()][pos] & !RANK_BITBOARD[7] & !FILE_BITBOARD[0];
    attacks |= RAYS[Dir::SOUTHEAST.idx()][pos] & !RANK_BITBOARD[0] & !FILE_BITBOARD[7];
    attacks |= RAYS[Dir::SOUTHWEST.idx()][pos] & !RANK_BITBOARD[0] & !FILE_BITBOARD[0];

    return attacks;
}

pub fn get_bishop_mv(sq: usize, own: u64, enemy: u64) -> u64 {
    let occupancy = own | enemy;
    let key = pext(occupancy, BISHOP_MASKS[sq]) as usize;

    return BISHOP_LOOKUP[BISHOP_BASE[sq] * 32 + key] & !own;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_king_knight_attacks_init1() {}
}
