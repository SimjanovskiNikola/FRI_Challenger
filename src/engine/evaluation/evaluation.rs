use crate::engine::board::board::Board;
use crate::engine::board::color::*;
use crate::engine::board::piece::{Piece, PieceTrait};
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::evaluation::imbalance_eval::ImbalanceEvalTrait;
use crate::engine::evaluation::init_eval::InitEvalTrait;
use crate::engine::evaluation::king_eval::KingEvalTrait;
use crate::engine::evaluation::material_eval::MaterialEvalTrait;
use crate::engine::evaluation::mobility_eval::MobilityEvalTrait;
use crate::engine::evaluation::passed_pawn_eval::PassedPawnEvalTrait;
use crate::engine::evaluation::pawn_eval::PawnEvalTrait;
use crate::engine::evaluation::piece_eval::PieceEvalTrait;
use crate::engine::evaluation::psqt_eval::PSQTEvalTrait;
use crate::engine::evaluation::space_eval::SpaceEvalTrait;
use crate::engine::evaluation::tempo_eval::TempoEvalTrait;
use crate::engine::evaluation::threats_eval::ThreatsEvalTrait;
use crate::engine::evaluation::trace_eval::TraceEvalTrait;
use crate::engine::misc::bitboard::Bitboard;

// The Numbers (Tapered Eval) for the evaluation are taken from -> STOCKFISH SF_9

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Evaluation {
    // Attack Maps
    pub attack_map: [Bitboard; 2],
    pub attacked_by: [Bitboard; 14],
    pub attacked_by_2: [Bitboard; 2],

    // Mobility Evaluation
    pub mobility_area: [Bitboard; 2],

    // Space Evaluation
    pub pawn_behind_masks: [Bitboard; 2],

    // Threats Evaluation
    pub weak_enemy: [Bitboard; 2],

    // Passed Pawn Evaluation
    pub candidate_passed: [Bitboard; 2],

    // Piece Evaluation
    pub outpost: [Bitboard; 2],
    pub open_file: [Bitboard; 2],
    pub pawn_att_span: [Bitboard; 2],
    pub king_att_count_pieces: [u64; 2],

    // King Evaluation
    pub king_att_weight: [isize; 2],
    pub king_att_count: [usize; 2],
    pub king_att: [usize; 2],
    pub king_pawn_dx: [u8; 2],
    pub king_shelter: [(isize, isize, isize); 2],
    pub checks: [Bitboard; 14],
    pub king_ring: [Bitboard; 2],

    // Trace
    pub mg_test: [[isize; 64]; 2],
    pub eg_test: [[isize; 64]; 2],
    pub vec_test: Vec<String>,

    // Evaluation
    pub material_eval: [(isize, isize); 2],
    pub psqt_eval: [(isize, isize); 2],
    pub mobility_eval: [(isize, isize); 2],
    pub pawn_eval: [(isize, isize); 2],

    // Score
    pub pawn_hash_hit: bool,
    pub phase: (isize, isize),
    pub score: [(isize, isize); 2],
}

impl Evaluation {
    pub fn init() -> Self {
        Self {
            // Attack Maps
            attack_map: [0; 2],
            attacked_by: [0; 14],
            attacked_by_2: [0; 2],

            // Mobility Evaluation
            mobility_area: [0; 2],

            // Space Evaluation
            pawn_behind_masks: [0; 2],

            // Threats Evaluation
            weak_enemy: [0; 2],

            // Passed Pawn Evaluation
            candidate_passed: [0; 2],

            // Piece Evaluation
            outpost: [0; 2],
            king_att_count_pieces: [0; 2],
            open_file: [u64::MAX; 2],
            pawn_att_span: [0; 2],

            // King Evaluation
            king_ring: [0; 2],
            checks: [0; 14],
            king_att_weight: [0; 2],
            king_att_count: [0; 2],
            king_att: [0; 2],
            king_pawn_dx: [6; 2],
            king_shelter: [(0, 0, 0); 2],

            // Trace
            mg_test: [[0; 64]; 2],
            eg_test: [[0; 64]; 2],
            vec_test: Vec::with_capacity(200),

            // Evaluation
            material_eval: [(0, 0); 2],
            psqt_eval: [(0, 0); 2],
            mobility_eval: [(0, 0); 2],
            pawn_eval: [(0, 0); 2],

            // Score
            pawn_hash_hit: false,
            phase: (0, 0),
            score: [(0, 0); 2],
        }
    }

