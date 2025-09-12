use crate::engine::board::board::Board;
use crate::engine::board::color::{Color, ColorTrait};
use crate::engine::board::piece::*;
use crate::engine::board::square::{get_file, get_rank};
use crate::engine::evaluation::common_eval::CommonEvalTrait;
use crate::engine::evaluation::eval_defs::CLR_RANK;
use crate::engine::generated::pawn::{ISOLATED_PAWN_LOOKUP, PAWN_FORWARD_SPANS};
use crate::engine::misc::bitboard::{Bitboard, BitboardTrait};
use crate::engine::misc::const_utility::{FILE_BITBOARD, RANK_BITBOARD};

pub const UNBLOCKED_STORM: [[isize; 7]; 8] = [
    [85, 50, 45, 50, 97, -166, -289],
    [46, 20, -10, 37, 45, 122, -25],
    [-6, -14, -22, -2, 34, 168, 51],
    [-15, -29, -15, 11, 4, 101, -11],
    [-15, -29, -15, 11, 4, 101, -11],
    [-6, -14, -22, -2, 34, 168, 51],
    [46, 20, -10, 37, 45, 122, -25],
    [85, 50, 45, 50, 97, -166, -289],
];

pub const BLOCKED_STORM: [[isize; 7]; 2] = [[0, -1, -4, -7, -10, 76, 0], [0, 2, 6, 10, 15, 78, 0]];

pub const WEAKNESS: [[isize; 7]; 8] = [
    [-6, 25, 18, 39, 58, 93, 81],
    [-43, -63, -11, -29, -49, 35, 61],
    [-10, -45, 3, 32, -2, 23, 75],
    [-39, -166, -67, -48, -52, -29, -13],
    [-39, -166, -67, -48, -52, -29, -13],
    [-10, -45, 3, 32, -2, 23, 75],
    [-43, -63, -11, -29, -49, 35, 61],
    [-6, 25, 18, 39, 58, 93, 81],
];

pub const FLANK_ADDITIONAL_FILE: [usize; 8] = [2, 3, 0, 5, 2, 7, 4, 5];

pub const FLANK_MASK: [u64; 2] = [
    (RANK_BITBOARD[0] | RANK_BITBOARD[1] | RANK_BITBOARD[2] | RANK_BITBOARD[3] | RANK_BITBOARD[4]),
    (RANK_BITBOARD[7] | RANK_BITBOARD[6] | RANK_BITBOARD[5] | RANK_BITBOARD[4] | RANK_BITBOARD[3]),
];

pub trait KingEvalTrait {
    fn king_eval(&mut self, clr: Color);
    fn king_danger(&mut self, clr: Color) -> isize;
    fn flank_defense(&mut self, clr: Color) -> isize;
    fn flank_attack(&mut self, clr: Color) -> isize;
    fn king_blockers(&mut self, clr: Color);
    fn endgame_shelter(&mut self, clr: Color) -> isize;
    fn knight_defender(&mut self, clr: Color) -> u64;
    fn unsafe_checks(&mut self, clr: Color) -> u64;
    fn weak_squares(&mut self, clr: Color) -> u64;
    fn weak_bonus(&mut self, clr: Color) -> u64;
    fn king_attacks(&mut self, clr: Color) -> isize;
    fn king_attackers_weight(&mut self, clr: Color) -> isize;
    fn king_attackers_count(&mut self, clr: Color) -> isize;
    fn safe_check(&mut self, clr: Color, piece: Piece) -> u64;
    fn check(&mut self, clr: Color);
    // fn king_pawn_distance(&mut self, clr: Color);
    fn shelter(&mut self, clr: Color) -> (isize, isize, isize);
    // fn shelter_storm(&mut self, clr: Color);
    // fn shelter_strength(&mut self, clr: Color);
    fn storm_square(&mut self, sq: usize, clr: Color) -> (isize, isize);
    fn strength_square(&mut self, sq: usize, clr: Color) -> isize;
    fn pawnless_flank(&mut self, sq: usize, clr: Color) -> bool;
}

