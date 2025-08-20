use crate::engine::board::mv_gen::BoardGenMoveTrait;
use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::moves::Flag;
use crate::engine::board::structures::moves::Move;
use crate::engine::board::structures::piece::Piece;
use crate::engine::board::structures::piece::PieceTrait;
use crate::engine::misc::bit_pos_utility::idx_to_position;
use crate::engine::misc::const_utility::FILE_LETTERS;

pub fn print_move_list(moves: &[(Move, isize)]) {
    for (idx, (mv, _)) in moves.iter().enumerate() {
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
        // FIXME: When refactoring decide if this should be removed
        // NOTE: By Doing that you enable PV also from quiescence search
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

    for (rev, _) in &moves {
        let mv_notation =
            move_notation(rev.from, rev.to, rev.flag.get_promo_piece()).to_lowercase();
        if notation == mv_notation {
            return *rev;
        }
    }
    panic!("Something is wrong with the move: {:?}", notation);
}
