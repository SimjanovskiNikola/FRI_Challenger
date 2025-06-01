use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color::*;
use crate::engine::board::structures::piece::*;
use crate::engine::board::structures::square::get_rank;
use crate::engine::misc::bit_pos_utility::get_bit_rank;
use crate::engine::misc::bitboard::Bitboard;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::misc::bitboard::Iterator;
use crate::engine::misc::const_utility::RANK_BITBOARD;
use crate::engine::move_generator::bishop::get_bishop_mask;
use crate::engine::move_generator::bishop::get_bishop_mv;
use crate::engine::move_generator::bishop::has_bishop_pair;
use crate::engine::move_generator::king::get_king_mask;
use crate::engine::move_generator::king::get_king_mv;
use crate::engine::move_generator::king::has_near_open_files;
use crate::engine::move_generator::knight::get_knight_mask;
use crate::engine::move_generator::knight::get_knight_mv;
use crate::engine::move_generator::pawn::get_pawn_att_mask;
use crate::engine::move_generator::pawn::is_blocked_pawn;
use crate::engine::move_generator::pawn::is_isolated_pawn;
use crate::engine::move_generator::pawn::is_passed_pawn;
use crate::engine::move_generator::queen::get_queen_mask;
use crate::engine::move_generator::rook::get_rook_mask;
use crate::engine::move_generator::rook::get_rook_mv;
use crate::engine::move_generator::rook::is_rook_on_open_file;
use crate::engine::move_generator::rook::is_rook_on_semi_open_file;

// The Numbers (Tapered Eval) for the evaluation are taken from -> STOCKFISH SF_9
// All the evaluation are made for white side

#[rustfmt::skip]
pub const CLR_SQ: [[usize; 64]; 2] = [
    [
        0,  1,  2,  3,  4,  5,  6,  7,
        8,  9,  10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63,
    ],
    [ 
        56, 57, 58, 59, 60, 61, 62, 63,
        48, 49, 50, 51, 52, 53, 54, 55,
        40, 41, 42, 43, 44, 45, 46, 47,
        32, 33, 34, 35, 36, 37, 38, 39,
        24, 25, 26, 27, 28, 29, 30, 31,
        16, 17, 18, 19, 20, 21, 22, 23,
        8,  9,  10, 11, 12, 13, 14, 15,
        0,  1,  2,  3,  4,  5,  6,  7,
   ]
];

pub const CLR_RANK: [[usize; 8]; 2] = [[0, 1, 2, 3, 4, 5, 6, 7], [7, 6, 5, 4, 3, 2, 1, 0]];
pub const PSQT_FILE: [usize; 8] = [0, 1, 2, 3, 3, 2, 1, 0];

// NOTE: PAWN
const ISOLATED_PAWN_PEN: (isize, isize) = (-13, -18);
const BACKWARD_PAWN_PEN: (isize, isize) = (-24, -12);
const DOUBLED_PAWN_PEN: (isize, isize) = (-22, -44);
const PASSED_PAWN_REW: [[(isize, isize); 8]; 2] = [
    [(0, 0), (0, 0), (5, 2), (10, 5), (15, 10), (35, 20), (65, 30), (100, 50)], // UNPROTECTED PASSED PAWN [BASED ON RANK]
    [(0, 0), (0, 0), (10, 5), (20, 10), (35, 20), (55, 35), (80, 50), (125, 80)], // PROTECTED PASSED PAWN [BASED ON RANK]
];

// NOTE: KNIGHT
#[rustfmt::skip]
const KNIGHT_OUTPOST_REW: [(isize, isize); 2] = [
    (22, 6),     // UNPROTECTED BY PAWN
    (36, 12),    // PROTECTED BY PAWN
];

const KNIGHT_MOBILITY: [(isize, isize); 9] =
    [(-75, -76), (-57, -54), (-9, -28), (-2, -10), (6, 5), (14, 12), (22, 26), (29, 29), (36, 29)];

// NOTE: KING
const PCE_KING_ATT_WEIGHT: [isize; 6] = [0, 0, 78, 56, 45, 11];

// NOTE: BISHOP
const BISHOP_BATTERY_RW: (isize, isize) = (20, 30);

const BISHOP_PAIR_WT: (isize, isize) = (25, 25); //FIXME:

