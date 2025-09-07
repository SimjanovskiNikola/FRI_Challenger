use crate::engine::attacks::knight::get_knight_mask;
use crate::engine::attacks::pawn::{
    get_all_pawn_forward_mask, get_all_pawn_left_att_mask, get_all_pawn_right_att_mask,
};
use crate::engine::board::board::Board;
use crate::engine::board::color::{Color, ColorTrait};
use crate::engine::board::piece::*;
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::evaluation::mobility_eval::MobilityEvalTrait;
use crate::engine::misc::bitboard::{BitboardTrait, Iterator};
use crate::engine::misc::const_utility::RANK_BITBOARD;

pub const ROOK_THREAT: [(isize, isize); 6] =
    [(3, 46), (37, 68), (0, 0), (42, 60), (0, 38), (58, 41)];

pub const MINOR_THREAT: [(isize, isize); 6] =
    [(5, 32), (57, 41), (0, 0), (77, 56), (88, 119), (79, 161)];

pub trait ThreatsEvalTrait {
    fn threats_eval(&mut self, clr: Color);
    fn safe_pawn(&mut self, clr: Color) -> u64;
    fn threat_safe_pawn(&mut self, clr: Color) -> u64;
    fn weak_enemy(&mut self, clr: Color) -> u64;
    fn minor_threat(&mut self, clr: Color);
    fn rook_threat(&mut self, clr: Color);
    fn hanging(&mut self, clr: Color) -> u64;
    fn king_threat(&mut self, clr: Color) -> u64;
    fn slider_on_queen(&mut self, clr: Color) -> isize;
    fn knight_on_queen(&mut self, clr: Color) -> isize;
    fn restricted(&mut self, clr: Color) -> u64;
    fn weak_queen_protection(&mut self, clr: Color) -> u64;
    fn pawn_push_threat(&mut self, clr: Color) -> u64;
}

impl ThreatsEvalTrait for Board {
    #[inline(always)]
    fn threats_eval(&mut self, clr: Color) {
        let bonus = self.hanging(clr).count() as isize;
        self.sum(clr, None, None, (69 * bonus, 36 * bonus));

        if self.king_threat(clr) > 0 {
            self.sum(clr, None, Some(KING + clr), (24, 89));
        }

        let bonus = self.pawn_push_threat(clr).count() as isize;
        self.sum(clr, None, None, (48 * bonus, 39 * bonus));

        let bonus = self.threat_safe_pawn(clr).count() as isize;
        self.sum(clr, None, None, (173 * bonus, 94 * bonus));

        let bonus = self.slider_on_queen(clr);
        self.sum(clr, None, None, (60 * bonus, 18 * bonus));

        let bonus = self.knight_on_queen(clr);
        self.sum(clr, None, None, (16 * bonus, 11 * bonus));

        let bonus = self.restricted(clr).count() as isize;
        self.sum(clr, None, None, (7 * bonus, 7 * bonus));

        let bonus = self.weak_queen_protection(clr).count() as isize;
        self.sum(clr, None, None, (14 * bonus, 0));

        self.minor_threat(clr);
        self.rook_threat(clr);
    }

    #[inline(always)]
    fn safe_pawn(&mut self, clr: Color) -> u64 {
        let bb = (self.pawn_bb(clr) & self.eval.defend_map[clr.idx()])
            | (self.pawn_bb(clr) & !self.eval.attack_map[clr.opp().idx()]);
        bb
    }

    #[inline(always)]
    fn threat_safe_pawn(&mut self, clr: Color) -> u64 {
        let bb = self.knight_bb(clr.opp())
            | self.bishop_bb(clr.opp())
            | self.rook_bb(clr.opp())
            | self.queen_bb(clr.opp());

        (bb & get_all_pawn_left_att_mask(self.safe_pawn(clr), clr))
            | (bb & get_all_pawn_right_att_mask(self.safe_pawn(clr), clr))
    }

    #[inline(always)]
    fn weak_enemy(&mut self, clr: Color) -> u64 {
        let weak_enemy_bb = self.occ_bb(clr.opp())
            & !get_all_pawn_left_att_mask(self.pawn_bb(clr.opp()), clr.opp())
            & !get_all_pawn_right_att_mask(self.pawn_bb(clr.opp()), clr.opp())
            & self.eval.attack_map[clr.idx()];
        let att_twice = weak_enemy_bb & self.eval.attacked_by_2[clr.idx()];
        let not_def_twice = weak_enemy_bb & !self.eval.defended_by_2[clr.opp().idx()];

        att_twice | not_def_twice
    }

    #[inline(always)]
    fn minor_threat(&mut self, clr: Color) {
        let bishop = BISHOP + clr;
        let knight = KNIGHT + clr;
        let mut bb = (self.weak_enemy(clr) | (self.occ_bb(clr.opp()) & !self.pawn_bb(clr.opp())))
            & (self.eval.attacked_by[bishop.idx()] | self.eval.attacked_by[knight.idx()]);

        while let Some(sq) = bb.next() {
            let piece = self.piece_sq(sq);
            self.sum(clr, Some(sq), Some(piece), MINOR_THREAT[piece.arr_idx()]);
        }
    }

    #[inline(always)]
    fn rook_threat(&mut self, clr: Color) {
        let piece = ROOK + clr;
        let mut bb = self.weak_enemy(clr) & self.eval.attacked_by[piece.idx()];

        while let Some(sq) = bb.next() {
            let piece = self.piece_sq(sq);
            self.sum(clr, Some(sq), Some(piece), ROOK_THREAT[piece.arr_idx()]);
        }
    }

