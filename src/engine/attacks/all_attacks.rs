use crate::engine::shared::{
    helper_func::{bit_pos_utility::*, bitboard},
    structures::directions::{Dir, DIRECTIONS},
};
use super::{pawn_attacks::PawnAttacks, ray_attacks::Rays};
use lazy_static::lazy_static;

#[macro_export]
macro_rules! make_rays {
    ($ray_fn:ident) => {{
        let mut rays: [u64; 64] = [0u64; 64];

        for row in 0..8 {
            for col in 0..8 {
                rays[row * 8 + col] = $ray_fn(row as i8, col as i8);
            }
        }

        rays
    }};
}

// DEPRECATE:
// #[macro_export]
// macro_rules! define_ray {
//     ($name:ident, $offset_fn:expr) => {
//         pub fn $name(row: i8, col: i8) -> u64 {
//             let mut bitboard = 0;

//             for offset in 1..8 {
//                 let (row_offset, col_offset) = $offset_fn(row, col, offset);
//                 bitboard = set_bit(bitboard, row_offset, col_offset);
//             }

//             return bitboard;
//         }
//     };
// }

const KING_OFFSET_POS: [(i8, i8); 8] =
    [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
const KNIGHT_OFFSET_POS: [(i8, i8); 8] =
    [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (2, -1), (2, 1), (1, -2), (1, 2)];

lazy_static! {
    pub static ref ATTACKS: Attacks = Attacks::init();
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attacks {
    pub king: [u64; 64],
    pub knight: [u64; 64],
    pub pawn: PawnAttacks,
    pub rays: [[u64; 64]; 8],
}

impl Attacks {
    pub fn init() -> Self {
        return Self {
            king: make_rays!(king_att_bitboard),
            knight: make_rays!(knight_att_bitboard),
            pawn: PawnAttacks::init(),
            rays: ray_init(),
        };
    }
}

pub fn ray_init() -> [[u64; 64]; 8] {
    let mut rays = [[0u64; 64]; 8];
    for dir in DIRECTIONS {
        for row in 0..8 {
            for col in 0..8 {
                rays[dir.idx()][row * 8 + col] = rays_att_bitboard(dir, row as i8, col as i8);
            }
        }
    }
    return rays;
}

pub fn rays_att_bitboard(dir: Dir, row: i8, col: i8) -> u64 {
    let mut bitboard = 0;
    for i in 1..8 {
        let (row_offset, col_offset) = dir.dir_offset();
        bitboard = set_bit(bitboard, row + row_offset * i, col + col_offset * i);
    }
    return bitboard;
}

pub fn king_att_bitboard(row: i8, col: i8) -> u64 {
    let mut bitboard = 0;
    for idx in 0..8 {
        let (row_offset, col_offset) = KING_OFFSET_POS[idx];
        bitboard = set_bit(bitboard, row + row_offset, col + col_offset);
    }
    return bitboard;
}

pub fn knight_att_bitboard(row: i8, col: i8) -> u64 {
    let mut bitboard = 0;
    for idx in 0..8 {
        let (row_offset, col_offset) = KNIGHT_OFFSET_POS[idx];
        bitboard = set_bit(bitboard, row + row_offset, col + col_offset);
    }
    return bitboard;
}

pub fn first_hit(dir: Dir, bitboard: u64) -> Option<usize> {
    if bitboard == 0 {
        return None;
    } else if dir.is_forward() {
        return Some(bit_scan_lsb(bitboard));
    } else {
        return Some(bit_scan_msb(bitboard));
    }
}

pub fn blocked_ray_att(dir: Dir, ray_family: &[u64; 64], ray: u64, own: u64, enemy: u64) -> u64 {
    let first_own_hit = first_hit(dir, ray & own);
    let first_enemy_hit = first_hit(dir, ray & enemy);

    match (first_own_hit, first_enemy_hit) {
        (None, None) => return ray,
        (None, Some(enemy_idx)) => return ray ^ ray_family[enemy_idx],
        (Some(own_idx), None) => return ray ^ (ray_family[own_idx] | (1 << own_idx)),
        (Some(own_idx), Some(enemy_idx)) => {
            return ray ^ (ray_family[own_idx] | (1 << own_idx) | ray_family[enemy_idx])
        }
    }
}

#[cfg(test)]
mod tests {
    use bitboard::BitboardTrait;

    use crate::engine::{
        attacks,
        shared::helper_func::{bit_pos_utility::extract_all_bits, print_utility::print_bitboard},
    };

    use super::*;

    #[rustfmt::skip]
    const ALL_KING_MOVES: [usize; 64] = [
        3, 5, 5, 5, 5, 5, 5, 3,
        5, 8, 8, 8, 8, 8, 8, 5,
        5, 8, 8, 8, 8, 8, 8, 5,
        5, 8, 8, 8, 8, 8, 8, 5,
        5, 8, 8, 8, 8, 8, 8, 5,
        5, 8, 8, 8, 8, 8, 8, 5,
        5, 8, 8, 8, 8, 8, 8, 5,
        3, 5, 5, 5, 5, 5, 5, 3,
    ];

    #[rustfmt::skip]
    const ALL_KNIGHT_MOVES: [usize; 64] = [
        2, 3, 4, 4, 4, 4, 3, 2,
        3, 4, 6, 6, 6, 6, 4, 3,
        4, 6, 8, 8, 8, 8, 6, 4,
        4, 6, 8, 8, 8, 8, 6, 4,
        4, 6, 8, 8, 8, 8, 6, 4,
        4, 6, 8, 8, 8, 8, 6, 4,
        3, 4, 6, 6, 6, 6, 4, 3,
        2, 3, 4, 4, 4, 4, 3, 2,
    ];

    #[test]
    fn test_king_knight_attacks_init() {
        let attacks = Attacks::init();
        for i in 0..64 {
            assert_eq!(extract_all_bits(attacks.king[i]).len(), ALL_KING_MOVES[i]);
            assert_eq!(extract_all_bits(attacks.knight[i]).len(), ALL_KNIGHT_MOVES[i]);
        }
    }

    #[test]
    fn test_king_attacks_random_pos() {
        let attacks = Attacks::init();
        assert_eq!(extract_all_bits(attacks.king[0]), [1, 8, 9]);
        assert_eq!(extract_all_bits(attacks.king[40]), [32, 33, 41, 48, 49]);
        assert_eq!(extract_all_bits(attacks.king[55]), [46, 47, 54, 62, 63]);
        assert_eq!(extract_all_bits(attacks.king[17]), [8, 9, 10, 16, 18, 24, 25, 26]);

        assert_eq!(extract_all_bits(attacks.knight[0]), [10, 17]);
        assert_eq!(extract_all_bits(attacks.knight[40]), [25, 34, 50, 57]);
        assert_eq!(extract_all_bits(attacks.knight[17]), [0, 2, 11, 27, 32, 34]);
        assert_eq!(extract_all_bits(attacks.knight[55]), [38, 45, 61]);
    }

    // **** START: Test Rays ****

    fn get_occupancy() -> (u64, u64) {
        let mut own_occupancy = 0;
        let mut enemy_occupancy = 0;

        for i in 0..16 {
            if i != 5 {
                own_occupancy.set_bit(i);
            }
        }
        own_occupancy.set_bit(22);

        for i in 48..64 {
            if i != 57 && i != 49 {
                enemy_occupancy.set_bit(i);
            }
        }
        enemy_occupancy.set_bit(41);
        enemy_occupancy.set_bit(42);
        return (own_occupancy, enemy_occupancy);
    }

    // ********

    #[test]
    fn test_rays() {
        let rays = Attacks::init();
        let (row, col) = (5, 6);
        let idx = position_to_idx(row, col, Some(true)) as usize;

        let mut expected_sw_6_7 = 0;
        let mut expected_w_6_7 = 0;
        let mut expected_ne_6_7 = 0;

        for i in 1..8 {
            if is_inside_board_bounds_row_col(row - i, col - i) {
                expected_sw_6_7 = set_bit(expected_sw_6_7, row - i, col - i);
            }
            if is_inside_board_bounds_row_col(row, col - i) {
                expected_w_6_7 = set_bit(expected_w_6_7, row, col - i);
            }
            if is_inside_board_bounds_row_col(row + i, col + i) {
                expected_ne_6_7 = set_bit(expected_ne_6_7, row + i, col + i);
            }
        }
        assert_eq!(rays.rays[Dir::SOUTHWEST.idx()][idx], expected_sw_6_7);
        assert_eq!(rays.rays[Dir::WEST.idx()][idx], expected_w_6_7);
        assert_eq!(rays.rays[Dir::NORTHEAST.idx()][idx], expected_ne_6_7);
    }

    #[test]
    fn test_blocked_rays() {
        let rays = Attacks::init();
        let (own, enemy) = get_occupancy();
        let idx = position_to_idx(4, 4, Some(true));
        let ray_arr = rays.rays[Dir::NORTHWEST.idx()];
        let blocked_ray =
            blocked_ray_att(Dir::NORTHWEST, &ray_arr, ray_arr[idx as usize], enemy, own);

        assert_eq!(blocked_ray, 1 << (idx + 7));
    }

    #[test]
    fn test_print_rays() {
        let (rays, idx) = (Attacks::init(), 43);
        print_bitboard(rays.rays[Dir::NORTH.idx()][idx], Some(idx as i8));
        print_bitboard(rays.rays[Dir::NORTHEAST.idx()][idx], Some(idx as i8));
        print_bitboard(rays.rays[Dir::EAST.idx()][idx], Some(idx as i8));
        print_bitboard(rays.rays[Dir::SOUTHEAST.idx()][idx], Some(idx as i8));
        print_bitboard(rays.rays[Dir::SOUTH.idx()][idx], Some(idx as i8));
        print_bitboard(rays.rays[Dir::SOUTHWEST.idx()][idx], Some(idx as i8));
        print_bitboard(rays.rays[Dir::WEST.idx()][idx], Some(idx as i8));
        print_bitboard(rays.rays[Dir::NORTHWEST.idx()][idx], Some(idx as i8));
    }
}
