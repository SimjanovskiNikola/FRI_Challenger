use crate::engine::misc::bitboard::Bitboard;
use std::array;

// ************************************************
//                      COMMON                    *
// ************************************************

// First 3 Ranks and 4 Center Files for every Color
pub const CLR_CENTER: [u64; 2] = [1010580480, 16954726998343680];

// Absolute Ranks based on color
pub const CLR_RANK: [[usize; 8]; 2] = [[0, 1, 2, 3, 4, 5, 6, 7], [7, 6, 5, 4, 3, 2, 1, 0]];

// DEPRECATE:
pub const PSQT_FILE: [usize; 8] = [0, 1, 2, 3, 3, 2, 1, 0];

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

// Middle & Endgame limits (Used to determine phase)
pub const MG_LIMIT: isize = 15258;
pub const EG_LIMIT: isize = 3915;

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

pub const KNIGHT_MOBILITY: [(isize, isize); 9] =
    [(-62, -81), (-53, -56), (-12, -31), (-4, -16), (3, 5), (13, 11), (22, 17), (28, 20), (33, 25)];

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

#[rustfmt::skip]
pub const BISHOP_MOBILITY: [(isize, isize); 14] = [
    (-48, -59), (-20, -23), (16, -3), (26, 13), (38, 24), (51, 42), (55, 54), 
    (63, 57), (63, 65), (68, 73), (81, 78), (81, 86), (91, 88), (98, 97),
];

// ************************************************
//                       ROOK                     *
// ************************************************
#[rustfmt::skip]
pub const ROOK_MOBILITY: [(isize, isize); 15] = [
    (-60, -78), (-20, -17), (2, 23), (3, 39), (3, 70), (11, 99), (22, 103), (31, 121),
    (40, 134), (40, 139), (41, 158), (48, 164), (57, 168), (57, 169), (62, 172),
];

#[rustfmt::skip]
const ROOK_FILE_RW: [(isize, isize); 2] = [
    (20, 7),    // SEMI-OPEN FILE 
    (45, 20)    // OPEN FILE
];

// DEPRECATE:
const ROOK_BATTERY_RW: (isize, isize) = (20, 30); // MINE EVAL
const ROOK_ON_PAWN: (isize, isize) = (8, 24);

// ************************************************
//                      QUEEN                     *
// ************************************************
#[rustfmt::skip]
pub const QUEEN_MOBILITY: [(isize, isize); 28] = [
    (-30, -48), (-12, -30), (-8, -7), (-9, 19), (20, 40), (23, 55), (23, 59),
    (35, 75), (38, 78), (53, 96), (64, 96), (65, 100), (65, 121), (66, 127),
    (67, 131), (67, 133), (72, 136), (72, 141), (77, 147), (79, 150), (93, 151),
    (108, 168), (108, 168), (108, 171), (110, 182), (114, 182), (114, 192), (116, 219),
];

// ************************************************
//                   ALL PEACES                   *
// ************************************************

// NOTE: 1. MATERIAL EVALUATION
#[rustfmt::skip]
pub const PIECE_MATERIAL: [(isize, isize); 6] = [
    ( 124, 206), // Pawn
    ( 781, 854), // Knight
    (   0,   0), // King
    ( 825, 915), // Bishop
    (1276,1380), // Rook
    (2538,2682), // Queen 
];

