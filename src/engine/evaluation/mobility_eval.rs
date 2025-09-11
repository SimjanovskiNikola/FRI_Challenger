use crate::engine::attacks::bishop::get_bishop_mask;
use crate::engine::attacks::knight::get_knight_mask;
use crate::engine::attacks::queen::get_queen_mask;
use crate::engine::attacks::rook::get_rook_mask;
use crate::engine::board::board::Board;
use crate::engine::board::color::{Color, ColorTrait};
use crate::engine::board::piece::{
    Piece, PieceTrait, BISHOP, KNIGHT, PAWN, PIECES_WITHOUT_PAWN_KING, QUEEN, ROOK,
};
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::misc::bitboard::{BitboardTrait, Iterator};

pub const KNIGHT_MOBILITY: [(isize, isize); 9] =
    [(-62, -81), (-53, -56), (-12, -31), (-4, -16), (3, 5), (13, 11), (22, 17), (28, 20), (33, 25)];

#[rustfmt::skip]
pub const BISHOP_MOBILITY: [(isize, isize); 14] = [
    (-48, -59), (-20, -23), (16, -3), (26, 13), (38, 24), (51, 42), (55, 54), 
    (63, 57), (63, 65), (68, 73), (81, 78), (81, 86), (91, 88), (98, 97),
];

#[rustfmt::skip]
pub const ROOK_MOBILITY: [(isize, isize); 15] = [
    (-60, -78), (-20, -17), (2, 23), (3, 39), (3, 70), (11, 99), (22, 103), (31, 121),
    (40, 134), (40, 139), (41, 158), (48, 164), (57, 168), (57, 169), (62, 172),
];

#[rustfmt::skip]
pub const QUEEN_MOBILITY: [(isize, isize); 28] = [
    (-30, -48), (-12, -30), (-8, -7), (-9, 19), (20, 40), (23, 55), (23, 59),
    (35, 75), (38, 78), (53, 96), (64, 96), (65, 100), (65, 121), (66, 127),
    (67, 131), (67, 133), (72, 136), (72, 141), (77, 147), (79, 150), (93, 151),
    (108, 168), (108, 168), (108, 171), (110, 182), (114, 182), (114, 192), (116, 219),
];

pub trait MobilityEvalTrait {
    fn mobility_eval(&mut self, clr: Color);
    fn mobility_bonus(&mut self, piece: Piece, sq: usize) -> (isize, isize);
    fn mobility_area(&mut self, clr: Color) -> u64;
    fn mobility_piece(&mut self, sq: usize, piece: Piece, clr: Color) -> u64;
}

impl MobilityEvalTrait for Board {
    #[inline(always)]
    fn mobility_eval(&mut self, clr: Color) {
        let area = self.mobility_area(clr);
        for &pce in &PIECES_WITHOUT_PAWN_KING {
            let piece = pce + clr;
            let mut bb = self.bb(piece);
            while let Some(sq) = bb.next() {
                let safe_squares = (self.mobility_piece(sq, piece, clr) & area).count();
                let bonus = self.mobility_bonus(piece, safe_squares);
                self.sum(clr, Some(sq), Some(piece), bonus);
            }
        }
    }

    #[inline(always)]
    fn mobility_bonus(&mut self, piece: Piece, safe_squares: usize) -> (isize, isize) {
        match piece.kind() {
            KNIGHT => KNIGHT_MOBILITY[safe_squares],
            BISHOP => BISHOP_MOBILITY[safe_squares],
            ROOK => ROOK_MOBILITY[safe_squares],
            QUEEN => QUEEN_MOBILITY[safe_squares],
            _ => panic!("There is other peace that was not expected here"),
        }
    }

    #[inline(always)]
    fn mobility_piece(&mut self, sq: usize, piece: Piece, clr: Color) -> u64 {
        let (mut own, mut enemy) = self.both_occ_bb(clr);
        match piece.kind() {
            KNIGHT => get_knight_mask(sq, own, enemy, clr),
            BISHOP => {
                own &= !(self.queen_bb(clr));
                enemy &= !self.queen_bb(clr.opp());
                get_bishop_mask(sq, own, enemy, clr)
            }
            ROOK => {
                own &= !(self.queen_bb(clr) | self.rook_bb(clr));
                enemy &= !self.queen_bb(clr.opp());
                get_rook_mask(sq, own, enemy, clr)
            }
            QUEEN => get_queen_mask(sq, own, enemy, clr),
            _ => panic!("There is other peace that was not expected here"),
        }
    }

    #[inline(always)]
    fn mobility_area(&mut self, clr: Color) -> u64 {
        let bb = (u64::MAX)
            & !self.king_bb(clr)
            & !self.queen_bb(clr)
            & !self.pawn_bb(clr)
            & !self.eval.attacked_by[(PAWN + clr.opp()).idx()];
        bb
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
    fn mobility_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.mobility_eval(WHITE);
            board.mobility_eval(BLACK);
            eval_assert(board.calculate_score(), obj.mobility, 28, false); // FIXME: The difference is too quiet high
        }
    }
}
