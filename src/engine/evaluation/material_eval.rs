use crate::engine::board::board::Board;
use crate::engine::board::color::Color;
use crate::engine::board::piece::*;
use crate::engine::evaluation::common_eval::CommonEvalTrait;

#[rustfmt::skip]
pub const PIECE_MATERIAL: [(isize, isize); 6] = [
    ( 124, 206), // Pawn
    ( 781, 854), // Knight
    (   0,   0), // King
    ( 825, 915), // Bishop
    (1276,1380), // Rook
    (2538,2682), // Queen 
];

pub trait MaterialEvalTrait {
    fn material_eval(&mut self, clr: Color);
    fn non_pawn_material_eval(&mut self, clr: Color) -> isize;
    fn piece_material(&mut self, piece: Piece) -> (isize, isize);
}

impl MaterialEvalTrait for Board {
    #[inline(always)]
    fn material_eval(&mut self, clr: Color) {
        for &pce in &PIECES {
            let piece = pce + clr;
            let count = self.p_count[piece.idx()] as isize;
            let (mg_sum, eg_sum) = MaterialEvalTrait::piece_material(self, piece);
            self.sum(clr, None, Some(piece), (mg_sum * count, eg_sum * count));
        }
    }

    #[inline(always)]
    fn non_pawn_material_eval(&mut self, clr: Color) -> isize {
        let mut score = 0;
        for &pce in &PIECES_WITHOUT_PAWN {
            let piece = pce + clr;
            let count = self.p_count[piece.idx()] as isize;
            score += MaterialEvalTrait::piece_material(self, piece).0 * count;
        }
        score
    }

    #[inline(always)]
    fn piece_material(&mut self, piece: Piece) -> (isize, isize) {
        PIECE_MATERIAL[piece.arr_idx()]
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::color::{BLACK, WHITE};
    use crate::engine::board::fen::FenTrait;
    use crate::engine::evaluation::init_eval::InitEvalTrait;
    use crate::engine::evaluation::test_evaluation::{eval_assert, SF_EVAL};

    use super::*;

    // NOTE: 1. MATERIAL [TEST: WORKS]
    #[test]
    fn material_test() {
        for obj in &SF_EVAL {
            // if obj.fen != SF_EVAL[10].fen {
            //     continue;
            // }

            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.material_eval(WHITE);
            board.material_eval(BLACK);

            eval_assert(board.calculate_score(), obj.material, 0, false);
        }
    }
}
