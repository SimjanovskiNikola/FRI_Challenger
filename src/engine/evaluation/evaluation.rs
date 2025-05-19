use std::usize;

use crate::engine::board::board::Board;
use crate::engine::move_generation::mv_gen::{get_all_moves, get_occupancy};
use crate::engine::move_generator::generated::pawn::*;
use crate::engine::shared::helper_func::bit_pos_utility::get_bit_rank;
use crate::engine::shared::helper_func::bitboard::{BitboardTrait, Iterator};
use crate::engine::shared::helper_func::const_utility::OPP_SQ;
use crate::engine::shared::structures::color::*;
use crate::engine::shared::structures::piece::*;

const DOUBLE_PAWN_WT: isize = -15;
const BLOCKED_PAWN_WT: isize = -15;
const ISOLATED_PAWN_WT: isize = -15;
const MOBILITY_WT: isize = 1;
const ROOK_OPEN_FILE_WT: isize = 5;
const PASSED_PAWN_WT: [[isize; 8]; 2] =
    [[0, 5, 10, 20, 35, 60, 100, 0], [0, 100, 60, 35, 20, 10, 5, 0]];
const BISHOP_PAIR_WT: isize = 10;
const GAME_PHASE_INCREMENT: [usize; 14] = [0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 4, 4];

#[rustfmt::skip]
const PAWN_EVAL:[[isize; 64]; 2] = [[
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5,  -5,-10,  0,  0,-10, -5,  5,
    5,  10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
],
[
     0,   0,   0,   0,   0,   0,   0,   0,
   50,  50,  30,  20,  20,  30,  50,  50,
   30,  30,  20,  10,  10,  20,  30,  30,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0
]
];

#[rustfmt::skip]
const KNIGHT_EVAL:[[isize; 64]; 2] = [
   [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ],
    [
        -50, -40, -30, -30, -30, -30, -40, -50,
        -40, -20,   0,   0,   0,   0, -20, -40,
        -30,   0,  10,  15,  15,  10,   0, -30,
        -30,   5,  15,  20,  20,  15,   5, -30,
        -30,   0,  15,  20,  20,  15,   0, -30,
        -30,   5,  10,  15,  15,  10,   5, -30,
        -40, -20,   0,   5,   5,   0, -20, -40,
        -50, -40, -30, -30, -30, -30, -40, -50,
    ],
];

#[rustfmt::skip]
const BISHOP_EVAL:[[isize; 64]; 2] = [
   [ 
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ],
    [
        -20, -10, -10, -10, -10, -10, -10, -20,
        -10,   0,   0,   0,   0,   0,   0, -10,
        -10,   0,   5,  10,  10,   5,   0, -10,
        -10,   5,   5,  10,  10,   5,   5, -10,
        -10,   0,  10,  10,  10,  10,   0, -10,
        -10,  10,  10,  10,  10,  10,  10, -10,
        -10,   5,   0,   0,   0,   0,   5, -10,
        -20, -10, -10, -10, -10, -10, -10, -20,
    ]
];

#[rustfmt::skip]
const ROOK_EVAL:[[isize; 64]; 2] = [
    [
        0,  0,  0,  0,  0,  0,  0,  0,
        5, 10, 10, 10, 10, 10, 10,  5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        0,  0,  0,  5,  5,  0,  0,  0
    ],
    [
        0,  0,  0,  0,  0,  0,  0,  0,
        5, 10, 10, 10, 10, 10, 10,  5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        0,  0,  0,  5,  5,  0,  0,  0
    ]
];

#[rustfmt::skip]
const QUEEN_EVAL:[[isize; 64]; 2] = [
    [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -5,  0,  5,  5,  5,  5,  0, -5,
        0,  0,  5,  5,  5,  5,  0, -5,
        -10,  5,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ],
    [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -5,  0,  5,  5,  5,  5,  0, -5,
        0,  0,  5,  5,  5,  5,  0, -5,
        -10,  5,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ]
];

#[rustfmt::skip]
const KING_EVAL:[[isize; 64]; 2] = [
    [
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
         20, 20,  0,  0,  0,  0, 20, 20,
         20, 30, 10,  0,  0, 10, 30, 20
    ],
    [
        -50,-40,-30,-20,-20,-30,-40,-50,
        -30,-20,-10,  0,  0,-10,-20,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-30,  0,  0,  0,  0,-30,-30,
        -50,-30,-30,-30,-30,-30,-30,-50
    ]
 ];

pub trait Evaluation {
    fn evaluate_pos(&self) -> isize;

    fn material_balance(&self) -> isize;
    fn determine_phase(&self) -> usize;

    fn piece_eval(&self, piece: &Piece, sq: usize) -> isize;
    fn piece_sq_eval(piece: &Piece, phase: usize, sq: usize) -> isize; //FIXME: Fix Phase to enum

    fn pawn_eval(&self, piece: &Piece, sq: usize) -> isize;
    fn knight_eval(&self, piece: &Piece, sq: usize) -> isize;
    fn king_eval(&self, piece: &Piece, sq: usize) -> isize;
    fn bishop_eval(&self, piece: &Piece, sq: usize) -> isize;
    fn rook_eval(&self, piece: &Piece, sq: usize) -> isize;
    fn queen_eval(&self, piece: &Piece, sq: usize) -> isize;
}

