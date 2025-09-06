use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color::{Color, ColorTrait};
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::move_generator::bishop::has_bishop_pair;

// Quadratic interaction bonuses for own peaces NOTE: DONE
pub const QUADRATIC_OURS: [[isize; 6]; 6] = [
    [0, 0, 0, 0, 0, 0],   // Bishop pair DEPRECATE: Refactor This as it is not needed
    [40, 38, 0, 0, 0, 0], // Pawn
    [32, 255, -62, 0, 0, 0], // Knight
    [0, 104, 4, 0, 0, 0], // Bishop
    [-26, -2, 47, 105, -208, 0], // Rook
    [-189, 24, 117, 133, -134, -6], // Queen
];

// Quadratic interaction bonuses for their peaces NOTE: DONE
pub const QUADRATIC_THEIRS: [[isize; 6]; 6] = [
    [0, 0, 0, 0, 0, 0],    // Bishop pair DEPRECATE: Refactor This as it is not needed
    [36, 0, 0, 0, 0, 0],   // Pawn
    [9, 63, 0, 0, 0, 0],   // Knight
    [59, 65, 42, 0, 0, 0], // Bishop
    [46, 39, 24, -24, 0, 0], // Rook
    [97, 100, -42, 137, 268, 0], // Queen
];

pub trait ImbalanceEvalTrait {
    fn imbalance(&mut self, clr: Color);
    fn imb_piece_count(&mut self, num: usize, clr: Color) -> isize;
}

impl ImbalanceEvalTrait for Board {
    fn imbalance(&mut self, clr: Color) {
        let ours: [isize; 6] = [
            0,
            self.pawn_bb(clr).count() as isize,
            self.knight_bb(clr).count() as isize,
            self.bishop_bb(clr).count() as isize,
            self.rook_bb(clr).count() as isize,
            self.queen_bb(clr).count() as isize,
        ];
        let theirs: [isize; 6] = [
            0,
            self.pawn_bb(clr.opp()).count() as isize,
            self.knight_bb(clr.opp()).count() as isize,
            self.bishop_bb(clr.opp()).count() as isize,
            self.rook_bb(clr.opp()).count() as isize,
            self.queen_bb(clr.opp()).count() as isize,
        ];
        let mut bonus = 0;

        let has_our_bishop_pair = has_bishop_pair(self.bishop_bb(clr)) as isize;
        let has_their_bishop_pair = has_bishop_pair(self.bishop_bb(clr.opp())) as isize;

        for pt1 in 1..6 {
            if ours[pt1] == 0 {
                continue;
            }

            bonus += (QUADRATIC_OURS[pt1][0] * has_our_bishop_pair
                + QUADRATIC_OURS[pt1][1] * ours[1]
                + QUADRATIC_OURS[pt1][2] * ours[2]
                + QUADRATIC_OURS[pt1][3] * ours[3]
                + QUADRATIC_OURS[pt1][4] * ours[4]
                + QUADRATIC_OURS[pt1][5] * ours[5]
                + QUADRATIC_THEIRS[pt1][0] * has_their_bishop_pair
                + QUADRATIC_THEIRS[pt1][1] * theirs[1]
                + QUADRATIC_THEIRS[pt1][2] * theirs[2]
                + QUADRATIC_THEIRS[pt1][3] * theirs[3]
                + QUADRATIC_THEIRS[pt1][4] * theirs[4]
                + QUADRATIC_THEIRS[pt1][5] * theirs[5])
                * ours[pt1];
        }

        bonus += 1438 * has_our_bishop_pair;
        bonus /= 16;
        self.sum(clr, None, None, (bonus, bonus));
    }

    #[inline(always)]
    fn imb_piece_count(&mut self, num: usize, clr: Color) -> isize {
        match num {
            0 => 0, //self.king_bb(clr).count() as isize,
            1 => self.pawn_bb(clr).count() as isize,
            2 => self.knight_bb(clr).count() as isize,
            3 => self.bishop_bb(clr).count() as isize,
            4 => self.rook_bb(clr).count() as isize,
            5 => self.queen_bb(clr).count() as isize,
            _ => panic!("Sth is not right"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::fen::FenTrait;
    use crate::engine::board::structures::color::{BLACK, WHITE};
    use crate::engine::evaluation::init_eval::InitEvalTrait;
    use crate::engine::evaluation::test_evaluation::{eval_assert, SF_EVAL};

    use super::*;

    // NOTE: 3. IMBALANCE [TEST: WORKS]
    #[test]
    fn imbalance_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.imbalance(WHITE);
            board.imbalance(BLACK);

            eval_assert(board.calculate_score(), obj.imbalance, 0, false);
        }
    }
}