    #[inline(always)]
    fn hanging(&mut self, clr: Color) -> u64 {
        let weak_enemy_bb = self.weak_enemy(clr);
        let att_many =
            weak_enemy_bb & !self.pawn_bb(clr.opp()) & self.eval.attacked_by_2[clr.idx()];
        let not_defended = weak_enemy_bb & !self.eval.defend_map[clr.opp().idx()];

        not_defended | att_many
    }

    #[inline(always)]
    fn king_threat(&mut self, clr: Color) -> u64 {
        let king = KING + clr;
        self.eval.attacked_by[king.idx()] & self.weak_enemy(clr)
    }

    #[inline(always)]
    fn pawn_push_threat(&mut self, clr: Color) -> u64 {
        let clr_push_ranks = [2, 5];
        let both_occ = self.occ_bb(clr) | self.occ_bb(clr.opp());
        let mut pawn_threats = 0;
        let pawn_one_push = get_all_pawn_forward_mask(self.pawn_bb(clr), clr) & !both_occ;
        let pawn_two_push = get_all_pawn_forward_mask(
            pawn_one_push & RANK_BITBOARD[clr_push_ranks[clr.idx()]],
            clr,
        ) & !both_occ;
        pawn_threats =
            (pawn_one_push | pawn_two_push) & !self.eval.attacked_by[(PAWN + clr.opp()).idx()];

        pawn_threats = (pawn_threats & self.eval.attack_map[clr.idx()])
            | (pawn_threats & !self.eval.attack_map[clr.opp().idx()]);

        return (get_all_pawn_left_att_mask(pawn_threats, clr)
            | get_all_pawn_right_att_mask(pawn_threats, clr))
            & self.occ_bb(clr.opp());
    }

    #[inline(always)]
    fn slider_on_queen(&mut self, clr: Color) -> isize {
        if self.queen_bb(clr.opp()).count() != 1 {
            return 0;
        }

        // TODO: Try using self.eval.xray here
        let mut mobility_bb = (self.eval.attacked_by[(QUEEN + clr.opp()).idx()]
            | self.eval.defended_by[(QUEEN + clr.opp()).idx()])
            & self.mobility_area(clr);

        mobility_bb = mobility_bb
            & !self.pawn_bb(clr)
            & !self.eval.attacked_by[(PAWN + clr.opp()).idx()]
            & self.eval.attacked_by_2[clr.idx()];

        let diagonal = mobility_bb & self.eval.queen_diagonal[clr.opp().idx()];
        let orthogonal = mobility_bb & !self.eval.queen_diagonal[clr.opp().idx()];

        let v = if self.queen_bb(clr).count() == 0 { 2 } else { 1 };

        return ((diagonal & self.eval.x_ray[(BISHOP + clr).idx()])
            | (orthogonal & self.eval.x_ray[(ROOK + clr).idx()]))
        .count() as isize
            * v;
    }

    #[inline(always)]
    fn knight_on_queen(&mut self, clr: Color) -> isize {
        if self.queen_bb(clr.opp()).count() != 1 {
            return 0;
        }

        let sq = self.queen_bb(clr.opp()).get_lsb();
        let mut mobility_bb = self.mobility_area(clr)
            & get_knight_mask(sq, 0, 0, 0)
            & self.eval.attacked_by[(KNIGHT + clr).idx()];

        mobility_bb =
            mobility_bb & !self.pawn_bb(clr) & !self.eval.attacked_by[(PAWN + clr.opp()).idx()];

        mobility_bb = mobility_bb
            & (self.eval.attacked_by_2[clr.idx()] | !self.eval.attacked_by_2[clr.opp().idx()]);

        let v = if self.queen_bb(clr).count() == 0 { 2 } else { 1 };

        return mobility_bb.count() as isize * v;
    }

    #[inline(always)]
    fn restricted(&mut self, clr: Color) -> u64 {
        let restricted_bb = (self.eval.attack_map[clr.idx()] | self.eval.defend_map[clr.idx()])
            & (self.eval.attack_map[clr.opp().idx()] | self.eval.defend_map[clr.opp().idx()])
            & !self.eval.attacked_by[(PAWN + clr.opp()).idx()]
            & !self.eval.defended_by[(PAWN + clr.opp()).idx()]
            & !((self.eval.attacked_by_2[clr.opp().idx()]
                | self.eval.defended_by_2[clr.opp().idx()])
                & (!(self.eval.attacked_by_2[clr.idx()] | self.eval.defended_by_2[clr.idx()])
                    & (self.eval.attack_map[clr.idx()] | self.eval.defend_map[clr.idx()])));

        restricted_bb
    }

    #[inline(always)]
    fn weak_queen_protection(&mut self, clr: Color) -> u64 {
        let weak_enemy_bb = self.weak_enemy(clr);
        let mut queen_protect = 0;

        let mut bb = self.queen_bb(clr.opp());
        while let Some(sq) = bb.next() {
            queen_protect = weak_enemy_bb & self.get_mask(QUEEN + clr.opp(), sq);
        }

        return queen_protect;
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::color::{BLACK, WHITE};
    use crate::engine::board::fen::FenTrait;
    use crate::engine::evaluation::init_eval::InitEvalTrait;
    use crate::engine::evaluation::test_evaluation::{eval_assert, SF_EVAL};

    use super::*;

    // NOTE: 7. THREATS [TEST: WORKS]
    #[test]
    fn threats_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.threats_eval(WHITE);
            board.threats_eval(BLACK);

            eval_assert(board.calculate_score(), obj.threats, 0, false);
        }
    }
}
