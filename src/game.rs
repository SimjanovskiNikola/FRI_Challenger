use core::panic;
use std::usize;
use std::collections::VecDeque;

use crate::{
    utils::{index_to_position, position_to_bit, split_on},
    castling::CastlingRights,
    piece::{Piece, PieceColor, PieceType},
    square::{Square, SquareType},
};

pub type PiecePosition = u64;

pub struct Game {
    pub pieces: Vec<Piece>,
    pub squares: Vec<Square>,
    pub active_color: PieceColor,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<PiecePosition>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,
}

impl Game {
    pub fn initialize() -> Game {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        return Game::read_fen(fen_str);
    }

    pub fn to_string(&self) -> String {
        let mut board = "".to_owned();
        let mut temp = "".to_owned();
        for (i, square) in self.squares.iter().enumerate() {
            match square.square_type {
                SquareType::Empty => temp.push_str(&index_to_position(i)),
                SquareType::Occupied(idx) => temp.push_str(&self.pieces[idx].to_string()),
            }

            if (i + 1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear();
            }
        }
        return board;
    }

    pub fn read_fen(fen: &str) -> Game {
        let mut game: Game = Game {
            pieces: vec![],
            squares: vec![],
            active_color: PieceColor::White,
            castling_rights: CastlingRights::ALL,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };

        let (position, rest) = split_on(fen, ' ');
        let mut deque_squares = VecDeque::new();
        let mut piece_index = 0;
        let mut piece_position = 64;

        for row in position.splitn(8, '/') {
            piece_position -= 8;
            let (pieces, sqares) = Self::parse_row(&row, piece_index, piece_position);
            for p in pieces {
                game.pieces.push(p);
                piece_index += 1;
            }
            for s in sqares {
                deque_squares.push_front(s);
            }
        }
        game.squares = Vec::from(deque_squares);

        // COLOR
        let (color, rest) = split_on(rest, ' ');
        game.active_color = match color {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _ => panic!("Unknown color: {}", color),
        };

        // CASTLING RIGHTS
        let (castling_rights, rest) = split_on(rest, ' ');
        let mut castling: CastlingRights = CastlingRights::NONE;
        for ch in castling_rights.chars() {
            match ch {
                'K' => {
                    castling |= CastlingRights::WKINGSIDE;
                }
                'Q' => {
                    castling |= CastlingRights::WKINGSIDE;
                }
                'k' => {
                    castling |= CastlingRights::BKINGSIDE;
                }
                'q' => {
                    castling |= CastlingRights::BQUEENSIDE;
                }
                '-' => (),
                _ => panic!("Unknown Castling Rights: {}", ch),
            }
        }
        game.castling_rights = castling;

        // EnPassant
        let (en_passant, rest) = split_on(rest, ' ');
        match en_passant {
            "-" => {
                game.en_passant = None;
            }
            s => match position_to_bit(s) {
                Err(msg) => panic!("{}", msg),
                Ok(bit) => {
                    game.en_passant = Some(bit);
                }
            },
        }
        // halfmove_clock
        let (halfmove_clock, rest) = split_on(rest, ' ');
        match halfmove_clock.parse() {
            Ok(number) => {
                game.halfmove_clock = number;
            }
            Err(_) => panic!("Invalid halfmove: {}", halfmove_clock),
        }
        // fullmove_number
        let (fullmove_number, _) = split_on(rest, ' ');
        match fullmove_number.parse() {
            Ok(number) => {
                game.fullmove_number = number;
            }
            Err(_) => panic!("Invalid halfmove: {}", fullmove_number),
        }

        return game;
    }

    pub fn parse_row(
        row: &str,
        mut piece_index: usize,
        mut piece_position: usize,
    ) -> (Vec<Piece>, Vec<Square>) {
        let mut pieces = Vec::new();
        let mut squares = VecDeque::new();

        let mut piece_color: PieceColor;
        let mut piece_type: PieceType;

        for ch in row.chars() {
            let is_upper = ch.is_ascii_uppercase();
            piece_color = if is_upper { PieceColor::White } else { PieceColor::Black };
            piece_type = match ch.to_ascii_lowercase() {
                'p' => PieceType::Pawn,
                'r' => PieceType::Rook,
                'n' => PieceType::Knight,
                'b' => PieceType::Bishop,
                'q' => PieceType::Queen,
                'k' => PieceType::King,
                _ => PieceType::Pawn, //FIXME: This is error, it should be null
            };

            if ch.is_digit(10) {
                match ch.to_digit(10) {
                    None => panic!("Invalid input: {}", ch),
                    Some(ch) => {
                        for _ in 0..ch {
                            squares.push_front(Square { square_type: SquareType::Empty });
                            piece_position += 1;
                        }
                    }
                }
                continue;
            }

            let piece = Piece {
                position: (1 as u64) << piece_position,
                piece_color: piece_color,
                piece_type: piece_type,
            };
            let square = Square { square_type: SquareType::Occupied(piece_index) };
            pieces.push(piece);
            squares.push_front(square);
            piece_position += 1;
            piece_index += 1;
        }
        return (pieces, Vec::from(squares));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_works() {
        let game = Game::initialize();
        // TODO: Add Square Assertion
        // TODO: Add Piece Assertion
        assert_eq!(game.active_color, PieceColor::White);
        assert_eq!(game.castling_rights, CastlingRights::ALL);
        assert_eq!(game.en_passant, None);
        assert_eq!(game.halfmove_clock, 1);
        assert_eq!(game.fullmove_number, 0);
    }
}