// NOTE: 2. PSQT EVALUATION
#[rustfmt::skip]
pub const PSQT: [[(isize, isize); 64]; 6] = [
    [ // Pawn NOTE: DONE
        (   0,   0), (   0,   0), (   0,   0), (   0,   0), (   0,   0), (   0,   0), (   0,   0), (   0,   0),
        (   3, -10), (   3,  -6), (  10,  10), (  19,   0), (  16,  14), (  19,   7), (   7,  -5), (  -5, -19),
        (  -9, -10), ( -15, -10), (  11, -10), (  15,   4), (  32,   4), (  22,   3), (   5,  -6), ( -22,  -4),
        (  -4,   6), ( -23,  -2), (   6,  -8), (  20,  -4), (  40, -13), (  17, -12), (   4, -10), (  -8,  -9),
        (  13,  10), (   0,   5), ( -13,   4), (   1,  -5), (  11,  -5), (  -2,  -5), ( -13,  14), (   5,   9),
        (   5,  28), ( -12,  20), (  -7,  21), (  22,  28), (  -8,  30), (  -5,   7), ( -15,   6), (  -8,  13),
        (  -7,   0), (   7, -11), (  -3,  12), ( -13,  21), (   5,  25), ( -16,  19), (  10,   4), (  -8,   7),
        (   0,   0), (   0,   0), (   0,   0), (   0,   0), (   0,   0), (   0,   0), (   0,   0), (   0,   0),
    ],
    [ // Knight NOTE: DONE
        (-175, -96), ( -92, -65), ( -74, -49), ( -73, -21), ( -73, -21), ( -74, -49), ( -92, -65), (-175, -96),
        ( -77, -67), ( -41, -54), ( -27, -18), ( -15,  8), ( -15,  8), ( -27, -18), ( -41, -54), ( -77, -67),
        ( -61, -40), ( -17, -27), (   6,  -8), (  12,  29), (  12,  29), (   6,  -8), ( -17, -27), ( -61, -40),
        ( -35, -35), (   8, -2 ), (  40,  13), (  49,  28), (  49,  28), (  40,  13), (   8,  -2), ( -35, -35),
        ( -34, -45), (  13, -16), (  44,   9), (  51,  39), (  51,  39), (  44,   9), (  13, -16), ( -34, -45),
        ( -9,  -51), (  22, -44), (  58, -16), (  53,  17), (  53,  17), (  58, -16), (  22, -44), ( -9, - 51),
        ( -67, -69), ( -27, -50), (   4, -51), (  37,  12), (  37,  12), (   4, -51), ( -27, -50), ( -67, -69),
        (-201,-100), ( -83, -88), ( -56, -56), ( -26, -17), ( -26, -17), ( -56, -56), ( -83, -88), (-201,-100),
    ],
    [ // King NOTE: DONE
        ( 271,   1), ( 327,  45), ( 271,  85), ( 198,  76), ( 198,  76), ( 271,  85), ( 327,  45), ( 271,   1),
        ( 278,  53), ( 303, 100), ( 234, 133), ( 179, 135), ( 179, 135), ( 234, 133), ( 303, 100), ( 278,  53),
        ( 195,  88), ( 258, 130), ( 169, 169), ( 120, 175), ( 120, 175), ( 169, 169), ( 258, 130), ( 195,  88),
        ( 164, 103), ( 190, 156), ( 138, 172), (  98, 172), (  98, 172), ( 138, 172), ( 190, 156), ( 164, 103),
        ( 154,  96), ( 179, 166), ( 105, 199), (  70, 199), (  70, 199), ( 105, 199), ( 179, 166), ( 154,  96),
        ( 123,  92), ( 145, 172), (  81, 184), (  31, 191), (  31, 191), (  81, 184), ( 145, 172), ( 123,  92),
        (  88,  47), ( 120, 121), (  65, 116), (  33, 131), (  33, 131), (  65, 116), ( 120, 121), (  88,  47),
        (  59,  11), (  89,  59), (  45,  73), (  -1,  78), (  -1,  78), (  45,  73), (  89,  59), (  59,  11),
    ],
    [ // Bishop NOTE: DONE
        ( -53, -57), (  -5, -30), (  -8, -37), ( -23, -12), ( -23, -12), (  -8, -37), (  -5, -30), ( -53, -57),
        ( -15, -37), (   8, -13), (  19, -17), (   4,   1), (   4,   1), (  19, -17), (   8, -13), ( -15, -37),
        (  -7, -16), (  21,  -1), (  -5,  -2), (  17,  10), (  17,  10), (  -5,  -2), (  21,  -1), (  -7, -16),
        (  -5, -20), (  11,  -6), (  25,   0), (  39,  17), (  39,  17), (  25,   0), (  11,  -6), (  -5, -20),
        ( -12, -17), (  29,  -1), (  22, -14), (  31,  15), (  31,  15), (  22, -14), (  29,  -1), ( -12, -17),
        ( -16, -30), (   6,   6), (   1,   4), (  11,   6), (  11,   6), (   1,   4), (   6,   6), ( -16, -30),
        ( -17, -31), ( -14, -20), (   5,  -1), (   0,   1), (   0,   1), (   5,  -1), ( -14, -20), ( -17, -31),
        ( -48, -46), (   1, -42), ( -14, -37), ( -23, -24), ( -23, -24), ( -14, -37), (   1, -42), ( -48, -46),
    ],
    [ // Rook NOTE: DONE
        ( -31,  -9), ( -20, -13), ( -14, -10), (  -5,  -9), (  -5,  -9), ( -14, -10), ( -20, -13), ( -31,  -9),
        ( -21, -12), ( -13,  -9), (  -8,  -1), (   6,  -2), (   6,  -2), (  -8,  -1), ( -13,  -9), ( -21, -12),
        ( -25,   6), ( -11,  -8), (  -1,  -2), (   3,  -6), (   3,  -6), (  -1,  -2), ( -11,  -8), ( -25,   6),
        ( -13,  -6), (  -5,   1), (  -4,  -9), (  -6,   7), (  -6,   7), (  -4,  -9), (  -5,   1), ( -13,  -6),
        ( -27,  -5), ( -15,   8), (  -4,   7), (   3,  -6), (   3,  -6), (  -4,   7), ( -15,   8), ( -27,  -5),
        ( -22,   6), (  -2,   1), (   6,  -7), (  12,  10), (  12,  10), (   6,  -7), (  -2,   1), ( -22,   6),
        (  -2,   4), (  12,   5), (  16,  20), (  18,  -5), (  18,  -5), (  16,  20), (  12,   5), (  -2,   4),
        ( -17,  18), ( -19,   0), (  -1,  19), (   9,  13), (   9,  13), (  -1,  19), ( -19,   0), ( -17,  18),
    ],
    [ // Queen  NOTE: DONE
        (   3, -69), (  -5, -57), (  -5, -47), (   4, -26), (   4, -26), (  -5, -47), (  -5, -57), (   3, -69),
        (  -3, -55), (   5, -31), (   8, -22), (  12,  -4), (  12,  -4), (   8, -22), (   5, -31), (  -3, -55),
        (  -3, -39), (   6, -18), (  13,  -9), (   7,   3), (   7,   3), (  13,  -9), (   6, -18), (  -3, -39),
        (   4, -23), (   5,  -3), (   9,  13), (   8,  24), (   8,  24), (   9,  13), (   5,  -3), (   4, -23),
        (   0, -29), (  14,  -6), (  12,   9), (   5,  21), (   5,  21), (  12,   9), (  14,  -6), (   0, -29),
        (  -4, -38), (  10, -18), (   6, -12), (   8,   1), (   8,   1), (   6, -12), (  10, -18), (  -4, -38),
        (  -5, -50), (   6, -27), (  10, -24), (   8,  -8), (   8,  -8), (  10, -24), (   6, -27), (  -5, -50),
        (  -2, -75), (  -2, -52), (   1, -43), (  -2, -36), (  -2, -36), (   1, -43), (  -2, -52), (  -2, -75),
    ],
];

