use std::arch::x86_64::{_pdep_u64, _pext_u64};
use std::array;
use crate::engine::shared::helper_func::const_utility::*;
use crate::engine::shared::structures::directions::*;
use super::all_attacks::blocked_ray_att;
use super::all_attacks::ATTACKS;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ROOK_MASKS: [u64; 64] = array::from_fn(|sq| gen_rook_mov(sq));
    pub static ref BISHOP_MASKS: [u64; 64] = array::from_fn(|sq| gen_bishop_mov(sq));
    pub static ref ROOK_PEXT_TABLES: [u64; 102400] = gen_pext_table_rook();
    pub static ref BISHOP_PEXT_TABLES: [u64; 5248] = gen_pext_table_bishop();
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
}

fn gen_pext_table_bishop() -> [u64; 5248] {
    let mut lookup_table = [0u64; 5248];
    for sq in 0..64 {
        for occ in 0..((2 as u64).pow(BISHOP_MASKS[sq].count_ones())) {
            let extracted = insert_bits(BISHOP_MASKS[sq], occ as u64);
            let mut moves = 0u64;

            for dir in DIRECTIONS {
                if dir.is_diagonal() {
                    let ray = ATTACKS.rays[dir.idx()][sq];
                    moves |= blocked_ray_att(
                        DIRECTIONS[dir.idx()],
                        &ATTACKS.rays[dir.idx()],
                        ray,
                        0,
                        extracted,
                    );
                }
            }
            lookup_table[BISHOP_BASE[sq] * 32 + (occ as usize)] = moves;
        }
    }
    return lookup_table;
}

pub fn gen_pext_table_rook() -> [u64; 102400] {
    let mut lookup_table = [0u64; 102400];
    for sq in 0..64 {
        for occ in 0..((2 as u64).pow(ROOK_MASKS[sq].count_ones())) {
            let extracted = insert_bits(ROOK_MASKS[sq], occ as u64);
            let mut moves = 0u64;

            for dir in DIRECTIONS {
                if dir.is_orthogonal() {
                    let ray = ATTACKS.rays[dir.idx()][sq];
                    moves |= blocked_ray_att(
                        DIRECTIONS[dir.idx()],
                        &ATTACKS.rays[dir.idx()],
                        ray,
                        0,
                        extracted,
                    );
                }
            }
            lookup_table[ROOK_BASE[sq] * 1024 + (occ as usize)] = moves;
        }
    }

    return lookup_table;
}

pub fn gen_rook_mov(pos: usize) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= ATTACKS.rays[Dir::NORTH.idx()][pos] & !RANK_BITBOARD[7];
    attacks |= ATTACKS.rays[Dir::SOUTH.idx()][pos] & !RANK_BITBOARD[0];
    attacks |= ATTACKS.rays[Dir::EAST.idx()][pos] & !FILE_BITBOARD[7];
    attacks |= ATTACKS.rays[Dir::WEST.idx()][pos] & !FILE_BITBOARD[0];

    return attacks;
}

pub fn gen_bishop_mov(pos: usize) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= ATTACKS.rays[Dir::NORTHEAST.idx()][pos] & !RANK_BITBOARD[7] & !FILE_BITBOARD[7];
    attacks |= ATTACKS.rays[Dir::NORTHWEST.idx()][pos] & !RANK_BITBOARD[7] & !FILE_BITBOARD[0];
    attacks |= ATTACKS.rays[Dir::SOUTHEAST.idx()][pos] & !RANK_BITBOARD[0] & !FILE_BITBOARD[7];
    attacks |= ATTACKS.rays[Dir::SOUTHWEST.idx()][pos] & !RANK_BITBOARD[0] & !FILE_BITBOARD[0];

    return attacks;
}

pub fn gen_slide_mv(sq: usize, own: u64, enemy: u64, is_bishop: bool) -> u64 {
    let mask = if is_bishop { BISHOP_MASKS[sq] } else { ROOK_MASKS[sq] };
    let occupancy = own | enemy;
    let key = pext(occupancy, mask) as usize;

    if is_bishop {
        return BISHOP_PEXT_TABLES[BISHOP_BASE[sq] * 32 + key] & !own;
    } else {
        return ROOK_PEXT_TABLES[ROOK_BASE[sq] * 1024 + key] & !own;
    }
}

fn insert_bits(mask: u64, occupancy: u64) -> u64 {
    let mut result = 0;
    let mut bit = 0;
    for i in 0..64 {
        if (mask >> i) & 1 == 1 {
            if (occupancy >> bit) & 1 == 1 {
                result |= 1 << i;
            }
            bit += 1;
        }
    }
    return result;
}

pub fn pext(bitboard: u64, mask: u64) -> u64 {
    unsafe { _pext_u64(bitboard, mask) }
}

pub fn pdep(bitboard: u64, mask: u64) -> u64 {
    unsafe { _pdep_u64(bitboard, mask) }
}

#[cfg(test)]
mod tests {

    use crate::engine::shared::helper_func::{const_utility::SqPos, print_utility::print_bitboard};

    use super::*;

    #[test]
    fn test_king_knight_attacks_init1() {
        // stacker::grow(10 * 1024 * 1024, || {
        // let idx = SqPos::A1 as usize;
        // print_bitboard(gen_rook_mov(idx), Some(idx as i8));
        // print_bitboard(gen_bishop_mov(idx), Some(idx as i8));

        //     print_bitboard(RANK_BITBOARD[0], None);
        //     print_bitboard(RANK_BITBOARD[7], None);
        //     print_bitboard(FILE_BITBOARD[0], None);
        //     print_bitboard(FILE_BITBOARD[7], None);

        //     let rook_mv = gen_slide_mv(
        //         SqPos::A1 as usize,
        //         (1u64 << (SqPos::A2 as usize)) | (1u64 << (SqPos::E1 as usize)),
        //         0,
        //         false,
        //     );
        //     print_bitboard(rook_mv, Some(SqPos::A1 as i8));

        //     print_bitboard(
        //         insert_bits(
        //             ROOK_MASKS[SqPos::A1 as usize] | edges_of_board(),
        //             (1u64 << (SqPos::A2 as usize)) | (1u64 << (SqPos::E1 as usize)) as u64,
        //         ),
        //         Some(SqPos::A1 as i8),
        //     );

        //     print_bitboard(BISHOP_MASKS[27], None);

        //     let bishop_mv = gen_slide_mv(
        //         SqPos::D2 as usize,
        //         (1u64 << (SqPos::E3 as usize)) | (1u64 << (SqPos::F2 as usize)),
        //         (1u64 << (SqPos::D6 as usize)) | (1u64 << (SqPos::B4 as usize)),
        //         true,
        //     );
        //     print_bitboard(bishop_mv, Some(SqPos::D2 as i8));
        // });
    }
}