impl KingEvalTrait for Board {
    #[inline(always)]
    fn king_eval(&mut self, clr: Color) {
        let king_sq = self.king_sq(clr);

        let bonus = self.king_danger(clr);
        self.sum(clr, None, None, ((bonus * bonus) / 4096, bonus / 16));

        let bonus = self.eval.king_shelter[clr.idx()];
        self.sum(clr, None, None, (-bonus.0, 0)); // Shelter Strength
        self.sum(clr, None, None, (bonus.1, 0)); // Shelter Storm

        let bonus = if self.pawnless_flank(king_sq, clr) { 1 } else { 0 };
        self.sum(clr, None, None, (17 * bonus, 95 * bonus));

        let bonus = self.flank_attack(clr);
        self.sum(clr, None, None, (8 * bonus, 0));

        let bonus = self.eval.king_pawn_dx[clr.idx()] as isize;
        self.sum(clr, None, None, (0, -16 * bonus));

        // FIXME: This is not correct, the function is wrong
        let bonus = self.endgame_shelter(clr);
        self.sum(clr, None, None, (0, bonus));
    }

    #[inline(always)]
    fn king_danger(&mut self, clr: Color) -> isize {
        let count = self.king_attackers_count(clr);
        // println!("King Attackers Count: {:?}", count);

        let weight = self.king_attackers_weight(clr);
        // println!("King Attackers Weight: {:?}", weight);

        let king_att = self.king_attacks(clr);
        // println!("King Attacks: {:?}", king_att);

        let weak = self.weak_bonus(clr).count() as isize;
        // print_bitboard(self.eval.king_ring[clr.opp().idx()], None);
        // print_bitboard(self.weak_squares(clr), None);
        // print_bitboard(self.weak_bonus(clr), None);
        // println!("Weak Bonus: {:?}", weak);

        let unsafe_checks = self.unsafe_checks(clr).count() as isize;
        // println!("Unsafe Checks: {:?}", unsafe_checks);
        // print_bitboard(self.unsafe_checks(clr), None);

        let flank_att = self.flank_attack(clr);
        // println!("Flank Attack: {:?}", flank_att);

        let flank_def = self.flank_defense(clr);
        // println!("Flank defense: {:?}", flank_def);

        let no_queen = if self.queen_bb(clr).count() > 0 { 0 } else { 1 };
        // println!("No Queen: {:?}", no_queen);

        let knight_defender = if self.knight_defender(clr.opp()).count() > 0 { 1 } else { 0 };
        // println!("Knight Defender: {:?}", knight_defender);

        // let knight_safe = self.safe_check(clr, KNIGHT + clr);
        // println!("Knight Safe Check");
        // print_bitboard(knight_safe, None);

        // let rook_safe = self.safe_check(clr, ROOK + clr);
        // println!("Rook Safe Check");
        // print_bitboard(rook_safe, None);

        // let bishop_safe = self.safe_check(clr, BISHOP + clr);
        // println!("Bishop Safe Check");
        // print_bitboard(bishop_safe, None);

        // let queen_safe = self.safe_check(clr, QUEEN + clr);
        // println!("Queen Safe Check");
        // print_bitboard(queen_safe, None);

        let v = count * weight + 69 * king_att + 185 * weak - 100 * knight_defender
            + 148 * unsafe_checks
            - 4 * flank_def
            + (3 * flank_att * flank_att / 8)
            - 873 * no_queen
            - (6 * (self.eval.king_shelter[clr.idx()].0 - self.eval.king_shelter[clr.idx()].1) / 8) //self.shelter(clr).0 - self.shelter(clr).1
            + self.eval.mobility_eval[clr.idx()].0
            - self.eval.mobility_eval[clr.opp().idx()].0
            + 37
            + 772 * (self.safe_check(clr, QUEEN + clr).count() as f64).min(1.45) as isize
            + 1084 * (self.safe_check(clr, ROOK + clr).count() as f64).min(1.75) as isize
            + 645 * (self.safe_check(clr, BISHOP + clr).count() as f64).min(1.50) as isize
            + 792 * (self.safe_check(clr, KNIGHT + clr).count() as f64).min(1.62) as isize;
        // println!("V Score: {:?}", v);
        // println!("-------------------------------");

        if v > 100 {
            return v;
        };
        return 0;
    }

