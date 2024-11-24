use crate::engine::shared::helper_func::utils::*;

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

pub struct KnightAttacks {
    knight_rays: Vec<u64>,
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
        bitboard = set_bit(bitboard, (row, col), knight_attack_arr[idx]);
        bitboard = set_bit(bitboard, (row, col), knight_attack_arr[idx]);
    }

    return bitboard;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_attacks_initialize() {
        let attacks = KnightAttacks::initialize();
    }

    #[test]
    fn test_knight_attacks() {
        let attacks = KnightAttacks::initialize();
        let idx = 44;
        println!("{}", bitboard_to_string(attacks.knight_rays[idx], Some(idx)));
        println!("{}", bitboard_to_string(attacks.knight_rays[32], Some(32)));
        println!("{}", bitboard_to_string(attacks.knight_rays[14], Some(14)));
        println!("{}", bitboard_to_string(attacks.knight_rays[52], Some(52)));
        println!("{}", bitboard_to_string(attacks.knight_rays[62], Some(62)));
        println!("{}", bitboard_to_string(attacks.knight_rays[63], Some(63)));
    }
}
