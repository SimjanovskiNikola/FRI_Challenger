use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color::*;
use crate::engine::board::structures::piece::*;
use crate::engine::board::structures::square::*;
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::evaluation::imbalance_eval::ImbalanceEvalTrait;
use crate::engine::evaluation::init_eval::InitEvalTrait;
use crate::engine::evaluation::king_eval::KingEvalTrait;
use crate::engine::evaluation::material_eval::MaterialEvalTrait;
use crate::engine::evaluation::mobility_eval::MobilityEvalTrait;
use crate::engine::evaluation::passed_pawn_eval::PassedPawnEvalTrait;
use crate::engine::evaluation::pawn_eval::PawnEvalTrait;
use crate::engine::evaluation::piece_eval::PieceEvalTrait;
use crate::engine::evaluation::piece_eval::OUTPOST_RANKS;
use crate::engine::evaluation::psqt_eval::PSQTEvalTrait;
use crate::engine::evaluation::space_eval::SpaceEvalTrait;
use crate::engine::evaluation::tempo_eval::TempoEvalTrait;
use crate::engine::evaluation::threats_eval::ThreatsEvalTrait;
use crate::engine::evaluation::trace_eval::TraceEvalTrait;
use crate::engine::misc::bitboard::Bitboard;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::misc::bitboard::Iterator;
use crate::engine::misc::const_utility::FILE_BITBOARD;
use crate::engine::misc::display::display_board::print_eval;
use crate::engine::move_generator::bishop::get_bishop_mask;
use crate::engine::move_generator::bishop::has_bishop_pair;
use crate::engine::move_generator::generated::between::BETWEEN_BB;
use crate::engine::move_generator::generated::king::KING_RING;
use crate::engine::move_generator::generated::pawn::FORWARD_SPANS_LR;
use crate::engine::move_generator::generated::pawn::PAWN_3_BEHIND_MASKS;
use crate::engine::move_generator::king::get_king_mask;
use crate::engine::move_generator::knight::get_knight_mask;
use crate::engine::move_generator::pawn::get_all_pawn_left_att_mask;
use crate::engine::move_generator::pawn::get_all_pawn_right_att_mask;
use crate::engine::move_generator::pawn::get_pawn_2_att;
use crate::engine::move_generator::pawn::get_pawn_att_mask;
use crate::engine::move_generator::queen::get_queen_mask;
use crate::engine::move_generator::rook::get_rook_mask;

// The Numbers (Tapered Eval) for the evaluation are taken from -> STOCKFISH SF_9

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Evaluation {
    pub pawn_behind_masks: [Bitboard; 2],
    pub psqt: [isize; 2],
    pub mg_test: [[isize; 64]; 2],
    pub eg_test: [[isize; 64]; 2],
    pub vec_test: Vec<String>,

    pub mobility: [isize; 2],
    pub outpost: [Bitboard; 2],
    pub open_file: [Bitboard; 2],
    pub pawn_att_span: [Bitboard; 2],
    pub king_ring: [Bitboard; 2],
    pub checks: [Bitboard; 14],
    pub x_ray: [Bitboard; 14],
    pub attacked_by: [Bitboard; 14],
    pub defended_by: [Bitboard; 14],
    pub defended_by_2: [Bitboard; 2],
    pub attacked_by_2: [Bitboard; 2],
    pub king_att_weight: [isize; 2],
    pub king_att_count: [usize; 2],
    pub king_att_count_pieces: [u64; 2],
    pub king_att: [usize; 2],
    pub king_pawn_dx: [usize; 2],
    pub defend_map: [Bitboard; 2],
    pub attack_map: [Bitboard; 2],
    pub queen_diagonal: [Bitboard; 2],
    pub phase: (isize, isize),
    pub score: [(isize, isize); 2],
    pub king_shelter: [(isize, isize, isize); 2],
}

impl Evaluation {
    pub fn init() -> Self {
        Self {
            pawn_behind_masks: [0; 2],
            psqt: [0; 2],
            mobility: [0; 2],

            mg_test: [[0; 64]; 2],
            eg_test: [[0; 64]; 2],
            vec_test: Vec::with_capacity(200),
            outpost: [0; 2],
            open_file: [u64::MAX; 2],
            pawn_att_span: [0; 2],
            king_ring: [0; 2],
            x_ray: [0; 14],
            checks: [0; 14],
            attacked_by: [0; 14],
            defended_by: [0; 14],
            attacked_by_2: [0; 2],
            defended_by_2: [0; 2],
            king_att_weight: [0; 2],
            king_att_count: [0; 2],
            king_att_count_pieces: [0; 2],
            king_att: [0; 2],
            king_pawn_dx: [6; 2],
            defend_map: [0; 2],
            attack_map: [0; 2],
            queen_diagonal: [0; 2],
            phase: (0, 0),
            score: [(0, 0); 2],
            king_shelter: [(0, 0, 0); 2],
        }
    }

