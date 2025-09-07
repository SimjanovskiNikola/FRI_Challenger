use crate::engine::board::board::Board;
use crate::engine::evaluation::common_eval::CommonEvalTrait;

pub struct SFEval<'a> {
    pub fen: &'a str,
    pub phase: isize,
    pub eval: isize,
    pub material: isize,
    pub psqt: isize,
    pub imbalance: isize,
    pub pawns: isize,
    pub piece: isize,
    pub mobility: isize,
    pub threats: isize,
    pub passed_pawn: isize,
    pub space: isize,
    pub king: isize,
    pub winnable: isize,
    pub tempo: isize,
}

pub const SF_EVAL: [SFEval; 11] = [
    SFEval {
        fen: "r3r1k1/3q1pp1/p2pb2p/Np6/1P1QPn2/5N1P/1P3PP1/R3R1K1 w - - 0 0",
        phase: 106,
        eval: -34,

        material: -46,
        psqt: -56,
        imbalance: 36,
        pawns: 6,
        piece: 52,
        mobility: -30,
        threats: 4,
        passed_pawn: 0,
        space: 12,
        king: -67,
        winnable: -123465, // FIXME:
        tempo: 28,
    },
    SFEval {
        fen: "1rb1r1k1/2q2pp1/1b1p2np/1pp5/3Pn3/1B2BNNP/1P1Q1PP1/R3R1K1 w - - 0 0",
        phase: 128,
        eval: -244,

        material: -124,
        psqt: -2,
        imbalance: -50,
        pawns: -91,
        piece: 55,
        mobility: 18,
        threats: -127,
        passed_pawn: 0,
        space: -21,
        king: 39,
        winnable: -123465, // FIXME:
        tempo: 28,
    },
    SFEval {
        fen: "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0",
        phase: 89,
        eval: 404,

        material: 148,
        psqt: -50,
        imbalance: 19,
        pawns: 40,
        piece: 40,
        mobility: 61,
        threats: 165,
        passed_pawn: 0,
        space: 0,
        king: 102,
        winnable: -123465, // FIXME:
        tempo: -28,
    },
    SFEval {
        fen: "rnb1k2r/2p1ppPp/5bn1/p1p5/P2p2p1/P2P1P1P/6P1/RNB1KBNR b KQkq - 0 0",
        phase: 89,
        eval: -444,

        material: -151,
        psqt: -90,
        imbalance: -65,
        pawns: -25,
        piece: 7,
        mobility: -124,
        threats: -71,
        passed_pawn: 233,
        space: 0,
        king: -124,
        winnable: -123465, // FIXME:
        tempo: -28,
    },
    SFEval {
        fen: "r1bqk1r1/1p1p1n2/p1n2pN1/2p1b2Q/2P1Pp2/1PN5/PB4PP/R4RK1 w q - 0 0",
        phase: 128,
        eval: -836,

        material: -825,
        psqt: 154,
        imbalance: -148,
        king: -135,
        mobility: 103,
        passed_pawn: 10,
        pawns: 93,
        piece: -11,
        space: -15,
        threats: -104,
        winnable: -123465, // FIXME:
        tempo: 28,
    },
    SFEval {
        fen: "r1n2N1k/2n2K1p/3pp3/5Pp1/b5R1/8/1PPP4/8 w - - 0 0",
        phase: 20,
        eval: -1476,

        material: -1743,
        psqt: 124,
        imbalance: -123,
        king: -187,
        mobility: -21,
        passed_pawn: 231,
        pawns: -46,
        piece: 159,
        space: 0,
        threats: 154,
        winnable: -64,
        tempo: 28,
    },
    SFEval {
        fen: "4rrk1/Rpp3pp/6q1/2PPn3/4p3/2N5/1P2QPPP/5RK1 w - - 0 0",
        phase: 88,
        eval: 204,

        material: 149,
        psqt: -35,
        imbalance: 29,
        king: -204,
        mobility: -25,
        passed_pawn: 67,
        pawns: 143,
        piece: -19,
        space: 0,
        threats: 91,
        winnable: -3,
        tempo: 28,
    },
    SFEval {
        fen: "r3kb1r/3n1ppp/p3p3/1p1pP2P/P3PBP1/4P3/1q2B3/R2Q1K1R b kq - 0 0",
        phase: 107,
        eval: -348,

        material: -90,
        psqt: 151,
        imbalance: -21,
        king: -102,
        mobility: 25,
        passed_pawn: -38,
        pawns: -169,
        piece: -45,
        space: -3,
        threats: -24,
        winnable: -3,
        tempo: -28,
    },
    SFEval {
        fen: "rnb2rk1/pp2q2p/3p4/2pP2p1/2P1Pp2/2N5/PP1QBRPP/R5K1 w - - 0 0",
        phase: 106,
        eval: 172,

        material: 0,
        psqt: 137,
        imbalance: 0,
        king: -30,
        mobility: 68,
        passed_pawn: 0,
        pawns: -39,
        piece: 4,
        space: 20,
        threats: -14,
        winnable: 4,
        tempo: 28,
    },
    SFEval {
        fen: "8/2P1P3/b1B2p2/1pPRp3/2k3P1/P4pK1/nP3p1p/N7 w - - 0 0",
        phase: 106,
        eval: 1612,

        material: 1375,
        psqt: -82,
        imbalance: 10,
        king: 143,
        mobility: 165,
        passed_pawn: -336,
        pawns: 66,
        piece: -14,
        space: 0,
        threats: 117,
        winnable: 143,
        tempo: 28,
    },
    SFEval {
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        phase: 128,
        eval: 28,

        material: 0,
        psqt: 0,
        imbalance: 0,
        king: 0,
        mobility: 0,
        passed_pawn: 0,
        pawns: 0,
        piece: 0,
        space: 0,
        threats: 0,
        winnable: 0,
        tempo: 28,
    },
];

