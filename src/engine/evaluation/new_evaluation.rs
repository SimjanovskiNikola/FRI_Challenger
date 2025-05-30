use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color::*;
use crate::engine::board::structures::piece::*;
use crate::engine::misc::bit_pos_utility::get_bit_rank;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::misc::bitboard::Iterator;
use crate::engine::misc::const_utility::OPP_SQ;
use crate::engine::move_generator::bishop::get_bishop_mv;
use crate::engine::move_generator::bishop::has_bishop_pair;
use crate::engine::move_generator::king::get_king_mv;
use crate::engine::move_generator::king::has_near_open_files;
use crate::engine::move_generator::knight::get_knight_mv;
use crate::engine::move_generator::pawn::is_blocked_pawn;
use crate::engine::move_generator::pawn::is_isolated_pawn;
use crate::engine::move_generator::pawn::is_passed_pawn;
use crate::engine::move_generator::rook::get_rook_mv;
use crate::engine::move_generator::rook::is_rook_on_open_file;
use crate::engine::move_generator::rook::is_rook_on_semi_open_file;

const DOUBLE_PAWN_WT: isize = -15;
const BLOCKED_PAWN_WT: isize = -15;
const ISOLATED_PAWN_WT: isize = -15;
const MOBILITY_WT: isize = 1;
const ROOK_OPEN_FILE_WT: (isize, isize) = (8, 20); // NOTE: TAPERED ACHIEVED
const ROOK_SEMI_OPEN_FILE_WT: (isize, isize) = (4, 10); // NOTE: TAPERED ACHIEVED
const PASSED_PAWN_WT: [[isize; 8]; 2] =
    [[0, 5, 10, 20, 35, 60, 100, 0], [0, 100, 60, 35, 20, 10, 5, 0]]; // NOTE: TAPERED ACHIEVED
const BISHOP_PAIR_WT: (isize, isize) = (5, 20); // NOTE: TAPERED ACHIEVED
const GAME_PHASE_INCREMENT: [usize; 14] = [0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 4, 4];
const ROOK_TRAP_PENALTY: isize = -50;
const ROOK_LOW_NUM_MOVES: isize = 2;

const BISHOP_TRAP_PENALTY: isize = -50;
const BISHOP_LOW_NUM_MOVES: isize = 1;

const KNIGHT_TRAP_PENALTY: isize = -50;
const KNIGHT_LOW_NUM_MOVES: isize = 2;
const KNIGHT_VALUE_PER_PAWN_WT: (isize, isize) = (0, 1);

pub const CASTLE_BONUS_WT: [(isize, isize); 4] = [(30, 0), (20, 0), (30, 0), (20, 0)];
pub const KING_OPEN_FILES_PENALTY: (isize, isize) = (-40, 0);

const PIECE_WEIGHT: [(isize, isize); 6] =
    [(82, 94), (337, 281), (0, 0), (365, 297), (477, 512), (1025, 936)];

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
    fn evaluation(&self) -> isize;

    // fn material_balance(&self) -> isize;

    fn piece_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize;
    // fn piece_sq_eval(piece: &Piece, phase: usize, sq: usize) -> isize; //FIXME: Fix Phase to enum

    fn determine_phase(&self) -> usize;
    fn tapered(value: (isize, isize), phase: (isize, isize)) -> isize;
    fn material_eval(piece: Piece) -> (isize, isize);
    fn psqt_eval(piece: Piece, sq: usize) -> (isize, isize);
    fn insufficient_material(&self) -> bool;

    fn pawn_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize;
    fn knight_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize;
    fn king_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize;
    fn bishop_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize;
    fn rook_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize;
    fn queen_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize;
}

impl Evaluation for Board {
    #[inline(always)]
    fn tapered(value: (isize, isize), phase: (isize, isize)) -> isize {
        (phase.0 * value.0 + phase.1 * value.1) / 24
    }

    fn material_eval(piece: Piece) -> (isize, isize) {
        PIECE_WEIGHT[(piece.kind() / 2).idx() - 1]
    }

