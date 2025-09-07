use crate::engine::board::board::Board;
use crate::engine::board::color::{Color, ColorTrait};
use crate::engine::board::piece::{PieceTrait, PAWN};
use crate::engine::board::square::{get_file, get_rank};
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::evaluation::eval_defs::{CLR_RANK, PASSED_PAWN_REW};
use crate::engine::evaluation::pawn_eval::PawnEvalTrait;
use crate::engine::generated::pawn::{FORWARD_SPANS_LR, PAWN_ATTACK_LOOKUP, PAWN_FORWARD_SPANS};
use crate::engine::misc::bitboard::{BitboardTrait, Iterator};

pub trait PassedPawnEvalTrait {
    fn passed_pawn(&mut self, clr: Color);
    fn passed_leverable(&mut self, sq: usize, clr: Color) -> bool;
    fn passed_file(&mut self, sq: usize) -> isize;
    fn passed_blocked(&mut self, sq: usize, clr: Color) -> isize;
    fn king_proximity(&mut self, sq: usize, clr: Color) -> isize;
    fn candidate_passed(&mut self, sq: usize, clr: Color) -> bool;
}

impl PassedPawnEvalTrait for Board {
    #[inline(always)]
    fn passed_pawn(&mut self, clr: Color) {
        let piece = PAWN + clr;

        let mut bb = self.pawn_bb(clr);
        while let Some(sq) = bb.next() {
            if !self.passed_leverable(sq, clr) {
                continue;
            }
            let king_proximity = self.king_proximity(sq, clr);

            let passed_block = self.passed_blocked(sq, clr);

            let passed_file = self.passed_file(sq);
            self.sum(clr, Some(sq), Some(piece), (0, king_proximity));
            self.sum(clr, Some(sq), Some(piece), PASSED_PAWN_REW[clr.idx()][get_rank(sq)]);
            self.sum(clr, Some(sq), Some(piece), (passed_block, passed_block));
            self.sum(clr, Some(sq), Some(piece), (-11 * passed_file, -8 * passed_file));
        }
    }

