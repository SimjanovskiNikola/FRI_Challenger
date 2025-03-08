use crate::engine::shared::helper_func::bit_pos_utility::*;
use crate::engine::shared::helper_func::bitboard::BitboardTrait;
use crate::engine::shared::helper_func::const_utility::Rank;

use crate::engine::shared::structures::color::*;
use crate::engine::shared::structures::piece::*;

use super::generated::pawn::*;

// PAWN MOVE, ATTACK, EP
#[inline(always)]
pub fn get_pawn_mv(color: Color, sq: usize, own: u64, enemy: u64) -> u64 {
    let moves = PAWN_MOVE_LOOKUP[color.idx()][sq] & !(own | enemy);

    let bit = match color {
        WHITE => PAWN_MOVE_LOOKUP[color.idx()][sq].get_lsb(),
        BLACK => PAWN_MOVE_LOOKUP[color.idx()][sq].get_msb(),
        _ => panic!("There are only two colors, black and white"),
    };

    return if moves.is_set(bit) { moves } else { 0 };
}

#[inline(always)]
pub fn get_pawn_att(color: Color, sq: usize, own: u64, enemy: u64, ep: Option<u8>) -> u64 {
    let attacks = PAWN_ATTACK_LOOKUP[color.idx()][sq] & !own;
    match ep {
        Some(ep) => attacks & (enemy | get_pawn_ep(color, ep)),
        None => attacks & enemy,
    }
}

#[inline(always)]
pub fn get_pawn_ep(color: Color, ep: u8) -> u64 {
    let rank_ep = get_bit_rank(ep as usize);
    if (rank_ep == Rank::Six && color.is_white()) || (rank_ep == Rank::Three && color.is_black()) {
        1 << ep
    } else {
        0
    }
}

// FORWARD AND DIAGONAL MOVES
// DEPRECATE:
#[deprecated = "Leaving Here If I need this in the future, otherwise not needed"]
fn forward_move(row: i8, col: i8, color: Color) -> u64 {
    let mut bitboard = 0;
    if color == WHITE {
        if row < 7 {
            bitboard |= set_bit(bitboard, row + 1, col);
        }
        if row == 1 {
            bitboard |= set_bit(bitboard, row + 2, col);
        }
    } else {
        if row > 0 {
            bitboard |= set_bit(bitboard, row - 1, col);
        }
        if row == 6 {
            bitboard |= set_bit(bitboard, row - 2, col);
        }
    }

    bitboard
}

// DEPRECATE:
#[deprecated = "Leaving Here If I need this in the future, otherwise not needed"]
fn diagonal_move(row: i8, col: i8, color: Color) -> u64 {
    let mut bitboard = 0;
    if color == WHITE && row < 7 {
        bitboard |= set_bit(bitboard, row + 1, col + 1);
        bitboard |= set_bit(bitboard, row + 1, col - 1);
    } else if color == BLACK && row > 0 {
        bitboard |= set_bit(bitboard, row - 1, col + 1);
        bitboard |= set_bit(bitboard, row - 1, col - 1);
    }

    bitboard
}

