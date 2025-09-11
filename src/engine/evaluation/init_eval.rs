use crate::engine::attacks::bishop::get_bishop_mask;
use crate::engine::attacks::king::get_king_mask;
use crate::engine::attacks::pawn::{
    get_all_pawn_left_att_mask, get_all_pawn_right_att_mask, get_pawn_att_mask,
};
use crate::engine::board::board::Board;
use crate::engine::board::color::{ColorTrait, BLACK, COLORS, WHITE};
use crate::engine::board::piece::{PieceTrait, KING, PAWN, PIECES};
use crate::engine::board::square::get_file;
use crate::engine::evaluation::common_eval::{CommonEvalTrait, KING_ATT_WEIGHT};
use crate::engine::evaluation::eval_defs::CLR_CENTER;
use crate::engine::evaluation::king_eval::KingEvalTrait;
use crate::engine::evaluation::material_eval::MaterialEvalTrait;
use crate::engine::evaluation::mobility_eval::MobilityEvalTrait;
use crate::engine::evaluation::pawn_eval::PawnEvalTrait;
use crate::engine::evaluation::piece_eval::OUTPOST_RANKS;
use crate::engine::generated::king::KING_RING;
use crate::engine::generated::pawn::{FORWARD_SPANS_LR, PAWN_3_BEHIND_MASKS};
use crate::engine::misc::bitboard::{BitboardTrait, Iterator};
use crate::engine::misc::const_utility::FILE_BITBOARD;

pub const MG_LIMIT: isize = 15258;
pub const EG_LIMIT: isize = 3915;

pub trait InitEvalTrait {
    fn init(&mut self);
    fn king_init(&mut self);
    fn pawn_init(&mut self);
    fn piece_init(&mut self);
    fn determine_phase(&mut self);
}

impl InitEvalTrait for Board {
    #[inline(always)]
    fn init(&mut self) {
        self.eval.reset();
        self.determine_phase();

        self.pawn_init();
        self.piece_init();
        self.king_init();
    }

    #[inline(always)]
    fn pawn_init(&mut self) {
        for &clr in &COLORS {
            let (own, enemy) = self.both_occ_bb(clr);
            let piece = PAWN + clr;
            let mut bb = self.bb(piece);

            while let Some(sq) = bb.next() {
                self.eval.pawn_behind_masks[clr.idx()] |=
                    PAWN_3_BEHIND_MASKS[clr.idx()][sq] & CLR_CENTER[clr.idx()];

                if !self.backward_pawn(sq, clr)
                    && !self.blocked_pawn(sq, clr, self.pawn_bb(clr.opp()))
                {
                    self.eval.pawn_att_span[clr.idx()] |=
                        FORWARD_SPANS_LR[clr.idx()][sq] | get_pawn_att_mask(sq, own, enemy, clr);
                }

                self.eval.king_pawn_dx[clr.idx()] =
                    self.eval.king_pawn_dx[clr.idx()].min(self.king_dist(clr, sq));

                self.eval.open_file[clr.idx()] &= !FILE_BITBOARD[get_file(sq)]; // OPEN-FILE
            }
        }

        for &clr in &COLORS {
            self.eval.outpost[clr.idx()] = !self.eval.pawn_att_span[clr.opp().idx()]
                & OUTPOST_RANKS[clr.idx()]
                & (get_all_pawn_left_att_mask(self.pawn_bb(clr), clr)
                    | get_all_pawn_right_att_mask(self.pawn_bb(clr), clr));
        }
    }

    #[inline(always)]
    fn piece_init(&mut self) {
        for &clr in &COLORS {
            let (own, enemy) = self.both_occ_bb(clr);
            let area = self.mobility_area(clr);
            let king_sq = self.king_sq(clr.opp());
            let king_ring = self.king_ring(clr.opp());

            let opp_king_mask = get_king_mask(king_sq, 0, 0, clr.opp());

            for &pce in &PIECES {
                let piece = pce + clr;
                let mut bb = self.bb(piece);
                let mut attckers_count = 0;

                while let Some(sq) = bb.next() {
                    let piece_mask = self.x_ray_mask(piece, sq);

                    self.eval.attacked_by_2[clr.idx()] |=
                        self.eval.attack_map[clr.idx()] & piece_mask;

                    self.eval.attack_map[clr.idx()] |= piece_mask;

                    self.eval.attacked_by[piece.idx()] |= piece_mask;

                    match piece.kind() {
                        PAWN => {
                            if piece_mask & KING_RING[king_sq] != 0 {
                                attckers_count += 1;
                                self.eval.king_att_count[clr.idx()] +=
                                    (piece_mask & KING_RING[king_sq]).count();
                            }
                        }
                        KING => {}
                        _ => {
                            let safe_squares = (self.mobility_piece(sq, piece, clr) & area).count();
                            self.eval.mobility[clr.idx()] +=
                                self.mobility_bonus(piece, safe_squares).0;

                            if piece_mask & king_ring != 0 {
                                attckers_count += 1;
                                self.eval.king_att_count[clr.idx()] += 1;
                                self.eval.king_att_count_pieces[clr.idx()].set_bit(sq);
                            }

                            self.eval.king_att[clr.idx()] += (piece_mask & opp_king_mask).count();

                            self.eval.x_ray[piece.idx()] |= self.mobility_piece(sq, piece, clr);
                        }
                    }

                    if piece.is_queen() {
                        self.eval.queen_diagonal[clr.idx()] |= get_bishop_mask(sq, own, enemy, clr);
                    }
                }

                self.eval.king_att_weight[clr.idx()] +=
                    KING_ATT_WEIGHT[piece.arr_idx()] * attckers_count as isize;
            }
        }
    }

    #[inline(always)]
    fn king_init(&mut self) {
        for clr in COLORS {
            self.eval.king_ring[clr.idx()] = self.king_ring(clr);
            self.check(clr);
            self.eval.king_shelter[clr.idx()] = self.shelter(clr);
        }
    }

    #[inline(always)]
    fn determine_phase(&mut self) {
        let mut npm = self.non_pawn_material_eval(WHITE) + self.non_pawn_material_eval(BLACK);
        // println!("{:?}", self.non_pawn_material_eval(WHITE));
        // println!("{:?}", npm);

        npm = EG_LIMIT.max(npm.min(MG_LIMIT));
        // println!("{:?}", npm);

        let phase = ((npm - EG_LIMIT) * 128) / (MG_LIMIT - EG_LIMIT);
        // println!("{:?}", (npm, phase));

        self.eval.phase = (phase, 128 - phase);
        // println!("{:?}", (phase, 128 - phase));
    }
}