// NOTE: 3. IMBALANCE EVALUATION
// Quadratic interaction bonuses for own peaces NOTE: DONE
pub const QUADRATIC_OURS: [[isize; 6]; 6] = [
    [0, 0, 0, 0, 0, 0],   // Bishop pair DEPRECATE: Refactor This as it is not needed
    [40, 38, 0, 0, 0, 0], // Pawn
    [32, 255, -62, 0, 0, 0], // Knight
    [0, 104, 4, 0, 0, 0], // Bishop
    [-26, -2, 47, 105, -208, 0], // Rook
    [-189, 24, 117, 133, -134, -6], // Queen
];

// Quadratic interaction bonuses for their peaces NOTE: DONE
pub const QUADRATIC_THEIRS: [[isize; 6]; 6] = [
    [0, 0, 0, 0, 0, 0],    // Bishop pair DEPRECATE: Refactor This as it is not needed
    [36, 0, 0, 0, 0, 0],   // Pawn
    [9, 63, 0, 0, 0, 0],   // Knight
    [59, 65, 42, 0, 0, 0], // Bishop
    [46, 39, 24, -24, 0, 0], // Rook
    [97, 100, -42, 137, 268, 0], // Queen
];

// NOTE: 4. PAWNS EVALUATION
// NOTE: 5. PEACES EVALUATION
// NOTE: 6. MOBILITY EVALUATION
// NOTE: 7. THREATS EVALUATION
pub const ROOK_THREAT: [(isize, isize); 6] =
    [(3, 46), (37, 68), (0, 0), (42, 60), (0, 38), (58, 41)];

pub const MINOR_THREAT: [(isize, isize); 6] =
    [(5, 32), (57, 41), (0, 0), (77, 56), (88, 119), (79, 161)];

// NOTE: 8. PASSED PAWN EVALUATION
// NOTE: 9. SPACE EVALUATION
// NOTE: 10. KING EVALUATION
// NOTE: 11. TEMPO EVALUATION
pub const TEMPO_WT: isize = 28;
