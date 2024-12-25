use crate::{engine::shared::helper_func::bit_pos_utility::*, make_rays};

const KNIGHT_OFFSET_POS: [(i8, i8); 8] =
    [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (2, -1), (2, 1), (1, -2), (1, 2)];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KnightAttacks {
    pub knight_attacks: Vec<u64>,
}

impl KnightAttacks {
    pub fn init() -> Self {
        return Self { knight_attacks: make_rays!(knight_attacks) };
    }
}

pub fn knight_attacks(row: i8, col: i8) -> u64 {
    let mut bitboard = 0;

    for idx in 0..8 {
        let (row_offset, col_offset) = KNIGHT_OFFSET_POS[idx];
        bitboard = set_bit(bitboard, row + row_offset, col + col_offset);
    }

    return bitboard;
}

#[cfg(test)]
mod tests {
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
    fn test_knight_attacks_init() {
        let attacks = KnightAttacks::init();
        for i in 0..64 {
            assert_eq!(
                extract_all_bits(attacks.knight_attacks[i]).len(),
                ALL_KNIGHT_MOVES[i]
            );
        }
    }

    #[test]
    fn test_knight_attacks_random_pos() {
        let attacks = KnightAttacks::init();
        assert_eq!(extract_all_bits(attacks.knight_attacks[0]), [10, 17]);
        assert_eq!(extract_all_bits(attacks.knight_attacks[40]), [25, 34, 50, 57]);
        assert_eq!(extract_all_bits(attacks.knight_attacks[17]), [0, 2, 11, 27, 32, 34]);
        assert_eq!(extract_all_bits(attacks.knight_attacks[55]), [38, 45, 61]);
    }
}