#[rustfmt::skip]
const BISHOP_OUTPOST_REW: [(isize, isize); 2] = [
    (9, 2),  // UNPROTECTED BY PAWN
    (15, 5), // PROTECTED BY PAWN
];

#[rustfmt::skip]
const BISHOP_MOBILITY: [(isize, isize); 14] = [
    (-48, -59), (-20, -23), (16, -3), (26, 13), (38, 24), (51, 42), (55, 54), 
    (63, 57), (63, 65), (68, 73), (81, 78), (81, 86), (91, 88), (98, 97),
];

// NOTE: ROOK
const ROOK_BATTERY_RW: (isize, isize) = (20, 30); // MINE EVAL
const ROOK_ON_PAWN: (isize, isize) = (8, 24);

#[rustfmt::skip]
const ROOK_FILE_RW: [(isize, isize); 2] = [
    (20, 7),    // SEMI-OPEN FILE 
    (45, 20)    // OPEN FILE
];

#[rustfmt::skip]
const ROOK_MOBILITY: [(isize, isize); 15] = [
    (-58, -76), (-27, -18), (-15, 28), (-10, 55), (-5, 69), (-2, 82), (9, 112), (16, 118),
    (30, 132), (29, 142), (32, 155), (38, 165), (46, 166), (48, 169), (58, 171),
];

// NOTE: QUEEN
#[rustfmt::skip]
const QUEEN_MOBILITY: [(isize, isize); 28] = [
    (-39, -36), (-21, -15), (3, 8), (3, 18), (14, 34), (22, 54), (28, 61),
    (41, 73), (43, 79), (48, 92), (56, 94), (60, 104), (60, 113), (66, 120),
    (67, 123), (70, 126), (71, 133), (73, 136), (79, 140), (88, 143), (88, 148),
    (99, 166), (102, 170), (102, 175), (106, 184), (109, 191), (113, 206), (116, 212),
];

// NOTE: ALL PEACES

// const PIECE_WEIGHT: [(isize, isize); 6] =
//     [(82, 94), (337, 281), (0, 0), (365, 297), (477, 512), (1025, 936)]
const PIECE_WEIGHT: [(isize, isize); 6] =
    [(124, 206), (781, 854), (0, 0), (825, 915), (1276, 1380), (2538, 2682)];

const HANGING_PEN: (isize, isize) = (-69, -36);

const GAME_PHASE_INCREMENT: [usize; 6] = [0, 1, 0, 1, 2, 4];

