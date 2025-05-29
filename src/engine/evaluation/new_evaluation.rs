// use std::usize;

// use crate::engine::game::Game;
// use crate::engine::move_generation::mv_gen::{get_all_moves, get_occupancy};
// use crate::engine::move_generator::bishop::{get_bishop_mv, has_bishop_pair};
// use crate::engine::move_generator::generated::pawn::*;
// use crate::engine::move_generator::king::{get_king_mv, has_good_pawn_shield, has_near_open_files};
// use crate::engine::move_generator::knight::get_knight_mv;
// use crate::engine::move_generator::pawn::{is_blocked_pawn, is_isolated_pawn, is_passed_pawn};
// use crate::engine::move_generator::rook::{
//     get_rook_mv, is_rook_on_open_file, is_rook_on_semi_open_file,
// };
// use crate::engine::shared::helper_func::bit_pos_utility::get_bit_rank;
// use crate::engine::shared::helper_func::bitboard::{BitboardTrait, Iterator};
// use crate::engine::shared::helper_func::const_utility::OPP_SQ;
// use crate::engine::shared::structures::color::*;
// use crate::engine::shared::structures::piece::*;

// const DOUBLE_PAWN_WT: isize = -15;
// const BLOCKED_PAWN_WT: isize = -15;
// const ISOLATED_PAWN_WT: isize = -15;
// const MOBILITY_WT: isize = 1;
// const ROOK_OPEN_FILE_WT: (isize, isize) = (8, 20); // NOTE: TAPERED ACHIEVED
// const ROOK_SEMI_OPEN_FILE_WT: (isize, isize) = (8, 20); // NOTE: TAPERED ACHIEVED
// const PASSED_PAWN_WT: [[isize; 8]; 2] =
//     [[0, 5, 10, 20, 35, 60, 100, 0], [0, 100, 60, 35, 20, 10, 5, 0]]; // NOTE: TAPERED ACHIEVED
// const BISHOP_PAIR_WT: (isize, isize) = (5, 20); // NOTE: TAPERED ACHIEVED
// const GAME_PHASE_INCREMENT: [usize; 14] = [0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 4, 4];
// const ROOK_TRAP_PENALTY: isize = 50;
// const ROOK_LOW_NUM_MOVES: isize = 2;

// const BISHOP_TRAP_PENALTY: isize = 50;
// const BISHOP_LOW_NUM_MOVES: isize = 1;

// const KNIGHT_TRAP_PENALTY: isize = 50;
// const KNIGHT_LOW_NUM_MOVES: isize = 2;
// const KNIGHT_VALUE_PER_PAWN_WT: (isize, isize) = (0, 1);

// pub const CASTLE_BONUS_WT: [(isize, isize); 4] = [(30, 0), (20, 0), (30, 0), (20, 0)];
// pub const KING_OPEN_FILES_PENALTY: (isize, isize) = (30, 0);

// const PIECE_WEIGHT: [(isize, isize); 6] =
//     [(82, 94), (337, 281), (0, 0), (365, 297), (477, 512), (1025, 936)];

// #[rustfmt::skip]
// const PAWN_EVAL:[[isize; 64]; 2] = [[
//     0,  0,  0,  0,  0,  0,  0,  0,
//     50, 50, 50, 50, 50, 50, 50, 50,
//     10, 10, 20, 30, 30, 20, 10, 10,
//     5,  5, 10, 25, 25, 10,  5,  5,
//     0,  0,  0, 20, 20,  0,  0,  0,
//     5,  -5,-10,  0,  0,-10, -5,  5,
//     5,  10, 10,-20,-20, 10, 10,  5,
//     0,  0,  0,  0,  0,  0,  0,  0
// ],
// [
//      0,   0,   0,   0,   0,   0,   0,   0,
//    50,  50,  30,  20,  20,  30,  50,  50,
//    30,  30,  20,  10,  10,  20,  30,  30,
//     0,   0,   0,   0,   0,   0,   0,   0,
//     0,   0,   0,   0,   0,   0,   0,   0,
//     0,   0,   0,   0,   0,   0,   0,   0,
//     0,   0,   0,   0,   0,   0,   0,   0,
//     0,   0,   0,   0,   0,   0,   0,   0
// ]
// ];

