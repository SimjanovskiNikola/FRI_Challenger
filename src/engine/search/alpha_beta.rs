use super::iter_deepening::Search;
use crate::engine::board::moves::Move;
use crate::engine::board::piece::PieceTrait;
use crate::engine::evaluation::evaluation::EvaluationTrait;
use crate::engine::move_generator::make_move::BoardMoveTrait;
use crate::engine::move_generator::mv_gen::BoardGenMoveTrait;
use crate::engine::move_generator::mv_oredering::MoveOrderingTrait;
use crate::engine::protocols::time::time_over;

impl Search {
    #[inline(always)]
    pub fn add_killer(&mut self, mv: Move) {
        if !mv.flag.is_capture() {
            self.board.s_killers[self.board.ply()][0] = self.board.s_killers[self.board.ply()][1];
            self.board.s_killers[self.board.ply()][1] = Some(mv);
        }
    }

    #[inline(always)]
    pub fn add_history(&mut self, mv: Move, depth: i8) {
        if !mv.flag.is_capture() {
            self.board.s_history[mv.piece.idx()][mv.to as usize] += (depth * depth) as isize;
        }
    }

    #[inline(always)]
    pub fn add_to_pv(&mut self, mv: Move, ply: usize) {
        self.board.pv_moves[ply][0] = Some(mv);
        let child_len = self.board.pv_len.get(ply + 1).copied().unwrap_or(0);
        for i in 0..child_len {
            self.board.pv_moves[ply][i + 1] = self.board.pv_moves[ply + 1][i];
        }
        self.board.pv_len[ply] = child_len + 1;
    }

    #[inline(always)]
    pub fn add_fail_hard_info(&mut self) {
        self.info.fail_hard += 1;
    }

    #[inline(always)]
    pub fn add_fail_hard_first_info(&mut self, legal_mv_num: usize) {
        if legal_mv_num == 1 {
            self.info.fail_hard_first += 1;
        }
    }

    #[inline(always)]
    pub fn add_beta_cut_info(&mut self, depth: i8, legal_mv_num: usize) {
        self.info.beta_cut_count[depth as usize] += 1;
        self.info.beta_cut_index_sum[depth as usize] += legal_mv_num;
    }
    #[inline(always)]
    pub fn add_alpha_raise_info(&mut self, depth: i8, legal_mv_num: usize) {
        self.info.alpha_raise_count[depth as usize] += 1;
        self.info.alpha_raise_index_sum[depth as usize] += legal_mv_num;
    }

    #[inline(always)]
    fn in_check(&self) -> bool {
        self.board.sq_attack(self.board.king_sq(self.board.color()), self.board.color()) != 0
    }

    pub const FUTILITY_MARGINS: [isize; 5] = [0, 200, 800, 1250, 1600];