    fn psqt_eval(piece: Piece, sq: usize) -> (isize, isize) {
        match piece.kind() {
            PAWN => (PAWN_EVAL[0][sq], PAWN_EVAL[1][sq]),
            KNIGHT => (KNIGHT_EVAL[0][sq], KNIGHT_EVAL[1][sq]),
            BISHOP => (BISHOP_EVAL[0][sq], BISHOP_EVAL[1][sq]),
            ROOK => (ROOK_EVAL[0][sq], ROOK_EVAL[1][sq]),
            QUEEN => (QUEEN_EVAL[0][sq], QUEEN_EVAL[1][sq]),
            KING => (KING_EVAL[0][sq], KING_EVAL[1][sq]),
            _ => panic!("Not the right type, Something is wrong"),
        }
    }

    #[inline(always)]
    fn insufficient_material(&self) -> bool {
        // The Color is not relevant here, and that is why i use self.color()
        let (own, enemy) = self.both_occ_bb(self.color());
        if (own | enemy).count_ones() < 4 {
            let kings = self.bb(WHITE_KING) | self.bb(BLACK_KING);
            let knights = self.bb(WHITE_KNIGHT) | self.bb(BLACK_KNIGHT);
            let bishops = self.bb(WHITE_BISHOP) | self.bb(BLACK_BISHOP);
            if (kings | knights | bishops) == (own | enemy) {
                return true;
            }
        }
        return false;
    }

    fn determine_phase(&self) -> usize {
        let mut phase = 0;
        for piece in &CLR_PIECES {
            phase += self.bb(*piece).count() * GAME_PHASE_INCREMENT[piece.idx()];
        }
        phase
    }

    #[inline(always)]
    fn evaluation(&self) -> isize {
        let mut score: isize = 0;

        let (white_occ, black_occ) = self.both_occ_bb(WHITE);
        // TODO: Add Phase to the game
        let phase = self.determine_phase();
        let mg_phase = phase.min(24) as isize;
        let eg_phase = (24 - mg_phase) as isize;
        let both_phase = (mg_phase, eg_phase);

        for piece in &PIECES {
            let pce = piece + WHITE;
            let mut bb = self.bb(pce);
            while let Some(sq) = bb.next() {
                // Material Evaluation
                score += Self::tapered(Self::material_eval(pce), both_phase);

                // PSQT Evaluation
                score += Self::tapered(Self::psqt_eval(pce, OPP_SQ[sq]), both_phase);

                // Custom Piece Evaluation
                score += self.piece_eval(pce, sq, both_phase)
            }

            let pce = piece + BLACK;
            let mut bb = self.bb(pce);
            while let Some(sq) = bb.next() {
                // Material Evaluation
                score -= Self::tapered(Self::material_eval(pce), both_phase);

                // PSQT Evaluation
                score -= Self::tapered(Self::psqt_eval(pce, sq), both_phase);

                // Custom Piece Evaluation
                score -= self.piece_eval(pce, sq, both_phase)
            }
        }

        return score * self.color().sign();
    }

    #[inline(always)]
    fn piece_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize {
        match piece.kind() {
            PAWN => self.pawn_eval(piece, sq, phase),
            KNIGHT => self.knight_eval(piece, sq, phase),
            BISHOP => self.bishop_eval(piece, sq, phase),
            ROOK => self.rook_eval(piece, sq, phase),
            QUEEN => self.queen_eval(piece, sq, phase),
            KING => self.king_eval(piece, sq, phase),
            _ => panic!(" Not the right type, Something is wrong"),
        }
    }

    #[inline(always)]
    fn pawn_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize {
        let mut score: isize = 0;
        let clr = piece.color();
        let (own_pawns, enemy_pawns) = self.both_bb(piece);

        if is_passed_pawn(piece.color(), sq, enemy_pawns) {
            let rank = get_bit_rank(sq) as usize;
            score += PASSED_PAWN_WT[piece.color().idx()][rank] as isize;
        }

        if is_isolated_pawn(sq, own_pawns) {
            score += ISOLATED_PAWN_WT;
        }

        if is_blocked_pawn(piece.color(), sq, own_pawns) {
            score += DOUBLE_PAWN_WT;
        }

        score
    }

