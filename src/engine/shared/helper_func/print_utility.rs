use std::array;

use crate::engine::game::*;
use crate::engine::shared::helper_func::bit_pos_utility::*;
use crate::engine::shared::helper_func::const_utility::*;
use crate::engine::shared::structures::internal_move::*;
use crate::engine::shared::structures::piece::*;
use crate::engine::shared::structures::square::*;

//DEPRECATE:
pub fn print_bitboard(bitboard: u64, mark: Option<i8>) {
    println!(
        "Bitboard: \n------Start------\n{}-------End-------",
        bitboard_to_string(bitboard, mark)
    );
}

//DEPRECATE:
pub fn bitboard_to_string(bitboard: u64, mark: Option<i8>) -> String {
    let mut row = "".to_owned();
    let mut board = "".to_owned();

    for i in 0..64 {
        let value = (bitboard >> i) & 1;
        let s = if value == 0 { ".".to_owned() } else { value.to_string() };
        match mark {
            Some(idx) => {
                if i == idx {
                    row.push('X');
                } else {
                    row.push_str(&s);
                }
            }
            None => row.push_str(&s),
        }

        if (i + 1) % 8 == 0 {
            row.push('\n');
            board.insert_str(0, &row);
            row.clear();
        }
    }

    board
}

pub fn print_chess(game: &Game) {
    let mut chess_board: [String; 64] = array::from_fn(|_| ".".to_string());

    for (idx, sq) in game.squares.iter().enumerate() {
        match sq {
            Square::Empty => continue,
            Square::Occupied(piece) => chess_board[idx] = piece.to_figure(),
        };
    }

    for i in (0..9).rev() {
        if i == 0 {
            println!("-------------------");
        }
        for j in 0..9 {
            if i == 0 && j == 0 {
                print!("   ");
            } else if i == 0 {
                print!("{}|", FILE_LETTERS[j - 1]);
            } else if j == 0 {
                print!("{} |", i);
            } else {
                print!("{} ", chess_board[(i - 1) * 8 + (j - 1)]);
            }
        }
        println!();
    }
}

pub fn sq_notation(square: usize) -> String {
    let (rank, file) = idx_to_position(square, None);
    format!("{}{}", FILE_LETTERS[file], rank + 1)
}

pub fn move_notation(sq_from: usize, sq_to: usize, promotion: Option<Piece>) -> String {
    match promotion {
        Some(piece) => {
            let p_notation = piece.to_string();
            format!("{}{}{}", sq_notation(sq_from), sq_notation(sq_to), p_notation.trim())
        }
        None => format!("{}{}", sq_notation(sq_from), sq_notation(sq_to)),
    }
}

pub fn print_move_list(moves: &[InternalMove]) {
    for (idx, mv) in moves.iter().enumerate() {
        let promotion = match mv.flag {
            Flag::Promotion(_, cap_piece) => cap_piece,
            _ => None,
        };

        println!("{}. Move: {}, (Score: {})", idx, move_notation(mv.from, mv.to, promotion), 0);
    }
}

// NOTE: IMPROVEMENTS
// Make the board look better like this:

//   +---+---+---+---+---+---+---+---+
//   | r | n | b | q | k | b | n | r | 8
//   +---+---+---+---+---+---+---+---+
//   | p | p | p | p | p | p | p | p | 7
//   +---+---+---+---+---+---+---+---+
//   |   |   |   |   |   |   |   |   | 6
//   +---+---+---+---+---+---+---+---+
//   |   |   |   |   |   |   |   |   | 5
//   +---+---+---+---+---+---+---+---+
//   |   |   |   |   | P |   |   |   | 4
//   +---+---+---+---+---+---+---+---+
//   |   |   |   |   |   |   |   |   | 3
//   +---+---+---+---+---+---+---+---+
//   | P | P | P | P |   | P | P | P | 2
//   +---+---+---+---+---+---+---+---+
//   | R | N | B | Q | K | B | N | R | 1
//   +---+---+---+---+---+---+---+---+
//     a   b   c   d   e   f   g   h
