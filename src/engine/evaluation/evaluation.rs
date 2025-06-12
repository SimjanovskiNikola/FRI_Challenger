use std::array;

use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color;
use crate::engine::board::structures::color::*;
use crate::engine::board::structures::piece::*;
use crate::engine::board::structures::square::*;
use crate::engine::evaluation::eval_defs::*;
use crate::engine::misc::bit_pos_utility::get_bit_rank;
use crate::engine::misc::bitboard::Bitboard;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::misc::bitboard::Iterator;
use crate::engine::misc::const_utility::FILE_BITBOARD;
use crate::engine::misc::const_utility::RANK_BITBOARD;
use crate::engine::misc::print_utility::print_bitboard;
use crate::engine::misc::print_utility::print_board;
use crate::engine::misc::print_utility::print_eval;
use crate::engine::move_generator::bishop::get_bishop_mask;
use crate::engine::move_generator::bishop::get_bishop_mv;
use crate::engine::move_generator::bishop::has_bishop_pair;
use crate::engine::move_generator::generated::king::KING_RING;
use crate::engine::move_generator::generated::pawn::FORWARD_FILE_LR;
use crate::engine::move_generator::generated::pawn::ISOLATED_PAWN_LOOKUP;
use crate::engine::move_generator::generated::pawn::PAWN_3_BEHIND_MASKS;
use crate::engine::move_generator::generated::pawn::PAWN_ATTACK_LOOKUP;
use crate::engine::move_generator::generated::pawn::PAWN_FORWARD_SPANS;
use crate::engine::move_generator::generated::pawn::PAWN_MOVE_LOOKUP;
use crate::engine::move_generator::king::get_king_mask;
use crate::engine::move_generator::king::get_king_mv;
use crate::engine::move_generator::king::has_near_open_files;
use crate::engine::move_generator::knight::get_knight_mask;
use crate::engine::move_generator::knight::get_knight_mv;
use crate::engine::move_generator::pawn::get_all_pawn_forward_mask;
use crate::engine::move_generator::pawn::get_all_pawn_left_att_mask;
use crate::engine::move_generator::pawn::get_all_pawn_right_att_mask;
use crate::engine::move_generator::pawn::get_pawn_2_att;
use crate::engine::move_generator::pawn::get_pawn_att_mask;
use crate::engine::move_generator::pawn::is_blocked_pawn;
use crate::engine::move_generator::pawn::is_isolated_pawn;
use crate::engine::move_generator::pawn::is_passed_pawn;
use crate::engine::move_generator::queen::get_queen_mask;
use crate::engine::move_generator::rook::get_rook_mask;
use crate::engine::move_generator::rook::get_rook_mv;
use crate::engine::move_generator::rook::is_rook_on_open_file;
use crate::engine::move_generator::rook::is_rook_on_semi_open_file;

// The Numbers (Tapered Eval) for the evaluation are taken from -> STOCKFISH SF_9

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Evaluation {
    pub pawn_behind_masks: [Bitboard; 2],
    pub psqt: [isize; 2],
    pub test_arr: [String; 64],

    pub outpost: [Bitboard; 2],
    pub king_ring: [Bitboard; 2],
    pub attacked_by: [Bitboard; 14],
    pub defended_by: [Bitboard; 14],
    pub attacked_by_2: [Bitboard; 2],
    pub king_att_weight: [isize; 2],
    pub king_att_count: [usize; 2],
    pub defend_map: [Bitboard; 2],
    pub attack_map: [Bitboard; 2],
    pub phase: (isize, isize),
    pub score: isize,
}

impl Evaluation {
    pub fn init() -> Self {
        Self {
            pawn_behind_masks: [0; 2],
            psqt: [0; 2],
            test_arr: array::from_fn(|_| " ".to_string()),
            outpost: [0; 2],
            king_ring: [0; 2],
            attacked_by: [0; 14],
            defended_by: [0; 14],
            attacked_by_2: [0; 2],
            king_att_weight: [0; 2],
            king_att_count: [0; 2],
            defend_map: [0; 2],
            attack_map: [0; 2],
            phase: (0, 0),
            score: 0,
        }
    }

