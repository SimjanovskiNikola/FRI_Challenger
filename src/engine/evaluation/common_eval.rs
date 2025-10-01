use crate::engine::attacks::bishop::get_bishop_mask;
use crate::engine::attacks::king::get_king_mask;
use crate::engine::attacks::knight::get_knight_mask;
use crate::engine::attacks::pawn::{get_pawn_2_att, get_pawn_att_mask};
use crate::engine::attacks::queen::get_queen_mask;
use crate::engine::attacks::rook::get_rook_mask;
use crate::engine::board::board::Board;
use crate::engine::board::color::{BLACK, Color, ColorTrait, WHITE};
use crate::engine::board::piece::*;
use crate::engine::board::square::{get_file, get_rank};
use crate::engine::generated::between::BETWEEN_BB;
use crate::engine::generated::king::KING_RING;

pub static KING_ATT_WEIGHT: [isize; 6] = [0, 81, 0, 52, 44, 10];

// First 3 Ranks and 4 Center Files for every Color
pub static CLR_CENTER: [u64; 2] = [1010580480, 16954726998343680];

// Absolute Ranks based on color
pub static CLR_RANK: [[usize; 8]; 2] = [[0, 1, 2, 3, 4, 5, 6, 7], [7, 6, 5, 4, 3, 2, 1, 0]];

// Absolute Square based on color
#[rustfmt::skip]
pub static CLR_SQ: [[usize; 64]; 2] = [
    [
        0,  1,  2,  3,  4,  5,  6,  7,
        8,  9,  10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63,
    ],
    [ 
        56, 57, 58, 59, 60, 61, 62, 63,
        48, 49, 50, 51, 52, 53, 54, 55,
        40, 41, 42, 43, 44, 45, 46, 47,
        32, 33, 34, 35, 36, 37, 38, 39,
        24, 25, 26, 27, 28, 29, 30, 31,
        16, 17, 18, 19, 20, 21, 22, 23,
        8,  9,  10, 11, 12, 13, 14, 15,
        0,  1,  2,  3,  4,  5,  6,  7,
    ],
];

pub trait CommonEvalTrait {
    fn tapered(&mut self, value: (isize, isize)) -> isize;
    fn insufficient_material(&self) -> bool;
    fn front_sq(&mut self, sq: usize, clr: Color) -> usize;
    fn back_sq(&mut self, sq: usize, clr: Color) -> usize;
    fn pin_att(&mut self, from: usize, to: usize, piece: Piece) -> u64;

    fn sum(
        &mut self,
        color: Color,
        square: Option<usize>,
        piece: Option<Piece>,
        value: (isize, isize),
    );

    fn sum_into_arr(
        &mut self,
        color: Color,
        square: Option<usize>,
        piece: Option<Piece>,
        value: (isize, isize),
        score: &mut [(isize, isize); 2],
    );

    fn calculate_score(&mut self) -> isize;
    fn get_mask(&mut self, piece: Piece, sq: usize) -> u64;
    fn x_ray_mask(&mut self, piece: Piece, sq: usize) -> u64;

    fn king_dist(&mut self, clr: Color, sq: usize) -> usize;
    fn king_ring(&mut self, clr: Color) -> u64;
}

impl CommonEvalTrait for Board {
    #[inline(always)]
    fn insufficient_material(&self) -> bool {
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

    #[inline(always)]
    fn tapered(&mut self, value: (isize, isize)) -> isize {
        (self.eval.phase.0 * value.0 + self.eval.phase.1 * value.1) / 128
    }

    #[inline(always)]
    fn calculate_score(&mut self) -> isize {
        let mg = self.eval.score[WHITE.idx()].0 - self.eval.score[BLACK.idx()].0;
        let eg = self.eval.score[WHITE.idx()].1 - self.eval.score[BLACK.idx()].1;

        return self.tapered((mg, eg));
    }

    #[inline(always)]
    fn front_sq(&mut self, sq: usize, clr: Color) -> usize {
        (sq as isize + 8 * clr.sign()) as usize
    }

    #[inline(always)]
    fn back_sq(&mut self, sq: usize, clr: Color) -> usize {
        (sq as isize - 8 * clr.sign()) as usize
    }

    #[inline(always)]
    fn pin_att(&mut self, from: usize, to: usize, piece: Piece) -> u64 {
        let (from_file, from_rank) = (get_file(from), get_rank(from));
        let (to_file, to_rank) = (get_file(to), get_rank(to));

        match piece {
            p if p.is_queen() => BETWEEN_BB[from][to],
            p if p.is_bishop() => {
                let diagonal_move = from_file != to_file && from_rank != to_rank;
                BETWEEN_BB[from][to] * diagonal_move as u64
            }
            p if p.is_rook() => {
                let straight_move = from_file == to_file || from_rank == to_rank;
                BETWEEN_BB[from][to] * straight_move as u64
            }
            _ => 0,
        }
    }

    #[inline(always)]
    fn sum(
        &mut self,
        color: Color,
        square: Option<usize>,
        piece: Option<Piece>,
        value: (isize, isize),
    ) {
        // self.trace(color, square, piece, value);

        self.eval.score[color.idx()].0 += value.0;
        self.eval.score[color.idx()].1 += value.1;
    }

    #[inline(always)]
    fn sum_into_arr(
        &mut self,
        color: Color,
        square: Option<usize>,
        piece: Option<Piece>,
        value: (isize, isize),
        score: &mut [(isize, isize); 2],
    ) {
        // self.trace(color, square, piece, value);

        score[color.idx()].0 += value.0;
        score[color.idx()].1 += value.1;
    }

    // ************************************************
    //                    ATTACKS                     *
    // ************************************************

    #[inline(always)]
    fn x_ray_mask(&mut self, piece: Piece, sq: usize) -> u64 {
        let clr = piece.color();
        let (mut own, mut enemy) = self.both_occ_bb(clr);
        match piece.kind() {
            PAWN => get_pawn_att_mask(sq, own, enemy, clr),
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
            KING => get_king_mask(sq, own, enemy, clr),
            _ => panic!("Invalid Peace Type"),
        }
    }

    #[inline(always)]
    fn get_mask(&mut self, piece: Piece, sq: usize) -> u64 {
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

    #[inline(always)]
    fn king_dist(&mut self, clr: Color, sq: usize) -> usize {
        let (sq_rank, sq_file) = (get_rank(sq), get_file(sq));
        let (king_rank, king_file) = (get_rank(self.king_sq(clr)), get_file(self.king_sq(clr)));
        return (king_rank.abs_diff(sq_rank)).max(king_file.abs_diff(sq_file));
    }

    #[inline(always)]
    fn king_ring(&mut self, clr: Color) -> u64 {
        return KING_RING[self.king_sq(clr)] & !get_pawn_2_att(self.pawn_bb(clr), clr);
    }
}
