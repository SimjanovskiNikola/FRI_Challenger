use super::castling::CastlingRights;
use super::color::{Color, ColorTrait};
use super::piece::{Piece, PieceTrait, BISHOP, KING, KNIGHT, QUEEN, ROOK};
use super::state::BoardState;
use super::{moves::Move, piece::PAWN};
use crate::engine::evaluation::eval_defs::CLR_SQ;
use crate::engine::evaluation::evaluation::Evaluation;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::search::transposition_table::TTEntry;
use crate::engine::{
    board::fen::FenTrait,
    misc::{bitboard::Bitboard, const_utility::FEN_START},
};
const MAX_PLY: usize = 64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    // Peace Occupancy (Bitboard Representation)
    pub squares: [Option<Piece>; 64],
    pub bitboard: [Bitboard; 14],

    // Position Vectors (Moves until now)
    pub moves: Vec<Move>,
    pub history: Vec<BoardState>,
    pub state: BoardState,

    // TODO: Add This to Move Ordering Structure
    pub tt_mv: Option<TTEntry>,
    pub s_history: [[u64; 64]; 14],
    pub s_killers: [[Option<Move>; 2]; 64],
    pub pv_moves: [[Option<Move>; MAX_PLY]; MAX_PLY],
    pub pv_len: [usize; MAX_PLY],
    pub gen_moves: Vec<(Move, isize)>,

    pub eval: Evaluation,
}

impl Board {
    pub fn initialize() -> Board {
        Board::read_fen(FEN_START)
    }

