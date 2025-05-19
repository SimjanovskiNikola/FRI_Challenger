

use crate::engine::shared::helper_func::bitboard::Bitboard;
use crate::engine::shared::helper_func::const_utility::FEN_START;
use crate::engine::shared::structures::internal_move::Move;
use crate::engine::shared::structures::piece::Piece;
use super::fen::FenTrait;
use super::state::BoardState;



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    // Peace Occupancy (Bitboard Representation)
    pub squares: [Option<Piece>; 64],
    pub bitboard: [Bitboard; 14],

    // Position Vectors (Moves until now)
    pub moves: Vec<Move>,
    pub history: Vec<BoardState>,
    pub state: BoardState,

    pub s_history: [[u64; 64]; 14],
    pub s_killers: [[Option<Move>; 2]; 64],
    pub pv: Vec<Move>,
}

impl Board {
    pub fn initialize() -> Board {
        Board::read_fen(FEN_START)
    }

    pub fn create_board() -> Self {
        Self {
            squares: [None; 64],
            bitboard: [0 as Bitboard; 14],
         
            moves: Vec::with_capacity(1024),
            history: Vec::with_capacity(1024),
            state: BoardState::init(),
            s_history: [[0u64; 64]; 14],
            s_killers: [[None; 2]; 64],
            pv: Vec::new(),
            // info: SearchInfo::init(),
        }
    }

    pub fn reset_board(&mut self) {
        self.squares = [None; 64];
        self.bitboard = [0 as Bitboard; 14];
        self.moves = Vec::with_capacity(1024);
        self.history = Vec::with_capacity(1024);
        self.state =  BoardState::init();
    }

    pub fn ply(&self) -> usize{
        self.moves.len()
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

    use crate::engine::shared::structures::castling_struct::CastlingRights;
    use crate::engine::shared::structures::color::WHITE;

    use super::*;

    #[test]
    fn test_reset_board() {
        let mut board = Board::initialize();
        board.reset_board();

        assert_eq!(board.squares, [None; 64]);
        assert_eq!(board.bitboard, [0; 14]);
        assert_eq!(board.state.color, WHITE);
        assert_eq!(board.state.castling, CastlingRights::NONE);
        assert_eq!(board.state.ep, None);
        assert_eq!(board.state.half_move, 0);
        assert_eq!(board.state.full_move, 1);
        assert_eq!(board.moves.len(), 0);
        assert_eq!(board.history.len(), 0);
    }
}
