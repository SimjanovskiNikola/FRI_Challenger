use crate::engine::misc::{bitboard::Bitboard, const_utility::RANK_BITBOARD};
use std::array;

// ************************************************
//                      COMMON                    *
// ************************************************

// First 3 Ranks and 4 Center Files for every Color
pub const CLR_CENTER: [u64; 2] = [1010580480, 16954726998343680];

// Absolute Ranks based on color
pub const CLR_RANK: [[usize; 8]; 2] = [[0, 1, 2, 3, 4, 5, 6, 7], [7, 6, 5, 4, 3, 2, 1, 0]];

// Game Phase Increment
pub const GAME_PHASE_INCREMENT: [usize; 6] = [0, 1, 0, 1, 2, 4];

// Absolute Square based on color
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
    ],
];

// ************************************************
//                       PAWN                     *
// ************************************************

// NOTE: PAWN
pub const BLOCKED_RANKS: [u64; 2] = [281470681743360, 4294901760];
const ISOLATED_PAWN_PEN: (isize, isize) = (-13, -18);
const BACKWARD_PAWN_PEN: (isize, isize) = (-24, -12);
const DOUBLED_PAWN_PEN: (isize, isize) = (-22, -44);
// const PASSED_PAWN_REW: [[(isize, isize); 8]; 2] = [
//     [(0, 0), (0, 0), (5, 2), (10, 5), (15, 10), (35, 20), (65, 30), (100, 50)], // UNPROTECTED PASSED PAWN [BASED ON RANK]
//     [(0, 0), (0, 0), (10, 5), (20, 10), (35, 20), (55, 35), (80, 50), (125, 80)], // PROTECTED PASSED PAWN [BASED ON RANK]
// ];
pub const PASSED_PAWN_REW: [[(isize, isize); 8]; 2] = [
    [(0, 0), (10, 28), (17, 33), (15, 41), (62, 72), (168, 177), (276, 260), (0, 0)],
    [(0, 0), (276, 260), (168, 177), (62, 72), (15, 41), (17, 33), (10, 28), (0, 0)],
];

// NOTE: KNIGHT
#[rustfmt::skip]
const KNIGHT_OUTPOST_REW: [(isize, isize); 2] = [
    (22, 6),     // UNPROTECTED BY PAWN
    (36, 12),    // PROTECTED BY PAWN
];

// NOTE: KING
const PCE_KING_ATT_WEIGHT: [isize; 6] = [0, 0, 78, 56, 45, 11];

// ************************************************
//                     BISHOP                     *
// ************************************************
const BISHOP_BATTERY_RW: (isize, isize) = (20, 30);

const BISHOP_PAIR_WT: (isize, isize) = (90, 90); //FIXME:

#[rustfmt::skip]
const BISHOP_OUTPOST_REW: [(isize, isize); 2] = [
    (9, 2),  // UNPROTECTED BY PAWN
    (15, 5), // PROTECTED BY PAWN
];

// ************************************************
//                       ROOK                     *
// ************************************************

#[rustfmt::skip]
const ROOK_FILE_RW: [(isize, isize); 2] = [
    (20, 7),    // SEMI-OPEN FILE 
    (45, 20)    // OPEN FILE
];

// DEPRECATE:
const ROOK_BATTERY_RW: (isize, isize) = (20, 30); // MINE EVAL
const ROOK_ON_PAWN: (isize, isize) = (8, 24);

// NOTE: 10. KING EVALUATION