    #[inline(always)]
    fn passed_leverable(&mut self, sq: usize, clr: Color) -> bool {
        if !self.candidate_passed(sq, clr) {
            // println!("Candidate Not Passed on sq: {:?}", sq);
            return false;
        }

        if !self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp())) {
            return true;
        }

        let mut bb = PAWN_ATTACK_LOOKUP[clr.opp().idx()][sq] & self.pawn_bb(clr);

        while let Some(square) = bb.next() {
            let front_sq = self.front_sq(square, clr);
            let is_occupied = self.occ_bb(clr.opp()).is_set(front_sq);
            let is_more_def = self.eval.attack_map[clr.idx()].is_set(front_sq)
                || !self.eval.attacked_by_2[clr.opp().idx()].is_set(front_sq);
            if !is_occupied && is_more_def {
                return true;
            }
        }

        return false;
    }

    #[inline(always)]
    fn passed_file(&mut self, sq: usize) -> isize {
        let file = get_file(sq) as isize;
        file.min(7 - file)
    }

    #[inline(always)]
    fn passed_blocked(&mut self, sq: usize, clr: Color) -> isize {
        let (own, enemy) = self.both_occ_bb(clr);
        let clr_rank = CLR_RANK[clr.idx()][get_rank(sq) as usize];

        if clr_rank <= 2 || (own | enemy).is_set(self.front_sq(sq, clr)) {
            return 0;
        }

        let weight = 5 * clr_rank - 13;
        let forward = PAWN_FORWARD_SPANS[clr.idx()][sq];
        let backward = PAWN_FORWARD_SPANS[clr.opp().idx()][sq];
        let forward_lr = FORWARD_SPANS_LR[clr.idx()][sq];

        let mut defended_bb =
            forward & (self.eval.defend_map[clr.idx()] | self.eval.attack_map[clr.idx()]);

        // print_bitboard(forward, None);
        // print_bitboard(self.eval.defend_map[clr.opp().idx()], None);
        // print_bitboard(self.eval.attack_map[clr.opp().idx()], None);

        let mut unsafe_bb = forward
            & (self.eval.defend_map[clr.opp().idx()] | self.eval.attack_map[clr.opp().idx()]);

        // print_bitboard(unsafe_bb, None);

        let mut wunsafe_bb = forward_lr
            & (self.eval.defend_map[clr.opp().idx()] | self.eval.attack_map[clr.opp().idx()]);

        let mut is_defended1 = defended_bb.is_set(self.front_sq(sq, clr));

        let mut is_unsafe1 = unsafe_bb.is_set(self.front_sq(sq, clr));

        if (self.queen_bb(clr) | self.rook_bb(clr)) & backward != 0 {
            is_defended1 = true;
            defended_bb = 1;
        }

        if (self.queen_bb(clr.opp()) | self.rook_bb(clr.opp())) & backward != 0 {
            is_unsafe1 = true;
            unsafe_bb = 1;
        }

        let mut k = 0;

        // println!("Unsafe: {:?}", unsafe_bb.count());
        // println!("Wunsafe: {:?}", wunsafe_bb.count());
        // println!("is_unsafe1: {:?}", is_unsafe1);
        // println!("is_defended1: {:?}", is_defended1);

        if unsafe_bb == 0 && wunsafe_bb == 0 {
            k = 35;
        } else if unsafe_bb == 0 {
            k = 20;
        } else if !is_unsafe1 {
            k = 9;
        }

        if is_defended1 {
            k += 5;
        }

        return k * (weight as isize);
    }

    #[inline(always)]
    fn king_proximity(&mut self, sq: usize, clr: Color) -> isize {
        let mut score = 0;

        let clr_rank = CLR_RANK[clr.idx()][get_rank(sq) as usize];

        if clr_rank <= 2 {
            return 0;
        }

        let weight = (5 * clr_rank - 13) as isize;

        let front_sq = self.front_sq(sq, clr);

        score += (self.king_dist(clr.opp(), front_sq).min(5) * 19 / 4) as isize * weight;
        score -= (self.king_dist(clr, front_sq).min(5) * 2) as isize * weight;

        // Consider another push if the next square is the queening square
        if clr_rank != 6 {
            score -= self.king_dist(clr, front_sq).min(5) as isize * weight;
        }
        score
    }

    #[inline(always)]
    fn candidate_passed(&mut self, sq: usize, clr: Color) -> bool {
        let forward = PAWN_FORWARD_SPANS[clr.idx()][sq];
        let forward_lr = FORWARD_SPANS_LR[clr.idx()][sq];
        let our_pawns = self.pawn_bb(clr);
        let their_pawns = self.pawn_bb(clr.opp());

        // Own pawn ahead? Blocked by same-file pawn
        // println!("Try Candidate Passed : {:?}", sq);
        if forward & our_pawns != 0 {
            return false;
        }

        // No enemy pawn in any of the 3 forward files → clearly candidate
        if forward & their_pawns == 0 && forward_lr & their_pawns == 0 {
            return true;
        }

        let one_forward = self.front_sq(sq, clr);
        let double_forward = self.front_sq(one_forward, clr);
        if (double_forward < 64 && FORWARD_SPANS_LR[clr.idx()][double_forward] & their_pawns != 0)
            || PAWN_FORWARD_SPANS[clr.idx()][self.front_sq(sq, clr)] & their_pawns != 0
        {
            return false;
        }

        if FORWARD_SPANS_LR[clr.idx()][self.back_sq(sq, clr)] & their_pawns == 0
            && self.blocked_pawn(sq, clr, their_pawns)
            && CLR_RANK[clr.idx()][get_rank(sq)] > 3
        {
            let mut bb = PAWN_ATTACK_LOOKUP[clr.opp().idx()][sq] & self.pawn_bb(clr);
            while let Some(square) = bb.next() {
                let front_sq = self.front_sq(square, clr);
                let double_front_sq = self.front_sq(front_sq, clr);
                if !their_pawns.is_set(front_sq) && !their_pawns.is_set(double_front_sq) {
                    return true;
                }
            }
        }

        if self.blocked_pawn(sq, clr, their_pawns) {
            return false;
        }

        let lever_mask = PAWN_ATTACK_LOOKUP[clr.idx()][sq] & their_pawns;
        let leverpush_mask = PAWN_ATTACK_LOOKUP[clr.idx()][self.front_sq(sq, clr)] & their_pawns;
        let phalanx_mask = PAWN_ATTACK_LOOKUP[clr.idx()][self.back_sq(sq, clr)] & our_pawns;

        let lever = lever_mask.count();
        let leverpush = leverpush_mask.count();
        let phalanx = phalanx_mask.count();
        let supported = self.supported_pawn(sq, clr) as usize;

        if lever > supported + 1 {
            return false;
        }
        if leverpush > phalanx {
            return false;
        }
        if lever > 0 && leverpush > 0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::color::{BLACK, WHITE};
    use crate::engine::board::fen::FenTrait;
    use crate::engine::evaluation::init_eval::InitEvalTrait;
    use crate::engine::evaluation::test_evaluation::{eval_assert, SF_EVAL};

    use super::*;

    // NOTE: 8. PASSED PAWNS [FIXME: TEST: WORKS]
    #[test]
    fn passed_pawns_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.passed_pawn(WHITE);
            board.passed_pawn(BLACK);

            eval_assert(board.calculate_score(), obj.passed_pawn, 10, false);
        }
    }
}