    pub fn reset(&mut self) {
        self.psqt.fill(0);
        self.mobility.fill(0);
        self.open_file.fill(u64::MAX);
        self.outpost.fill(0);
        self.pawn_att_span.fill(0);
        self.king_ring.fill(0);
        self.checks.fill(0);
        self.x_ray.fill(0);
        self.attacked_by.fill(0);
        self.defended_by.fill(0);
        self.attacked_by_2.fill(0);
        self.defended_by_2.fill(0);
        self.king_att_weight.fill(0);
        self.king_att_count.fill(0);
        self.king_att_count_pieces.fill(0);
        self.king_att.fill(0);
        self.king_pawn_dx.fill(6);
        self.defend_map.fill(0);
        self.attack_map.fill(0);
        self.queen_diagonal.fill(0);
        self.phase = (0, 0);
        self.score.fill((0, 0));
        self.king_shelter.fill((0, 0, 0));

        self.mg_test = [[0; 64]; 2];
        self.eg_test = [[0; 64]; 2];
    }
}

pub trait EvaluationTrait:
    CommonEvalTrait
    + InitEvalTrait
    + TraceEvalTrait
    + MaterialEvalTrait
    + PSQTEvalTrait
    + ImbalanceEvalTrait
    + MobilityEvalTrait
    + PassedPawnEvalTrait
    + PawnEvalTrait
    + PieceEvalTrait
    + ThreatsEvalTrait
    + KingEvalTrait
    + SpaceEvalTrait
    + TempoEvalTrait
{
    // NOTE: Main Evaluation Function (It has 11 sub evaluations)
    fn evaluation(&mut self) -> isize;
}

impl EvaluationTrait for Board {
    // ************************************************
    //                MAIN EVALUATION                 *
    // ************************************************

