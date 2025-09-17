use super::iter_deepening::Search;
use crate::engine::evaluation::evaluation::EvaluationTrait;
use crate::engine::move_generator::make_move::BoardMoveTrait;
use crate::engine::move_generator::mv_gen::BoardGenMoveTrait;
use crate::engine::move_generator::mv_oredering::MoveOrderingTrait;
use crate::engine::protocols::time::time_over;

impl Search {
    pub fn quiescence_search(&mut self, mut alpha: isize, beta: isize, depth: i8) -> isize {
        self.info.nodes += 1;

        // let eval = self.board.evaluation();
        let eval = self.board.inc_evaluation();

        if self.board.ply() > 63 {
            return eval;
        }

        if eval > alpha {
            if eval >= beta {
                return eval;
            }
            alpha = eval;
        }

        // NOTE: Add + 2400 if there is a optimistic promotion + capturing queen
        let mut delta = 2600;
        if matches!(self.board.moves.last(), Some(mv) if mv.flag.is_promo()) {
            delta += 2400;
        }

        // FIXME: NOTE: Note tested yet
        // NOTE: DELTA PRUNING
        if eval < alpha - delta {
            return alpha;
        }

        // if let Some((score, _)) =
        //     TT.read().unwrap().probe(self.board.state.key, depth, alpha as i16, beta as i16)
        // {
        //     return score as isize;
        // }

        let mut best_mv = None;
        let mut best_score = alpha;
        let old_alpha: isize = alpha;
        let mut moves = self.board.gen_captures();
        self.board.score_moves(&mut moves);

        while let Some(mv) = self.board.next_move(&mut moves) {
            if (self.info.nodes & 8192) == 0 && time_over(&self) {
                break;
            }

            if !self.board.make_move(&mv) {
                continue;
            }
            let score = -self.quiescence_search(-beta, -alpha, depth - 1);
            self.board.undo_move();

            if score > alpha {
                if score >= beta {
                    // TT.write().unwrap().set(
                    //     self.board.state.key,
                    //     mv,
                    //     score as i16,
                    //     depth,
                    //     Bound::Lower,
                    // );
                    return beta;
                }
                alpha = score;
                best_score = score;
                best_mv = Some(mv);
            }
        }

        // if let Some(mv) = best_mv {
        //     let bound = if best_score > old_alpha { Bound::Exact } else { Bound::Upper };
        //     TT.write().unwrap().set(self.board.state.key, mv, alpha as i16, depth, bound);
        // }
        alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