// #[rustfmt::skip]
// const KNIGHT_EVAL:[[isize; 64]; 2] = [
//    [
//         -50,-40,-30,-30,-30,-30,-40,-50,
//         -40,-20,  0,  0,  0,  0,-20,-40,
//         -30,  0, 10, 15, 15, 10,  0,-30,
//         -30,  5, 15, 20, 20, 15,  5,-30,
//         -30,  0, 15, 20, 20, 15,  0,-30,
//         -30,  5, 10, 15, 15, 10,  5,-30,
//         -40,-20,  0,  5,  5,  0,-20,-40,
//         -50,-40,-30,-30,-30,-30,-40,-50,
//     ],
//     [
//         -50, -40, -30, -30, -30, -30, -40, -50,
//         -40, -20,   0,   0,   0,   0, -20, -40,
//         -30,   0,  10,  15,  15,  10,   0, -30,
//         -30,   5,  15,  20,  20,  15,   5, -30,
//         -30,   0,  15,  20,  20,  15,   0, -30,
//         -30,   5,  10,  15,  15,  10,   5, -30,
//         -40, -20,   0,   5,   5,   0, -20, -40,
//         -50, -40, -30, -30, -30, -30, -40, -50,
//     ],
// ];

// #[rustfmt::skip]
// const BISHOP_EVAL:[[isize; 64]; 2] = [
//    [
//         -20,-10,-10,-10,-10,-10,-10,-20,
//         -10,  0,  0,  0,  0,  0,  0,-10,
//         -10,  0,  5, 10, 10,  5,  0,-10,
//         -10,  5,  5, 10, 10,  5,  5,-10,
//         -10,  0, 10, 10, 10, 10,  0,-10,
//         -10, 10, 10, 10, 10, 10, 10,-10,
//         -10,  5,  0,  0,  0,  0,  5,-10,
//         -20,-10,-10,-10,-10,-10,-10,-20,
//     ],
//     [
//         -20, -10, -10, -10, -10, -10, -10, -20,
//         -10,   0,   0,   0,   0,   0,   0, -10,
//         -10,   0,   5,  10,  10,   5,   0, -10,
//         -10,   5,   5,  10,  10,   5,   5, -10,
//         -10,   0,  10,  10,  10,  10,   0, -10,
//         -10,  10,  10,  10,  10,  10,  10, -10,
//         -10,   5,   0,   0,   0,   0,   5, -10,
//         -20, -10, -10, -10, -10, -10, -10, -20,
//     ]
// ];

// #[rustfmt::skip]
// const ROOK_EVAL:[[isize; 64]; 2] = [
//     [
//         0,  0,  0,  0,  0,  0,  0,  0,
//         5, 10, 10, 10, 10, 10, 10,  5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         0,  0,  0,  5,  5,  0,  0,  0
//     ],
//     [
//         0,  0,  0,  0,  0,  0,  0,  0,
//         5, 10, 10, 10, 10, 10, 10,  5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         -5,  0,  0,  0,  0,  0,  0, -5,
//         0,  0,  0,  5,  5,  0,  0,  0
//     ]
// ];

// #[rustfmt::skip]
// const QUEEN_EVAL:[[isize; 64]; 2] = [
//     [
//         -20,-10,-10, -5, -5,-10,-10,-20,
//         -10,  0,  0,  0,  0,  0,  0,-10,
//         -10,  0,  5,  5,  5,  5,  0,-10,
//         -5,  0,  5,  5,  5,  5,  0, -5,
//         0,  0,  5,  5,  5,  5,  0, -5,
//         -10,  5,  5,  5,  5,  5,  0,-10,
//         -10,  0,  5,  0,  0,  0,  0,-10,
//         -20,-10,-10, -5, -5,-10,-10,-20
//     ],
//     [
//         -20,-10,-10, -5, -5,-10,-10,-20,
//         -10,  0,  0,  0,  0,  0,  0,-10,
//         -10,  0,  5,  5,  5,  5,  0,-10,
//         -5,  0,  5,  5,  5,  5,  0, -5,
//         0,  0,  5,  5,  5,  5,  0, -5,
//         -10,  5,  5,  5,  5,  5,  0,-10,
//         -10,  0,  5,  0,  0,  0,  0,-10,
//         -20,-10,-10, -5, -5,-10,-10,-20
//     ]
// ];

