use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color::{Color, ColorTrait};
use crate::engine::board::structures::piece::{PieceTrait, PAWN};
use crate::engine::board::structures::square::get_rank;
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::evaluation::eval_defs::{BLOCKED_RANKS, CLR_RANK};
use crate::engine::evaluation::evaluation::EvaluationTrait;
use crate::engine::misc::bitboard::{Bitboard, BitboardTrait, Iterator};
use crate::engine::move_generator::generated::pawn::{
    FORWARD_SPANS_LR, ISOLATED_PAWN_LOOKUP, PAWN_ATTACK_LOOKUP, PAWN_FORWARD_SPANS,
};
use crate::engine::move_generator::pawn::{get_all_pawn_forward_mask, get_pawn_att_mask};

pub trait PawnEvalTrait {
    fn pawns_eval(&mut self, clr: Color);
    fn single_pawn_eval(&mut self, sq: usize, clr: Color);
    fn isolated_pawn(&mut self, sq: usize, clr: Color) -> bool;
    fn opposed_pawn(&mut self, sq: usize, clr: Color) -> bool;
    fn phalanx_pawn(&mut self, sq: usize, clr: Color) -> bool;
    fn supported_pawn(&mut self, sq: usize, clr: Color) -> isize;
    fn backward_pawn(&mut self, sq: usize, clr: Color) -> bool;
    fn doubled_pawn(&mut self, sq: usize, clr: Color) -> bool;
    fn connected_pawn(&mut self, sq: usize, clr: Color) -> bool;
    fn connected_bonus(&mut self, sq: usize, clr: Color) -> isize;
    fn weak_unopposed_pawn(&mut self, sq: usize, clr: Color) -> bool;
    fn weak_lever(&mut self, sq: usize, clr: Color) -> bool;
    fn blocked_pawn(&mut self, sq: usize, clr: Color, bb: u64) -> bool;
    fn blocked_pawn_5th_6th_rank(&mut self, sq: usize, clr: Color) -> isize;
    fn doubled_isolated_pawn(&mut self, sq: usize, clr: Color) -> bool;
}

impl PawnEvalTrait for Board {
    #[inline(always)] // 4. Pawns Eval
    fn pawns_eval(&mut self, clr: Color) {
        let mut bb = self.pawn_bb(clr);
        while let Some(sq) = bb.next() {
            self.single_pawn_eval(sq, clr);
        }
    }

    #[inline(always)]
    fn single_pawn_eval(&mut self, sq: usize, clr: Color) {
        // println!("Sq: {:?}", sq);
        // println!("Double Isolated: {:?}", self.doubled_isolated_pawn(sq, clr));
        // println!("Isolated: {:?}", self.isolated_pawn(sq, clr));
        // println!("Backward Pawn: {:?}", self.backward_pawn(sq, clr));
        // println!("Doubled Pawn: {:?}", self.doubled_pawn(sq, clr));
        // println!("Connected Pawn: {:?}", self.connected_pawn(sq, clr));
        // println!("Weak Unopposed Pawn: {:?}", self.weak_unopposed_pawn(sq, clr));
        // println!("Weak Lever: {:?}", self.weak_lever(sq, clr));
        // println!("Blocked pawn 5th 6th: {:?}", self.blocked_pawn_5th_6th_rank(sq, clr));
        // println!("Connected: {:?}", self.blocked_pawn_5th_6th_rank(sq, clr));

        if self.doubled_isolated_pawn(sq, clr) {
            self.sum(clr, Some(sq), None, (-11, -56));
        } else if self.isolated_pawn(sq, clr) {
            self.sum(clr, Some(sq), None, (-5, -15));
        } else if self.backward_pawn(sq, clr) {
            self.sum(clr, Some(sq), None, (-9, -24));
        }

        // FIXME: Not correct (Needs to check how many doubled are on the same file)
        if self.doubled_pawn(sq, clr) {
            self.sum(clr, Some(sq), None, (-11, -56));
        }

        if self.connected_pawn(sq, clr) {
            let calc_bonus = self.connected_bonus(sq, clr);
            let bonus =
                (calc_bonus, calc_bonus * (CLR_RANK[clr.idx()][get_rank(sq)] as isize - 2) / 4);
            self.sum(clr, Some(sq), None, bonus);
            // FIXME: Check if it is ok to be this a minus sth
            // println!("Connected Bonus: {:?}", bonus);
        }

        if self.weak_unopposed_pawn(sq, clr) {
            self.sum(clr, Some(sq), None, (-13, -27));
        }

        if self.weak_lever(sq, clr) {
            self.sum(clr, Some(sq), None, (0, -56));
        }

        if self.blocked_pawn_5th_6th_rank(sq, clr) == 1 {
            self.sum(clr, Some(sq), None, (-11, -4));
        } else if self.blocked_pawn_5th_6th_rank(sq, clr) == 2 {
            self.sum(clr, Some(sq), None, (-3, 4));
        }
    }

    #[inline(always)]
    fn isolated_pawn(&mut self, sq: usize, clr: Color) -> bool {
        ISOLATED_PAWN_LOOKUP[sq] & self.pawn_bb(clr) == 0
    }

    #[inline(always)]
    fn opposed_pawn(&mut self, sq: usize, clr: Color) -> bool {
        PAWN_FORWARD_SPANS[clr.idx()][sq] & self.pawn_bb(clr.opp()) != 0
    }

