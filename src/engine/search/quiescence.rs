use super::iter_deepening::Search;
use crate::engine::board::make_move::BoardMoveTrait;
use crate::engine::board::mv_gen::BoardGenMoveTrait;
use crate::engine::evaluation::new_evaluation::EvaluationTrait;
use crate::engine::protocols::time::time_over;

const BIG_DELTA: isize = 900;

impl Search {
    pub fn quiescence_search(&mut self, mut alpha: isize, beta: isize) -> isize {
        let eval = self.board.evaluation();
        if eval >= beta {
            return beta;
        }

        self.info.nodes += 1;

        // NOTE: DELTA PRUNING
        if eval < alpha - BIG_DELTA {
            return alpha;
        }

        alpha = alpha.max(eval);

        let mut pos_rev = self.board.gen_captures();

        for rev in &mut pos_rev {
            if (self.info.nodes & 2047) == 0 && time_over(&self) {
                break;
            }

            if !self.board.make_move(rev) {
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