    pub fn reset(&mut self) {
        // Attack Maps
        self.attack_map.fill(0);
        self.attacked_by.fill(0);
        self.attacked_by_2.fill(0);

        // Space Evaluation
        self.pawn_behind_masks.fill(0);

        // Mobility Evaluation
        self.mobility_area.fill(0);

        // Threats Evaluation
        self.weak_enemy.fill(0);

        // Passed Pawn Evaluation
        self.candidate_passed.fill(0);

        // Piece Evaluation
        self.king_att_count_pieces.fill(0);
        self.open_file.fill(u64::MAX);
        self.outpost.fill(0);
        self.pawn_att_span.fill(0);

        // King Evaluation
        self.king_ring.fill(0);
        self.checks.fill(0);
        self.king_att_weight.fill(0);
        self.king_att_count.fill(0);
        self.king_att.fill(0);
        self.king_pawn_dx.fill(6);
        self.king_shelter.fill((0, 0, 0));

        // Trace
        // self.mg_test = [[0; 64]; 2];
        // self.eg_test = [[0; 64]; 2];

        // Evaluation
        // self.material_eval.fill((0, 0));
        // self.psqt_eval.fill((0, 0));
        self.mobility_eval.fill((0, 0));
        self.pawn_eval.fill((0, 0));

        // Score
        self.pawn_hash_hit = false;
        self.phase = (0, 0);
        self.score.fill((0, 0));
    }

    pub fn inc_reset(&mut self) {
        self.material_eval.fill((0, 0));
        self.psqt_eval.fill((0, 0));
    }

    pub fn full_reset(&mut self) {
        self.reset();
        self.inc_reset();
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
    fn evaluation(&mut self) -> isize;
    fn inc_evaluation(&mut self) -> isize;

    fn clear_eval(&mut self, piece: Piece, sq: usize);
    fn add_eval(&mut self, piece: Piece, sq: usize);
    fn quiet_eval(&mut self, piece: Piece, from: usize, to: usize);
}

impl EvaluationTrait for Board {
    fn evaluation(&mut self) -> isize {
        self.eval.reset();
        self.init();

        // 1. Piece Value
        self.material_eval(WHITE);
        self.material_eval(BLACK);

        // 2. PSQT
        self.psqt_eval(WHITE);
        self.psqt_eval(BLACK);

        // 3. Imbalance
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

        // 11. Tempo
        self.tempo(self.color());

        return self.calculate_score() * self.color().sign();
    }