// PSQT Table
#[rustfmt::skip]
const PSQT: [[(isize, isize); 64]; 6] = [
    [ // Pawn
        (  0, 0), (  0, 0), (  0, 0), ( 0, 0), ( 0, 0), (  0, 0), (  0, 0), (  0, 0),
        (  0, 0), (  0, 0), (  0, 0), ( 0, 0), ( 0, 0), (  0, 0), (  0, 0), (  0, 0),
        (-11, 7), (  6,-4), (  7, 8), ( 3,-2), ( 3,-2), (  7, 8), (  6,-4), (-11, 7),
        (-18,-4), ( -2,-5), ( 19, 5), (24, 4), (24, 4), ( 19, 5), ( -2,-5), (-18,-4),
        (-17, 3), ( -9, 3), ( 20,-8), (35,-3), (35,-3), ( 20,-8), ( -9, 3), (-17, 3),
        ( -6, 8), (  5, 9), (  3, 7), (21,-6), (21,-6), (  3, 7), (  5, 9), ( -6, 8),
        ( -6, 8), ( -8,-5), ( -6, 2), (-2, 4), (-2, 4), ( -6, 2), ( -8,-5), ( -6, 8),
        ( -4, 3), ( 20,-9), ( -8, 1), (-4,18), (-4,18), ( -8, 1), ( 20,-9), ( -4, 3)
    ],
    [ // Knight
        (-161,-105), (-96,-82), (-80,-46), (-73,-14), (-73,-14), (-80,-46), (-96,-82), (-161,-105),
        ( -83, -69), (-43,-54), (-21,-17), (-10,  9), (-10,  9), (-21,-17), (-43,-54), ( -83, -69),
        ( -71, -50), (-22,-39), (  0, -7), (  9, 28), (  9, 28), (  0, -7), (-22,-39), ( -71, -50),
        ( -25, -41), ( 18,-25), ( 43,  6), ( 47, 38), ( 47, 38), ( 43,  6), ( 18,-25), ( -25, -41),
        ( -26, -46), ( 16,-25), ( 38,  3), ( 50, 40), ( 50, 40), ( 38,  3), ( 16,-25), ( -26, -46),
        ( -11, -54), ( 37,-38), ( 56, -7), ( 65, 27), ( 65, 27), ( 56, -7), ( 37,-38), ( -11, -54),
        ( -63, -65), (-19,-50), (  5,-24), ( 14, 13), ( 14, 13), (  5,-24), (-19,-50), ( -63, -65),
        (-195,-109), (-67,-89), (-42,-50), (-29,-13), (-29,-13), (-42,-50), (-67,-89), (-195,-109)
    ],
    [ // Bihop
        (-44,-58), (-13,-31), (-25,-37), (-34,-19), (-34,-19), (-25,-37), (-13,-31), (-44,-58),
        (-20,-34), ( 20, -9), ( 12,-14), (  1,  4), (  1,  4), ( 12,-14), ( 20, -9), (-20,-34),
        ( -9,-23), ( 27,  0), ( 21, -3), ( 11, 16), ( 11, 16), ( 21, -3), ( 27,  0), ( -9,-23),
        (-11,-26), ( 28, -3), ( 21, -5), ( 10, 16), ( 10, 16), ( 21, -5), ( 28, -3), (-11,-26),
        (-11,-26), ( 27, -4), ( 16, -7), (  9, 14), (  9, 14), ( 16, -7), ( 27, -4), (-11,-26),
        (-17,-24), ( 16, -2), ( 12,  0), (  2, 13), (  2, 13), ( 12,  0), ( 16, -2), (-17,-24),
        (-23,-34), ( 17,-10), (  6,-12), ( -2,  6), ( -2,  6), (  6,-12), ( 17,-10), (-23,-34),
        (-35,-55), (-11,-32), (-19,-36), (-29,-17), (-29,-17), (-19,-36), (-11,-32), (-35,-55),
    ],
    [ // Rook
        (-25, 0), (-16, 0), (-16, 0), (-9, 0), (-9, 0), (-16, 0), (-16, 0), (-25, 0),
        (-21, 0), ( -8, 0), ( -3, 0), ( 0, 0), ( 0, 0), ( -3, 0), ( -8, 0), (-21, 0),
        (-21, 0), ( -9, 0), ( -4, 0), ( 2, 0), ( 2, 0), ( -4, 0), ( -9, 0), (-21, 0),
        (-22, 0), ( -6, 0), ( -1, 0), ( 2, 0), ( 2, 0), ( -1, 0), ( -6, 0), (-22, 0),
        (-22, 0), ( -7, 0), (  0, 0), ( 1, 0), ( 1, 0), (  0, 0), ( -7, 0), (-22, 0),
        (-21, 0), ( -7, 0), (  0, 0), ( 2, 0), ( 2, 0), (  0, 0), ( -7, 0), (-21, 0),
        (-12, 0), (  4, 0), (  8, 0), (12, 0), (12, 0), (  8, 0), (  4, 0), (-12, 0),
        (-23, 0), (-15, 0), (-11, 0), (-5, 0), (-5, 0), (-11, 0), (-15, 0), (-23, 0),
    ],
    [ // Queen
        ( 0,-71), (-4,-56), (-3,-42), (-1,-29), (-1,-29), (-3,-42), (-4,-56), ( 0,-71),
        (-4,-56), ( 6,-30), ( 9,-21), ( 8, -5), ( 8, -5), ( 9,-21), ( 6,-30), (-4,-56),
        (-2,-39), ( 6,-17), ( 9, -8), ( 9,  5), ( 9,  5), ( 9, -8), ( 6,-17), (-2,-39),
        (-1,-29), ( 8, -5), (10,  9), ( 7, 19), ( 7, 19), (10,  9), ( 8, -5), (-1,-29),
        (-3,-27), ( 9, -5), ( 8, 10), ( 7, 21), ( 7, 21), ( 8, 10), ( 9, -5), (-3,-27),
        (-2,-40), ( 6,-16), ( 8,-10), (10,  3), (10,  3), ( 8,-10), ( 6,-16), (-2,-40),
        (-2,-55), ( 7,-30), ( 7,-21), ( 6, -6), ( 6, -6), ( 7,-21), ( 7,-30), (-2,-55),
        (-1,-74), (-4,-55), (-1,-43), ( 0,-30), ( 0,-30), (-1,-43), (-4,-55), (-1,-74),
    ],
    [ // King
        (267,  0), (320, 48), (270, 75), (195, 84), (195, 84), (270, 75), (320, 48), (267,  0),
        (264, 43), (304, 92), (238,143), (180,132), (180,132), (238,143), (304, 92), (264, 43),
        (200, 83), (245,138), (176,167), (110,165), (110,165), (176,167), (245,138), (200, 83),
        (177,106), (185,169), (148,169), (110,179), (110,179), (148,169), (185,169), (177,106),
        (149,108), (177,163), (115,200), ( 66,203), ( 66,203), (115,200), (177,163), (149,108),
        (118, 95), (159,155), ( 84,176), ( 41,174), ( 41,174), ( 84,176), (159,155), (118, 95),
        ( 87, 50), (128, 99), ( 63,122), ( 20,139), ( 20,139), ( 63,122), (128, 99), ( 87, 50),
        ( 63,  9), ( 88, 55), ( 47, 80), (  0, 90), (  0, 90), ( 47, 80), ( 88, 55), ( 63,  9),
    ]
  ];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Evaluation {
    pub outpost: [Bitboard; 2],
    pub king_ring: [Bitboard; 2],
    pub attacked_by: [Bitboard; 14],
    pub defended_by: [Bitboard; 14],
    pub attacked_by_2: [Bitboard; 2],
    pub king_att_weight: [isize; 2],
    pub king_att_count: [usize; 2],
    pub defend_map: [Bitboard; 2],
    pub attack_map: [Bitboard; 2],
    pub phase: (isize, isize),
    pub score: isize,
}

