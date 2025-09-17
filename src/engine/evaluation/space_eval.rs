use crate::engine::attacks::pawn::get_all_pawn_forward_mask;
use crate::engine::attacks::pawn::get_all_pawn_left_att_mask;
use crate::engine::attacks::pawn::get_all_pawn_right_att_mask;
use crate::engine::board::board::Board;
use crate::engine::board::color::Color;
use crate::engine::board::color::ColorTrait;
use crate::engine::board::piece::PAWN;
use crate::engine::board::piece::PieceTrait;
use crate::engine::evaluation::common_eval::CLR_CENTER;
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::evaluation::material_eval::MaterialEvalTrait;
use crate::engine::misc::bitboard::BitboardTrait;

pub trait SpaceEvalTrait {
    fn space(&mut self, color: Color);
    fn space_area(&mut self, color: Color) -> usize;
}

impl SpaceEvalTrait for Board {
    #[inline(always)] // 9. Space
    fn space(&mut self, clr: Color) {
        if self.non_pawn_material_eval(clr) + self.non_pawn_material_eval(clr.opp()) < 12222 {
            return;
        }

        let own_pawns_blocked =
            get_all_pawn_forward_mask(self.pawn_bb(clr), clr) & self.pawn_bb(clr.opp());
        let enemy_pawns_blocked =
            get_all_pawn_forward_mask(self.pawn_bb(clr.opp()), clr.opp()) & self.pawn_bb(clr);
        let own_sq_blocked = get_all_pawn_left_att_mask(self.pawn_bb(clr.opp()), clr.opp())
            & get_all_pawn_right_att_mask(self.pawn_bb(clr.opp()), clr.opp())
            & get_all_pawn_forward_mask(self.pawn_bb(clr), clr);
        let enemy_sq_blocked = get_all_pawn_left_att_mask(self.pawn_bb(clr), clr)
            & get_all_pawn_right_att_mask(self.pawn_bb(clr), clr)
            & get_all_pawn_forward_mask(self.pawn_bb(clr.opp()), clr.opp());

        let blocked =
            (own_pawns_blocked | enemy_pawns_blocked | own_sq_blocked | enemy_sq_blocked).count();

        let weight = (self.bb(clr).count() - 3 + blocked.min(9)) as isize;

        let bonus = self.space_area(clr) as isize * weight * weight / 16;
        self.sum(clr, None, None, (bonus, 0));
    }

    #[inline(always)]
    fn space_area(&mut self, clr: Color) -> usize {
        let mut cnt = 0;
        let own_pawns_bb = self.pawn_bb(clr);
        let pawn_behind_bb = self.eval.pawn_behind_masks[clr.idx()];
        let opp_att_bb = self.eval.attack_map[clr.opp().idx()];
        let opp_pawn_att_bb = self.eval.attacked_by[(PAWN + clr.opp()).idx()];

        cnt += (CLR_CENTER[clr.idx()] & !opp_pawn_att_bb & !own_pawns_bb).count();
        cnt += (pawn_behind_bb & CLR_CENTER[clr.idx()] & !opp_att_bb & !own_pawns_bb).count();
        cnt
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::color::{BLACK, WHITE};
    use crate::engine::board::fen::FenTrait;
    use crate::engine::evaluation::init_eval::InitEvalTrait;
    use crate::engine::evaluation::test_evaluation::{SF_EVAL, eval_assert};
    use crate::engine::evaluation::trace_eval::TraceEvalTrait;

    use super::*;

    #[test]
    fn space_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.space(WHITE);
            board.space(BLACK);

            eval_assert(board.calculate_score(), obj.space, 0, false);
        }
    }
}