    #[inline(always)]
    pub fn create() -> Self {
        Self {
            squares: [None; 64],
            bitboard: [0 as Bitboard; 14],

            moves: Vec::with_capacity(1024),
            history: Vec::with_capacity(1024),
            state: BoardState::init(),

            // Move Ordering
            tt_mv: None,
            s_history: [[0u64; 64]; 14],
            s_killers: [[None; 2]; 64],
            pv_moves: [[None; 64]; 64],
            pv_len: [0; 64],

            gen_moves: Vec::with_capacity(256),

            eval: Evaluation::init(),
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.squares = [None; 64];
        self.bitboard = [0 as Bitboard; 14];
        self.moves = Vec::with_capacity(1024);
        self.history = Vec::with_capacity(1024);
        self.state = BoardState::init();
        self.tt_mv = None;
        self.s_history = [[0u64; 64]; 14]; // FIXME: Don't  create new, just fill with 0's
        self.s_killers = [[None; 2]; 64]; // FIXME: Don't  create new, just fill with 0's
                                          // self.s_pv = [None; 64];
        self.gen_moves.clear();

        self.eval.reset();
    }

    #[inline(always)]
    pub fn pv_clear(&mut self) {
        self.pv_len.fill(0);
        self.pv_moves.fill([None; MAX_PLY]);
    }

    #[inline(always)]
    pub fn get_pv(&self) -> Vec<Move> {
        let len = self.pv_len[0].min(MAX_PLY);
        (0..len).filter_map(|i| self.pv_moves[0][i]).collect()
    }

    #[inline(always)]
    pub fn bb(&self, piece: Piece) -> u64 {
        self.bitboard[piece.idx()]
    }

    #[inline(always)]
    pub fn both_bb(&self, piece: Piece) -> (u64, u64) {
        (self.bb(piece), self.bb(piece.opp()))
    }

    #[inline(always)]
    pub fn pawn_bb(&self, color: Color) -> u64 {
        self.bitboard[(PAWN + color) as usize]
    }

    #[inline(always)]
    pub fn knight_bb(&self, color: Color) -> u64 {
        self.bitboard[(KNIGHT + color) as usize]
    }

    #[inline(always)]
    pub fn king_bb(&self, color: Color) -> u64 {
        self.bitboard[(KING + color) as usize]
    }

    #[inline(always)]
    pub fn bishop_bb(&self, color: Color) -> u64 {
        self.bitboard[(BISHOP + color) as usize]
    }

    #[inline(always)]
    pub fn rook_bb(&self, color: Color) -> u64 {
        self.bitboard[(ROOK + color) as usize]
    }

    #[inline(always)]
    pub fn queen_bb(&self, color: Color) -> u64 {
        self.bitboard[(QUEEN + color) as usize]
    }

    #[inline(always)]
    pub fn occ_bb(&self, color: Color) -> u64 {
        self.bitboard[color.idx()]
    }

    #[inline(always)]
    pub fn both_occ_bb(&self, color: Color) -> (u64, u64) {
        (self.occ_bb(color), self.occ_bb(color.opp()))
    }

    #[inline(always)]
    pub fn king_sq(&self, color: Color) -> usize {
        self.king_bb(color).get_lsb()
    }

    #[inline(always)]
    pub fn ply(&self) -> usize {
        self.moves.len()
    }

    #[inline(always)]
    pub fn key(&self) -> u64 {
        self.state.key
    }

    #[inline(always)]
    pub fn ep(&self) -> Option<u8> {
        self.state.ep
    }

    #[inline(always)]
    pub fn castling(&self) -> CastlingRights {
        self.state.castling
    }

    #[inline(always)]
    pub fn color(&self) -> Color {
        self.state.color
    }

    #[inline(always)]
    pub fn half_move(&self) -> u8 {
        self.state.half_move
    }

    #[inline(always)]
    pub fn full_move(&self) -> u16 {
        self.state.full_move
    }

    #[inline(always)]
    pub fn phase(&self) -> isize {
        self.state.phase
    }

    #[inline(always)]
    pub fn ply_reset(&mut self) {
        self.moves.clear();
    }

    #[inline(always)]
    pub fn piece_sq(&self, sq: usize) -> Piece {
        match self.squares[sq] {
            Some(piece) => piece,
            None => unreachable!("There is no piece to be captured at this location"),
        }
    }

    // FIXME: IS THIS REALLY NEEDED HERE ??????????
    #[inline(always)]
    pub fn is_repetition(board: &Board) -> bool {
        let his_len = board.history.len();
        let half_move = board.half_move() as usize;
        assert!(his_len >= half_move, "It is Negative {:?} {:?}", his_len, board.half_move());

        for i in (his_len - half_move)..his_len {
            if board.history[i].key == board.key() {
                return true;
            }
        }

        false
    }

    // FIXME: IS THIS REALLY NEEDED AND USED SOMEWHERE ???????
    #[inline(always)]
    pub fn get_killer(&self, idx: usize) -> Option<Move> {
        assert!(idx == 0 || idx == 1, "Index is nor 0 nor 1: {idx}");
        self.s_killers[self.ply()][idx]
    }

    // FIXME: Is this really needed here ?
    #[inline(always)]
    pub fn mirror(&mut self) {
        for idx in 0..(self.squares.len() / 2) {
            let first_piece = match self.squares[idx] {
                Some(p) => Some(p.opp()),
                None => None,
            };
            let second_piece = match self.squares[CLR_SQ[1][idx]] {
                Some(p) => Some(p.opp()),
                None => None,
            };
            self.squares[idx] = second_piece;
            self.squares[CLR_SQ[1][idx]] = first_piece;
        }

        for idx in (0..self.bitboard.len()).step_by(2) {
            let bb = self.bitboard[idx];
            self.bitboard[idx] = self.bitboard[idx + 1].swap_bytes();
            self.bitboard[idx + 1] = bb.swap_bytes();
        }

        self.state.color = self.state.color.opp();
        self.state.ep = match self.state.ep {
            Some(sq) => Some(CLR_SQ[1][sq as usize] as u8),
            None => None,
        };

        // self.state.castling = 0; TODO:
        // self.state.key = self.generate_pos_key(); // TODO: Update Zobrist key Structure
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::engine::board::structures::castling::CastlingRights;
    use crate::engine::board::structures::color::{BLACK, WHITE};
    use crate::engine::evaluation::evaluation::{Evaluation, EvaluationTrait};
    use crate::engine::misc::const_utility::{
        FEN_CASTLE_ONE, FEN_MATE_IN_5, FEN_POS_FIVE, FEN_POS_FOUR, FEN_POS_THREE,
    };
    use crate::engine::misc::print_utility::{print_bitboard, print_chess};

    #[test]
    fn test_reset_board() {
        let mut board = Board::initialize();
        board.reset();

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

    fn test_mirror_framework(fen: &str) {
        let mut board = Board::read_fen(fen);
        let eval = board.evaluation();
        // print_chess(&board);
        board.mirror();
        // print_chess(&board);
        board.eval.reset();
        let mirror_eval = board.evaluation();
        // assert_eq!(eval, mirror_eval)
    }

    #[test]
    fn test_mirror_start() {
        test_mirror_framework(FEN_START);
    }

    #[test]
    fn test_mirror_mate_in_5() {
        test_mirror_framework(FEN_MATE_IN_5);
    }

    #[test]
    fn test_mirror_pos_4() {
        test_mirror_framework(FEN_POS_FOUR);
    }

    #[test]
    fn test_mirror_pos_5() {
        test_mirror_framework(FEN_POS_FIVE);
    }

    #[test]
    fn test_mirror_pos_3() {
        test_mirror_framework(FEN_POS_THREE);
    }

    #[test]
    fn test_mirror_castle_one() {
        test_mirror_framework(FEN_CASTLE_ONE);
    }
}
