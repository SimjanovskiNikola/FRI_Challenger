use core::panic;
use std::usize;

use crate::{
    engine::shared::helper_func::bit_pos_utility::position_to_bit,
    engine::shared::structures::castling_struct::CastlingRights,
    engine::shared::structures::piece::*, engine::shared::structures::square::*,
};

use super::shared::helper_func::bitboard::{Bitboard, BitboardTrait};
use super::shared::helper_func::const_utility::FEN_START;
use super::shared::helper_func::print_utility::*;
use super::shared::structures::color::{Color, BLACK, WHITE};
use super::shared::structures::internal_move::InternalMove;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    pub squares: [Square; 64],
    pub occupancy: [Bitboard; 2],
    pub bitboard: [Bitboard; 14],
    pub active_color: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Bitboard>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,

    pub moves: Vec<InternalMove>,
    // TODO: Include the attacked squares !!!
}

impl Game {
    pub fn initialize() -> Game {
        return Game::read_fen(FEN_START);
    }

    pub fn to_string(&self) -> String {
        let mut board = "".to_owned();
        let mut temp = "".to_owned();
        for (i, square) in self.squares.iter().enumerate() {
            match square {
                Square::Empty => temp.push_str(". "), //(&index_to_position(i)),
                Square::Occupied(piece) => temp.push_str(&piece.to_string()),
            }

            if (i + 1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear();
            }
        }
        return board;
    }

    pub fn reset_board(&mut self) {
        self.squares = [Square::Empty; 64];
        self.occupancy = [0 as Bitboard; 2];
        // FIXME: Rename it to pieces and use 2D Array
        self.bitboard = [0 as Bitboard; 14];
        self.active_color = WHITE;
        self.castling_rights = CastlingRights::NONE;
        self.en_passant = None;
        self.halfmove_clock = 0;
        self.fullmove_number = 1;
        self.moves = vec![];
    }

    //TODO: Move everything for the fen reading into another file that will be an extension for this one

    //TODO: Add the attacked square everytime the pieces upadate

    // NOTE: Reads Fen String
    pub fn read_fen(fen: &str) -> Game {
        let mut game: Game = Game {
            squares: [Square::Empty; 64],
            occupancy: [0 as Bitboard; 2],
            bitboard: [0 as Bitboard; 14],
            active_color: WHITE,
            castling_rights: CastlingRights::NONE,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            moves: vec![],
        };

        // Position
        let (position, rest) = split_on(fen, ' ');
        Game::set_position(&mut game, position);

        // Active Color
        let (active_color, rest) = split_on(rest, ' ');
        Game::set_active_color(&mut game, active_color);

        // Castling Rights
        let (castling_rights, rest) = split_on(rest, ' ');
        Game::set_castling_rights(&mut game, castling_rights);

        // En Passant
        let (en_passant, rest) = split_on(rest, ' ');
        Game::set_en_passant(&mut game, en_passant);

        // Half Move Clock
        let (halfmove_clock, rest) = split_on(rest, ' ');
        Game::set_halfmove_clock(&mut game, halfmove_clock);

        // Full Move Clock
        let (fullmove_number, _) = split_on(rest, ' ');
        Game::set_fullmove_number(&mut game, fullmove_number);

        return game;
    }

    // NOTE: Sets the position of the Fen String
    pub fn set_position(game: &mut Game, position: &str) {
        // TEST: MAYBE Here is reverset, maybe the first index is 63
        let mut idx: usize = 64;
        for row in position.splitn(8, '/') {
            for ch in row.chars().rev() {
                idx -= 1;
                let mut piece: Option<Piece> = None;
                match ch {
                    'p' => piece = Some(BLACK_PAWN),
                    'n' => piece = Some(BLACK_KNIGHT),
                    'b' => piece = Some(BLACK_BISHOP),
                    'r' => piece = Some(BLACK_ROOK),
                    'q' => piece = Some(BLACK_QUEEN),
                    'k' => piece = Some(BLACK_KING),
                    'P' => piece = Some(WHITE_PAWN),
                    'N' => piece = Some(WHITE_KNIGHT),
                    'B' => piece = Some(WHITE_BISHOP),
                    'R' => piece = Some(WHITE_ROOK),
                    'Q' => piece = Some(WHITE_QUEEN),
                    'K' => piece = Some(WHITE_KING),
                    '1' => idx -= 0,
                    '2' => idx -= 1,
                    '3' => idx -= 2,
                    '4' => idx -= 3,
                    '5' => idx -= 4,
                    '6' => idx -= 5,
                    '7' => idx -= 6,
                    '8' => idx -= 7,
                    c => panic!("Invalid Character: {c}"),
                };
                match piece {
                    Some(piece) => {
                        game.bitboard[piece.idx()].set_bit(idx);
                        game.squares[idx] = Square::Occupied(piece);
                    }
                    _ => (),
                }
            }
        }

        Game::set_occupancy(game, WHITE);
        Game::set_occupancy(game, BLACK);
    }

