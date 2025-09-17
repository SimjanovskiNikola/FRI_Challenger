use crate::engine::attacks::bishop::has_bishop_pair;
use crate::engine::board::board::Board;
use crate::engine::board::color::{Color, ColorTrait};
use crate::engine::evaluation::common_eval::CommonEvalTrait;

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
}

impl ImbalanceEvalTrait for Board {
    fn imbalance(&mut self, clr: Color) {
        let ours: [isize; 6] = [
            has_bishop_pair(self.bishop_bb(clr)) as isize,
            self.pawn_count(clr) as isize,
            self.knight_count(clr) as isize,
            self.bishop_count(clr) as isize,
            self.rook_count(clr) as isize,
            self.queen_count(clr) as isize,
        ];

        let theirs: [isize; 6] = [
            has_bishop_pair(self.bishop_bb(clr.opp())) as isize,
            self.pawn_count(clr.opp()) as isize,
            self.knight_count(clr.opp()) as isize,
            self.bishop_count(clr.opp()) as isize,
            self.rook_count(clr.opp()) as isize,
            self.queen_count(clr.opp()) as isize,
        ];
        let mut bonus = 0;

        for pt1 in 1..6 {
            if ours[pt1] == 0 {
                continue;
            }

            bonus += ours[pt1]
                * (QUADRATIC_OURS[pt1].iter().zip(ours).map(|(x, y)| x * y).sum::<isize>()
                    + QUADRATIC_THEIRS[pt1].iter().zip(theirs).map(|(x, y)| x * y).sum::<isize>());
            // bonus += (QUADRATIC_OURS[pt1][0] * ours[0]
            //     + QUADRATIC_OURS[pt1][1] * ours[1]
            //     + QUADRATIC_OURS[pt1][2] * ours[2]
            //     + QUADRATIC_OURS[pt1][3] * ours[3]
            //     + QUADRATIC_OURS[pt1][4] * ours[4]
            //     + QUADRATIC_OURS[pt1][5] * ours[5]
            //     + QUADRATIC_THEIRS[pt1][0] * theirs[0]
            //     + QUADRATIC_THEIRS[pt1][1] * theirs[1]
            //     + QUADRATIC_THEIRS[pt1][2] * theirs[2]
            //     + QUADRATIC_THEIRS[pt1][3] * theirs[3]
            //     + QUADRATIC_THEIRS[pt1][4] * theirs[4]
            //     + QUADRATIC_THEIRS[pt1][5] * theirs[5])
            //     * ours[pt1];
        }

        bonus += 1438 * ours[0];
        bonus /= 16;
        self.sum(clr, None, None, (bonus, bonus));
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::color::{BLACK, WHITE};
    use crate::engine::board::fen::FenTrait;
    use crate::engine::evaluation::init_eval::InitEvalTrait;
    use crate::engine::evaluation::test_evaluation::{SF_EVAL, eval_assert};

    use super::*;

    // NOTE: 3. IMBALANCE [TEST: WORKS]
    #[test]
    fn imbalance_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.imbalance(WHITE);
            board.imbalance(BLACK);

            eval_assert(board.calculate_score(), obj.imbalance, 1, false);
        }
    }
}
