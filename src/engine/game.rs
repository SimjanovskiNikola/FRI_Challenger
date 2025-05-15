use super::fen::fen::FenTrait;
use super::search::searcher::SearchInfo;
use super::search::transposition_table::TTTable;
use super::shared::helper_func::bitboard::*;
use super::shared::helper_func::const_utility::*;
use super::shared::structures::color::*;
use super::shared::structures::internal_move::PositionIrr;
use super::shared::structures::internal_move::PositionRev;
use super::shared::structures::piece::Piece;
use crate::engine::shared::structures::castling_struct::CastlingRights;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    // Peace Occupancy (Bitboard Representation)
    pub squares: [Option<Piece>; 64],
    pub bitboard: [Bitboard; 14],

    // Position Vectors (Moves until now)
    pub pos_rev: Vec<PositionRev>,
    pub pos_irr: Vec<PositionIrr>,

    // Position Key
    pub key: u64,

    // Fen Parameters
    pub color: Color,
    pub castling: CastlingRights,
    pub ep: Option<u8>,
    pub half_move: u8,
    pub full_move: u16,

    // Moves Played from the position that is on the board.
    pub ply: usize,

    // Move Ordering Technics
    pub s_history: [[u64; 64]; 14],
    pub s_killers: [[Option<PositionRev>; 2]; 64],

    pub pv: Vec<PositionRev>,
    // Search Info and UCI commands FIXME: Split maybe in two structs
    // pub info: SearchInfo,
}

impl Game {
    pub fn initialize() -> Game {
        Game::read_fen(FEN_START)
    }

    pub fn create_board() -> Self {
        Self {
            squares: [None; 64],
            bitboard: [0 as Bitboard; 14],
            color: WHITE,
            castling: CastlingRights::NONE,
            ep: None,
            half_move: 0,
            full_move: 1,
            key: 0,

            pos_rev: Vec::with_capacity(1024),
            pos_irr: Vec::with_capacity(1024),
            s_history: [[0u64; 64]; 14],
            s_killers: [[None; 2]; 64],
            ply: 0,
            pv: Vec::new(),
            // info: SearchInfo::init(),
        }
    }

    pub fn reset_board(&mut self) {
        self.squares = [None; 64];
        self.bitboard = [0 as Bitboard; 14];
        self.color = WHITE;
        self.castling = CastlingRights::NONE;
        self.ep = None;
        self.half_move = 0;
        self.full_move = 1;
        self.pos_rev = Vec::with_capacity(1024);
        self.pos_irr = Vec::with_capacity(1024);
        // self.info = SearchInfo::init();
    }

    // pub fn mirror_board(&mut self){
    //      pub squares: [Option<Piece>; 64],
    // pub bitboard: [Bitboard; 14],

    // // Fen Parameters
    // self.color = self.color.opp();
    // pub castling: CastlingRights,
    // self.ep = match self.ep {
    //     Some(pos) => ,
    //     None => todo!(),
    // }
    // pub ep: Option<u8>,
    // pub half_move: u8,
    // pub full_move: u16,

    // // Moves Played from the position that is on the board.
    // pub ply: usize,

    // // Transposition Table
    // pub tt: TTTable,

    // // Move Ordering Technics
    // pub s_history: [[u64; 64]; 14],
    // pub s_killers: [[Option<PositionRev>; 2]; 64],

    // // Search Info and UCI commands FIXME: Split maybe in two structs
    // pub info: SearchInfo,
    // }

    #[inline(always)]
    pub fn bitboard(&self, idx: u8) -> u64 {
        self.bitboard[idx as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_board() {
        let mut game = Game::initialize();
        game.reset_board();

        assert_eq!(game.squares, [None; 64]);
        assert_eq!(game.bitboard, [0; 14]);
        assert_eq!(game.color, WHITE);
        assert_eq!(game.castling, CastlingRights::NONE);
        assert_eq!(game.ep, None);
        assert_eq!(game.half_move, 0);
        assert_eq!(game.full_move, 1);
        assert_eq!(game.pos_rev.len(), 0);
        assert_eq!(game.pos_irr.len(), 0);
    }
}