    #[inline(always)]
    fn flank_defense(&mut self, clr: Color) -> isize {
        let king_sq = self.king_sq(clr.opp());
        let flanks = ISOLATED_PAWN_LOOKUP[king_sq]
            | FILE_BITBOARD[get_file(king_sq)]
            | FILE_BITBOARD[FLANK_ADDITIONAL_FILE[get_file(king_sq)]];

        let att_1 = flanks & FLANK_MASK[clr.opp().idx()] & self.eval.attack_map[clr.opp().idx()];

        // println!("Attack 1");
        // print_bitboard(att_1, None);

        return att_1.count() as isize;
        // self.eval.sum(clr, None, None, (count as isize * 2, count as isize * 2));
    }

    #[inline(always)]
    fn flank_attack(&mut self, clr: Color) -> isize {
        let king_sq = self.king_sq(clr.opp());
        let flanks = ISOLATED_PAWN_LOOKUP[king_sq]
            | FILE_BITBOARD[get_file(king_sq)]
            | FILE_BITBOARD[FLANK_ADDITIONAL_FILE[get_file(king_sq)]];

        let att_1 = flanks & FLANK_MASK[clr.opp().idx()] & self.eval.attack_map[clr.idx()];

        let att_2 = flanks & FLANK_MASK[clr.opp().idx()] & self.eval.attacked_by_2[clr.idx()];

        return (att_1.count() + att_2.count()) as isize;
    }

    #[inline(always)]
    fn king_blockers(&mut self, clr: Color) {
        todo!()
    }

    #[inline(always)]
    fn endgame_shelter(&mut self, clr: Color) -> isize {
        self.eval.king_shelter[clr.idx()].2
    }

    #[inline(always)]
    fn knight_defender(&mut self, clr: Color) -> u64 {
        self.eval.attacked_by[(KNIGHT + clr).idx()] & self.eval.attacked_by[(KING + clr).idx()]
    }

    #[inline(always)]
    fn unsafe_checks(&mut self, clr: Color) -> u64 {
        let knight_unsafe =
            self.eval.checks[(KNIGHT + clr).idx()] & !self.safe_check(clr, KNIGHT + clr);

        let bishop_unsafe =
            self.eval.checks[(BISHOP + clr).idx()] & !self.safe_check(clr, BISHOP + clr);

        let rook_unsafe = self.eval.checks[(ROOK + clr).idx()] & !self.safe_check(clr, ROOK + clr);

        return knight_unsafe | bishop_unsafe | rook_unsafe;
    }

    #[inline(always)]
    fn safe_check(&mut self, clr: Color, piece: Piece) -> u64 {
        let checks = match piece.kind() {
            PAWN => 0,
            KNIGHT => self.eval.checks[piece.idx()],
            KING => 0,
            BISHOP => self.eval.checks[piece.idx()] & !self.eval.checks[(QUEEN + clr).idx()],
            ROOK => self.eval.checks[piece.idx()],
            QUEEN => {
                self.eval.checks[piece.idx()]
                    & !self.eval.checks[(ROOK + clr).idx()]
                    & !self.eval.attacked_by[(QUEEN + clr.opp()).idx()]
            }
            _ => panic!("There is other peace that was not expected here"),
        };

        let weak_squares = self.weak_squares(clr) & self.eval.attacked_by_2[clr.idx()];

        return !self.occ_bb(clr)
            & (!self.eval.attack_map[clr.opp().idx()] | weak_squares)
            & checks;
    }

