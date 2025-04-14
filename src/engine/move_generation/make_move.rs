use crate::engine::attacks::generated::zobrist_keys::*;
use crate::engine::game::Game;
use crate::engine::shared::helper_func::bitboard::BitboardTrait;
use crate::engine::shared::helper_func::print_utility::print_bitboard;
use crate::engine::shared::helper_func::print_utility::print_chess;
use crate::engine::shared::structures::castling_struct::CASTLE_DATA;
use crate::engine::shared::structures::castling_struct::ROOK_SQ;
use crate::engine::shared::structures::color::ColorTrait;
use crate::engine::shared::structures::internal_move::*;
use crate::engine::shared::structures::piece::*;
use core::panic;

use super::mv_gen::sq_attack;

pub trait GameMoveTrait {
    fn make_move(&mut self, rev: &PositionRev, irr: &PositionIrr) -> bool;
    fn undo_move(&mut self);
    fn make_null_move(&mut self) -> bool;
    fn undo_null_move(&mut self) -> bool;
    fn generate_pos_key(&mut self);
    fn add_piece(&mut self, sq: usize, piece: Piece);
    fn clear_piece(&mut self, sq: usize);
    fn replace_piece(&mut self, from_sq: usize, to_sq: usize);
    fn quiet_mv(&mut self, from_sq: usize, to_sq: usize, piece: Piece);
}

impl GameMoveTrait for Game {
    fn make_move(&mut self, rev: &PositionRev, irr: &PositionIrr) -> bool {
        match rev.flag {
            Flag::Quiet => self.quiet_mv(rev.from as usize, rev.to as usize, rev.piece),
            Flag::Capture(_) => self.replace_piece(rev.from as usize, rev.to as usize),
            Flag::EP => {
                self.replace_piece(rev.from as usize, rev.to as usize);
                self.clear_piece((rev.to + 16 * rev.piece.color() - 8) as usize);
            }
            Flag::Promotion(piece, _) => {
                self.clear_piece(rev.from as usize);
                self.add_piece(rev.to as usize, piece);
            }
            Flag::KingCastle => {
                let sq = &ROOK_SQ[rev.piece.color().idx()][0];
                self.quiet_mv(rev.from as usize, rev.to as usize, rev.piece);
                self.quiet_mv(sq.0, sq.1, ROOK + rev.piece.color());
            }
            Flag::QueenCastle => {
                let sq = &ROOK_SQ[rev.piece.color().idx()][1];
                self.quiet_mv(rev.from as usize, rev.to as usize, rev.piece);
                self.quiet_mv(sq.0, sq.1, ROOK + rev.piece.color());
            }
        }

        self.color.change_color();

        //If the castleRight is set, and if the king is on place and rook is on place than retain otherwise clear

        for c in &CASTLE_DATA {
            if !self.castling.is_set(c.2)
                || !self.bitboard[(ROOK + c.3) as usize].is_set(c.0)
                || !self.bitboard[(KING + c.3) as usize].is_set(c.1)
            {
                self.castling.clear(c.2);
            }
        }

        if rev.piece.is_pawn() && rev.from.abs_diff(rev.to) == 16 {
            self.ep = Some(rev.to + 16 * rev.piece.color() - 8);
        } else {
            self.ep = None
        }

        if rev.piece.is_pawn() || matches!(rev.flag, Flag::Capture(_)) {
            self.half_move = 0
        } else {
            self.half_move += 1;
        }

        if self.pos_rev.len() % 2 == 0 {
            self.full_move += 1;
        }

        self.ply += 1;

        self.generate_pos_key();

        self.pos_irr.push(*irr);
        self.pos_rev.push(*rev);

        let king_sq = self.bitboard[(KING + rev.piece.color()) as usize].get_lsb();

        if sq_attack(self, king_sq, rev.piece.color()) != 0 {
            self.undo_move();
            return false;
        }

        true
    }

