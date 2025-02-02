use crate::engine::shared::helper_func::bit_pos_utility::*;
use crate::make_rays;
use lazy_static::lazy_static;

// NOTE: CONSTANTS
lazy_static! {
    pub static ref KNIGHT_LOOKUP: [u64; 64] = make_rays!(knight_att_bitboard);
}

const KNIGHT_OFFSET_POS: [(i8, i8); 8] =
    [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (2, -1), (2, 1), (1, -2), (1, 2)];

// NOTE: MASK CREATION
pub fn knight_att_bitboard(row: i8, col: i8) -> u64 {
    let mut bitboard = 0;
    for idx in 0..8 {
        let (row_offset, col_offset) = KNIGHT_OFFSET_POS[idx];
        bitboard = set_bit(bitboard, row + row_offset, col + col_offset);
    }
    return bitboard;
}

// NOTE: GET KING MOVES
pub fn get_knight_mv(sq: usize, own: u64, _: u64) -> u64 {
    return KNIGHT_LOOKUP[sq] & !own;
}

#[cfg(test)]
mod tests {

    use crate::engine::shared::helper_func::bit_pos_utility::extract_all_bits;
    use super::*;

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
    fn test_knight_mask() {
        for i in 0..64 {
            assert_eq!(extract_all_bits(KNIGHT_LOOKUP[i]).len(), ALL_KNIGHT_MOVES[i]);
        }
    }

    #[test]
    fn test_knight_attacks_random_pos() {
        assert_eq!(extract_all_bits(KNIGHT_LOOKUP[0]), [10, 17]);
        assert_eq!(extract_all_bits(KNIGHT_LOOKUP[40]), [25, 34, 50, 57]);
        assert_eq!(extract_all_bits(KNIGHT_LOOKUP[17]), [0, 2, 11, 27, 32, 34]);
        assert_eq!(extract_all_bits(KNIGHT_LOOKUP[55]), [38, 45, 61]);
    }
}