    #[inline(always)]
    fn phalanx_pawn(&mut self, sq: usize, clr: Color) -> bool {
        PAWN_ATTACK_LOOKUP[clr.opp().idx()][(sq as isize + 8 * clr.sign()) as usize]
            & self.pawn_bb(clr)
            != 0
    }

    #[inline(always)]
    fn supported_pawn(&mut self, sq: usize, clr: Color) -> isize {
        (PAWN_ATTACK_LOOKUP[clr.opp().idx()][sq] & self.pawn_bb(clr)).count() as isize
    }

    #[inline(always)]
    fn backward_pawn(&mut self, sq: usize, clr: Color) -> bool {
        let front_sq = self.front_sq(sq, clr);
        ((FORWARD_SPANS_LR[clr.opp().idx()][sq]
            | PAWN_ATTACK_LOOKUP[clr.opp().idx()][(sq as isize + 8 * clr.sign()) as usize])
            & self.pawn_bb(clr)
            == 0)
            && (self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp()))
                || self.eval.attacked_by[(PAWN + clr.opp()).idx()].is_set(front_sq))
    }

    #[inline(always)]
    fn doubled_pawn(&mut self, sq: usize, clr: Color) -> bool {
        self.pawn_bb(clr).is_set(self.back_sq(sq, clr)) && self.supported_pawn(sq, clr) == 0
        // PAWN_FORWARD_SPANS[clr.opp().idx()][sq] & self.pawn_bb(clr) != 0
    }

    #[inline(always)]
    fn connected_pawn(&mut self, sq: usize, clr: Color) -> bool {
        self.supported_pawn(sq, clr) > 0 || self.phalanx_pawn(sq, clr)
    }

    #[inline(always)]
    fn connected_bonus(&mut self, sq: usize, clr: Color) -> isize {
        if !self.connected_pawn(sq, clr) {
            return 0;
        }

        let r = CLR_RANK[clr.idx()][get_rank(sq)];
        if r < 1 || r > 6 {
            return 0;
        }

        let seed = [0, 7, 8, 12, 29, 48, 86, 0];
        let op = self.opposed_pawn(sq, clr);
        let ph = self.phalanx_pawn(sq, clr);
        let su = self.supported_pawn(sq, clr);
        let bl = self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp()));

        // println!("Sq: {:?}", sq);
        // println!("Opposed Pawn: {:?}", self.opposed_pawn(sq, clr));
        // println!("Phalanx: {:?}", self.phalanx_pawn(sq, clr));
        // println!("Supported: {:?}", self.supported_pawn(sq, clr));
        // println!("Blocked Pawn: {:?}", self.doubled_pawn(sq, clr));
        // println!("Bonus: {:?}", seed[r] * (2 + ph as isize - op as isize) + 21 * su as isize);

        return seed[r] * (2 + ph as isize - op as isize) + 21 * su as isize;
    }

    #[inline(always)]
    fn weak_unopposed_pawn(&mut self, sq: usize, clr: Color) -> bool {
        !self.opposed_pawn(sq, clr) && (self.isolated_pawn(sq, clr) || self.backward_pawn(sq, clr))
    }

    #[inline(always)]
    fn weak_lever(&mut self, sq: usize, clr: Color) -> bool {
        self.supported_pawn(sq, clr) == 0
            && (get_pawn_att_mask(sq, 0, 0, clr) & self.pawn_bb(clr.opp())).count() == 2
    }

    #[inline(always)]
    fn blocked_pawn(&mut self, sq: usize, clr: Color, bb: u64) -> bool {
        get_all_pawn_forward_mask(Bitboard::init(sq), clr) & bb != 0
    }

    #[inline(always)] // Blocked only on the 5th and 6 rank
    fn blocked_pawn_5th_6th_rank(&mut self, sq: usize, clr: Color) -> isize {
        if BLOCKED_RANKS[clr.idx()].is_set(sq)
            && self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp()))
        {
            return CLR_RANK[clr.idx()][get_rank(sq)].abs_diff(3) as isize;
        }
        return 0;
    }

    #[inline(always)]
    fn doubled_isolated_pawn(&mut self, sq: usize, clr: Color) -> bool {
        PAWN_FORWARD_SPANS[clr.opp().idx()][sq] & self.pawn_bb(clr) != 0
            && self.opposed_pawn(sq, clr)
            && self.isolated_pawn(sq, clr)
            && self.isolated_pawn(sq, clr.opp())

        // self.doubled_pawn(sq, clr)
        //     && self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp()))
        //     && self.isolated_pawn(sq, clr)
        //     && self.isolated_pawn((sq as isize + 8 * clr.sign()) as usize, clr)
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::fen::FenTrait;
    use crate::engine::board::structures::color::{BLACK, WHITE};
    use crate::engine::evaluation::init_eval::InitEvalTrait;
    use crate::engine::evaluation::test_evaluation::SF_EVAL;

    use super::*;

    // NOTE: 4. PAWNS [TEST: WORKS]
    #[test]
    fn pawns_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.pawns_eval(WHITE);
            board.pawns_eval(BLACK);

            assert_eq!(board.calculate_score(), obj.pawns);
        }
    }
}
