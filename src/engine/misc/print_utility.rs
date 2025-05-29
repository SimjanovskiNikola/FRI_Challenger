use crate::engine::board::mv_gen::BoardGenMoveTrait;
use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::moves::*;
use crate::engine::board::structures::piece::*;
use crate::engine::misc::bit_pos_utility::*;
use crate::engine::misc::const_utility::*;
use core::panic;
use std::array;

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
        None => " ".to_string(),
        Some(piece) => piece.to_figure(),
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
                print!("| {} ", chess_board[(i - 1) * 8 + (j)]);
            }
        }
        println!()
    }
    println!();
    println!();
}

pub fn print_move_list(moves: &[Move]) {
    for (idx, mv) in moves.iter().enumerate() {
        let promotion = match mv.flag {
            Flag::Promotion(piece, _) => Some(piece),
            _ => None,
        };

        println!("{}. Move: {}, (Score: {})", idx, move_notation(mv.from, mv.to, promotion), 0);
    }
}

pub fn get_move_list(moves: &[Move], depth: u8) -> String {
    let mut move_list_resp: String = String::new();
    for (idx, mv) in moves.iter().enumerate() {
        if idx >= depth as usize {
            break;
        }
        let promotion = match mv.flag {
            Flag::Promotion(piece, _) => Some(piece),
            _ => None,
        };
        move_list_resp.push_str(" ");
        move_list_resp.push_str(move_notation(mv.from, mv.to, promotion).as_str());
    }

    return move_list_resp;
}

pub fn get_pv_move_list(moves: &[Option<Move>], depth: u8) -> String {
    let mut move_list_resp: String = String::new();
    for (idx, op_mv) in moves.iter().enumerate() {
        let mv = match op_mv {
            Some(mv) => mv,
            None => break,
        };

        if idx >= depth as usize {
            break;
        }
        let promotion = match mv.flag {
            Flag::Promotion(piece, _) => Some(piece),
            _ => None,
        };
        move_list_resp.push_str(" ");
        move_list_resp.push_str(move_notation(mv.from, mv.to, promotion).as_str());
    }

    return move_list_resp;
}

pub fn move_notation(sq_from: u8, sq_to: u8, promotion: Option<Piece>) -> String {
    match promotion {
        Some(piece) => {
            let p_notation = piece.to_char();
            format!("{}{}{}", sq_notation(sq_from), sq_notation(sq_to), p_notation)
        }
        None => format!("{}{}", sq_notation(sq_from), sq_notation(sq_to)),
    }
}

pub fn sq_notation(square: u8) -> String {
    let (rank, file) = idx_to_position(square as usize, None);
    format!("{}{}", FILE_LETTERS[file], rank + 1)
}

pub fn from_move_notation(notation: &str, board: &mut Board) -> Move {
    let notation = notation.to_lowercase();
    let moves = board.gen_moves();

    for rev in &moves {
        let mv_notation =
            move_notation(rev.from, rev.to, rev.flag.get_promo_piece()).to_lowercase();
        if notation == mv_notation {
            return *rev;
        }
    }
    panic!("Something is wrong with the move: {:?}", notation);
}