    fn undo_move(&mut self) {
        let (rev, irr) = match (self.pos_rev.pop(), self.pos_irr.pop()) {
            (Some(rev), Some(irr)) => (rev, irr),
            (None, None) => return,
            (_, _) => panic!("There is something wrong"),
        };

        self.generate_pos_key();
        self.ply -= 1;
        self.full_move = irr.full_move;
        self.half_move = irr.half_move;
        self.ep = irr.ep;
        self.castling = irr.castle;
        self.color.change_color();

        match rev.flag {
            Flag::Quiet => self.quiet_mv(rev.to as usize, rev.from as usize, rev.piece),
            Flag::Capture(piece) => {
                self.replace_piece(rev.to as usize, rev.from as usize);
                self.add_piece(rev.to as usize, piece);
            }
            Flag::EP => {
                self.replace_piece(rev.to as usize, rev.from as usize);
                self.add_piece(
                    (rev.to + 16 * rev.piece.color() - 8) as usize,
                    PAWN + rev.piece.color().opp(),
                );
            }
            Flag::Promotion(_, cap_piece) => {
                self.clear_piece(rev.to as usize);
                if let Some(piece) = cap_piece {
                    self.add_piece(rev.to as usize, piece)
                }
                self.add_piece(rev.from as usize, rev.piece);
            }
            Flag::KingCastle => {
                let sq = &ROOK_SQ[rev.piece.color().idx()][0];
                self.quiet_mv(rev.to as usize, rev.from as usize, rev.piece);
                self.quiet_mv(sq.1, sq.0, ROOK + rev.piece.color());
            }
            Flag::QueenCastle => {
                let sq = &ROOK_SQ[rev.piece.color().idx()][1];
                self.quiet_mv(rev.to as usize, rev.from as usize, rev.piece);
                self.quiet_mv(sq.1, sq.0, ROOK + rev.piece.color());
            }
        }
    }

    #[inline(always)]
    fn quiet_mv(&mut self, from_sq: usize, to_sq: usize, piece: Piece) {
        self.squares[from_sq] = None;
        self.squares[to_sq] = Some(piece);

        self.bitboard[piece.idx()] ^= (1u64 << to_sq) | (1u64 << from_sq);
        self.bitboard[piece.color().idx()] ^= (1u64 << to_sq) | (1u64 << from_sq);
        self.key ^= PIECE_KEYS[to_sq][piece.idx()] | PIECE_KEYS[from_sq][piece.idx()];
    }

    #[inline(always)]
    fn add_piece(&mut self, sq: usize, piece: Piece) {
        match self.squares[sq] {
            None => (),
            Some(_) => self.clear_piece(sq),
        }
        self.squares[sq] = Some(piece);
        self.bitboard[piece.idx()].set_bit(sq);
        self.bitboard[piece.color().idx()].set_bit(sq);
        self.key ^= PIECE_KEYS[sq][piece.idx()];
    }

    #[inline(always)]
    fn clear_piece(&mut self, sq: usize) {
        match self.squares[sq] {
            None => panic!("Clearing a Peace that does not exist"),
            Some(piece) => {
                self.squares[sq] = None;
                self.bitboard[piece.idx()].clear_bit(sq);
                self.bitboard[piece.color().idx()].clear_bit(sq);
                self.key ^= PIECE_KEYS[sq][piece.idx()];
            }
        }
    }

    #[inline(always)]
    fn replace_piece(&mut self, from_sq: usize, to_sq: usize) {
        let piece = match self.squares[from_sq] {
            None => {
                print_chess(self);
                panic!(
                    "There is no piece on square: {:#?}, \n other data: {:#?}",
                    from_sq, self.squares[from_sq]
                )
            }
            Some(piece) => piece,
        };

        self.clear_piece(from_sq);
        self.add_piece(to_sq, piece);
    }

    #[inline(always)]
    fn generate_pos_key(&mut self) {
        self.key ^= (SIDE_KEY * self.color as u64) | CASTLE_KEYS[self.castling.idx()];

        if let Some(idx) = self.ep {
            self.key ^= EP_KEYS[idx as usize]
        }
    }

    fn make_null_move(&mut self) -> bool {
        todo!()
    }

    fn undo_null_move(&mut self) -> bool {
        todo!()
    }
}
