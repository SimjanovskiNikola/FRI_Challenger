use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color::Color;
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::evaluation::evaluation::EvaluationTrait;

pub const TEMPO_WT: isize = 28;

pub trait TempoEvalTrait {
    fn tempo(&mut self, clr: Color);
}

impl TempoEvalTrait for Board {
    #[inline(always)]
    fn tempo(&mut self, clr: Color) {
        self.sum(clr, None, None, (TEMPO_WT, TEMPO_WT));
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::fen::FenTrait;
    use crate::engine::evaluation::init_eval::InitEvalTrait;
    use crate::engine::evaluation::test_evaluation::SF_EVAL;

    use super::*;

    #[test]
    fn tempo_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.tempo(board.color());

            assert_eq!(board.calculate_score(), obj.tempo);
        }
    }
}