// #[rustfmt::skip]
// const KING_EVAL:[[isize; 64]; 2] = [
//     [
//         -30,-40,-40,-50,-50,-40,-40,-30,
//         -30,-40,-40,-50,-50,-40,-40,-30,
//         -30,-40,-40,-50,-50,-40,-40,-30,
//         -30,-40,-40,-50,-50,-40,-40,-30,
//         -20,-30,-30,-40,-40,-30,-30,-20,
//         -10,-20,-20,-20,-20,-20,-20,-10,
//          20, 20,  0,  0,  0,  0, 20, 20,
//          20, 30, 10,  0,  0, 10, 30, 20
//     ],
//     [
//         -50,-40,-30,-20,-20,-30,-40,-50,
//         -30,-20,-10,  0,  0,-10,-20,-30,
//         -30,-10, 20, 30, 30, 20,-10,-30,
//         -30,-10, 30, 40, 40, 30,-10,-30,
//         -30,-10, 30, 40, 40, 30,-10,-30,
//         -30,-10, 20, 30, 30, 20,-10,-30,
//         -30,-30,  0,  0,  0,  0,-30,-30,
//         -50,-30,-30,-30,-30,-30,-30,-50
//     ]
//  ];

// pub trait New_Evaluation {
//     fn evaluation(&self) -> isize;

//     fn material_balance(&self) -> isize;
//     fn determine_phase(&self) -> usize;
//     fn tapered(value: (isize, isize), phase: (isize, isize)) -> isize;

//     fn piece_eval(&self, piece: &Piece, sq: usize) -> isize;
//     fn piece_sq_eval(piece: &Piece, phase: usize, sq: usize) -> isize; //FIXME: Fix Phase to enum

//     fn pawn_eval(&self, piece: &Piece, sq: usize) -> isize;
//     fn knight_eval(&self, piece: &Piece, sq: usize) -> isize;
//     fn king_eval(&self, piece: &Piece, sq: usize) -> isize;
//     fn bishop_eval(&self, piece: &Piece, sq: usize) -> isize;
//     fn rook_eval(&self, piece: &Piece, sq: usize) -> isize;
//     fn queen_eval(&self, piece: &Piece, sq: usize) -> isize;
//     fn insufficient_material(&self) -> Option<isize>;
// }

// impl Evaluation for Game {
//     #[inline(always)]
//     fn tapered(value: (isize, isize), phase: (isize, isize)) -> isize {
//         (phase.0 * value.0 + phase.1 * value.1) / 24
//     }

//     #[inline(always)]
//     fn evaluation(&self) -> isize {
//         let mut score: isize = 0;

//         let (white_occ, black_occ) = get_occupancy(&WHITE, &self);
//         // TODO: Add Phase to the game
//         let mg_phase = self.phase.min(24) as isize;
//         let eg_phase = (24 - mg_phase) as isize;

//         // TODO: ADD WHITE PEACES AND BLACK PEACES
//         // NOTE: WHITE PEACES
//         for piece in &PIECES {
//             let mut bb = self.bitboard[piece.idx()];
//             while let Some(sq) = bb.next() {
//                 // // FIXME: Not Good: (self.kind() / 2).idx() - 1
//                 // score +=
//                 //     Self::tapered(PIECE_WEIGHT[(self.kind() / 2).idx() - 1], mg_phase, eg_phase);

//                 // score += New_Evaluation::tapered(
//                 //     (Self::piece_sq_eval(piece, 0, sq), Self::piece_sq_eval(piece, 1, sq)),
//                 //     mg_phase,
//                 //     eg_phase,
//                 // );

//                 score += self.piece_eval(piece, sq)
//             }
//         }

//         return score * self.color.sign();
//     }

//     #[inline(always)]
//     fn piece_eval(&self, piece: &Piece, sq: usize) -> isize {
//         match piece.kind() {
//             PAWN => self.pawn_eval(piece, sq),
//             KNIGHT => self.knight_eval(piece, sq),
//             BISHOP => self.bishop_eval(piece, sq),
//             ROOK => self.rook_eval(piece, sq),
//             QUEEN => self.queen_eval(piece, sq),
//             KING => self.king_eval(piece, sq),
//             _ => panic!(" Not the right type, Something is wrong"),
//         }
//     }

//     #[inline(always)]
//     fn pawn_eval(&self, piece: &Piece, sq: usize) -> isize {
//         let mut score: isize = 0;
//         let (own_pawns, enemy_pawns) = (
//             self.bitboard[(PAWN + piece.color()).idx()],
//             self.bitboard[(PAWN + piece.color().opp()).idx()],
//         );

//         if is_passed_pawn(piece.color(), sq, enemy_pawns) {
//             let rank = get_bit_rank(sq) as usize;
//             score += PASSED_PAWN_WT[piece.color().idx()][rank] as isize;
//         }

//         if is_isolated_pawn(sq, own_pawns) {
//             score += ISOLATED_PAWN_WT;
//         }

