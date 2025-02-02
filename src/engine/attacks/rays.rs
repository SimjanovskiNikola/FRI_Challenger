use crate::engine::shared::{helper_func::bit_pos_utility::*, structures::directions::*};
use lazy_static::lazy_static;

// NOTE: CONSTANTS
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

lazy_static! {
    pub static ref RAYS: [[u64; 64]; 8] = ray_init();
}

// NOTE: GET Rays
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

    use crate::engine::shared::{
        helper_func::{bitboard::BitboardTrait, print_utility::print_bitboard},
        structures::directions::Dir,
    };

    use super::*;

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

    #[test]
    fn test_rays() {
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
        assert_eq!(RAYS[Dir::SOUTHWEST.idx()][idx], expected_sw_6_7);
        assert_eq!(RAYS[Dir::WEST.idx()][idx], expected_w_6_7);
        assert_eq!(RAYS[Dir::NORTHEAST.idx()][idx], expected_ne_6_7);
    }

    #[test]
    fn test_blocked_rays() {
        let (own, enemy) = get_occupancy();
        let idx = position_to_idx(4, 4, Some(true));
        let ray_arr = RAYS[Dir::NORTHWEST.idx()];
        let blocked_ray =
            blocked_ray_att(Dir::NORTHWEST, &ray_arr, ray_arr[idx as usize], enemy, own);

        assert_eq!(blocked_ray, 1 << (idx + 7));
    }

    #[test]
    fn test_print_rays() {
        let idx = 43;
        print_bitboard(RAYS[Dir::NORTH.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS[Dir::NORTHEAST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS[Dir::EAST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS[Dir::SOUTHEAST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS[Dir::SOUTH.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS[Dir::SOUTHWEST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS[Dir::WEST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS[Dir::NORTHWEST.idx()][idx], Some(idx as i8));
    }
}
