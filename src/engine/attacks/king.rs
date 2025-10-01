use crate::engine::board::color::Color;
use crate::engine::generated::king::KING_LOOKUP;

#[inline(always)]
/// Gets King moves considering other pieces on the board and excluding own pieces
pub const fn get_king_mv(sq: usize, own: u64, _: u64, _: Color) -> u64 {
    KING_LOOKUP[sq] & !own
}

#[inline(always)]
/// Gets only the mask of possible moves, ignoring other pieces on the board
pub const fn get_king_mask(sq: usize, _: u64, _: u64, _: Color) -> u64 {
    KING_LOOKUP[sq]
}

#[cfg(test)]
mod tests {

    use crate::engine::misc::bit_pos_utility::extract_all_bits;

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