    pub fn reset(&mut self) {
        self.outpost.fill(0);
        self.king_ring.fill(0);
        self.attacked_by.fill(0);
        self.defended_by.fill(0);
        self.attacked_by_2.fill(0);
        self.king_att_weight.fill(0);
        self.king_att_count.fill(0);
        self.defend_map.fill(0);
        self.attack_map.fill(0);
        self.phase = (0, 0);
        self.score = 0;
        self.test_arr = array::from_fn(|_| " ".to_string());
    }
}

pub trait EvaluationTrait {
    // NOTE: Functions That Initialize the Evaluation Structure
    fn init(&mut self);
    fn pawn_init(&mut self);
    fn piece_init(&mut self);
    fn determine_phase(&self) -> usize;
    fn tapered(&self, value: (isize, isize)) -> isize;
    fn insufficient_material(&self) -> bool;

    // NOTE: Main Evaluation Function (It has 11 sub evaluations)
    fn evaluation(&mut self) -> isize;

    // NOTE: 1. MATERIAL Evaluation
    fn material_eval(&self, clr: Color) -> isize;
    fn non_pawn_material_eval(&self, clr: Color) -> isize;
    fn piece_material(&self, piece: Piece) -> isize;

    // NOTE: 2. PSQT Evaluation
    fn psqt_eval(&self, clr: Color) -> isize;
    fn piece_psqt(&self, piece: Piece, sq: usize) -> isize;

    // NOTE: 3. IMBALANCE Evaluation
    fn imbalance(&self, clr: Color) -> isize;
    fn imb_piece_count(&self, num: usize, clr: Color) -> isize;

    // NOTE: 4. PAWNS Evaluation FIXME:
    fn pawns_eval(&self, clr: Color) -> isize;
    fn single_pawn_eval(&self, sq: usize, clr: Color) -> isize;
    fn isolated_pawn(&self, sq: usize, clr: Color) -> bool;
    fn opposed_pawn(&self, sq: usize, clr: Color) -> bool;
    fn phalanx_pawn(&self, sq: usize, clr: Color) -> bool;
    fn supported_pawn(&self, sq: usize, clr: Color) -> bool;
    fn backward_pawn(&self, sq: usize, clr: Color) -> bool;
    fn doubled_pawn(&self, sq: usize, clr: Color) -> bool;
    fn connected_pawn(&self, sq: usize, clr: Color) -> bool;
    fn connected_bonus(&self, sq: usize, clr: Color) -> isize;
    fn weak_unopposed_pawn(&self, sq: usize, clr: Color) -> bool;
    fn weak_lever(&self, sq: usize, clr: Color) -> bool;
    fn blocked_pawn(&self, sq: usize, clr: Color, bb: u64) -> bool;
    fn blocked_pawn_5th_6th_rank(&self, sq: usize, clr: Color) -> isize;
    fn doubled_isolated_pawn(&self, sq: usize, clr: Color) -> bool;

    // NOTE: 5. PEACES Evaluation TODO: FIXME:
    // NOTE: 6. MOBILITY Evaluation
    fn mobility_eval(&self, clr: Color) -> isize;
    fn mobility_bonus(&self, piece: Piece, sq: usize) -> isize;
    fn mobility_area(&self, clr: Color) -> u64;
    fn mobility_piece(&self, sq: usize, piece: Piece, clr: Color) -> u64;

    // NOTE: 7. THREATS Evaluation TODO: FIXME:
    // NOTE: 8. PASSED PAWNS Evaluation
    fn passed_pawn(&self, clr: Color) -> isize;
    fn passed_leverable(&self, sq: usize, clr: Color) -> bool;
    fn passed_file(&self, sq: usize) -> isize;
    fn passed_blocked(&self, sq: usize, clr: Color) -> isize;
    fn king_proximity(&self, sq: usize, clr: Color) -> isize;
    fn candidate_passed(&self, sq: usize, clr: Color) -> bool;

    // NOTE: 9. SPACE Evaluation FIXME:
    fn space(&self, color: Color) -> isize;
    fn space_area(&self, color: Color) -> usize;

    // NOTE: 10. KING Evaluation TODO: FIXME:
    // NOTE: 11. TEMPO Evaluation
    fn tempo(color: Color) -> isize;

    // fn piece_sq_eval(piece: &Piece, phase: usize, sq: usize) -> isize; //FIXME: Fix Phase to enum

