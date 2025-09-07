use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::misc::directions::Dir;

pub fn first_hit(dir: Dir, bitboard: u64) -> Option<usize> {
    if bitboard == 0 {
        None
    } else if dir.is_forward() {
        Some(bitboard.get_msb())
    } else {
        Some(bitboard.get_lsb())
    }
}

pub fn blocked_ray_att(dir: Dir, ray_family: &[u64; 64], ray: u64, own: u64, enemy: u64) -> u64 {
    let first_own_hit = first_hit(dir, ray & own);
    let first_enemy_hit = first_hit(dir, ray & enemy);

    match (first_own_hit, first_enemy_hit) {
        (None, None) => ray,
        (None, Some(enemy_idx)) => ray ^ ray_family[enemy_idx],
        (Some(own_idx), None) => ray ^ (ray_family[own_idx] | (1 << own_idx)),
        (Some(own_idx), Some(enemy_idx)) => {
            ray ^ (ray_family[own_idx] | (1 << own_idx) | ray_family[enemy_idx])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::generated::rays::RAYS_LOOKUP;
    use crate::engine::misc::bit_pos_utility::*;
    use crate::engine::misc::bitboard::BitboardTrait;
    use crate::engine::misc::display::display_board::print_bitboard;

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
        assert_eq!(RAYS_LOOKUP[Dir::SOUTHWEST.idx()][idx], expected_sw_6_7);
        assert_eq!(RAYS_LOOKUP[Dir::WEST.idx()][idx], expected_w_6_7);
        assert_eq!(RAYS_LOOKUP[Dir::NORTHEAST.idx()][idx], expected_ne_6_7);
    }

    #[test]
    fn test_blocked_rays() {
        let (own, enemy) = get_occupancy();
        let idx = position_to_idx(4, 4, Some(true));
        let ray_arr = RAYS_LOOKUP[Dir::NORTHWEST.idx()];
        let blocked_ray =
            blocked_ray_att(Dir::NORTHWEST, &ray_arr, ray_arr[idx as usize], enemy, own);

        assert_eq!(blocked_ray, 1 << (idx + 7));
    }

    #[test]
    fn test_print_rays() {
        let idx = 43;
        print_bitboard(RAYS_LOOKUP[Dir::NORTH.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS_LOOKUP[Dir::NORTHEAST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS_LOOKUP[Dir::EAST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS_LOOKUP[Dir::SOUTHEAST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS_LOOKUP[Dir::SOUTH.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS_LOOKUP[Dir::SOUTHWEST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS_LOOKUP[Dir::WEST.idx()][idx], Some(idx as i8));
        print_bitboard(RAYS_LOOKUP[Dir::NORTHWEST.idx()][idx], Some(idx as i8));
    }
}