pub fn assert_all_eval(board: &mut Board, obj: &SFEval) {
    if board.calculate_score() != obj.material {
        println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.material);
    } else {
        println!("assertion `{:?} == {:?}` success", board.calculate_score(), obj.material);
    }
}

// Because Some of the evaluations are not exact, we allow a small difference
pub fn eval_assert(actual: isize, expected: isize, diff: usize, only_trace: bool) {
    let abs_diff = actual.abs_diff(expected);
    if abs_diff <= diff {
        if only_trace {
            println!("assertion `{:?} == {:?}` Success", actual, expected);
        } else {
            assert!(abs_diff <= diff, "assertion `{:?} == {:?}` Success", actual, expected);
        }
    } else {
        if only_trace {
            println!("assertion `{:?} == {:?}` Failed", actual, expected);
        } else {
            assert!(abs_diff <= diff, "assertion `{:?} == {:?}` Failed", actual, expected);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::{self, fen::FenTrait};

    use super::*;

    // Calculating Stockfish evaluation of certain element based on phase
    // The Stockfish phase goes from 128 to 0
    fn stockfish_eval(phase: isize, mg_value: isize, eg_value: isize) -> isize {
        (phase * mg_value + (128 - phase) * eg_value) / 128
    }

    // FIXME: DEPRECATE: TEST
    #[test]
    fn testing() {
        let phase: isize = 0;

        println!("      material: {:?},", stockfish_eval(phase, 0, 0)); // material
        println!("          psqt: {:?},", stockfish_eval(phase, 0, 0)); // psqt
        println!("      imbalance: {:?},", stockfish_eval(phase, 0, 0)); // imbalance
        println!("           king: {:?},", stockfish_eval(phase, 0, 0)); // king
        println!("      mobility: {:?},", stockfish_eval(phase, 0, 0)); // mobility
        println!("   passed_pawn: {:?},", stockfish_eval(phase, 0, 0)); // passed pawns
        println!("         pawns: {:?},", stockfish_eval(phase, 0, 0)); // pawns
        println!("         piece: {:?},", stockfish_eval(phase, 0, 0)); // pieces
        println!("         space: {:?},", stockfish_eval(phase, 0, 0)); // space
        println!("       threats: {:?},", stockfish_eval(phase, 0, 0)); // threats
        println!("      winnable: {:?},", stockfish_eval(phase, 0, 0)); // winnable
    }
}