    fn get_mask(&self, piece: Piece, sq: usize) -> u64;

    fn opp_color_bishops(&self, clr: Color) -> bool;
    fn king_dist(&self, clr: Color, sq: usize) -> usize;
    fn king_ring(&self, clr: Color) -> u64;
}

impl EvaluationTrait for Board {
    // ************************************************
    //                     INIT                       *
    // ************************************************

    fn init(&mut self) {
        // TODO: FIX the phase so that everything is in one function
        let phase = self.determine_phase() as isize;
        self.eval.phase = (phase.min(24), 24 - phase.min(24));

        // TODO: Create Pawn Init so that it doesn't have duplicate code
        let clr = WHITE;
        let (own, enemy) = self.both_occ_bb(clr);
        let mut bb = self.pawn_bb(clr);
        while let Some(sq) = bb.next() {
            self.eval.pawn_behind_masks[clr.idx()] |=
                PAWN_3_BEHIND_MASKS[clr.idx()][sq] & CLR_CENTER[clr.idx()];

            self.eval.attacked_by[(PAWN + clr).idx()] |= get_pawn_att_mask(sq, own, enemy, clr);
        }

        // TODO: Create Pawn Init so that it doesn't have duplicate code
        let clr = BLACK;
        let (own, enemy) = self.both_occ_bb(clr);
        let mut bb = self.pawn_bb(clr);
        while let Some(sq) = bb.next() {
            self.eval.pawn_behind_masks[clr.idx()] |=
                PAWN_3_BEHIND_MASKS[clr.idx()][sq] & CLR_CENTER[clr.idx()];

            self.eval.attacked_by[(PAWN + clr).idx()] |= get_pawn_att_mask(sq, own, enemy, clr)
        }

        // TODO: Create PIECE Init so that it doesn't have duplicate code
        self.eval.test_arr = array::from_fn(|_| " ".to_string());

        let clr = WHITE;
        for piece in &PIECES {
            let mut bb = self.bb(piece + clr);
            while let Some(sq) = bb.next() {
                self.eval.psqt[clr.idx()] += self.piece_psqt(*piece + clr, sq);
                self.eval.test_arr[sq] = self.piece_psqt(*piece + clr, sq).to_string();

                // self.eval.attack_map[clr.opp()] =
                // self.eval.defend_map[clr.opp()] =
            }
        }
        // print_eval(&self.eval.test_arr);

        // TODO: Create PIECE Init so that it doesn't have duplicate code
        self.eval.test_arr = array::from_fn(|_| " ".to_string());
        let clr = BLACK;
        for piece in &PIECES {
            let mut bb = self.bb(piece + clr);
            while let Some(sq) = bb.next() {
                self.eval.psqt[clr.idx()] += self.piece_psqt(*piece + clr, sq);
                self.eval.test_arr[sq] = self.piece_psqt(*piece + clr, sq).to_string();
            }
        }

        // print_eval(&self.eval.test_arr);
    }

    #[inline(always)]
    fn insufficient_material(&self) -> bool {
        // The Color is not relevant here, and that is why i use self.color()
        let (own, enemy) = self.both_occ_bb(self.color());
        if (own | enemy).count_ones() < 4 {
            let kings = self.bb(WHITE_KING) | self.bb(BLACK_KING);
            let knights = self.bb(WHITE_KNIGHT) | self.bb(BLACK_KNIGHT);
            let bishops = self.bb(WHITE_BISHOP) | self.bb(BLACK_BISHOP);
            if (kings | knights | bishops) == (own | enemy) {
                return true;
            }
        }
        return false;
    }

    // ************************************************
    //                MAIN EVALUATION                 *
    // ************************************************

