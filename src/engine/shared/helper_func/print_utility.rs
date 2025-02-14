use std::array;

use crate::engine::game::*;
use crate::engine::shared::helper_func::bit_pos_utility::*;
use crate::engine::shared::helper_func::const_utility::*;
use crate::engine::shared::structures::internal_move::*;
use crate::engine::shared::structures::piece::*;
use crate::engine::shared::structures::square::*;

pub fn print_bitboard(bitboard: u64, mark: Option<i8>) {
    let chess_board: [String; 64] = array::from_fn(|idx| {
        if matches!(mark, Some(x) if (x as usize) == idx) {
            return "X".to_string();
        }

        return match (bitboard >> idx) & 1 {
            1 => "O".to_string(),
            _ => " ".to_string(),
        };
    });

    print_board(&chess_board);
}

pub fn print_chess(game: &Game) {
    let chess_board: [String; 64] = array::from_fn(|idx| match game.squares[idx] {
        Square::Empty => " ".to_string(),
        Square::Occupied(piece) => piece.to_figure(),
    });

    print_board(&chess_board);
}

pub fn print_board(chess_board: &[String; 64]) {
    for i in (0..9).rev() {
        println!("+---+---+---+---+---+---+---+---+");

        for j in 0..9 {
            if j == 8 {
                if i != 0 {
                    print!(" {}", i);
                }
            } else if i == 0 {
                print!("  {} ", FILE_LETTERS[j]);
            } else {
                if j == 7 {
                    print!("| {} |", chess_board[(i - 1) * 8 + (j)]);
                } else {
                    print!("| {} ", chess_board[(i - 1) * 8 + (j)]);
                }
            }
        }
        println!()
    }
    println!();
    println!();
}

pub fn print_move_list(moves: &[InternalMove]) {
    for (idx, mv) in moves.iter().enumerate() {
        let promotion = match mv.flag {
            Flag::Promotion(promo_piece, _) => Some(promo_piece),
            _ => None,
        };

        println!("{}. Move: {}, (Score: {})", idx, move_notation(mv.from, mv.to, promotion), 0);
    }
}

pub fn move_notation(sq_from: usize, sq_to: usize, promotion: Option<Piece>) -> String {
    match promotion {
        Some(piece) => {
            let p_notation = piece.to_char();
            format!("{}{}{}", sq_notation(sq_from), sq_notation(sq_to), p_notation)
        }
        None => format!("{}{}", sq_notation(sq_from), sq_notation(sq_to)),
    }
}

pub fn sq_notation(square: usize) -> String {
    let (rank, file) = idx_to_position(square, None);
    format!("{}{}", FILE_LETTERS[file], rank + 1)
}