//         if is_blocked_pawn(piece.color(), sq, own_pawns) {
//             score += DOUBLE_PAWN_WT;
//         }

//         score
//     }

//     #[inline(always)]
//     fn knight_eval(&self, piece: &Piece, sq: usize, phase: (isize, isize)) -> isize {
//         let clr = piece.color();
//         let (own, enemy) = get_occupancy(piece, self);
//         let (own_pawns, enemy_pawns) = (
//             self.bitboard[(PAWN + piece.color()).idx()],
//             self.bitboard[(PAWN + piece.color().opp()).idx()],
//         );

//         // Material Evaluation
//         let score = Self::tapered(PIECE_WEIGHT[(self.kind() / 2).idx() - 1], phase);

//         // PSQT Evaluation
//         // TODO: ADD PSQT as tapered
//         score += Self::tapered((KNIGHT_EVAL[phase.0][sq], KNIGHT_EVAL[phase.1][sq]), phase);

//         // Increase value if there are more pawns on the board
//         score += (16 - (own_pawns.count_ones() + enemy.count_ones()))
//             * Self::tapered(KNIGHT_VALUE_PER_PAWN_WT, phase);

//         let moves = get_knight_mv(sq, own, enemy).count_ones();
//         score += moves * MOBILITY_WT;

//         if moves <= KNIGHT_LOW_NUM_MOVES {
//             score += KNIGHT_TRAP_PENALTY;
//         }

//         return score;
//     }

//     #[inline(always)]
//     fn king_eval(&self, piece: &Piece, sq: usize, phase: (isize, isize)) -> isize {
//         let clr = piece.color();
//         let (own, enemy) = get_occupancy(piece, self);

//         // Material Evaluation
//         let score = Self::tapered(PIECE_WEIGHT[(self.kind() / 2).idx() - 1], phase);

//         // PSQT Evaluation
//         // TODO: ADD PSQT as tapered
//         score += Self::tapered((KING_EVAL[phase.0][sq], KING_EVAL[phase.1][sq]), phase);

//         if has_near_open_files(sq, self.get_pawn_bb(clr)) {
//             score += Self.tapered(KING_OPEN_FILES_PENALTY, phase);
//         }

//         if Some(c) = self.get_castle(clr) {
//             score += Self.tapered(CASTLE_BONUS_WT(c), phase);
//         }

//         let moves = get_king_mv(sq, own, enemy).count_ones();
//         score += moves * MOBILITY_WT;

//         return score;
//     }

//     #[inline(always)]
//     fn bishop_eval(&self, piece: &Piece, sq: usize, phase: (isize, isize)) -> isize {
//         let clr = piece.color();
//         let (own, enemy) = get_occupancy(piece, self);

//         // Material Evaluation
//         let score = Self::tapered(PIECE_WEIGHT[(self.kind() / 2).idx() - 1], phase);

//         // PSQT Evaluation
//         // TODO: ADD PSQT as tapered
//         score += Self::tapered((BISHOP_EVAL[phase.0][sq], BISHOP_EVAL[phase.1][sq]), phase);

//         // Do we have bishop pair evaluation
//         // TODO: Implement functions like this: To get Pawn Bitboard
//         if has_bishop_pair(self.bishop_bb(clr)) {
//             score += Self::tapered(BISHOP_PAIR_WT, phase);
//         }

//         let moves = get_bishop_mv(sq, own, enemy).count_ones();
//         score += moves * MOBILITY_WT;

//         // Checks if bishop is trapped (has extremely low number of moves)
//         if moves <= BISHOP_LOW_NUM_MOVES {
//             score += BISHOP_TRAP_PENALTY;
//         }

//         return score;
//     }

//     #[inline(always)]
//     fn rook_eval(&self, piece: &Piece, sq: usize, phase: (isize, isize)) -> isize {
//         let clr = piece.color();
//         let (own, enemy) = get_occupancy(piece, self);

//         // Material Evaluation
//         let score = Self::tapered(PIECE_WEIGHT[(self.kind() / 2).idx() - 1], phase);

//         // PSQT Evaluation
//         // TODO: ADD PSQT as tapered
//         score += Self::tapered((ROOK_EVAL[phase.0][sq], ROOK_EVAL[phase.1][sq]), phase);

//         // Is Rook on open or semi-open file evaluation
//         // TODO: Implement functions like this: To get Pawn Bitboard
//         if is_rook_on_open_file(sq, self.pawn_bb(piece.color())) {
//             score += Self::tapered(ROOK_OPEN_FILE_WT, phase);
//         } else if is_rook_on_semi_open_file(sq, self.pawn_bb(clr), self.pawn_bb(clr.opp())) {
//             score += Self::tapered(ROOK_SEMI_OPEN_FILE_WT, phase);
//         }