    fn evaluation(&mut self) -> isize {
        self.init();
        let mut score = 0;

        // 1. Piece Value NOTE: DONE
        score += self.material_eval(WHITE) - self.material_eval(BLACK);

        // 2. PSQT NOTE: DONE
        score += self.psqt_eval(WHITE) - self.psqt_eval(BLACK);

        // 3. Imbalance NOTE: DONE
        score += (self.imbalance(WHITE) - self.imbalance(BLACK)) / 16;

        // 4. Pawns
        score += self.pawns_eval(WHITE) - self.pawns_eval(BLACK);
        // println!("Pawns White: {:?}", self.pawns_eval(WHITE));
        // println!("Pawns Black: {:?}", self.pawns_eval(BLACK));

        // 5. Pieces
        // score += self.pieces_eval(WHITE) - self.pieces_eval(BLACK);

        // 6. Mobility
        score += self.mobility_eval(WHITE) - self.mobility_eval(BLACK);
        // println!("Mobility White: {:?}", self.mobility_eval(WHITE));
        // println!("Mobility Black: {:?}", self.mobility_eval(BLACK));

        // 7. Threats
        // score += threats_mg(pos) - threats_mg(BLACK);

        // 8. Passed Pawns
        score += self.passed_pawn(WHITE) - self.passed_pawn(BLACK);
        // println!("Passed White: {:?}", self.passed_pawn(WHITE));
        // println!("Passed Black: {:?}", self.passed_pawn(BLACK));

        // 9. Space
        score += self.tapered((self.space(WHITE) - self.space(BLACK), 0));
        // println!("Space: {:?}", self.space(WHITE));
        // println!("Space: {:?}", self.space(BLACK));

        // 10. King
        // score += king_mg(pos) - king_mg(BLACK);

        // 11. Tempo NOTE: DONE
        score += Self::tempo(self.color());

        return score;
    }

    // ************************************************
    //              MATERIAL EVALUATION               *
    // ************************************************

    fn material_eval(&self, color: Color) -> isize {
        let mut score = 0;
        for piece in &PIECES {
            let count = self.bb(piece + color).count() as isize;
            score += self.piece_material(piece + color) * count;
        }
        score
    }

    fn non_pawn_material_eval(&self, color: Color) -> isize {
        let mut score = 0;
        for piece in &PIECES_WITHOUT_PAWN {
            let count = self.bb(piece + color).count() as isize;
            score += self.piece_material(piece + color) * count;
        }
        score
    }

    fn piece_material(&self, piece: Piece) -> isize {
        self.tapered(PIECE_MATERIAL[piece.arr_idx()])
    }

    // ************************************************
    //                PSQT EVALUATION                 *
    // ************************************************

    fn psqt_eval(&self, clr: Color) -> isize {
        // TODO: Create Easy functions for easy acccess of the things inside self.eval
        self.eval.psqt[clr.idx()]
    }

    fn piece_psqt(&self, piece: Piece, sq: usize) -> isize {
        let fixed_sq = CLR_SQ[piece.color().idx()][sq];
        self.tapered(PSQT[piece.arr_idx()][fixed_sq])
    }

    // ************************************************
    //                IMBALANCE EVALUATION                 *
    // ************************************************

    fn imbalance(&self, clr: Color) -> isize {
        let mut bonus = 0;
        for pt1 in 0..6 {
            let cnt = self.imb_piece_count(pt1, clr);
            if cnt == 0 {
                continue;
            }

            let mut v = 0;
            for pt2 in 0..pt1 + 1 {
                v += QUADRATIC_OURS[pt1][pt2] * self.imb_piece_count(pt2, clr);
                v += QUADRATIC_THEIRS[pt1][pt2] * self.imb_piece_count(pt2, clr.opp());
            }

            if has_bishop_pair(self.bishop_bb(clr)) {
                v += QUADRATIC_OURS[pt1][0];
            }

            if has_bishop_pair(self.bishop_bb(clr.opp())) {
                v += QUADRATIC_THEIRS[pt1][0];
            }

            bonus += cnt * v;
        }

        if has_bishop_pair(self.bishop_bb(clr)) {
            bonus += 1438;
        }

        bonus
    }