use rand::Rng;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn zobrist_keys() {
        let mut rand: u64 = rand::rng().random();

        for i in 0..48 {
            println!("[");
            for i in 0..14 {
                rand = rand::rng().random();
                println!("{:?},", rand);
            }
            println!("],");
        }
    }

    #[test]
    fn test_second_row_white_pawn() {
        let row = 1;
        for col in 0..8 {
            let bitboard = forward_move(row, col, WHITE);
            let lsb = bitboard.get_lsb();
            let msb = bitboard.get_msb();
            assert_eq!(lsb as i8, position_to_idx(row + 1, col, None));
            assert_eq!(msb as i8, position_to_idx(row + 2, col, None));
        }
    }

    #[test]
    fn test_second_row_black_pawn() {
        let row = 1;
        for col in 0..8 {
            let bitboard = forward_move(row, col, BLACK);
            let lsb = bitboard.get_lsb();
            assert_eq!(lsb as i8, position_to_idx(row - 1, col, None));
        }
    }

    #[test]
    fn test_seventh_row_black_pawn() {
        let row = 6;
        for col in 0..8 {
            let bitboard = forward_move(row, col, BLACK);
            let lsb = bitboard.get_lsb();
            let msb = bitboard.get_msb();
            assert_eq!(msb as i8, position_to_idx(row - 1, col, None));
            assert_eq!(lsb as i8, position_to_idx(row - 2, col, None));
        }

        // println!("{:#?}", PAWN_ATTACK);
        // println!("{:#?}", PAWN_MOVE);
    }

    #[test]
    fn test_seventh_row_white_pawn() {
        let row = 6;
        for col in 0..8 {
            let bitboard = forward_move(row, col, WHITE);
            let lsb = bitboard.get_lsb();
            assert_eq!(lsb as i8, position_to_idx(row + 1, col, None));
        }
    }

    #[test]
    fn test_middle_row_white_pawn() {
        for row in 2..7 {
            for col in 0..8 {
                let bitboard = forward_move(row, col, WHITE);
                let lsb = bitboard.get_lsb();
                assert_eq!(lsb as i8, position_to_idx(row + 1, col, None));
            }
        }
    }

    #[test]
    fn test_middle_row_black_pawn() {
        for row in 1..6 {
            for col in 0..8 {
                let bitboard = forward_move(row, col, BLACK);
                let lsb = bitboard.get_lsb();
                assert_eq!(lsb as i8, position_to_idx(row - 1, col, None));
            }
        }
    }

    // #[test]
    // fn test_forward_edges_pawn_attacks() {
    //     for color in [WHITE, BLACK] {
    //         for row in [0, 7] {
    //             for col in 0..8 {
    //                 let bitboard = forward_move(row, col, color);
    //                 print_bitboard(bitboard, None);
    //                 assert_eq!(bitboard, 0);
    //             }
    //         }
    //     }
    // }

    // #[test]
    // fn test_diagonal_edges_pawn_attacks() {
    //     for color in [WHITE, BLACK] {
    //         for row in [0, 7] {
    //             for col in 0..8 {
    //                 let bitboard = diagonal_move(row, col, color);
    //                 print_bitboard(bitboard, None);
    //                 assert_eq!(bitboard, 0);
    //             }
    //         }
    //     }
    // }

    #[test]
    fn test_diagonal_white_pawn() {
        for row in 1..6 {
            for col in 1..6 {
                let bitboard = diagonal_move(row, col, WHITE);
                let lsb = bitboard.get_lsb();
                let msb = bitboard.get_msb();

                assert_eq!(lsb as i8, position_to_idx(row + 1, col - 1, None));
                assert_eq!(msb as i8, position_to_idx(row + 1, col + 1, None));
            }
        }
    }

    #[test]
    fn test_diagonal_white_pawn_col_edge() {
        for row in 1..6 {
            let col = 0;
            let bitboard = diagonal_move(row, col, WHITE);
            let lsb = bitboard.get_lsb();

            assert_eq!(lsb as i8, position_to_idx(row + 1, col + 1, None));
        }

        for row in 1..6 {
            let col = 6;
            let bitboard = diagonal_move(row, col, WHITE);
            let lsb = bitboard.get_lsb();

            assert_eq!(lsb as i8, position_to_idx(row + 1, col - 1, None));
        }
    }

    #[test]
    fn test_diagonal_black_pawn() {
        for row in 1..6 {
            for col in 1..6 {
                let bitboard = diagonal_move(row, col, BLACK);
                let lsb = bitboard.get_lsb();
                let msb = bitboard.get_msb();

                assert_eq!(lsb as i8, position_to_idx(row - 1, col - 1, None));
                assert_eq!(msb as i8, position_to_idx(row - 1, col + 1, None));
            }
        }
    }

    #[test]
    fn test_diagonal_black_pawn_col_edge() {
        for row in 1..6 {
            let col = 0;
            let bitboard = diagonal_move(row, col, BLACK);
            let lsb = bitboard.get_lsb();

            assert_eq!(lsb as i8, position_to_idx(row - 1, col + 1, None));
        }

        for row in 1..6 {
            let col = 6;
            let bitboard = diagonal_move(row, col, BLACK);
            let lsb = bitboard.get_lsb();

            assert_eq!(lsb as i8, position_to_idx(row - 1, col - 1, None));
        }
    }
}
