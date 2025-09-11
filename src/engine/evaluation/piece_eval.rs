use crate::engine::attacks::bishop::{get_bishop_mask, BLACK_SQUARES, WHITE_SQUARES};
use crate::engine::attacks::pawn::get_all_pawn_forward_mask;
use crate::engine::attacks::rook::get_rook_mask;
use crate::engine::board::board::Board;
use crate::engine::board::color::{Color, ColorTrait};
use crate::engine::board::piece::{PieceTrait, BISHOP, KNIGHT, PAWN, ROOK};
use crate::engine::board::square::get_file;
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::generated::between::BETWEEN_BB;
use crate::engine::misc::bitboard::{BitboardTrait, Iterator};
use crate::engine::misc::const_utility::{FILE_BITBOARD, RANK_BITBOARD};

pub const OUTPOST_RANKS: [u64; 2] = [
    RANK_BITBOARD[3] | RANK_BITBOARD[4] | RANK_BITBOARD[5],
    RANK_BITBOARD[4] | RANK_BITBOARD[3] | RANK_BITBOARD[2],
];

pub const QUEEN_INFILTRATION: [u64; 2] = [
    RANK_BITBOARD[7] | RANK_BITBOARD[6] | RANK_BITBOARD[5] | RANK_BITBOARD[4],
    RANK_BITBOARD[0] | RANK_BITBOARD[1] | RANK_BITBOARD[2] | RANK_BITBOARD[3],
];

pub trait PieceEvalTrait {
    fn piece_eval(&mut self, clr: Color);
    fn outpost(&mut self, clr: Color) -> u64;
    fn reachable_outpost(&mut self, clr: Color) -> u64;
    fn minor_behind_pawn(&mut self, clr: Color) -> isize;
    fn bishop_pawns(&mut self, clr: Color) -> isize;
    fn trapped_rook(&mut self, clr: Color);
    fn weak_queen(&mut self, clr: Color) -> isize;
    fn king_protector(&mut self, clr: Color);
    fn outpost_total(&mut self, clr: Color);
    fn rook_on_queen_file(&mut self, clr: Color);
    fn bishop_xray_pawns(&mut self, clr: Color) -> isize;
    fn rook_on_king_ring(&mut self, clr: Color) -> isize;
    fn bishop_on_king_ring(&mut self, clr: Color) -> isize;
    fn queen_infaltration(&mut self, clr: Color) -> isize;
}

impl PieceEvalTrait for Board {
    #[inline(always)]
    fn piece_eval(&mut self, clr: Color) {
        let bonus = self.minor_behind_pawn(clr);
        self.sum(clr, None, None, (18 * bonus, 3 * bonus));

        let bonus = self.bishop_pawns(clr);
        self.sum(clr, None, None, (-3 * bonus, -5 * bonus));

        let bonus = self.bishop_xray_pawns(clr);
        self.sum(clr, None, None, (-4 * bonus, -5 * bonus));

        self.rook_on_queen_file(clr);

        let bonus = self.rook_on_king_ring(clr);
        self.sum(clr, None, None, (16 * bonus, 0));

        let bonus = self.bishop_on_king_ring(clr);
        self.sum(clr, None, None, (24 * bonus, 0));

        self.trapped_rook(clr);
        // FIXME, If Castle is not awailable add 2 * 55 / 2 * 13
        // self.sum(clr, None, None, (-55 * bonus, -13 * bonus));

        let bonus = self.weak_queen(clr);
        self.sum(clr, None, None, (-56 * bonus, -15 * bonus));

        let bonus = self.queen_infaltration(clr);
        self.sum(clr, None, None, (-2 * bonus, 14 * bonus));

        self.king_protector(clr);

        self.outpost_total(clr);

        let bonus = (self.rook_bb(clr)
            & (self.eval.open_file[clr.idx()] & self.eval.open_file[clr.opp().idx()]))
        .count() as isize;
        self.sum(clr, None, None, (48 * bonus, 29 * bonus));

        let bonus = (self.rook_bb(clr) & self.eval.open_file[clr.idx()]).count() as isize;
        self.sum(clr, None, None, (19 * bonus, 7 * bonus));
    }