    fn imb_piece_count(&self, num: usize, clr: Color) -> isize {
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

    // ************************************************
    //                PAWNS EVALUATION                *
    // ************************************************

    // 4. Pawns Eval
    fn pawns_eval(&self, clr: Color) -> isize {
        let mut score = 0;
        let mut bb = self.pawn_bb(clr);
        while let Some(sq) = bb.next() {
            score += self.single_pawn_eval(sq, clr);
        }

        score
    }

    fn single_pawn_eval(&self, sq: usize, clr: Color) -> isize {
        let mut score = 0;
        if self.doubled_isolated_pawn(sq, clr) {
            score += self.tapered((-11, -56));
        } else if self.isolated_pawn(sq, clr) {
            score += self.tapered((-5, -15));
        } else if self.backward_pawn(sq, clr) {
            score += self.tapered((-9, -24));
        }

        if self.doubled_pawn(sq, clr) {
            score += self.tapered((-11, -56));
        }

        if self.connected_pawn(sq, clr) {
            let bonus = self.connected_bonus(sq, clr);
            // FIXME: Check if it is ok to be this a minus sth
            score += self.tapered((
                bonus,
                bonus * (CLR_RANK[clr.idx()][get_rank(sq)] as isize - 3) as isize / 4,
            ));
        }

        if self.weak_unopposed_pawn(sq, clr) {
            score += self.tapered((-13, -27));
        }

        if self.weak_lever(sq, clr) {
            score += self.tapered((0, -56));
        }

        if self.blocked_pawn_5th_6th_rank(sq, clr) == 1 {
            score += self.tapered((-11, -4));
        } else if self.blocked_pawn_5th_6th_rank(sq, clr) == 2 {
            score += self.tapered((-3, 4));
        }

        score
    }

    fn isolated_pawn(&self, sq: usize, clr: Color) -> bool {
        ISOLATED_PAWN_LOOKUP[sq] & self.pawn_bb(clr) != 0
    }

    fn opposed_pawn(&self, sq: usize, clr: Color) -> bool {
        PAWN_FORWARD_SPANS[clr.idx()][sq] & self.pawn_bb(clr.opp()) != 0
    }

    fn phalanx_pawn(&self, sq: usize, clr: Color) -> bool {
        PAWN_ATTACK_LOOKUP[clr.opp().idx()][(sq as isize + 8 * clr.sign()) as usize]
            & self.pawn_bb(clr)
            != 0
    }

    fn supported_pawn(&self, sq: usize, clr: Color) -> bool {
        PAWN_ATTACK_LOOKUP[clr.opp().idx()][sq] & self.pawn_bb(clr) != 0
    }

    fn backward_pawn(&self, sq: usize, clr: Color) -> bool {
        (FORWARD_FILE_LR[clr.opp().idx()][sq] & self.pawn_bb(clr) == 0)
            && (self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp()))
                || self.eval.attacked_by[(PAWN + clr.opp()).idx()].is_set(sq))
    }

    fn doubled_pawn(&self, sq: usize, clr: Color) -> bool {
        PAWN_FORWARD_SPANS[clr.opp().idx()][sq] & self.pawn_bb(clr) != 0
    }

    fn connected_pawn(&self, sq: usize, clr: Color) -> bool {
        self.supported_pawn(sq, clr) || self.phalanx_pawn(sq, clr)
    }

    fn connected_bonus(&self, sq: usize, clr: Color) -> isize {
        if (!self.connected_pawn(sq, clr)) {
            return 0;
        }
        let seed = [0, 7, 8, 12, 29, 48, 86];
        let op = self.opposed_pawn(sq, clr);
        let ph = self.phalanx_pawn(sq, clr);
        let su = self.supported_pawn(sq, clr);
        let bl = self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp()));

        let r = CLR_RANK[clr.idx()][get_rank(sq)];
        if r < 2 || r > 7 {
            return 0;
        }

        return seed[r - 1] * (2 + ph as isize - op as isize) + 21 * su as isize;
    }

    fn weak_unopposed_pawn(&self, sq: usize, clr: Color) -> bool {
        !self.opposed_pawn(sq, clr) && (self.isolated_pawn(sq, clr) || self.backward_pawn(sq, clr))
    }

    fn weak_lever(&self, sq: usize, clr: Color) -> bool {
        !self.supported_pawn(sq, clr)
            && (get_pawn_att_mask(sq, 0, 0, clr) & self.pawn_bb(clr.opp())).count() == 2
    }

    fn blocked_pawn(&self, sq: usize, clr: Color, bb: u64) -> bool {
        get_all_pawn_forward_mask(Bitboard::init(sq), clr) & bb != 0
    }

    // Blocked only on the 5th and 6 rank
    fn blocked_pawn_5th_6th_rank(&self, sq: usize, clr: Color) -> isize {
        if BLOCKED_RANKS[clr.idx()].is_set(sq)
            && self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp()))
        {
            return get_rank(sq).abs_diff(4) as isize;
        }
        return 0;
    }

    fn doubled_isolated_pawn(&self, sq: usize, clr: Color) -> bool {
        self.doubled_pawn(sq, clr)
            && self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp()))
            && self.isolated_pawn(sq, clr)
            && self.isolated_pawn((sq as isize + 8 * clr.sign()) as usize, clr)
    }

    // ************************************************
    //             6. MOBILITY EVALUATION             *
    // ************************************************

    fn mobility_eval(&self, clr: Color) -> isize {
        let mut score = 0;
        let area = self.mobility_area(clr);
        for pce in [KNIGHT, BISHOP, ROOK, QUEEN] {
            let piece = pce + clr;
            let mut bb = self.bb(piece);
            while let Some(sq) = bb.next() {
                let safe_squares = (self.mobility_piece(sq, piece, clr) & area).count();
                score += self.mobility_bonus(piece, safe_squares);
            }
        }
        score
    }

    fn mobility_bonus(&self, piece: Piece, safe_sqaures: usize) -> isize {
        match piece.kind() {
            KNIGHT => self.tapered(KNIGHT_MOBILITY[safe_sqaures]),
            BISHOP => self.tapered(BISHOP_MOBILITY[safe_sqaures]),
            ROOK => self.tapered(ROOK_MOBILITY[safe_sqaures]),
            QUEEN => self.tapered(QUEEN_MOBILITY[safe_sqaures]),
            _ => panic!("There is other peace that was not expected here"),
        }
    }

    fn mobility_piece(&self, sq: usize, piece: Piece, clr: Color) -> u64 {
        let (own, enemy) = self.both_occ_bb(clr);
        match piece.kind() {
            KNIGHT => get_knight_mv(sq, own, enemy, clr),
            BISHOP => get_bishop_mask(sq, own, enemy, clr),
            ROOK => get_rook_mask(sq, own, enemy, clr),
            QUEEN => get_queen_mask(sq, own, enemy, clr),
            _ => panic!("There is other peace that was not expected here"),
        }
    }

    fn mobility_area(&self, clr: Color) -> u64 {
        (u64::MAX)
            & !self.king_bb(clr)
            & !self.queen_bb(clr)
            & !self.pawn_bb(clr)
            & !get_all_pawn_left_att_mask(self.pawn_bb(clr.opp()), clr.opp())
            & !get_all_pawn_right_att_mask(self.pawn_bb(clr.opp()), clr.opp())
    }

    // 8 Passed Pawns

    // fn passed_pawn(&self, clr: Color) -> isize {
    //     let mut score = 0;
    //     let bb = self.pawn_bb(clr);
    //     for sq in bb.next() {
    //         if !passed_leverable(pos, square) {
    //             return 0;
    //         }

    //         score += king_proximity(pos, square); // FIXME: Only In endgame
    //         score += self.tapered(PASSED_PAWN_REW[clr.idx()][get_rank(sq)]);
    //         score += self.passed_block(pos, square);
    //         score -= self.tapered((11, 8)) * PSQT_FILE[get_file(sq)] as isize;
    //     }
    //     score
    // }

    // fn passed_block(&self, sq: usize, clr: Color) {

    // }

    // 9. Space
    fn space(&self, clr: Color) -> isize {
        if self.non_pawn_material(clr) + self.non_pawn_material(clr.opp()) < 12222 {
            return 0;
        }

        let blocked = (get_all_pawn_forward_mask(self.pawn_bb(clr), clr)
            & !self.eval.attacked_by[(PAWN + clr.opp()).idx()]
            & !self.pawn_bb(clr.opp()))
        .count();
        let weight = (self.bb(clr).count() - 3 + blocked.min(9)) as isize;

        println!("{:?}",);
        return self.space_area(clr) as isize * weight * weight / 16;
    }

    fn space_area(&self, clr: Color) -> usize {
        let mut cnt = 0;
        let own_pawns_bb = self.pawn_bb(clr);
        let pawn_behind_bb = self.eval.pawn_behind_masks[clr.idx()];
        let opp_att_bb = self.eval.attack_map[clr.opp().idx()];
        let opp_pawn_att_bb = self.eval.attacked_by[(PAWN + clr.opp()).idx()];

        cnt += (CLR_CENTER[clr.idx()] & !opp_pawn_att_bb & !own_pawns_bb).count();
        // print_bitboard(CLR_CENTER[clr.idx()], None);
        // print_bitboard(opp_pawn_att_bb, None);
        // print_bitboard(own_pawns_bb, None);
        // print_bitboard((CLR_CENTER[clr.idx()] & !opp_pawn_att_bb & !own_pawns_bb), None);

        cnt += (pawn_behind_bb & CLR_CENTER[clr.idx()] & !opp_att_bb & !own_pawns_bb).count();
        print_bitboard(CLR_CENTER[clr.idx()], None);
        print_bitboard(pawn_behind_bb, None);
        print_bitboard(opp_att_bb, None);
        print_bitboard(own_pawns_bb, None);
        print_bitboard(pawn_behind_bb & CLR_CENTER[clr.idx()] & !opp_att_bb & !own_pawns_bb, None);
        cnt
    }

    fn king_dist(&self, clr: Color, sq: usize) -> usize {
        let (sq_rank, sq_file) = (get_rank(sq), get_file(sq));
        let (king_rank, king_file) = (get_rank(self.king_sq(clr)), get_file(self.king_sq(clr)));
        return (king_rank.abs_diff(sq_rank)).max(king_file.abs_diff(sq_file));
    }

    fn king_ring(&self, clr: Color) -> u64 {
        return KING_RING[self.king_sq(clr)] & !get_pawn_2_att(self.pawn_bb(clr), clr);
    }

    fn opp_color_bishops(&self, clr: Color) -> bool {
        let clr_bishop = self.bishop_bb(clr).count();
        let opp_clr_bishop = self.bishop_bb(clr.opp()).count();

        return clr_bishop == 1
            && opp_clr_bishop == 1
            && has_bishop_pair(self.bishop_bb(clr) | self.bishop_bb(clr.opp()));
    }

    #[inline(always)]
    fn get_mask(&self, piece: Piece, sq: usize) -> u64 {
        let (own, enemy) = self.both_occ_bb(piece.color());
        match piece.kind() {
            PAWN => get_pawn_att_mask(sq, own, enemy, piece.color()),
            KNIGHT => get_knight_mask(sq, own, enemy, piece.color()),
            BISHOP => get_bishop_mask(sq, own, enemy, piece.color()),
            ROOK => get_rook_mask(sq, own, enemy, piece.color()),
            QUEEN => get_queen_mask(sq, own, enemy, piece.color()),
            KING => get_king_mask(sq, own, enemy, piece.color()),
            _ => panic!("Invalid Peace Type"),
        }
    }

    // fn create_att_by_2(&mut self, piece: Piece, sq: usize) {}

    // fn is_defended(&mut self, piece: Piece, sq: usize) -> bool {
    //     self.eval.defend_map[piece.color().idx()].is_set(sq)
    // }

    #[inline(always)]
    fn tapered(&self, value: (isize, isize)) -> isize {
        (self.eval.phase.0 * value.0 + self.eval.phase.1 * value.1) / 24
    }

    fn determine_phase(&self) -> usize {
        let mut phase = 0;
        for piece in &CLR_PIECES {
            phase += self.bb(*piece).count() * GAME_PHASE_INCREMENT[piece.arr_idx()];
        }
        phase
    }

    fn passed_pawn(&self, clr: Color) -> isize {
        let mut score = 0;

        let mut bb = self.pawn_bb(clr);
        while let Some(sq) = bb.next() {
            if !self.passed_leverable(sq, clr) {
                continue;
            }
            score += self.tapered((0, self.king_proximity(sq, clr)));
            score += self.tapered(PASSED_PAWN_REW[clr.idx()][get_rank(sq)]);
            // score += self.passed_block(pos, square);
            score += self.tapered((-11, -8)) * self.passed_file(sq)
        }
        score
    }

    fn passed_leverable(&self, sq: usize, clr: Color) -> bool {
        if !self.candidate_passed(sq, clr) {
            return false;
        }

        if !self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp())) {
            return true;
        }

        // for (var i = -1; i <=1; i+=2) {
        //     var s1 = {x:square.x + i, y:square.y};
        //     var s2 = {x:square.x + i, y:7-square.y};
        //     if (
        //         board(pos, square.x + i, square.y + 1) == "P" &&
        //         "pnbrqk".indexOf(board(pos, square.x + i, square.y)) < 0 &&
        //         (attack(pos, s1) > 0 || attack(colorflip(pos), s2) <= 1)
        //     )
        //         return true;
        // }
        return false;
    }

    fn passed_file(&self, sq: usize) -> isize {
        let file = get_file(sq);
        (file - 1).min(8 - file) as isize
    }

    fn passed_blocked(&self, sq: usize, clr: Color) -> isize {
        todo!()
    }

    fn king_proximity(&self, sq: usize, clr: Color) -> isize {
        let mut score = 0;

        let (rank, file) = (get_rank(sq), get_file(sq));
        let clr_rank = CLR_RANK[clr.idx()][rank];

        let own_king_sq = self.king_sq(clr);
        let (own_rank, own_file) = (get_rank(own_king_sq), get_file(own_king_sq));

        let enemy_king_sq = self.king_sq(clr.opp());
        let (enemy_rank, enemy_file) = (get_rank(enemy_king_sq), get_file(enemy_king_sq));

        let weight = if clr_rank > 2 { 5 * clr_rank - 13 } else { 0 };
        if weight <= 0 {
            return 0;
        }

        score += ((((file - own_file + 1).abs_diff(0)).max((rank - own_rank).abs_diff(0))).min(5)
            * 19
            / 4)
            * weight;

        score += ((((file - enemy_file + 1).abs_diff(0)).max((rank - enemy_rank).abs_diff(0)))
            .min(5)
            * 2)
            * weight;

        // NOTE: Not sure about the rank of this
        if clr_rank > 1 {
            score += (((file - enemy_file + 2).abs_diff(0)).max((rank - enemy_rank).abs_diff(0)))
                .min(5)
                * weight;
        }
        score as isize
    }

    fn candidate_passed(&self, sq: usize, clr: Color) -> bool {
        let forward = PAWN_FORWARD_SPANS[clr.idx()][sq];
        let forward_lr = FORWARD_FILE_LR[clr.idx()][sq];
        let our_pawns = self.pawn_bb(clr);
        let their_pawns = self.pawn_bb(clr.opp());

        // Own pawn ahead? Blocked by same-file pawn
        if forward & our_pawns != 0 {
            return false;
        }

        // No enemy pawn in any of the 3 forward files → clearly candidate
        if forward_lr & their_pawns == 0 {
            return true;
        }

        // Enemy pawn directly in front?
        if self.blocked_pawn(sq, clr, their_pawns) {
            return false;
        }

        let lever_mask = PAWN_ATTACK_LOOKUP[clr.idx()][sq] & their_pawns;
        let leverpush_mask = PAWN_ATTACK_LOOKUP[clr.idx()][sq + 8 * clr.opp().idx()] & their_pawns;
        let phalanx_mask = get_pawn_att_mask(sq, 0, 0, clr) & our_pawns;

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

    // ************************************************
    //                  11. TEMPO                     *
    // ************************************************

    fn tempo(color: Color) -> isize {
        return TEMPO_WT * color.sign();
    }

    fn pawn_init(&mut self) {
        todo!()
    }

    fn piece_init(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::board::fen::FenTrait;

    use super::*;

    struct SFEval<'a> {
        fen: &'a str,
        eval: isize,
        imbalance: isize,
    }

    const SF_EVAL: [SFEval; 1] = [SFEval {
        fen: "r3r1k1/3q1pp1/p2pb2p/Np6/1P1QPn2/5N1P/1P3PP1/R3R1K1 w - - 0 0",
        eval: -34,
        imbalance: 35,
    }];

    // IMBALANCE
    #[test]
    fn imbalance_test() {
        for obj in &SF_EVAL {
            let board = Board::read_fen(obj.fen);
            let imbalance_eval = (board.imbalance(WHITE) - board.imbalance(BLACK)) / 16;
            let offset = 10;
            let diff = imbalance_eval.abs_diff(obj.imbalance);
            assert!(
                diff < offset,
                "Imbalance Eval: {:?}, Actual Eval: {:?}",
                imbalance_eval,
                obj.imbalance
            );
        }
    }

    // let mut arr: [String; 64] = array::from_fn(|_| " ".to_string());
    // while let Some(sq) = bb.next() {
    //     arr[sq] = v.to_string()
    // }
    // print_eval(&arr);
}