    pub fn set_occupancy(&mut self, color: Color) {
        self.occupancy[color as usize] = 0;
        for idx in ((color as usize)..self.bitboard.len()).step_by(2) {
            self.occupancy[color as usize].union(self.bitboard[idx]);
        }
    }

    // NOTE: Sets the active color of the Fen String
    pub fn set_active_color(game: &mut Game, active_color: &str) {
        game.active_color = match active_color {
            "w" => WHITE,
            "b" => BLACK,
            _ => panic!("Unknown color: {}", active_color),
        };
    }

    // NOTE: Sets the castling rights of the Fen String
    pub fn set_castling_rights(game: &mut Game, castling_rights: &str) {
        for ch in castling_rights.chars() {
            match ch {
                'K' => game.castling_rights |= CastlingRights::WKINGSIDE,
                'Q' => game.castling_rights |= CastlingRights::WQUEENSIDE,
                'k' => game.castling_rights |= CastlingRights::BKINGSIDE,
                'q' => game.castling_rights |= CastlingRights::BQUEENSIDE,
                '-' => (),
                _ => panic!("Unknown Castling Rights: {}", ch),
            }
        }
    }

    // NOTE: Sets the en passant square of the Fen String
    pub fn set_en_passant(game: &mut Game, en_passant: &str) {
        match en_passant {
            "-" => game.en_passant = None,
            s => match position_to_bit(s) {
                Err(msg) => panic!("{}", msg),
                Ok(bit) => game.en_passant = Some(bit),
            },
        }
    }

    // NOTE: Sets the half move clock of the Fen String
    pub fn set_halfmove_clock(game: &mut Game, halfmove_clock: &str) {
        match halfmove_clock.parse() {
            Ok(number) => game.halfmove_clock = number,
            Err(_) => panic!("Invalid halfmove: {}", halfmove_clock),
        }
    }

    // NOTE: Sets the full move number of the Fen String
    pub fn set_fullmove_number(game: &mut Game, fullmove_number: &str) {
        match fullmove_number.parse() {
            Ok(number) => game.fullmove_number = number,
            Err(_) => panic!("Invalid halfmove: {}", fullmove_number),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::shared::helper_func::const_utility::FEN_MIDDLE_GAME;

    use super::*;

    #[test]
    fn initialize_works() {
        let game = Game::initialize();
        // TODO: Add Square Assertion
        // TODO: Add Piece Assertion
        assert_eq!(game.active_color, WHITE);
        assert_eq!(game.castling_rights.as_usize(), CastlingRights::ALL.as_usize()); //FIXME: The casteling rights are not summed together
        assert_eq!(game.en_passant, None);
        assert_eq!(game.halfmove_clock, 0);
        assert_eq!(game.fullmove_number, 1);

        game.bitboard[WHITE_PAWN.idx()].print(None)
    }

    #[test]
    fn test_fen_middle_game() {
        let game = Game::read_fen(FEN_MIDDLE_GAME);
        assert_eq!(game.active_color, WHITE);
        assert_eq!(game.castling_rights.as_usize(), CastlingRights::ALL.as_usize()); //FIXME: The casteling rights are not summed together
        assert_eq!(game.en_passant, None);
        assert_eq!(game.halfmove_clock, 0);
        assert_eq!(game.fullmove_number, 1);

        game.bitboard[WHITE_PAWN.idx()].print(None);
    }

    #[test]
    fn test_occupancy_start_position() {
        let game = Game::initialize();
        let mut white_occupancy = 0;
        let mut black_occupancy = 0;

        for i in 0..16 {
            white_occupancy |= 1 << i;
        }

        for i in 48..64 {
            black_occupancy |= 1 << i;
        }

        assert_eq!(game.occupancy[WHITE.idx()], white_occupancy);
        assert_eq!(game.occupancy[BLACK.idx()], black_occupancy);

        println!("{}", bitboard_to_string(black_occupancy, None))
    }

    // #[test]
    // fn test_move_piece() {
    //     let mut game = Game::initialize();
    //     println!("{}", game.to_string());
    //     game.move_peace(1 << 0, 16);
    //     println!("{}", game.to_string());

    //     assert_eq!(game.pieces[24].position, 1 << 16);
    //     assert_eq!(game.squares[0].square_type, SquareType::Empty);
    //     assert_eq!(game.squares[16].square_type, SquareType::Occupied(24));
    //     assert_eq!(game.en_passant, None);
    // }

    // #[test]
    // fn test_en_passant_is_set() {
    //     let mut game = Game::initialize();
    //     println!("{}", game.to_string());
    //     game.move_peace(1 << 12, 36);
    //     println!("{}", game.to_string());

    //     game.move_peace(1 << 51, 35);
    //     println!("{}", game.to_string());

    //     assert_eq!(game.en_passant, Some(1 << 43));
    // }
}