impl Evaluation {
    pub fn init() -> Self {
        Self {
            outpost: [0; 2],
            king_ring: [0; 2],
            attacked_by: [0; 14],
            defended_by: [0; 14],
            attacked_by_2: [0; 2],
            king_att_weight: [0; 2],
            king_att_count: [0; 2],
            defend_map: [0; 2],
            attack_map: [0; 2],
            phase: (0, 0),
            score: 0,
        }
    }

    pub fn reset(&mut self) {
        self.outpost.fill(0);
        self.king_ring.fill(0);
        self.attacked_by.fill(0);
        self.defended_by.fill(0);
        self.attacked_by_2.fill(0);
        self.king_att_weight.fill(0);
        self.king_att_count.fill(0);
        self.defend_map.fill(0);
        self.attack_map.fill(0);
        self.phase = (0, 0);
        self.score = 0;
    }
}

//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
const DOUBLE_PAWN_WT: isize = -15;
const BLOCKED_PAWN_WT: isize = -15;
const ISOLATED_PAWN_WT: isize = -15;
// const MOBILITY_WT: isize = 1;
// const ROOK_OPEN_FILE_WT: (isize, isize) = (8, 20); // NOTE: TAPERED ACHIEVED
// const ROOK_SEMI_OPEN_FILE_WT: (isize, isize) = (4, 10); // NOTE: TAPERED ACHIEVED
const PASSED_PAWN_WT: [[isize; 8]; 2] =
    [[0, 5, 10, 20, 35, 60, 100, 0], [0, 100, 60, 35, 20, 10, 5, 0]]; // NOTE: TAPERED ACHIEVED
                                                                      // const ROOK_TRAP_PENALTY: isize = -50;
                                                                      // const ROOK_LOW_NUM_MOVES: isize = 2;

const BISHOP_TRAP_PENALTY: isize = -50;
const BISHOP_LOW_NUM_MOVES: isize = 1;

const KNIGHT_TRAP_PENALTY: isize = -50;
const KNIGHT_LOW_NUM_MOVES: isize = 2;
const KNIGHT_VALUE_PER_PAWN_WT: (isize, isize) = (0, 1);

pub const CASTLE_BONUS_WT: [(isize, isize); 4] = [(30, 0), (20, 0), (30, 0), (20, 0)];
pub const KING_OPEN_FILES_PENALTY: (isize, isize) = (-40, 0);

//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:
//   DEPRECATE:

pub trait EvaluationTrait {
    fn evaluation(&mut self) -> isize;

    // fn material_balance(&self) -> isize;

