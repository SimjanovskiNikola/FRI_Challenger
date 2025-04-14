use crate::engine::game::Game;
use crate::engine::move_generation::make_move::GameMoveTrait;
use crate::engine::shared::helper_func::bit_pos_utility::*;
use crate::engine::shared::helper_func::bitboard::BitboardTrait;
use crate::engine::shared::structures::castling_struct::*;
use crate::engine::shared::structures::color::*;
use crate::engine::shared::structures::piece::*;

// TODO: Validate if the fen is correct

pub trait FenTrait {
    fn read_fen(fen: &str) -> Self;
    fn set_position(&mut self, position: &str);
    fn set_en_passant(&mut self, square: &str);
    fn set_color(&mut self, color: &str);
    fn set_castling(&mut self, castling: &str);
    fn set_half_move_clock(&mut self, half_move: &str);
    fn set_full_move_number(&mut self, full_move: &str);
}

impl FenTrait for Game {
    fn read_fen(fen: &str) -> Self {
        let mut game: Game = Game::create_board();
        let data: Vec<&str> = fen.split(" ").collect();

        if data.len() != 6 {
            panic!("Something is wrong with the fen string");
        }

        Game::set_position(&mut game, data[0]);
        Game::set_color(&mut game, data[1]);
        Game::set_castling(&mut game, data[2]);
        Game::set_en_passant(&mut game, data[3]);
        Game::set_half_move_clock(&mut game, data[4]);
        Game::set_full_move_number(&mut game, data[5]);

        game.generate_pos_key();

        game
    }

    fn set_position(&mut self, position: &str) {
        let mut idx: usize = 64;
        for row in position.splitn(8, '/') {
            for ch in row.chars().rev() {
                match ch {
                    '1'..='8' => idx -= ch.to_digit(10).unwrap() as usize,
                    'p' | 'n' | 'b' | 'r' | 'q' | 'k' | 'P' | 'N' | 'B' | 'R' | 'Q' | 'K' => {
                        self.add_piece(idx - 1, Piece::from_char(ch));
                        idx -= 1;
                    }
                    _ => panic!("Invalid Character: {ch}"),
                };
            }
        }
    }

    fn set_color(&mut self, color: &str) {
        self.color = match color {
            "w" => WHITE,
            "b" => BLACK,
            _ => panic!("Unknown color: {}", color),
        };
    }

    fn set_en_passant(&mut self, square: &str) {
        self.ep = match square {
            "-" => None,
            s => match position_to_bit(s) {
                Ok(bit) => Some(bit.get_lsb() as u8),
                Err(e) => panic!("Unknown En Passant Position: {}", e),
            },
        }
    }

    fn set_castling(&mut self, castling: &str) {
        for ch in castling.chars() {
            match ch {
                'K' => self.castling.add(CastlingRights::WKINGSIDE),
                'Q' => self.castling.add(CastlingRights::WQUEENSIDE),
                'k' => self.castling.add(CastlingRights::BKINGSIDE),
                'q' => self.castling.add(CastlingRights::BQUEENSIDE),
                '-' => (),
                _ => panic!("Unknown Castling Rights: {}", ch),
            }
        }
    }

    fn set_half_move_clock(&mut self, half_move: &str) {
        self.half_move = match half_move.parse() {
            Ok(number) => number,
            Err(_) => panic!("Invalid halfmove: {}", half_move),
        }
    }

    fn set_full_move_number(&mut self, full_move: &str) {
        self.full_move = match full_move.parse() {
            Ok(number) => number,
            Err(_) => panic!("Invalid fullmove: {}", full_move),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::shared::helper_func::bitboard::BitboardTrait;
    use crate::engine::shared::helper_func::const_utility::*;
    use crate::engine::shared::structures::square::SqPos;

    use super::*;

    #[test]
    fn initialize_works() {
        let game = Game::initialize();
        assert_eq!(game.color, WHITE);
        assert_eq!(game.castling, CastlingRights::ALL);
        assert_eq!(game.ep, None);
        assert_eq!(game.half_move, 0);
        assert_eq!(game.full_move, 1);

        game.bitboard[WHITE_PAWN.idx()].print(None)
    }

    #[test]
    fn test_fen_middle_game() {
        let game = Game::read_fen(FEN_MIDDLE_GAME);
        assert_eq!(game.color, WHITE);
        assert_eq!(game.castling, CastlingRights::ALL);
        assert_eq!(game.ep, None);
        assert_eq!(game.half_move, 0);
        assert_eq!(game.full_move, 1);

        game.bitboard[WHITE_PAWN.idx()].print(None);
    }

    #[test]
    fn test_fen_pawns_black_game() {
        let game = Game::read_fen(FEN_PAWNS_BLACK);
        assert_eq!(game.color, BLACK);
        assert_eq!(game.castling, CastlingRights::ALL);
        assert_eq!(game.ep, Some(SqPos::E3 as u8));
        assert_eq!(game.half_move, 0);
        assert_eq!(game.full_move, 1);

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

        assert_eq!(game.bitboard(WHITE), white_occupancy);
        assert_eq!(game.bitboard(BLACK), black_occupancy);
    }
}
