use crate::engine::shared::helper_func::bit_pos_utility::*;

const knight_attack_arr: [(i64, i64); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (2, -1),
    (2, 1),
    (1, -2),
    (1, 2),
];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KnightAttacks {
    pub knight_rays: Vec<u64>,
}

impl KnightAttacks {
    pub fn initialize() -> Self {
        let mut attacks = vec![];
        for row in 0..8 {
            for col in 0..8 {
                let attack = knight_attacks(row, col);
                attacks.push(attack);
            }
        }
        return Self { knight_rays: attacks };
    }
}

pub fn knight_attacks(row: i64, col: i64) -> u64 {
    let mut bitboard = 0;

    for idx in 0..8 {
        let x = knight_attack_arr[idx].0;
        let y = knight_attack_arr[idx].1;
        bitboard = set_bit(bitboard, (row + x) as usize, (col + y) as usize);
        // bitboard = set_bit(bitboard, (row, col), knight_attack_arr[idx]);
    }

    return bitboard;
}

#[cfg(test)]
mod tests {
    use crate::engine::shared::helper_func::print_utility::bitboard_to_string;

    use super::*;

    #[test]
    fn test_knight_attacks_initialize() {
        let attacks = KnightAttacks::initialize();
    }

    #[test]
    fn test_knight_attacks() {
        let attacks = KnightAttacks::initialize();
        // println!("{}", bitboard_to_string(attacks.knight_rays[0], Some(0)));
        assert_eq!(extract_all_bits(attacks.knight_rays[0]), [10, 17]);
        // println!("{}", bitboard_to_string(attacks.knight_rays[40], Some(40)));
        assert_eq!(extract_all_bits(attacks.knight_rays[40]), [25, 34, 50, 57]);
        // println!("{}", bitboard_to_string(attacks.knight_rays[17], Some(17)));
        assert_eq!(extract_all_bits(attacks.knight_rays[17]), [0, 2, 11, 27, 32, 34]);
        // println!("{}", bitboard_to_string(attacks.knight_rays[55], Some(55)));
        assert_eq!(extract_all_bits(attacks.knight_rays[55]), [38, 45, 61]);
    }
}