    fn piece_eval(&self, piece: Piece, sq: usize) -> isize;
    // fn piece_sq_eval(piece: &Piece, phase: usize, sq: usize) -> isize; //FIXME: Fix Phase to enum

    fn determine_phase(&self) -> usize;
    fn tapered(&self, value: (isize, isize)) -> isize;
    fn material_eval(piece: Piece) -> (isize, isize);
    fn psqt_eval(piece: Piece, sq: usize) -> (isize, isize);
    fn hanging_peace_eval(&self, piece: Piece, sq: usize) -> isize;
    fn insufficient_material(&self) -> bool;

    fn pawn_eval(&self, piece: Piece, sq: usize) -> isize;
    fn knight_eval(&self, piece: Piece, sq: usize) -> isize;
    fn king_eval(&self, piece: Piece, sq: usize) -> isize;
    fn bishop_eval(&self, piece: Piece, sq: usize) -> isize;
    fn rook_eval(&self, piece: Piece, sq: usize) -> isize;
    fn queen_eval(&self, piece: Piece, sq: usize) -> isize;

    fn get_mask(&self, piece: Piece, sq: usize) -> u64;
    fn mobility(&mut self, piece: Piece, sq: usize);
}

impl EvaluationTrait for Board {
    #[inline(always)]
    fn get_mask(&self, piece: Piece, sq: usize) -> u64 {
        let (own, enemy) = self.both_occ_bb(piece.color());
        match piece.kind() {
            PAWN => get_pawn_att_mask(sq, own, enemy, piece.color()),
            KNIGHT => get_knight_mask(sq, own, enemy, piece.color()),
            BISHOP => get_bishop_mask(sq, own, enemy, piece.color()),
            ROOK => get_rook_mask(sq, own, enemy, piece.color()),
            QUEEN => get_queen_mask(sq, own, enemy, piece.color()),
            KING => get_king_mask(sq, own, enemy, piece.color()),
            _ => panic!("Invalid Peace Type"),
        }
    }

    #[inline(always)]
    fn mobility(&mut self, piece: Piece, sq: usize) {
        let bb = self.get_mask(piece, sq);
        let (own, enemy) = self.both_occ_bb(piece.color());
        self.eval.attacked_by[piece.idx()] |= bb;
        self.eval.attack_map[piece.color().idx()] |= bb;
        self.eval.defend_map[piece.color().idx()] |= bb & own;

        // if true {
        //     self.eval.king_att_count[piece.color().opp().idx()] += 1;
        //     self.eval.king_att_weight[piece.color().opp().idx()] +=
        //         self.tapered(PIECE_WEIGHT[piece.arr_idx()])
        // }
    }

    // fn create_att_by_2(&mut self, piece: Piece, sq: usize) {}

    // fn is_defended(&mut self, piece: Piece, sq: usize) -> bool {
    //     self.eval.defend_map[piece.color().idx()].is_set(sq)
    // }

    #[inline(always)]
    fn tapered(&self, value: (isize, isize)) -> isize {
        (self.eval.phase.0 * value.0 + self.eval.phase.1 * value.1) / 24
    }

    fn material_eval(piece: Piece) -> (isize, isize) {
        PIECE_WEIGHT[piece.arr_idx()]
    }