    #[inline(always)]
    fn outpost(&mut self, clr: Color) -> u64 {
        (self.knight_bb(clr) | self.bishop_bb(clr)) & self.eval.outpost[clr.idx()]
    }

    #[inline(always)]
    fn reachable_outpost(&mut self, clr: Color) -> u64 {
        let att = self.eval.attacked_by[(KNIGHT + clr).idx()]
            | self.eval.attacked_by[(BISHOP + clr).idx()];
        self.eval.outpost[clr.idx()] & !self.occ_bb(clr) & att
        // let reachable_bb = self.eval.outpost[clr.idx()] & !self.occ_bb(clr) & att;
        // (reachable_bb.count() * 2) as isize
    }

    #[inline(always)]
    fn minor_behind_pawn(&mut self, clr: Color) -> isize {
        let all_pawns = self.pawn_bb(clr) | self.pawn_bb(clr.opp());
        ((self.knight_bb(clr) | self.bishop_bb(clr))
            & get_all_pawn_forward_mask(all_pawns, clr.opp()))
        .count() as isize
    }

    #[inline(always)]
    fn bishop_pawns(&mut self, clr: Color) -> isize {
        let mut score = 0;
        let mut blocked = get_all_pawn_forward_mask(self.pawn_bb(clr), clr)
            & (self.occ_bb(clr) | self.occ_bb(clr.opp()))
            & (FILE_BITBOARD[2] | FILE_BITBOARD[3] | FILE_BITBOARD[4] | FILE_BITBOARD[5]);
        blocked = get_all_pawn_forward_mask(blocked, clr.opp());
        // print_bitboard(blocked, None);

        for squares in [WHITE_SQUARES, BLACK_SQUARES] {
            let bishops_on_sq = self.bishop_bb(clr) & squares;
            // print_bitboard(bishops_on_sq, None);

            let pawns_on_sq = self.pawn_bb(clr) & squares;
            // print_bitboard(pawns_on_sq, None);

            // let blocked_on_sq = blocked & squares;
            // print_bitboard(blocked_on_sq, None);

            let att_bishops =
                if self.eval.attacked_by[(PAWN + clr).idx()] & bishops_on_sq > 0 { 0 } else { 1 };
            // print_bitboard(att_bishops, None);

            score += pawns_on_sq.count()
                * (blocked.count() * bishops_on_sq.count() + att_bishops * bishops_on_sq.count());

            // println!("Temp Score: {:?}", score);
        }

        // println!("Final Score: {:?}", score);
        return score as isize;
    }

    #[inline(always)]
    fn trapped_rook(&mut self, clr: Color) {
        let mut bb = self.rook_bb(clr) & !self.eval.open_file[clr.idx()];
        let king_file = get_file(self.king_sq(clr));
        while let Some(sq) = bb.next() {
            if (self.x_ray_mask(ROOK + clr, sq).count() <= 3)
                && ((king_file < 4) == (get_file(sq) < king_file))
            {
                let mut castling = 2;
                if self.state.castling.long(clr) != 0 || self.state.castling.short(clr) != 0 {
                    castling = 1;
                }
                self.sum(clr, Some(sq), Some(ROOK + clr), (-55 * castling, -13 * castling));
            }
        }
    }

    #[inline(always)]
    fn weak_queen(&mut self, clr: Color) -> isize {
        let mut bb = self.queen_bb(clr);
        let mut count = 0;
        while let Some(sq_to) = bb.next() {
            let mut bb = (get_rook_mask(sq_to, 0, 0, clr) & self.rook_bb(clr.opp()))
                | (get_bishop_mask(sq_to, 0, 0, clr) & self.bishop_bb(clr.opp()));

            while let Some(sq_from) = bb.next() {
                if BETWEEN_BB[sq_from][sq_to].count() == 3 {
                    count += 1;
                    break;
                }
            }
        }

        return count;
    }