    pub fn alpha_beta(
        &mut self,
        mut alpha: isize,
        beta: isize,
        mut depth: i8,
        is_nmp: bool,
    ) -> isize {
        // If we reached the final depth than make sure there is no horizon effect
        debug_assert!(depth >= 0, "Depth is smaller than 0");

        if depth == 0 {
            return self.quiescence_search(alpha, beta, depth);
        }

        // Check if the position happened before or is draw
        // TODO: There is some bug regarding repetition
        if self.board.state.half_move >= 100 || self.board.is_repetition() {
            return 0;
        }

        if self.board.ply() > 63 || depth >= 63 {
            return self.board.inc_eval();
        }

        let in_check: bool = self.in_check();
        // self.board.sq_attack(self.board.king_sq(self.board.color()), self.board.color()) != 0;

        // NOTE: Check extension
        if in_check {
            depth += 1;
        }

        let is_pvs = alpha != beta - 1;

        // if !is_pvs
        //     && !is_nmp
        //     && let Some((score, _)) =
        //         self.board.tt.probe(self.board.state.key, depth, alpha as i16, beta as i16)
        // {
        //     return score as isize;
        // }

        self.info.nodes += 1;

        // FIXME: Uncomment for use of Futility Pruning
        // let do_futility_pruning = if depth > 4 || is_pvs || in_check {
        //     // Only apply at shallow depths, in non-PV nodes, and when not in check.
        //     false
        // } else {
        //     // Prune if the static eval is significantly worse than alpha.
        //     self.board.inc_eval() + Self::FUTILITY_MARGINS[depth as usize] <= alpha
        // };

        // NOTE: Null move Pruning
        let color = self.board.color();
        let is_pawn_ending = self.board.occ_bb(color)
            & !(self.board.pawn_bb(color) | self.board.king_bb(color))
            == 0;
        let nmp_allowed = !in_check && !is_nmp && !is_pawn_ending && !is_pvs;

        if nmp_allowed {
            let r = (depth - 1).min(3 + depth / 4);
            let mv = Move::null_move();

            self.board.make_move(&mv);
            let score = -self.alpha_beta(-beta, -beta + 1, depth - 1 - r, true);
            self.board.undo_move();
            if score >= beta {
                return beta;
            }
        }

        let mut best_mv = None;
        let mut best_score = alpha;
        let mut legal_mv_num = 0;
        let old_alpha: isize = alpha;

        let mut moves = self.board.gen_moves();
        self.board.score_moves(&mut moves);

        let ply = self.board.ply();
        self.board.pv_len[ply] = 0;

        while let Some(mv) = self.board.next_move(&mut moves) {
            // Check Time every 8192 Nodes
            if (self.info.nodes & 8192) == 0 && time_over(&self) {
                return 0;
            }

            if !self.board.make_move(&mv) {
                continue;
            }
            legal_mv_num += 1;

            // Don't prune captures, promotions, or checks.
            // Also, don't prune the first move, as it's likely the best.
            // FIXME: Uncomment for use of Futility Pruning
            // if do_futility_pruning
            //     && legal_mv_num > 1
            //     && !mv.flag.is_capture()
            //     && !mv.flag.is_promo()
            //     && !self.in_check()
            // {
            //     self.board.undo_move();
            //     continue; // Prune this move
            // }

            let mut score: isize;
            if legal_mv_num == 1 {
                score = -self.alpha_beta(-beta, -alpha, depth - 1, false);
            } else {
                // Late Move Reductions, Add if enemy king is in check
                if legal_mv_num >= 5
                    && depth >= 3
                    && !is_pvs
                    && !in_check
                    && !mv.flag.is_capture()
                    && !mv.flag.is_promo()
                    && !mv.piece.is_pawn()
                {
                    // Base Reduction: Etherial
                    let mut r: i8 = (0.7844
                        + ((depth as f32).ln() * (legal_mv_num as f32).ln() / 2.4696))
                        as i8;
                    r = r.max(1).min(depth - 2);
                    // let reduction = 1;
                    score = -self.alpha_beta(-alpha - 1, -alpha, depth - 1 - r, false);
                } else {
                    score = alpha + 1; // To enter the PVS search
                }
                // NOTE: LMR Ends Here

                // FIXME: TEST: Shouldn't the above true for pvs be in this line here ????
                if score > alpha {
                    score = -self.alpha_beta(-alpha - 1, -alpha, depth - 1, false);

                    if alpha < score && score < beta {
                        score = -self.alpha_beta(-beta, -alpha, depth - 1, false);
                    }
                }
            }

            self.board.undo_move();

            if score > alpha {
                // NOTE: Adding Alpha Raise info. (Comment Out before release)
                // NOTE: Used for checking how good the move ordering is.
                self.add_alpha_raise_info(depth, legal_mv_num);

                if score >= beta {
                    self.add_killer(mv);

                    // NOTE: Adding Beta Cut info. (Comment Out before release)
                    // NOTE: Used for checking how good the move ordering is.
                    self.add_beta_cut_info(depth, legal_mv_num);
                    self.add_fail_hard_first_info(legal_mv_num);
                    self.add_fail_hard_info();

                    // if !is_pvs && !is_nmp {
                    //     self.board.tt.set(
                    //         self.board.state.key,
                    //         mv,
                    //         score as i16,
                    //         depth,
                    //         Bound::Lower,
                    //     );
                    // }
                    return beta;
                }

                alpha = score;
                best_score = score;
                best_mv = Some(mv);

                self.add_to_pv(mv, ply);
                self.add_history(mv, depth);
            }
        }

        // NOTE: Checking if the position is draw or checkmate
        if legal_mv_num == 0 {
            return match in_check {
                true => -1000000 + (self.board.ply() as isize),
                false => 0,
            };
        }

        // // NOTE: Storing the best value in the transposition table
        // if !is_pvs && !is_nmp {
        //     if let Some(mv) = best_mv {
        //         let bound = if best_score > old_alpha { Bound::Exact } else { Bound::Upper };
        //         self.board.tt.set(self.board.state.key, mv, alpha as i16, depth, bound);
        //     }
        // }

        alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