    fn psqt_eval(piece: Piece, sq: usize) -> (isize, isize) {
        let fixed_sq = CLR_SQ[piece.color().idx()][sq];
        PSQT[piece.arr_idx()][fixed_sq]
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
    fn evaluation(&mut self) -> isize {
        self.eval.reset();

        let mut score: isize = 0;
        let mut phase: isize = 0;

        // Check if the game has sufficient material, otherwise it is a draw
        if self.insufficient_material() {
            return 0;
        }

        // TODO: PAWN STRUCTURE
        // TODO: KING SAFETY AND PAWN STRUCTURE AROUND KING

        // Initialize Mobility, Phase
        for piece in &CLR_PIECES {
            let mut bb = self.bb(*piece);
            phase += (bb.count() * GAME_PHASE_INCREMENT[piece.arr_idx()]) as isize;
            while let Some(sq) = bb.next() {
                self.mobility(*piece, sq);
            }
        }

        self.eval.phase = (phase.min(24), 24 - phase.min(24));

        // Evaluate every peace
        for piece in &CLR_PIECES {
            let mut bb = self.bb(*piece);
            while let Some(sq) = bb.next() {
                // Material Evaluation
                score += self.tapered(PIECE_WEIGHT[piece.arr_idx()]) * piece.color().sign();

                // PSQT Evaluation
                score += self.tapered(Self::psqt_eval(*piece, sq)) * piece.color().sign();

                // Custom Piece Evaluation
                score += self.piece_eval(*piece, sq) * piece.color().sign();
            }
        }

        return score * self.color().sign();
    }

    #[inline(always)]
    fn piece_eval(&self, piece: Piece, sq: usize) -> isize {
        match piece.kind() {
            PAWN => self.pawn_eval(piece, sq),
            KNIGHT => self.knight_eval(piece, sq),
            BISHOP => self.bishop_eval(piece, sq),
            ROOK => self.rook_eval(piece, sq),
            QUEEN => self.queen_eval(piece, sq),
            KING => self.king_eval(piece, sq),
            _ => 0,
            // _ => panic!(" Not the right type, Something is wrong"),
        }
    }

    fn hanging_peace_eval(&self, piece: Piece, sq: usize) -> isize {
        if self.eval.attack_map[piece.color().opp().idx()].is_set(sq)
            && !self.eval.defend_map[piece.color().idx()].is_set(sq)
        {
            self.tapered(HANGING_PEN)
        } else {
            0
        }
    }

    #[inline(always)]
    fn pawn_eval(&self, piece: Piece, sq: usize) -> isize {
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
    fn knight_eval(&self, piece: Piece, sq: usize) -> isize {
        let mut score = 0;
        let clr = piece.color();
        let (own, enemy) = self.both_occ_bb(piece.color());

        // Knight on outpost
        if self.eval.outpost[clr.opp().idx()].is_set(sq) {
            let b: usize = self.eval.defended_by[(PAWN + clr).idx()].is_set(sq).into();
            score += self.tapered(BISHOP_OUTPOST_REW[b])
        }

        // // Mobility
        let moves = get_knight_mv(sq, own, enemy, clr);
        let safe_from_pawns = moves & !self.eval.attacked_by[(PAWN + clr.opp()).idx()]; // Safe from instant capturing from pawns
        let good_moves = (safe_from_pawns & !self.eval.attack_map[clr.opp().idx()])     // Squares not attacked at all
            | (safe_from_pawns                                                          // Squares attacked but also defended
                & self.eval.attack_map[clr.opp().idx()]
                & self.eval.defend_map[clr.idx()]);
        score += self.tapered(KNIGHT_MOBILITY[good_moves.count()]);

        // Hanging Peace
        score += self.hanging_peace_eval(piece, sq);

        return score;
    }

    #[inline(always)]
    fn king_eval(&self, piece: Piece, sq: usize) -> isize {
        let mut score = 0;
        let clr = piece.color();
        let (own, enemy) = self.both_occ_bb(piece.color()); // get_occupancy(piece, self);

        // Open Files near the king
        if has_near_open_files(sq, self.pawn_bb(clr)) {
            score += self.tapered(KING_OPEN_FILES_PENALTY);
        }

        // // Castling
        // if let Some(c) = self.get_castle(clr) {
        //     score += Self::tapered(CASTLE_BONUS_WT[c], phase);
        // }

        // Mobility
        // let moves = get_king_mv(sq, own, enemy, clr).count_ones() as isize;
        // score += moves * MOBILITY_WT;

        return score;
    }

    #[inline(always)]
    fn bishop_eval(&self, piece: Piece, sq: usize) -> isize {
        let mut score = 0;
        let clr = piece.color();
        let (own, enemy) = self.both_occ_bb(piece.color());

        // Bishop on outpost
        if self.eval.outpost[clr.opp().idx()].is_set(sq) {
            let b: usize = self.eval.defended_by[(PAWN + clr).idx()].is_set(sq).into();
            score += self.tapered(BISHOP_OUTPOST_REW[b])
        }

        // Bishop Pair
        if has_bishop_pair(self.bishop_bb(piece.color())) {
            score += self.tapered(BISHOP_PAIR_WT);
        }

        // // Mobility
        let moves = get_bishop_mv(sq, own, enemy, clr);
        let safe_from_pawns = moves & !self.eval.attacked_by[(PAWN + clr.opp()).idx()]; // Safe from instant capturing from pawns
        let good_moves = (safe_from_pawns & !self.eval.attack_map[clr.opp().idx()])     // Squares not attacked at all
            | (safe_from_pawns                                                          // Squares attacked but also defended
                & self.eval.attack_map[clr.opp().idx()]
                & self.eval.defend_map[clr.idx()]);
        score += self.tapered(BISHOP_MOBILITY[good_moves.count()]);

        // Hanging Peace
        if self.eval.attack_map[clr.opp().idx()].is_set(sq)
            && !self.eval.defend_map[clr.idx()].is_set(sq)
        {
            score += self.tapered(HANGING_PEN);
        }

        return score;
    }

    #[inline(always)]
    fn rook_eval(&self, piece: Piece, sq: usize) -> isize {
        let mut score = 0;
        let clr = piece.color();
        let (own, enemy) = self.both_occ_bb(piece.color());
        let rank = get_rank(sq);

        // Invasion, aligning with enemy pawns
        if CLR_RANK[clr.idx()][rank] >= 5 {
            score += self.tapered(ROOK_ON_PAWN)
                * (self.pawn_bb(clr.opp()) & RANK_BITBOARD[rank]).count() as isize;
        }

        // Open File / Semi-Open File
        if is_rook_on_open_file(sq, self.pawn_bb(clr), self.pawn_bb(clr.opp())) {
            score += self.tapered(ROOK_FILE_RW[1]);
        } else if is_rook_on_semi_open_file(sq, self.pawn_bb(clr)) {
            score += self.tapered(ROOK_FILE_RW[0]);
        }

        // // Mobility
        // FIXME: What if
        let moves = get_rook_mask(sq, own, enemy, clr);
        let safe_from_pieces = (moves & !own)
            & !(self.eval.attacked_by[(PAWN + clr.opp()).idx()]
                | self.eval.attacked_by[(KNIGHT + clr.opp()).idx()]
                | self.eval.attacked_by[(BISHOP + clr.opp()).idx()]); // Safe from instant capturing from pawns, knights and bishops

        let good_moves = (safe_from_pieces & !self.eval.attack_map[clr.opp().idx()])     // Squares not attacked at all
                    | (safe_from_pieces                                                          // Squares attacked but also defended
                        & self.eval.attack_map[clr.opp().idx()]
                        & self.eval.defend_map[clr.idx()]);
        score += self.tapered(ROOK_MOBILITY[good_moves.count()]);

        // Connected Rooks or Battery (Rook - Rook)
        if ((moves & own) & self.rook_bb(clr)) != 0 {
            score += self.tapered(ROOK_BATTERY_RW)
        }

        // TODO: Trapped Rook By King, add even more if it cannot castle

        // Hanging Peace
        score += self.hanging_peace_eval(piece, sq);

        return score;
    }

    #[inline(always)]
    fn queen_eval(&self, piece: Piece, sq: usize) -> isize {
        let mut score = 0;
        let clr = piece.color();
        let (own, enemy) = self.both_occ_bb(piece.color());

        let moves = get_queen_mask(sq, own, enemy, clr);
        let safe_from_pieces = (moves & !own)
            & !(self.eval.attacked_by[(PAWN + clr.opp()).idx()]
                | self.eval.attacked_by[(KNIGHT + clr.opp()).idx()]
                | self.eval.attacked_by[(BISHOP + clr.opp()).idx()] // Safe from instant capturing from pawns, knights, bishops and rooks
                | self.eval.attacked_by[(ROOK + clr.opp()).idx()]);

        let good_moves = (safe_from_pieces & !self.eval.attack_map[clr.opp().idx()])     // Squares not attacked at all
                    | (safe_from_pieces                                                          // Squares attacked but also defended
                        & self.eval.attack_map[clr.opp().idx()]
                        & self.eval.defend_map[clr.idx()]);
        score += self.tapered(QUEEN_MOBILITY[good_moves.count()]);

        score += self.hanging_peace_eval(piece, sq);
        // println!("Queen Mobility{:?}", self.tapered(QUEEN_MOBILITY[good_moves.count()]));

        // Battery (Queen-Bishop / Queen-Rook)
        if ((moves & own) & self.rook_bb(clr)) != 0 {
            score += self.tapered(ROOK_BATTERY_RW)
        }

        if ((moves & own) & self.bishop_bb(clr)) != 0 {
            score += self.tapered(BISHOP_BATTERY_RW)
        }

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
