use crate::engine::board::color::Color;

use crate::engine::generated::knight::KNIGHT_LOOKUP;

#[inline(always)]
/// Gets Knight moves considering other pieces on the board and excluding own pieces
pub const fn get_knight_mv(sq: usize, own: u64, _: u64, _: Color) -> u64 {
    KNIGHT_LOOKUP[sq] & !own
}

#[inline(always)]
/// Gets only the mask of possible moves, ignoring other pieces on the board
pub const fn get_knight_mask(sq: usize, _: u64, _: u64, _: Color) -> u64 {
    KNIGHT_LOOKUP[sq]
}

#[cfg(test)]
mod tests {

    use crate::engine::misc::bit_pos_utility::extract_all_bits;

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
