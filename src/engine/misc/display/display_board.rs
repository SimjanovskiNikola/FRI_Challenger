use std::array;

use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::piece::PieceTrait;
use crate::engine::misc::const_utility::FILE_LETTERS;

pub fn print_bitboard(bitboard: u64, mark: Option<i8>) {
    let chess_board: [String; 64] = array::from_fn(|idx| {
        if matches!(mark, Some(x) if (x as usize) == idx) {
            return "X".to_string();
        }

        match (bitboard >> idx) & 1 {
            1 => "O".to_string(),
            _ => " ".to_string(),
        }
    });

    print_board(&chess_board);
}

pub fn print_chess(board: &Board) {
    let chess_board: [String; 64] = array::from_fn(|idx| match board.squares[idx] {
        0 => " ".to_string(),
        piece => piece.to_figure(),
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
            } else if j == 7 {
                print!("| {} |", chess_board[(i - 1) * 8 + (j)]);
            } else {
                print!("| {:^1} ", chess_board[(i - 1) * 8 + (j)]);
            }
        }
        println!()
    }
    println!();
    println!();
}

pub fn print_eval(chess_board: &[String; 64]) {
    for i in (0..9).rev() {
        println!("+-------+--------+--------+--------+--------+--------+--------+-------+");

        for j in 0..9 {
            if j == 8 {
                if i != 0 {
                    print!(" {}", i);
                }
            } else if i == 0 {
                print!("   {}   ", FILE_LETTERS[j]);
            } else if j == 7 {
                print!("| {:^4} |", chess_board[(i - 1) * 8 + (j)]);
            } else {
                print!("| {:^4} ", chess_board[(i - 1) * 8 + (j)]);
            }
        }
        println!()
    }
    println!();
    println!();
}
