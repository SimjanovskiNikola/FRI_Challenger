use crate::engine::shared::helper_func::bit_pos_utility::*;
use crate::make_rays;
use lazy_static::lazy_static;

use super::generated::king::KING_LOOKUP;

// NOTE: GET KING MOVES
pub fn get_king_mv(sq: usize, own: u64, _: u64) -> u64 {
    return KING_LOOKUP[sq] & !own;
}

#[cfg(test)]
mod tests {

    use crate::engine::shared::helper_func::bit_pos_utility::extract_all_bits;
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

    #[test]
    fn test_king_mask() {
        for i in 0..64 {
            assert_eq!(extract_all_bits(KING_LOOKUP[i]).len(), ALL_KING_MOVES[i]);
        }
    }

    #[test]
    fn test_king_mask_random_pos() {
        assert_eq!(extract_all_bits(KING_LOOKUP[0]), [1, 8, 9]);
        assert_eq!(extract_all_bits(KING_LOOKUP[40]), [32, 33, 41, 48, 49]);
        assert_eq!(extract_all_bits(KING_LOOKUP[55]), [46, 47, 54, 62, 63]);
        assert_eq!(extract_all_bits(KING_LOOKUP[17]), [8, 9, 10, 16, 18, 24, 25, 26]);
    }
}
