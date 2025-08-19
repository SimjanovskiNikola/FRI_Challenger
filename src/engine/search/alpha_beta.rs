use super::iter_deepening::Search;
use super::transposition_table::Bound;
use crate::engine::board::make_move::BoardMoveTrait;
use crate::engine::board::mv_gen::next_move;
use crate::engine::board::mv_gen::BoardGenMoveTrait;
use crate::engine::board::structures::moves::Move;
use crate::engine::board::structures::piece::PieceTrait;
use crate::engine::board::structures::piece::KING;
use crate::engine::evaluation::evaluation::EvaluationTrait;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::protocols::time::time_over;
use crate::engine::search::transposition_table::TT;

impl Search {
    pub fn alpha_beta(
        &mut self,
        mut alpha: isize,
        mut beta: isize,
        mut depth: u8,
        take_null: bool,
    ) -> isize {
        // If we reached the final depth than make sure there is no horizon effect
        if depth == 0 {
            return self.quiescence_search(alpha, beta);
        }

        // Check if the position happened before or is draw
        // TODO: There is some bug regarding repetition
        if self.board.state.half_move >= 100 || self.board.is_repetition() {
            return 0;
        }

        if self.board.ply() > 63 {
            return self.board.evaluation();
        }

        let in_check: bool =
            self.board.sq_attack(self.board.king_sq(self.board.color()), self.board.color()) != 0;

        if in_check {
            depth += 1;
        }

        if let Some((score, _)) =
            TT.read().unwrap().probe(self.board.state.key, depth, alpha as i16, beta as i16)
        {
            return score as isize;
        }
        self.info.nodes += 1;

        let mut best_mv = None;
        let mut best_score = alpha;
        let mut legal_mv_num = 0;
        let old_alpha: isize = alpha;

        let mut moves = self.board.gen_moves();
        let ply = self.board.ply();

        self.board.pv_len[ply] = 0;

        while let Some(mv) = next_move(&mut moves) {
            // Check Time every 2047 Nodes
            if (self.info.nodes & 2047) == 0 && time_over(&self) {
                return 0;
            }

            if !self.board.make_move(&mv) {
                continue;
            }
            legal_mv_num += 1;
            let score = -self.alpha_beta(-beta, -alpha, depth - 1, true);
            self.board.undo_move();

            if score > alpha {
                self.info.alpha_raise_count[depth as usize] += 1;
                self.info.alpha_raise_index_sum[depth as usize] += legal_mv_num;

                if score >= beta {
                    // TODO: Implement Function for this
                    self.info.beta_cut_count[depth as usize] += 1;
                    self.info.beta_cut_index_sum[depth as usize] += legal_mv_num;
                    if legal_mv_num == 1 {
                        self.info.fail_hard_first += 1; // NOTE: ORDERING INFO
                    }
                    if !mv.flag.is_capture() {
                        self.board.s_killers[self.board.ply()][0] =
                            self.board.s_killers[self.board.ply()][1];
                        self.board.s_killers[self.board.ply()][1] = Some(mv);
                    }
                    // TT.write().unwrap().set(
                    //     self.board.state.key,
                    //     mv,
                    //     score as i16,
                    //     depth,
                    //     Bound::Upper,
                    // );
                    self.info.fail_hard += 1; // NOTE: ORDERING INFO
                    return beta;
                }

                alpha = score;
                best_score = score;
                best_mv = Some(mv);

                // TODO: Create Function for this
                self.board.pv_moves[ply][0] = Some(mv);
                let child_len = self.board.pv_len.get(ply + 1).copied().unwrap_or(0);
                for i in 0..child_len {
                    self.board.pv_moves[ply][i + 1] = self.board.pv_moves[ply + 1][i];
                }
                self.board.pv_len[ply] = child_len + 1;

                if !mv.flag.is_capture() {
                    self.board.s_history[mv.piece.idx()][mv.to as usize] += depth as u64;
                }
            }
        }

        // Checking for if the position is draw or checkmate
        if legal_mv_num == 0 {
            return match in_check {
                true => -1000000 + (self.board.ply() as isize),
                false => 0,
            };
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
