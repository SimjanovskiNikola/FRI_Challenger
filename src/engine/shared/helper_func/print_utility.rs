use std::array;

use crate::engine::{
    game::Game,
    shared::{
        helper_func::{bit_pos_utility::*, const_utility::FILE_LETTERS},
        structures::{
            piece_struct::{Color, Piece, PieceType},
            square_struct::Square,
        },
    },
};

pub fn print_bitboard(bitboard: u64, mark: Option<i8>) {
    println!(
        "Bitboard: \n------Start------\n{}-------End-------",
        bitboard_to_string(bitboard, mark)
    );
}

pub fn bitboard_to_string(bitboard: u64, mark: Option<i8>) -> String {
    let mut row = "".to_owned();
    let mut board = "".to_owned();

    for i in 0..64 {
        let value = (bitboard >> i) & 1;
        let s = if value == 0 { ". ".to_owned() } else { format!("{} ", value.to_string()) };
        match mark {
            Some(idx) => {
                if i == idx {
                    row.push_str("X");
                } else {
                    row.push_str(&s);
                }
            }
            None => row.push_str(&s),
        }

        if (i + 1) % 8 == 0 {
            row.push_str("\n");
            board.insert_str(0, &row);
            row.clear();
        }
    }
    return board;
}

pub fn print_chess(game: &Game) {
    let mut chess_board: [String; 64] = array::from_fn(|_| ".".to_string());

    for i in 0..2 {
        for bitboard in game.piece_bitboard[i] {
            for pos in extract_all_bits(bitboard) {
                match game.squares[pos] {
                    Square::Empty => continue,
                    Square::Occupied(piece) => chess_board[pos] = piece.chess_figure(),
                };
            }
        }
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
        println!("");
    }
}

pub fn sq_notation(square: usize) -> String {
    let (rank, file) = idx_to_position(square, None);
    return format!("{}{}", FILE_LETTERS[file], rank + 1);
}

pub fn move_notation(sq_from: usize, sq_to: usize, promotion: Option<Piece>) -> String {
    match promotion {
        Some(piece) => {
            let p_notation = piece.to_string();
            return format!("{}{}{}", sq_notation(sq_from), sq_notation(sq_to), p_notation.trim());
        }
        None => return format!("{}{}", sq_notation(sq_from), sq_notation(sq_to)),
    }
}

pub fn print_move_list(game: &Game) {
    for (idx, mv) in game.moves.iter().enumerate() {
        println!("{}. Move: {}, (Score: {})", idx, move_notation(mv.from, mv.to, mv.promotion), 0);
    }
}

pub fn split_on(s: &str, sep: char) -> (&str, &str) {
    for (i, item) in s.chars().enumerate() {
        if item == sep {
            return (&s[0..i], &s[i + 1..]);
        }
    }
    return (&s[..], "");
}