    #[inline(always)]
    fn king_protector(&mut self, clr: Color) {
        let mut bb = self.knight_bb(clr);
        while let Some(sq) = bb.next() {
            let dx = self.king_dist(clr, sq) as isize;
            self.sum(clr, Some(sq), Some(KNIGHT), (-8 * dx, -9 * dx));
        }

        let mut bb = self.bishop_bb(clr);
        while let Some(sq) = bb.next() {
            let dx = self.king_dist(clr, sq) as isize;
            self.sum(clr, Some(sq), Some(BISHOP), (-6 * dx, -9 * dx));
        }
    }

    #[inline(always)]
    fn outpost_total(&mut self, clr: Color) {
        let mut bb = self.knight_bb(clr);
        while let Some(sq) = bb.next() {
            let reachable_bb = self.eval.outpost[clr.idx()]
                & self.x_ray_mask(KNIGHT + clr, sq)
                & !self.occ_bb(clr);
            if !self.eval.outpost[clr.idx()].is_set(sq) && reachable_bb > 0 {
                self.sum(clr, Some(sq), Some(KNIGHT + clr), (31, 22));
                break;
            }
        }

        let bonus = (self.knight_bb(clr) & self.eval.outpost[clr.idx()]).count() as isize;
        self.sum(clr, None, Some(KNIGHT + clr), (bonus * 56, bonus * 36));

        let bonus = (self.bishop_bb(clr) & self.eval.outpost[clr.idx()]).count() as isize;
        self.sum(clr, None, Some(BISHOP + clr), (bonus * 30, bonus * 23));
        // NOTE: FIXME: NOT FULL EVAL BUT AN OK ONE
        // Only the +2 is missing
    }

    #[inline(always)]
    fn rook_on_queen_file(&mut self, clr: Color) {
        let mut bb = self.rook_bb(clr);
        let all_queens = self.queen_bb(clr) | self.queen_bb(clr.opp());
        while let Some(sq) = bb.next() {
            if all_queens & FILE_BITBOARD[get_file(sq)] != 0 {
                self.sum(clr, Some(sq), Some(ROOK), (6, 11));
            }
        }
    }

    #[inline(always)]
    fn bishop_xray_pawns(&mut self, clr: Color) -> isize {
        let mut count = 0;
        let mut bb = self.bishop_bb(clr);
        while let Some(sq) = bb.next() {
            count += (get_bishop_mask(sq, 0, 0, clr) & self.pawn_bb(clr.opp())).count();
        }

        return count as isize;
    }

    #[inline(always)]
    fn rook_on_king_ring(&mut self, clr: Color) -> isize {
        let mut count = 0;
        let mut bb = self.rook_bb(clr) & !self.eval.king_att_count_pieces[clr.idx()];
        while let Some(sq) = bb.next() {
            if self.eval.king_ring[clr.opp().idx()] & RANK_BITBOARD[get_file(sq)] != 0 {
                count += 1;
            }
        }
        count
    }

    #[inline(always)]
    fn bishop_on_king_ring(&mut self, clr: Color) -> isize {
        let mut count = 0;
        let mut bb = self.bishop_bb(clr) & !self.eval.king_att_count_pieces[clr.idx()];
        while let Some(sq) = bb.next() {
            if self.eval.king_ring[clr.opp().idx()]
                & get_bishop_mask(sq, self.pawn_bb(clr), self.pawn_bb(clr.opp()), clr)
                != 0
            {
                count += 1;
            }
        }
        count
    }

    #[inline(always)]
    fn queen_infaltration(&mut self, clr: Color) -> isize {
        let bb = self.queen_bb(clr)
            & QUEEN_INFILTRATION[clr.idx()]
            & !self.eval.attacked_by[(PAWN + clr.opp()).idx()]
            & !self.eval.pawn_att_span[clr.opp().idx()];
        bb.count() as isize
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::color::{BLACK, WHITE};
    use crate::engine::board::fen::FenTrait;
    use crate::engine::evaluation::init_eval::InitEvalTrait;
    use crate::engine::evaluation::test_evaluation::{eval_assert, SF_EVAL};

    use super::*;

    #[test]
    fn pieces_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.piece_eval(WHITE);
            board.piece_eval(BLACK);
            eval_assert(board.calculate_score(), obj.piece, 45, false); // FIXME: The diff is quite high here
        }
    }
}
