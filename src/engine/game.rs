use core::panic;
use std::{ io::Empty, usize };
use std::collections::VecDeque;
use crate::engine::shared::helper_func::bit_pos_utility::bit_scan_lsb;
use crate::engine::shared::helper_func::print_utility::bitboard_to_string;

use crate::FEN_START;
use crate::{
    engine::shared::helper_func::bit_pos_utility::{ position_to_bit },
    engine::shared::structures::castling_struct::CastlingRights,
    engine::shared::structures::piece_struct::{ Piece, PieceColor, PieceType },
    engine::shared::structures::square_struct::{ Square, SquareType },
};

use super::shared::helper_func::print_utility::split_on;

pub type PiecePosition = u64;
pub type Bitboard = u64;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    pub pieces: Vec<Piece>,
    pub squares: Vec<Square>,
    pub active_color: PieceColor,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<PiecePosition>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,

    pub white_occupancy: u64,
    pub black_occupancy: u64,
}

impl Game {
    pub fn initialize() -> Game {
        return Game::read_fen(FEN_START);
    }

    pub fn to_string(&self) -> String {
        let mut board = "".to_owned();
        let mut temp = "".to_owned();
        for (i, square) in self.squares.iter().enumerate() {
            match square.square_type {
                SquareType::Empty => temp.push_str(". "), //(&index_to_position(i)),
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
            white_occupancy: 0,
            black_occupancy: 0,
        };

        let (position, rest) = split_on(fen, ' ');
        let mut deque_squares = VecDeque::new();
        let mut piece_index = 0;
        let mut piece_position = 64;

        for row in position.splitn(8, '/') {
            piece_position -= 8;
            let (pieces, sqares) = Self::parse_row(
                &mut game,
                &row,
                piece_index,
                piece_position
            );
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
                    castling |= CastlingRights::WQUEENSIDE;
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
            s =>
                match position_to_bit(s) {
                    Err(msg) => panic!("{}", msg),
                    Ok(bit) => {
                        game.en_passant = Some(bit);
                    }
                }
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

    pub fn move_peace(self: &mut Self, mut piece_position: u64, new_position: usize) {
        let square_idx = bit_scan_lsb(piece_position);
        let square = self.squares[square_idx];
        let piece_idx = match square.square_type {
            SquareType::Empty => panic!("Tried to move a piece from an empty square"),
            SquareType::Occupied(idx) => idx,
        };
        println!("{:?}", piece_idx);
        let piece = self.pieces[piece_idx];

        self.pieces[piece_idx].position = 1 << new_position;

        self.squares[square_idx].square_type = SquareType::Empty;

        match self.squares[new_position].square_type {
            SquareType::Empty => {
                self.squares[new_position].square_type = SquareType::Occupied(piece_idx);
                println!("{:?}", self.squares[new_position]);
            }
            SquareType::Occupied(other_idx) => {
                self.pieces.remove(other_idx);
                self.squares[new_position].square_type = SquareType::Occupied(piece_idx);
            }
        }
    }

    pub fn parse_row(
        &mut self,
        row: &str,
        mut piece_index: usize,
        mut piece_position: usize
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

            let bitboard = 1 << piece_position;
            match piece.piece_color {
                PieceColor::White => {
                    self.white_occupancy |= bitboard;
                }
                PieceColor::Black => {
                    self.black_occupancy |= bitboard;
                }
            }

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
        assert_eq!(game.castling_rights, CastlingRights::ALL); //FIXME: The casteling rights are not summed together
        assert_eq!(game.en_passant, None);
        assert_eq!(game.halfmove_clock, 0);
        assert_eq!(game.fullmove_number, 1);
    }

    #[test]
    fn test_occupancy_start_position() {
        let start = Game::initialize();
        let mut white_occupancy = 0;
        let mut black_occupancy = 0;

        for i in 0..16 {
            white_occupancy |= 1 << i;
        }

        for i in 48..64 {
            black_occupancy |= 1 << i;
        }

        assert_eq!(start.white_occupancy, white_occupancy);
        assert_eq!(start.black_occupancy, black_occupancy);

        println!("{}", bitboard_to_string(black_occupancy, None))
    }

    #[test]
    fn test_move_piece() {
        // FIXME: He did it right, I should look more closely why it fails
        let mut game = Game::initialize();
        println!("{}", game.to_string());
        game.move_peace(1 << 0, 16);
        println!("{}", game.to_string());

        assert_eq!(game.pieces[24].position, 1 << 16);
        assert_eq!(game.squares[0].square_type, SquareType::Empty);
        assert_eq!(game.squares[16].square_type, SquareType::Occupied(24));
    }
}