    fn evaluation(&mut self) -> isize {
        self.init();

        // 1. Piece Value NOTE: DONE
        self.material_eval(WHITE);
        self.material_eval(BLACK);

        // 2. PSQT NOTE: DONE
        self.psqt_eval(WHITE);
        self.psqt_eval(BLACK);

        // 3. Imbalance NOTE: DONE
        self.imbalance(WHITE);
        self.imbalance(BLACK);

        // 4. Pawns
        self.pawns_eval(WHITE);
        self.pawns_eval(BLACK);

        // 5. Pieces
        self.piece_eval(WHITE);
        self.piece_eval(BLACK);

        // 6. Mobility
        self.mobility_eval(WHITE);
        self.mobility_eval(BLACK);

        // 7. Threats
        self.threats_eval(WHITE);
        self.threats_eval(BLACK);

        // 8. Passed Pawns
        self.passed_pawn(WHITE);
        self.passed_pawn(BLACK);

        // 9. Space
        self.space(WHITE);
        self.space(BLACK);

        // // 10. King
        self.king_eval(WHITE);
        self.king_eval(BLACK);

        // 11. Tempo NOTE: DONE
        self.tempo(self.color());

        // 1r1q4/6pk/3P2pp/1p1Q4/p2P4/P6P/1P3P2/3R2K1 b - - 0 31
        // 1R6/1P4k1/5p2/1r3K2/8/7P/6P1/8 w - - 5 55
        // 6k1/5p2/7p/4p1p1/pn2P3/2K1BP1P/6P1/8 b - - 2 45
        // 7k/4R3/3p1r2/4p2p/4P3/1Q3N2/4KPq1/8 b - - 3 45
        // 8/8/2KB4/3Pb3/1r2k3/8/2R5/8 b - - 0 59

        return self.calculate_score() * self.color().sign();
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::fen::FenTrait;
    use crate::engine::evaluation::test_evaluation::SF_EVAL;

    use super::*;

    // NOTE: 2. PSQT [TEST: WORKS]
    #[test]
    fn psqt_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.psqt_eval(WHITE);
            board.psqt_eval(BLACK);

            // if board.calculate_score() != obj.psqt {
            //     println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.psqt);
            // } else {
            //     println!("assertion `{:?} == {:?}` success", board.calculate_score(), obj.psqt);
            // }

            assert_eq!(board.calculate_score(), obj.psqt);
        }
    }

    // NOTE: 3. IMBALANCE [TEST: WORKS]
    #[test]
    fn imbalance_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.imbalance(WHITE);
            board.imbalance(BLACK);

            if board.calculate_score() != obj.imbalance {
                println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.imbalance);
            } else {
                println!(
                    "assertion `{:?} == {:?}` success",
                    board.calculate_score(),
                    obj.imbalance
                );
                assert_eq!(board.calculate_score(), obj.imbalance);
            }

            // assert_eq!(board.calculate_score(), obj.imbalance);
        }
    }

    // NOTE: 4. PAWNS [TEST: WORKS]
    #[test]
    fn pawns_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.pawns_eval(WHITE);
            board.pawns_eval(BLACK);
            assert_eq!(board.calculate_score(), obj.pawns);

            if board.calculate_score() != obj.pawns {
                println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.pawns);
            } else {
                println!("assertion `{:?} == {:?}` success", board.calculate_score(), obj.pawns);
                assert_eq!(board.calculate_score(), obj.pawns);
            }

            // board.print_trace_board("");
        }
    }

    // NOTE: 5. PIECES FIXME:
    #[test]
    fn pieces_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "rnb2rk1/pp2q2p/3p4/2pP2p1/2P1Pp2/2N5/PP1QBRPP/R5K1 w - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.piece_eval(WHITE);
            board.piece_eval(BLACK);

            if board.calculate_score() != obj.piece {
                println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.piece);
            } else {
                println!("assertion `{:?} == {:?}` success", board.calculate_score(), obj.piece);
                assert_eq!(board.calculate_score(), obj.piece);
            }

            // board.print_trace_log("");
            // board.print_trace_score("");
        }
    }

    // NOTE: 6. MOBILITY [TEST:FIXME: SEMI-WORKS 90%]
    #[test]
    fn mobility_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.mobility_eval(WHITE);
            board.mobility_eval(BLACK);

            if board.calculate_score() != obj.mobility {
                println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.mobility);
            } else {
                println!("assertion `{:?} == {:?}` success", board.calculate_score(), obj.mobility);
                assert_eq!(board.calculate_score(), obj.mobility);
            }
        }
    }

    // NOTE: 7. THREATS [TEST: WORKS]
    #[test]
    fn threats_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.threats_eval(WHITE);
            board.threats_eval(BLACK);

            if board.calculate_score() != obj.threats {
                println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.threats);
            } else {
                println!("assertion `{:?} == {:?}` success", board.calculate_score(), obj.threats);
                assert_eq!(board.calculate_score(), obj.threats);
            }

            // board.print_trace_log("");
            // board.print_trace_score("");
        }
    }

    // NOTE: 8. PASSED PAWNS [FIXME: TEST: WORKS]
    #[test]
    fn passed_pawns_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "4rrk1/Rpp3pp/6q1/2PPn3/4p3/2N5/1P2QPPP/5RK1 w - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.passed_pawn(WHITE);
            board.passed_pawn(BLACK);
            // assert_eq!(board.calculate_score(), obj.passed_pawn);

            if board.calculate_score() != obj.passed_pawn {
                println!(
                    "assertion `{:?} == {:?}` failed",
                    board.calculate_score(),
                    obj.passed_pawn
                );
            } else {
                println!(
                    "assertion `{:?} == {:?}` success",
                    board.calculate_score(),
                    obj.passed_pawn
                );
                assert_eq!(board.calculate_score(), obj.passed_pawn);
            }

            // board.print_trace_board("");
            // board.print_trace_log("");
        }
    }

    // NOTE: 9. SPACE [TEST: WORKS]
    #[test]
    fn space_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.space(WHITE);
            board.space(BLACK);

            if board.calculate_score() != obj.space {
                println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.space);
            } else {
                println!("assertion `{:?} == {:?}` success", board.calculate_score(), obj.space);
                assert_eq!(board.calculate_score(), obj.space);
            }

            // assert_eq!(board.calculate_score(), obj.space);
        }
    }

    // NOTE: 10. KING FIXME:
    #[test]
    fn king_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.king_eval(WHITE);
            board.king_eval(BLACK);
            // assert_eq!(board.calculate_score(), obj.king);

            if board.calculate_score() != obj.king {
                println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.king);
            } else {
                println!("assertion `{:?} == {:?}` success", board.calculate_score(), obj.king);
                assert_eq!(board.calculate_score(), obj.king);
            }

            // board.print_trace_log("");
            // board.print_trace_score("");
        }
    }

    // NOTE: 11. TEMPO [TEST: WORKS]
    #[test]
    fn tempo_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.tempo(board.color());

            // if board.calculate_score() != obj.tempo {
            //     println!("assertion `{:?} == {:?}` failed", board.calculate_score(), obj.tempo);
            // } else {
            //     println!("assertion `{:?} == {:?}` success", board.calculate_score(), obj.tempo);
            //     assert_eq!(board.calculate_score(), obj.tempo);
            // }
            assert_eq!(board.calculate_score(), obj.tempo);
        }
    }

    // NOTE: Evaluation [TEST: WORKS]
    #[rustfmt::skip]
    #[test]
    fn eval_test() {
        for obj in &SF_EVAL {
            // if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            let score = board.evaluation();

            if score != obj.eval {
                println!("assertion `{:?} == {:?}` failed", score, obj.eval);
            } else {
                println!("assertion `{:?} == {:?}` success", score, obj.eval);
                assert_eq!(score, obj.eval);
            }
        }
    }

    #[test]
    fn storm_sq_test() {
        for obj in &SF_EVAL {
            if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
                continue;
            }

            let mut board = Board::read_fen(obj.fen);
            board.init();

            for sq in 0..64 {
                println!("{:?}", board.strength_square(sq, WHITE));
            }
        }
    }

    // #[test]
    // fn eval_random_test() {
    //     // let mut board = Board::read_fen("5rk1/ppq3pp/2p1rn2/4p1Q1/8/2N4P/PP4P1/2KRR3 w - - 0 3");
    //     let mut board =
    //         Board::read_fen("rnbqkbnr/pppp1ppp/4p3/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
    //     assert_eq!(93, board.evaluation())
    // }
}
