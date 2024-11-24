use crate::engine::{
    attacks::knight_attacks::*,
    shared::{ helper_func::utils::*, structures::piece_struct::PieceColor },
};

pub struct PawnAttacks {
    white_forward_moves: Vec<u64>,
    white_diagonal_moves: Vec<u64>,
    black_forward_moves: Vec<u64>,
    black_diagonal_moves: Vec<u64>,
}

impl PawnAttacks {
    fn initialize() -> Self {
        let mut w_forward = vec![];
        let mut w_diagonal = vec![];
        let mut b_forward = vec![];
        let mut b_diagonal = vec![];

        for row in 0..8 {
            for col in 0..8 {
                let f = forward_move(row, col, PieceColor::White);
                let d = diagonal_move(row, col, PieceColor::White);
                w_forward.push(f);
                w_diagonal.push(d);

                let f = forward_move(row, col, PieceColor::Black);
                let d = diagonal_move(row, col, PieceColor::Black);
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

fn forward_move(row: i64, col: i64, piece_color: PieceColor) -> u64 {
    if row == 0 || row == 7 {
        return 0;
    }

    let mut bitboard = 0;
    if piece_color == PieceColor::White {
        if row < 7 {
            bitboard |= set_bit(bitboard, (row, col), (1, 0));
        }
        if row == 1 {
            bitboard |= set_bit(bitboard, (row, col), (2, 0));
        }
    } else {
        if row > 0 {
            bitboard |= set_bit(bitboard, (row, col), (-1, 0));
        }
        if row == 6 {
            bitboard |= set_bit(bitboard, (row, col), (-2, 0));
        }
    }
    return bitboard;
}

fn diagonal_move(row: i64, col: i64, piece_color: PieceColor) -> u64 {
    if row == 0 || row == 7 {
        return 0;
    }

    let mut bitboard = 0;
    if piece_color == PieceColor::White {
        if row < 7 {
            bitboard |= set_bit(bitboard, (row, col), (1, 1));
            bitboard |= set_bit(bitboard, (row, col), (1, -1));
        }
    } else {
        if row > 0 {
            bitboard |= set_bit(bitboard, (row, col), (-1, 1));
            bitboard |= set_bit(bitboard, (row, col), (-1, -1));
        }
    }
    return bitboard;
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    #[test]
    fn test_pawn_attacks_initialize() {
        let attacks = PawnAttacks::initialize();
    }

    #[test]
    fn test_second_row_white_pawn() {
        let row: usize = 1;
        for col in 0..8 {
            let col: usize = col;
            let bitboard = forward_move(row as i64, col as i64, PieceColor::White);
            let lsb = bit_scan(bitboard);
            let msb = bit_scan_backward(bitboard);
            assert_eq!(lsb, position_to_idx(row + 1, col));
            assert_eq!(msb, position_to_idx(row + 2, col));
        }
    }

    #[test]
    fn test_second_row_black_pawn() {
        let row: usize = 1;
        for col in 0..8 {
            let col: usize = col;
            let bitboard = forward_move(row as i64, col as i64, PieceColor::Black);
            let lsb = bit_scan(bitboard);
            assert_eq!(lsb, position_to_idx(row - 1, col));
        }
    }

    #[test]
    fn test_seventh_row_black_pawn() {
        let row: usize = 6;
        for col in 0..8 {
            let col: usize = col;
            let bitboard = forward_move(row as i64, col as i64, PieceColor::Black);
            let lsb = bit_scan(bitboard);
            let msb = bit_scan_backward(bitboard);
            assert_eq!(msb, position_to_idx(row - 1, col));
            assert_eq!(lsb, position_to_idx(row - 2, col));
        }
    }

    #[test]
    fn test_seventh_row_white_pawn() {
        let row: usize = 6;
        for col in 0..8 {
            let bitboard = forward_move(row as i64, col as i64, PieceColor::White);
            let lsb = bit_scan(bitboard);
            assert_eq!(lsb, position_to_idx(row + 1, col));
        }
    }

    #[test]
    fn test_middle_row_white_pawn() {
        for row in 2..7 {
            for col in 0..8 {
                let bitboard = forward_move(row as i64, col as i64, PieceColor::White);
                let lsb = bit_scan(bitboard);
                assert_eq!(lsb, position_to_idx(row + 1, col));
            }
        }
    }

    #[test]
    fn test_middle_row_black_pawn() {
        for row in 1..6 {
            for col in 0..8 {
                let bitboard = forward_move(row as i64, col as i64, PieceColor::Black);
                let lsb = bit_scan(bitboard);
                assert_eq!(lsb, position_to_idx(row - 1, col));
            }
        }
    }

    #[test]
    fn test_forward_edges_pawn_attacks() {
        for color in [PieceColor::White, PieceColor::Black] {
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
        for color in [PieceColor::White, PieceColor::Black] {
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
                let bitboard = diagonal_move(row as i64, col as i64, PieceColor::White);
                let lsb = bit_scan(bitboard);
                let msb = bit_scan_backward(bitboard);

                assert_eq!(lsb, position_to_idx(row + 1, col - 1));
                assert_eq!(msb, position_to_idx(row + 1, col + 1));
            }
        }
    }

    #[test]
    fn test_diagonal_white_pawn_col_edge() {
        for row in 1..6 {
            let col = 0;
            let bitboard = diagonal_move(row as i64, col as i64, PieceColor::White);
            let lsb = bit_scan(bitboard);

            assert_eq!(lsb, position_to_idx(row + 1, col + 1));
        }

        for row in 1..6 {
            let col = 6;
            let bitboard = diagonal_move(row as i64, col as i64, PieceColor::White);
            let lsb = bit_scan(bitboard);

            assert_eq!(lsb, position_to_idx(row + 1, col - 1));
        }
    }

    #[test]
    fn test_diagonal_black_pawn() {
        for row in 1..6 {
            for col in 1..6 {
                let bitboard = diagonal_move(row as i64, col as i64, PieceColor::Black);
                let lsb = bit_scan(bitboard);
                let msb = bit_scan_backward(bitboard);

                assert_eq!(lsb, position_to_idx(row - 1, col - 1));
                assert_eq!(msb, position_to_idx(row - 1, col + 1));
            }
        }
    }

    #[test]
    fn test_diagonal_black_pawn_col_edge() {
        for row in 1..6 {
            let col = 0;
            let bitboard = diagonal_move(row as i64, col as i64, PieceColor::Black);
            let lsb = bit_scan(bitboard);

            assert_eq!(lsb, position_to_idx(row - 1, col + 1));
        }

        for row in 1..6 {
            let col = 6;
            let bitboard = diagonal_move(row as i64, col as i64, PieceColor::Black);
            let lsb = bit_scan(bitboard);

            assert_eq!(lsb, position_to_idx(row - 1, col - 1));
        }
    }
}