    #[inline(always)]
    fn weak_squares(&mut self, clr: Color) -> u64 {
        let enemy_att_2 = self.eval.attacked_by_2[clr.opp().idx()];

        let not_att_2_times = self.eval.attack_map[clr.idx()] & !enemy_att_2;

        self.eval.attack_map[clr.idx()] & (not_att_2_times & !self.eval.attack_map[clr.opp().idx()])
            | (not_att_2_times
                & (self.eval.attacked_by[(KING + clr.opp()).idx()]
                    | self.eval.attacked_by[(QUEEN + clr.opp()).idx()]))
    }

    #[inline(always)]
    fn weak_bonus(&mut self, clr: Color) -> u64 {
        self.weak_squares(clr) & self.eval.king_ring[clr.opp().idx()]
    }

    #[inline(always)]
    fn king_attacks(&mut self, clr: Color) -> isize {
        self.eval.king_att[clr.idx()] as isize
    }

    #[inline(always)]
    fn king_attackers_weight(&mut self, clr: Color) -> isize {
        self.eval.king_att_weight[clr.idx()]
    }

    #[inline(always)]
    fn king_attackers_count(&mut self, clr: Color) -> isize {
        self.eval.king_att_count[clr.idx()] as isize
    }

    #[inline(always)]
    fn check(&mut self, clr: Color) {
        let king_sq = self.king_sq(clr.opp());
        self.eval.checks[(KNIGHT + clr).idx()] = self.eval.attacked_by[(KNIGHT + clr).idx()]
            & self.x_ray_mask(KNIGHT + clr.opp(), king_sq);

        self.eval.checks[(BISHOP + clr).idx()] = self.eval.attacked_by[(BISHOP + clr).idx()]
            & self.x_ray_mask(BISHOP + clr.opp(), king_sq);

        self.eval.checks[(ROOK + clr).idx()] =
            self.eval.attacked_by[(ROOK + clr).idx()] & self.x_ray_mask(ROOK + clr.opp(), king_sq);

        self.eval.checks[(QUEEN + clr).idx()] = self.eval.attacked_by[(QUEEN + clr).idx()]
            & self.x_ray_mask(QUEEN + clr.opp(), king_sq);
    }

    #[inline(always)]
    fn shelter(&mut self, clr: Color) -> (isize, isize, isize) {
        let king_sq = self.king_sq(clr.opp());
        let mut king_strenght = self.strength_square(king_sq, clr);
        let mut king_storm = self.storm_square(king_sq, clr);
        // println!("Storm SQ king{:?}", self.storm_square(king_sq, clr));

        if self.castling().short(clr.opp()) != 0 {
            let short_castle_sq = if clr.opp().is_white() { 6 } else { 62 };
            let short_castle_strength = self.strength_square(short_castle_sq, clr);
            let short_castle_storm = self.storm_square(short_castle_sq, clr);
            // println!("Storm SQ short{:?}", self.storm_square(short_castle_sq, clr));

            if (short_castle_storm.0 - short_castle_strength) < (king_storm.0 - king_strenght) {
                king_strenght = short_castle_strength;
                king_storm = short_castle_storm;
            }
        }

        if self.castling().long(clr.opp()) != 0 {
            let long_castle_sq = if clr.opp().is_white() { 2 } else { 58 };
            let long_castle_strength = self.strength_square(long_castle_sq, clr);
            let long_castle_storm = self.storm_square(long_castle_sq, clr);
            // println!("Storm SQ long{:?}", self.storm_square(long_castle_sq, clr));

            if (long_castle_storm.0 - long_castle_strength) < (king_storm.0 - king_strenght) {
                king_strenght = long_castle_strength;
                king_storm = long_castle_storm;
            }
        }

        return (king_strenght, king_storm.0, king_storm.1);
    }