    fn inc_evaluation(&mut self) -> isize {
        self.eval.reset();

        if let Some(pawn_entry) = self.pawn_tt.get(self.pk_key()) {
            // self.eval.pawn_behind_masks = pawn_entry.pawn_behind_masks;
            // self.eval.pawn_att_span = pawn_entry.pawn_att_span;
            // self.eval.king_pawn_dx = pawn_entry.king_pawn_dx;
            // self.eval.open_file = pawn_entry.open_file;
            self.eval.king_shelter =
                pawn_entry.shelter.map(|(x, y, z)| (x as isize, y as isize, z as isize));
            self.eval.pawn_eval = pawn_entry.pawn_eval.map(|(x, y)| (x as isize, y as isize));
            self.eval.candidate_passed = pawn_entry.candidate_passed;
            self.eval.pawn_hash_hit = true;
        }

        self.init();

        // 1. Piece Value
        self.sum(WHITE, None, None, self.eval.material_eval[WHITE.idx()]);
        self.sum(BLACK, None, None, self.eval.material_eval[BLACK.idx()]);

        // 2. PSQT
        self.sum(WHITE, None, None, self.eval.psqt_eval[WHITE.idx()]);
        self.sum(BLACK, None, None, self.eval.psqt_eval[BLACK.idx()]);

        // 3. Imbalance
        self.imbalance(WHITE);
        self.imbalance(BLACK);

        // 4. Pawns
        if !self.eval.pawn_hash_hit {
            self.pawns_eval(WHITE);
            self.pawns_eval(BLACK);
        } else {
            self.sum(
                WHITE,
                None,
                None,
                (self.eval.pawn_eval[WHITE.idx()].0, self.eval.pawn_eval[WHITE.idx()].1),
            );
            self.sum(
                BLACK,
                None,
                None,
                (self.eval.pawn_eval[BLACK.idx()].0, self.eval.pawn_eval[BLACK.idx()].1),
            );
        }

        // 5. Pieces
        self.piece_eval(WHITE);
        self.piece_eval(BLACK);

        // 6. Mobility
        self.sum(WHITE, None, None, self.eval.mobility_eval[WHITE.idx()]);
        self.sum(BLACK, None, None, self.eval.mobility_eval[BLACK.idx()]);
        // self.mobility_eval(WHITE);
        // self.mobility_eval(BLACK);

        // 7. Threats
        self.threats_eval(WHITE);
        self.threats_eval(BLACK);

        // 8. Passed Pawns
        self.passed_pawn(WHITE);
        self.passed_pawn(BLACK);

        // 9. Space
        self.space(WHITE);
        self.space(BLACK);

        // 10. King
        self.king_eval(WHITE);
        self.king_eval(BLACK);

        // 11. Tempo
        self.tempo(self.color());

        if !self.eval.pawn_hash_hit {
            let king_shelter =
                self.eval.king_shelter.map(|(x, y, z)| (x as i16, y as i16, z as i16));
            self.pawn_tt.set(
                self.pk_key(),
                self.eval.pawn_behind_masks,
                self.eval.pawn_att_span,
                self.eval.king_pawn_dx,
                self.eval.open_file,
                king_shelter,
                self.eval.pawn_eval.map(|(x, y)| (x as i16, y as i16)),
                self.eval.candidate_passed,
            );
        }

        return self.calculate_score() * self.color().sign();
    }

    fn clear_eval(&mut self, piece: Piece, sq: usize) {
        self.eval.material_eval[piece.color().idx()].0 -=
            MaterialEvalTrait::piece_material(self, piece).0;
        self.eval.material_eval[piece.color().idx()].1 -=
            MaterialEvalTrait::piece_material(self, piece).1;

        self.eval.psqt_eval[piece.color().idx()].0 -= PSQTEvalTrait::piece_psqt(self, piece, sq).0;
        self.eval.psqt_eval[piece.color().idx()].1 -= PSQTEvalTrait::piece_psqt(self, piece, sq).1;
    }
    fn add_eval(&mut self, piece: Piece, sq: usize) {
        self.eval.material_eval[piece.color().idx()].0 +=
            MaterialEvalTrait::piece_material(self, piece).0;
        self.eval.material_eval[piece.color().idx()].1 +=
            MaterialEvalTrait::piece_material(self, piece).1;

        self.eval.psqt_eval[piece.color().idx()].0 += PSQTEvalTrait::piece_psqt(self, piece, sq).0;
        self.eval.psqt_eval[piece.color().idx()].1 += PSQTEvalTrait::piece_psqt(self, piece, sq).1;
    }

    fn quiet_eval(&mut self, piece: Piece, from: usize, to: usize) {
        self.eval.psqt_eval[piece.color().idx()].0 += PSQTEvalTrait::piece_psqt(self, piece, to).0
            - PSQTEvalTrait::piece_psqt(self, piece, from).0;

        self.eval.psqt_eval[piece.color().idx()].1 += PSQTEvalTrait::piece_psqt(self, piece, to).1
            - PSQTEvalTrait::piece_psqt(self, piece, from).1;
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::fen::FenTrait;
    use crate::engine::evaluation::test_evaluation::{SF_EVAL, eval_assert};

    use super::*;

    // NOTE: Evaluation [TEST: WORKS]
    #[rustfmt::skip]
    #[test]
    fn eval_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            let score = board.evaluation();
            eval_assert(score * board.color().sign(), obj.eval, 130, false);
        }
    }

    // #[test]
    // fn storm_sq_test() {
    //     for obj in &SF_EVAL {
    //         if obj.fen != "3r2k1/2p2bpp/p2r4/P2PpP2/BR1q4/7P/5PP1/2R1Q1K1 b - - 0 0" {
    //             continue;
    //         }

    //         let mut board = Board::read_fen(obj.fen);
    //         board.init();

    //         for sq in 0..64 {
    //             println!("{:?}", board.strength_square(sq, WHITE));
    //         }
    //     }
    // }
}