//         let moves = get_rook_mv(sq, own, enemy).count_ones();
//         score += moves * MOBILITY_WT;

//         // Checks if rook is trapped (has extremely low number of moves)
//         if moves <= ROOK_LOW_NUM_MOVES {
//             score += ROOK_TRAP_PENALTY;
//         }

//         return score;
//     }

//     #[inline(always)]
//     fn queen_eval(&self, piece: &Piece, sq: usize, phase: (isize, isize)) -> isize {
//         // Material Evaluation
//         let score = Self::tapered(PIECE_WEIGHT[(self.kind() / 2).idx() - 1], phase.0, phase.1);

//         // PSQT Evaluation
//         // TODO: ADD PSQT as tapered
//         score += New_Evaluation::tapered((QUEEN_EVAL[phase.0][sq], QUEEN_EVAL[phase.1][sq]), phase);

//         // NOTE: IT IS BAD TO ADD MOBILITY FOR THE QUEEN
//         // NOTE: IT MOVES EARLY WHICH IS BAD

//         return score;
//     }

//     // TODO: Change name to psqt
//     #[inline(always)]
//     fn piece_sq_eval(piece: &Piece, phase: usize, mut sq: usize) -> isize {
//         if piece.color().is_white() {
//             sq = OPP_SQ[sq]
//         }

//         match piece.kind() {
//             PAWN => PAWN_EVAL[phase][sq],
//             KNIGHT => KNIGHT_EVAL[phase][sq],
//             BISHOP => BISHOP_EVAL[phase][sq],
//             ROOK => ROOK_EVAL[phase][sq],
//             QUEEN => QUEEN_EVAL[phase][sq],
//             KING => KING_EVAL[phase][sq],
//             _ => panic!("Not the right type, Something is wrong"),
//         }
//     }

//     #[inline(always)]
//     fn material_balance(&self) -> isize {
//         let mut score = 0;
//         for piece in &PIECES {
//             score += piece.weight()
//                 * (self.bitboard[(piece + WHITE).idx()].count() as isize
//                     - self.bitboard[(piece + BLACK).idx()].count() as isize)
//         }
//         score
//     }

// #[inline(always)]
// fn insufficient_material(board: &Board) -> bool {
//     let (own, enemy) = self.both_occ_bb(board.color());
//     if (own | enemy).count_ones() < 4 {
//         let kings = self.bb(WHITE_KING) | self.bb(BLACK_KING);
//         let knights = self.bb(WHITE_KNIGHT) | self.bb(BLACK_KNIGHT);
//         let bishops = self.bb(WHITE_BISHOP) | self.bb(BLACK_BISHOP);
//         if (kings | knights | bishops) == (own | enemy) {
//             return true;
//         }
//     }
// }

//     fn determine_phase(&self) -> usize {
//         let mut phase = 0;
//         for piece in &CLR_PIECES {
//             phase += self.bitboard[piece.idx()].count() * GAME_PHASE_INCREMENT[piece.idx()];
//         }
//         phase
//     }
// }

// // NOTE: For Each Peace
// // 1. How much are on the board of that type (Material on the board)
// // 2. How much is the square they are sitting on valuable (md, eg)
// // 3. Unique parameters that give advantage (Rook -> Open Files, Rook -> Connectivity Pawn -> Passed Pawn, etc...)
// // 4. Mobility
// // 5.

// // fn eval_ending(&self, side: Color) -> Option<Score> {
// //     let occupied = self.bitboard(WHITE) | self.bitboard(BLACK);

// //     let kings = self.bitboard(WHITE | KING) | self.bitboard(BLACK | KING);
// //     if kings.count() < 2 {
// //         if self.bitboard(side | KING).count() == 0 {
// //             return Some(-INF); // Loss
// //         } else {
// //             return Some(INF); // Win
// //         }
// //     }

// //     // Draw by insufficient material
// //     if occupied.count() < 4 {
// //         let knights = self.bitboard(WHITE | KNIGHT) | self.bitboard(BLACK | KNIGHT);
// //         let bishops = self.bitboard(WHITE | BISHOP) | self.bitboard(BLACK | BISHOP);
// //         if (kings | knights | bishops) == occupied {
// //             return Some(0); // Draw
// //         }
// //     }

// //     None
// // }