    #[inline(always)]
    fn storm_square(&mut self, sq: usize, clr: Color) -> (isize, isize) {
        let mut v = 0;
        let mut ev = 5;

        let file = get_file(sq);
        let sq = sq + ((file == 0) as usize) - ((file == 7) as usize);

        for square in (sq - 1)..(sq + 2) {
            // FIXME: ALL Squares forward ????????????
            let us_bb: u64 = (PAWN_FORWARD_SPANS[clr.opp().idx()][square] | Bitboard::init(square))
                & (self.pawn_bb(clr.opp()) & !self.eval.attacked_by[(PAWN + clr).idx()]);

            let them_bb: u64 = (PAWN_FORWARD_SPANS[clr.opp().idx()][square]
                | Bitboard::init(square))
                & self.pawn_bb(clr);

            let mut us = 0;
            let mut them = 0;

            if us_bb != 0 {
                us = get_rank(if clr.is_white() { us_bb.get_msb() } else { us_bb.get_lsb() });
            }

            if them_bb != 0 {
                them = get_rank(if clr.is_white() { them_bb.get_msb() } else { them_bb.get_lsb() });
            }

            // println!("Sq: {:?}, Square: {:?}, Us: {:?}, Them: {:?}", sq, square, us, them);
            if us > 0 && (them as isize) == (us as isize) - clr.sign() {
                // v += BLOCKED_STORM[0][CLR_RANK[clr.idx()][them]];
                // ev += BLOCKED_STORM[1][CLR_RANK[clr.idx()][them]];

                if them == 0 {
                    v += BLOCKED_STORM[0][0];
                    ev += BLOCKED_STORM[1][0];
                } else {
                    v += BLOCKED_STORM[0][CLR_RANK[clr.idx()][them]];
                    ev += BLOCKED_STORM[1][CLR_RANK[clr.idx()][them]];
                }
            } else {
                // println!("First: {:?}", get_rank(square));
                // println!("Them: {:?}", them);
                // println!("CLR_RANK: {:?}", CLR_RANK[clr.idx()]);
                // println!("GET Rank{:?}", get_rank(them));
                if them == 0 {
                    v += UNBLOCKED_STORM[get_file(square)][0];
                } else {
                    v += UNBLOCKED_STORM[get_file(square)][CLR_RANK[clr.idx()][them]];
                }
            }
        }
        return (v, ev);
    }

    #[inline(always)]
    fn strength_square(&mut self, sq: usize, clr: Color) -> isize {
        let mut score = 5;

        let file = get_file(sq);
        let sq = sq + ((file == 0) as usize) - ((file == 7) as usize);

        for square in (sq - 1)..(sq + 2) {
            let mut us = 0;
            let us_bb: u64 = (PAWN_FORWARD_SPANS[clr.opp().idx()][square] | Bitboard::init(square))
                & (self.pawn_bb(clr.opp()) & !self.eval.attacked_by[(PAWN + clr).idx()]);

            if us_bb != 0 {
                us = get_rank(if clr.is_white() { us_bb.get_msb() } else { us_bb.get_lsb() });
            }

            if us == 0 {
                score += WEAKNESS[get_file(square)][0];
            } else {
                score += WEAKNESS[get_file(square)][CLR_RANK[clr.idx()][us]];
            }
        }

        return score;
    }

    #[inline(always)]
    fn pawnless_flank(&mut self, sq: usize, clr: Color) -> bool {
        let all_pawns = self.pawn_bb(clr) | self.pawn_bb(clr.opp());
        let file = get_file(sq);
        let flanks = ISOLATED_PAWN_LOOKUP[sq]
            | FILE_BITBOARD[file]
            | FILE_BITBOARD[FLANK_ADDITIONAL_FILE[file]];

        return (flanks & all_pawns) == 0;
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
    fn king_test() {
        for obj in &SF_EVAL {
            let mut board = Board::read_fen(obj.fen);
            board.init();
            board.king_eval(WHITE);
            board.king_eval(BLACK);

            eval_assert(board.calculate_score(), obj.king, 34, false); // FIXME: The Difference is too high
        }
    }
}