    #[inline(always)]
    fn knight_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize {
        let mut score = 0;

        let clr = piece.color();
        let (own, enemy) = self.both_occ_bb(piece.color());
        let (own_pawns, enemy_pawns) = self.both_bb(PAWN + piece.color());

        // Increase Value If More Pawns
        score += (16 - (own_pawns | enemy_pawns).count_ones() as isize)
            * Self::tapered(KNIGHT_VALUE_PER_PAWN_WT, phase);

        // Movement
        let moves = get_knight_mv(sq, own, enemy, clr).count_ones() as isize;
        score += moves * MOBILITY_WT;

        if moves <= KNIGHT_LOW_NUM_MOVES {
            score += KNIGHT_TRAP_PENALTY;
        }

        return score;
    }

    #[inline(always)]
    fn king_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize {
        let mut score = 0;
        let clr = piece.color();
        let (own, enemy) = self.both_occ_bb(piece.color()); // get_occupancy(piece, self);

        // Open Files near the king
        if has_near_open_files(sq, self.pawn_bb(clr)) {
            score += Self::tapered(KING_OPEN_FILES_PENALTY, phase);
        }

        // // Castling
        // if let Some(c) = self.get_castle(clr) {
        //     score += Self::tapered(CASTLE_BONUS_WT[c], phase);
        // }

        // Mobility
        let moves = get_king_mv(sq, own, enemy, clr).count_ones() as isize;
        score += moves * MOBILITY_WT;

        return score;
    }

    #[inline(always)]
    fn bishop_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize {
        let mut score = 0;
        let clr = piece.color();
        let (own, enemy) = self.both_occ_bb(piece.color());

        // Bishop Pair
        if has_bishop_pair(self.bishop_bb(clr)) {
            score += Self::tapered(BISHOP_PAIR_WT, phase);
        }

        // Mobility
        let moves = get_bishop_mv(sq, own, enemy, clr).count_ones() as isize;
        score += moves * MOBILITY_WT;

        // Trapped
        if moves <= BISHOP_LOW_NUM_MOVES {
            score += BISHOP_TRAP_PENALTY;
        }

        return score;
    }

    #[inline(always)]
    fn rook_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize {
        let mut score = 0;
        let clr = piece.color();
        let (own, enemy) = self.both_occ_bb(piece.color());

        // Open File / Semi-Open File
        if is_rook_on_open_file(sq, self.pawn_bb(clr), self.pawn_bb(clr.opp())) {
            score += Self::tapered(ROOK_OPEN_FILE_WT, phase);
        } else if is_rook_on_semi_open_file(sq, self.pawn_bb(clr)) {
            score += Self::tapered(ROOK_SEMI_OPEN_FILE_WT, phase);
        }

        // Movement
        let moves = get_rook_mv(sq, own, enemy, clr).count_ones() as isize;
        score += moves * MOBILITY_WT;

        // Trapped
        if moves <= ROOK_LOW_NUM_MOVES {
            score += ROOK_TRAP_PENALTY;
        }

        return score;
    }

    #[inline(always)]
    fn queen_eval(&self, piece: Piece, sq: usize, phase: (isize, isize)) -> isize {
        let mut score = 0;
        let clr = piece.color();

        // NOTE: It is bad to add movement evaluation for the queen

        return score;
    }

    // // TODO: Change name to psqt
    // #[inline(always)]
    // fn piece_sq_eval(piece: &Piece, phase: usize, mut sq: usize) -> isize {
    //     if piece.color().is_white() {
    //         sq = OPP_SQ[sq]
    //     }

    //     match piece.kind() {
    //         PAWN => PAWN_EVAL[phase][sq],
    //         KNIGHT => KNIGHT_EVAL[phase][sq],
    //         BISHOP => BISHOP_EVAL[phase][sq],
    //         ROOK => ROOK_EVAL[phase][sq],
    //         QUEEN => QUEEN_EVAL[phase][sq],
    //         KING => KING_EVAL[phase][sq],
    //         _ => panic!("Not the right type, Something is wrong"),
    //     }
    // }

    // #[inline(always)]
    // fn material_balance(&self) -> isize {
    //     let mut score = 0;
    //     for piece in &PIECES {
    //         score += piece.weight()
    //             * (self.bitboard[(piece + WHITE).idx()].count() as isize
    //                 - self.bitboard[(piece + BLACK).idx()].count() as isize)
    //     }
    //     score
    // }
}
