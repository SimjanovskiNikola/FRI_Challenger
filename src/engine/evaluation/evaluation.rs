use crate::engine::{
    game::Game,
    shared::{
        helper_func::bitboard::BitboardTrait,
        structures::{
            color::{Color, ColorTrait},
            piece::*,
        },
    },
};

const KING_WT: usize = 20000;
const QUEEN_WT: usize = 900;
const ROOK_WT: usize = 500;
const BISHOP_WT: usize = 350;
const KNIGHT_WT: usize = 325;
const PAWN_WT: usize = 100;
// TODO: Add (-0.5 for doubled, blocked, or isolated Pawns)
// TODO: Add (+0.1 for Mobility)

#[rustfmt::skip]
const PAWN_EVAL:[isize;64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5,  -5,-10,  0,  0,-10, -5,  5,
    5,  10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

#[rustfmt::skip]
const KNIGHT_EVAL:[isize;64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

#[rustfmt::skip]
const BISHOP_EVAL:[isize;64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

#[rustfmt::skip]
const ROOK_EVAL:[isize;64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

#[rustfmt::skip]
const QUEEN_EVAL:[isize;64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

#[rustfmt::skip]
const KING_MG_EVAL:[isize;64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20
];

#[rustfmt::skip]
const KING_EG_EVAL:[isize;64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];

#[rustfmt::skip]
const OPP_SQ:[usize;64] = [
    0,  1,  2,  3,  4,  5,  6,  7,
    8,  9,  10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55,
    56, 57, 58, 59, 60, 61, 62, 63,
];

#[rustfmt::skip]
const OK_SQ:[usize;64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
    8,  9,  10, 11, 12, 13, 14, 15,
    0,  1,  2,  3,  4,  5,  6,  7,
];

trait Evaluation {
    fn evaluate_pos(game: &Game, color: Color) -> isize;
    fn material_sq(game: &Game, color: Color) -> isize;
    fn piece_eval(piece: &Piece, sq: usize) -> isize;
    fn material_balance(game: &Game, color: Color) -> isize;
}

impl Evaluation for Game {
    #[inline(always)]
    fn evaluate_pos(game: &Game, color: Color) -> isize {
        (Self::material_balance(game, color) as isize) + Self::material_sq(game, color)
    }

    #[inline(always)]
    fn material_sq(game: &Game, color: Color) -> isize {
        let mut eval_score: isize = 0;
        for piece in &CLR_PIECES {
            let mut bb = game.bitboard[(piece + color) as usize];
            while bb != 0 {
                let sq = bb.pop_lsb();
                if color.is_white() {
                    eval_score += Self::piece_eval(piece, OK_SQ[sq]);
                } else {
                    eval_score -= Self::piece_eval(piece, OPP_SQ[sq]);
                }
            }
        }

        (color as isize * -1) * (eval_score)
    }

    #[inline(always)]
    fn piece_eval(piece: &Piece, sq: usize) -> isize {
        match *piece {
            PAWN => PAWN_EVAL[sq],
            KNIGHT => KNIGHT_EVAL[sq],
            BISHOP => BISHOP_EVAL[sq],
            ROOK => ROOK_EVAL[sq],
            QUEEN => QUEEN_EVAL[sq],
            KING => KING_MG_EVAL[sq],
            _ => panic!(" Not the right type, Something is wrong"),
        }
    }

    #[inline(always)]
    fn material_balance(game: &Game, color: Color) -> isize {
        (KING_WT
            * (game.bitboard[(KING + color).idx()].count()
                - game.bitboard[(KING + color.opp()).idx()].count())
            + QUEEN_WT
                * (game.bitboard[(QUEEN + color).idx()].count()
                    - game.bitboard[(QUEEN + color.opp()).idx()].count())
            + ROOK_WT
                * (game.bitboard[(ROOK + color).idx()].count()
                    - game.bitboard[(ROOK + color.opp()).idx()].count())
            + KNIGHT_WT
                * (game.bitboard[(KNIGHT + color).idx()].count()
                    - game.bitboard[(KNIGHT + color.opp()).idx()].count())
            + BISHOP_WT
                * (game.bitboard[(BISHOP + color).idx()].count()
                    - game.bitboard[(BISHOP + color.opp()).idx()].count())
            + PAWN_WT
                * (game.bitboard[(PAWN + color).idx()].count()
                    - game.bitboard[(PAWN + color.opp()).idx()].count())) as isize
    }
}