impl Evaluation for Board {
    #[inline(always)]
    fn evaluate_pos(&self) -> isize {
        let mut score: isize = 0;

        let (white_occ, black_occ) = get_occupancy(&WHITE, &self);
        let phase = self.determine_phase();
        let mg_phase = phase.min(24) as isize;
        let eg_phase = (24 - mg_phase) as isize;

        for piece in &CLR_PIECES {
            let mut bb = self.bitboard[piece.idx()];
            while let Some(sq) = bb.next() {
                let mut temp_score = 0;
                temp_score += piece.weight();

                temp_score += (mg_phase * Self::piece_sq_eval(piece, 0, sq)
                    + eg_phase * Self::piece_sq_eval(piece, 1, sq))
                    / 24;

                temp_score += self.piece_eval(piece, sq);

                if piece.color().is_black() {
                    temp_score += get_all_moves(*piece, sq, &self, black_occ, white_occ).count()
                        as isize
                        * MOBILITY_WT;
                } else {
                    temp_score += get_all_moves(*piece, sq, &self, white_occ, black_occ).count()
                        as isize
                        * MOBILITY_WT;
                }

                score += temp_score * piece.color().sign();
            }
        }

        return score * self.state.color.sign();
    }

    #[inline(always)]
    fn piece_eval(&self, piece: &Piece, sq: usize) -> isize {
        match piece.kind() {
            PAWN => self.pawn_eval(piece, sq),
            KNIGHT => self.knight_eval(piece, sq),
            BISHOP => self.bishop_eval(piece, sq),
            ROOK => self.rook_eval(piece, sq),
            QUEEN => self.queen_eval(piece, sq),
            KING => self.king_eval(piece, sq),
            _ => panic!(" Not the right type, Something is wrong"),
        }
    }

    #[inline(always)]
    fn pawn_eval(&self, piece: &Piece, sq: usize) -> isize {
        let mut score: isize = 0;
        let (own_pawns, enemy_pawns) = (
            self.bitboard[(PAWN + piece.color()).idx()],
            self.bitboard[(PAWN + piece.color().opp()).idx()],
        );

        if PASSED_PAWN_LOOKUP[piece.color().idx()][sq] & enemy_pawns == 0 {
            let rank = get_bit_rank(sq) as usize;
            score += PASSED_PAWN_WT[piece.color().idx()][rank] as isize;
        }

        if ISOLATED_PAWN_LOOKUP[sq] & own_pawns == 0 {
            score += ISOLATED_PAWN_WT;
        }

        if BLOCKED_PAWN_LOOKUP[piece.color().idx()][sq] & own_pawns == 0 {
            score += DOUBLE_PAWN_WT;
        }

        score
    }

    #[inline(always)]
    fn knight_eval(&self, piece: &Piece, _sq: usize) -> isize {
        0
    }

    #[inline(always)]
    fn king_eval(&self, piece: &Piece, _sq: usize) -> isize {
        0
    }

    #[inline(always)]
    fn bishop_eval(&self, piece: &Piece, _sq: usize) -> isize {
        if self.bitboard(*piece).count() >= 2 {
            BISHOP_PAIR_WT
        } else {
            0
        }
    }

    #[inline(always)]
    fn rook_eval(&self, piece: &Piece, sq: usize) -> isize {
        0
    }

    #[inline(always)]
    fn queen_eval(&self, piece: &Piece, sq: usize) -> isize {
        0
    }

    #[inline(always)]
    fn piece_sq_eval(piece: &Piece, phase: usize, mut sq: usize) -> isize {
        if piece.color().is_white() {
            sq = OPP_SQ[sq]
        }

        match piece.kind() {
            PAWN => PAWN_EVAL[phase][sq],
            KNIGHT => KNIGHT_EVAL[phase][sq],
            BISHOP => BISHOP_EVAL[phase][sq],
            ROOK => ROOK_EVAL[phase][sq],
            QUEEN => QUEEN_EVAL[phase][sq],
            KING => KING_EVAL[phase][sq],
            _ => panic!("Not the right type, Something is wrong"),
        }
    }

    #[inline(always)]
    fn material_balance(&self) -> isize {
        let mut score = 0;
        for piece in &PIECES {
            score += piece.weight()
                * (self.bitboard[(piece + WHITE).idx()].count() as isize
                    - self.bitboard[(piece + BLACK).idx()].count() as isize)
        }
        score
    }

    fn determine_phase(&self) -> usize {
        let mut phase = 0;
        for piece in &CLR_PIECES {
            phase += self.bitboard[piece.idx()].count() * GAME_PHASE_INCREMENT[piece.idx()];
        }
        phase
    }
}

// NOTE: For Each Peace
// 1. How much are on the board of that type (Material on the board)
// 2. How much is the square they are sitting on valuable (md, eg)
// 3. Unique parameters that give advantage (Rook -> Open Files, Rook -> Connectivity Pawn -> Passed Pawn, etc...)
// 4. Mobility
// 5.

// fn eval_ending(&self, side: Color) -> Option<Score> {
//     let occupied = self.bitboard(WHITE) | self.bitboard(BLACK);

//     let kings = self.bitboard(WHITE | KING) | self.bitboard(BLACK | KING);
//     if kings.count() < 2 {
//         if self.bitboard(side | KING).count() == 0 {
//             return Some(-INF); // Loss
//         } else {
//             return Some(INF); // Win
//         }
//     }

//     // Draw by insufficient material
//     if occupied.count() < 4 {
//         let knights = self.bitboard(WHITE | KNIGHT) | self.bitboard(BLACK | KNIGHT);
//         let bishops = self.bitboard(WHITE | BISHOP) | self.bitboard(BLACK | BISHOP);
//         if (kings | knights | bishops) == occupied {
//             return Some(0); // Draw
//         }
//     }

//     None
// }
