use super::iter_deepening::Search;
use crate::engine::board::make_move::BoardMoveTrait;
use crate::engine::board::mv_gen::{next_move, BoardGenMoveTrait};
use crate::engine::evaluation::evaluation::EvaluationTrait;
use crate::engine::protocols::time::time_over;
use crate::engine::search::transposition_table::TT;

const BIG_DELTA: isize = 900;

impl Search {
    pub fn quiescence_search(&mut self, mut alpha: isize, beta: isize) -> isize {
        let eval = self.board.evaluation();

        if self.board.ply() > 63 {
            return eval;
        }

        if eval >= beta {
            return beta;
        }

        self.info.nodes += 1;

        // NOTE: DELTA PRUNING
        if eval < alpha - BIG_DELTA {
            return alpha;
        }

        // NOTE: STANDARD PAT PRUNING
        if eval > alpha {
            if eval >= beta {
                return eval;
            }
            alpha = eval;
        }

        if let Some((score, _)) =
            TT.read().unwrap().probe(self.board.state.key, 0, alpha as i16, beta as i16)
        {
            return score as isize;
        }

        alpha = alpha.max(eval);

        let mut pos_rev = self.board.gen_captures();

        while let Some(rev) = next_move(&mut pos_rev) {
            if (self.info.nodes & 2047) == 0 && time_over(&self) {
                break;
            }

            if !self.board.make_move(&rev) {
                continue;
            }
            let score = -self.quiescence_search(-beta, -alpha);
            self.board.undo_move();

            if score >= beta {
                return beta;
            }

            alpha = alpha.max(score);
        }

        alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
