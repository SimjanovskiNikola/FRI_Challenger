use std::usize;
use crate::engine::shared::structures::castling_struct::CastlingRights;
use crate::engine::shared::structures::square::*;
use super::move_generation::fen::FenTrait;
use super::shared::helper_func::bitboard::*;
use super::shared::helper_func::const_utility::*;
use super::shared::structures::color::*;
use super::shared::structures::internal_move::InternalMove;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    pub squares: [Square; 64],
    pub occupancy: [Bitboard; 2],
    pub bitboard: [Bitboard; 14],
    pub color: Color,
    pub castling: CastlingRights,
    pub ep: Option<Bitboard>,
    pub half_move: usize,
    pub full_move: usize,
    pub pos_key: u64,

    // pub moves: Vec<InternalMove>,
    pub moves: [Option<InternalMove>; 2048],
    pub mv_idx: usize,
}

impl Game {
    pub fn initialize() -> Game {
        return Game::read_fen(FEN_START);
    }

    pub fn create_board() -> Self {
        return Self {
            squares: [Square::Empty; 64],
            occupancy: [0 as Bitboard; 2],
            bitboard: [0 as Bitboard; 14],
            color: WHITE,
            castling: CastlingRights::NONE,
            ep: None,
            half_move: 0,
            full_move: 1,
            pos_key: 0,

            moves: [None; 2048],
            mv_idx: 0,
        };
    }

    pub fn reset_board(&mut self) {
        self.squares = [Square::Empty; 64];
        self.occupancy = [0 as Bitboard; 2];
        self.bitboard = [0 as Bitboard; 14];
        self.color = WHITE;
        self.castling = CastlingRights::NONE;
        self.ep = None;
        self.half_move = 0;
        self.full_move = 1;
        self.moves = [None; 2048];
        self.mv_idx = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_board() {
        let mut game = Game::initialize();
        game.reset_board();

        assert_eq!(game.squares, [Square::Empty; 64]);
        assert_eq!(game.bitboard, [0; 14]);
        assert_eq!(game.occupancy, [0; 2]);
        assert_eq!(game.color, WHITE);
        assert_eq!(game.castling, CastlingRights::NONE);
        assert_eq!(game.ep, None);
        assert_eq!(game.half_move, 0);
        assert_eq!(game.full_move, 1);
        assert_eq!(game.mv_idx, 0);
    }
}
