use crate::engine::board::castling::*;
use crate::engine::board::color::Color;
use crate::engine::generated::king::KING_LOOKUP;
use crate::engine::generated::pawn::ISOLATED_PAWN_LOOKUP;

#[inline(always)]
pub fn get_king_mv(sq: usize, own: u64, _: u64, _: Color) -> u64 {
    KING_LOOKUP[sq] & !own
}

#[inline(always)]
pub const fn get_king_mask(sq: usize, _own: u64, _: u64, _: Color) -> u64 {
    KING_LOOKUP[sq]
}

// TODO: TEST ME
pub fn has_good_pawn_shield(own_pawns: u64, castling: Option<Castling>) -> bool {
    if let Some(c) = castling {
        (own_pawns & CASTLE_PAWN_SHIELD[c.count_ones() as usize]).count_ones() == 3
    } else {
        false
    }
}

// TODO: TEST ME
#[inline(always)]
pub fn has_near_open_files(sq: usize, own_pawns: u64) -> bool {
    ISOLATED_PAWN_LOOKUP[sq] & own_pawns == 0
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
