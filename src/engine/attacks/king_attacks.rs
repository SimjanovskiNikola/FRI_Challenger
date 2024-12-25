use crate::{engine::shared::helper_func::bit_pos_utility::*, make_rays};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KingAttacks {
    pub king_attacks: Vec<u64>,
}

impl KingAttacks {
    pub fn init() -> Self {
        return Self { king_attacks: make_rays!(king_attacks) };
    }
}

fn king_attacks(row: i8, col: i8) -> u64 {
    let mut bitboard = 0;
    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            if !(row_offset == 0 && col_offset == 0) {
                bitboard |= set_bit(bitboard, row + row_offset, col + col_offset);
            }
        }
    }

    return bitboard;
}

#[cfg(test)]
mod tests {
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
    fn test_king_attacks_init() {
        let attacks = KingAttacks::init();
        for i in 0..64 {
            assert_eq!(
                extract_all_bits(attacks.king_attacks[i]).len(),
                ALL_KING_MOVES[i]
            );
        }
    }

    #[test]
    fn test_king_attacks_random_pos() {
        let attacks = KingAttacks::init();
        assert_eq!(extract_all_bits(attacks.king_attacks[0]), [1, 8, 9]);
        assert_eq!(extract_all_bits(attacks.king_attacks[40]), [32, 33, 41, 48, 49]);
        assert_eq!(extract_all_bits(attacks.king_attacks[55]), [46, 47, 54, 62, 63]);
        assert_eq!(
            extract_all_bits(attacks.king_attacks[17]),
            [8, 9, 10, 16, 18, 24, 25, 26]
        );
    }
}
