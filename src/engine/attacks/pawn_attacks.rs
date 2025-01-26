use crate::engine::shared::{
    helper_func::bit_pos_utility::*,
    structures::color::{Color, BLACK, WHITE},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PawnAttacks {
    pub white_forward_moves: Vec<u64>,
    pub white_diagonal_moves: Vec<u64>,
    pub black_forward_moves: Vec<u64>,
    pub black_diagonal_moves: Vec<u64>,
}

impl PawnAttacks {
    pub fn init() -> Self {
        let mut w_forward = vec![];
        let mut w_diagonal = vec![];
        let mut b_forward = vec![];
        let mut b_diagonal = vec![];

        for row in 0..8 {
            for col in 0..8 {
                let f = forward_move(row, col, WHITE);
                let d = diagonal_move(row, col, WHITE);
                w_forward.push(f);
                w_diagonal.push(d);

                let f = forward_move(row, col, BLACK);
                let d = diagonal_move(row, col, BLACK);
                b_forward.push(f);
                b_diagonal.push(d);
            }
        }

        return Self {
            white_forward_moves: w_forward,
            white_diagonal_moves: w_diagonal,
            black_forward_moves: b_forward,
            black_diagonal_moves: b_diagonal,
        };
    }
}

fn forward_move(row: i8, col: i8, piece_color: Color) -> u64 {
    if row == 0 || row == 7 {
        return 0;
    }

    let mut bitboard = 0;
    if piece_color == WHITE {
        if row < 7 {
            bitboard |= set_bit(bitboard, row + 1, col + 0);
        }
        if row == 1 {
            bitboard |= set_bit(bitboard, row + 2, col + 0);
        }
    } else {
        if row > 0 {
            bitboard |= set_bit(bitboard, row - 1, col + 0);
        }
        if row == 6 {
            bitboard |= set_bit(bitboard, row - 2, col + 0);
        }
    }
    return bitboard;
}

fn diagonal_move(row: i8, col: i8, piece_color: Color) -> u64 {
    if row == 0 || row == 7 {
        return 0;
    }

    let mut bitboard = 0;
    if piece_color == WHITE {
        if row < 7 {
            bitboard |= set_bit(bitboard, row + 1, col + 1);
            bitboard |= set_bit(bitboard, row + 1, col - 1);
        }
    } else {
        if row > 0 {
            bitboard |= set_bit(bitboard, row - 1, col + 1);
            bitboard |= set_bit(bitboard, row - 1, col - 1);
        }
    }
    return bitboard;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pawn_attacks_init() {
        let _ = PawnAttacks::init();
    }

    #[test]
    fn test_second_row_white_pawn() {
        let row = 1;
        for col in 0..8 {
            let bitboard = forward_move(row, col, WHITE);
            let lsb = bit_scan_lsb(bitboard);
            let msb = bit_scan_msb(bitboard);
            assert_eq!(lsb as i8, position_to_idx(row + 1, col, None));
            assert_eq!(msb as i8, position_to_idx(row + 2, col, None));
        }
    }

    #[test]
    fn test_second_row_black_pawn() {
        let row = 1;
        for col in 0..8 {
            let bitboard = forward_move(row, col, BLACK);
            let lsb = bit_scan_lsb(bitboard);
            assert_eq!(lsb as i8, position_to_idx(row - 1, col, None));
        }
    }

    #[test]
    fn test_seventh_row_black_pawn() {
        let row = 6;
        for col in 0..8 {
            let bitboard = forward_move(row, col, BLACK);
            let lsb = bit_scan_lsb(bitboard);
            let msb = bit_scan_msb(bitboard);
            assert_eq!(msb as i8, position_to_idx(row - 1, col, None));
            assert_eq!(lsb as i8, position_to_idx(row - 2, col, None));
        }
    }

    #[test]
    fn test_seventh_row_white_pawn() {
        let row = 6;
        for col in 0..8 {
            let bitboard = forward_move(row, col, WHITE);
            let lsb = bit_scan_lsb(bitboard);
            assert_eq!(lsb as i8, position_to_idx(row + 1, col, None));
        }
    }

    #[test]
    fn test_middle_row_white_pawn() {
        for row in 2..7 {
            for col in 0..8 {
                let bitboard = forward_move(row, col, WHITE);
                let lsb = bit_scan_lsb(bitboard);
                assert_eq!(lsb as i8, position_to_idx(row + 1, col, None));
            }
        }
    }

    #[test]
    fn test_middle_row_black_pawn() {
        for row in 1..6 {
            for col in 0..8 {
                let bitboard = forward_move(row, col, BLACK);
                let lsb = bit_scan_lsb(bitboard);
                assert_eq!(lsb as i8, position_to_idx(row - 1, col, None));
            }
        }
    }

    #[test]
    fn test_forward_edges_pawn_attacks() {
        for color in [WHITE, BLACK] {
            for row in [0, 7] {
                for col in 0..8 {
                    let bitboard = forward_move(row, col, color);
                    assert_eq!(bitboard, 0);
                }
            }
        }
    }

    #[test]
    fn test_diagonal_edges_pawn_attacks() {
        for color in [WHITE, BLACK] {
            for row in [0, 7] {
                for col in 0..8 {
                    let bitboard = diagonal_move(row, col, color);
                    assert_eq!(bitboard, 0);
                }
            }
        }
    }

    #[test]
    fn test_diagonal_white_pawn() {
        for row in 1..6 {
            for col in 1..6 {
                let bitboard = diagonal_move(row, col, WHITE);
                let lsb = bit_scan_lsb(bitboard);
                let msb = bit_scan_msb(bitboard);

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
            let lsb = bit_scan_lsb(bitboard);

            assert_eq!(lsb as i8, position_to_idx(row + 1, col + 1, None));
        }

        for row in 1..6 {
            let col = 6;
            let bitboard = diagonal_move(row, col, WHITE);
            let lsb = bit_scan_lsb(bitboard);

            assert_eq!(lsb as i8, position_to_idx(row + 1, col - 1, None));
        }
    }

    #[test]
    fn test_diagonal_black_pawn() {
        for row in 1..6 {
            for col in 1..6 {
                let bitboard = diagonal_move(row, col, BLACK);
                let lsb = bit_scan_lsb(bitboard);
                let msb = bit_scan_msb(bitboard);

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
            let lsb = bit_scan_lsb(bitboard);

            assert_eq!(lsb as i8, position_to_idx(row - 1, col + 1, None));
        }

        for row in 1..6 {
            let col = 6;
            let bitboard = diagonal_move(row, col, BLACK);
            let lsb = bit_scan_lsb(bitboard);

            assert_eq!(lsb as i8, position_to_idx(row - 1, col - 1, None));
        }
    }
}
