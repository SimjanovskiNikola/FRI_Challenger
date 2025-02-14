use core::panic;
use crate::engine::{
    game::Game,
    shared::{
        helper_func::{bitboard::BitboardTrait, print_utility::print_chess},
        structures::{
            castling_struct::CASTLE_DATA,
            internal_move::{Flag, InternalMove},
            piece::{Piece, PieceTrait, KING, ROOK},
            square::Square,
        },
    },
};

use lazy_static::lazy_static;
use rand::Rng;

use super::mv_gen::sq_attack;

lazy_static! {
    pub static ref PieceKeys: [[u64; 14]; 64] = [[rand::rng().random(); 14]; 64];
    pub static ref EpKeys: [u64; 64] = [rand::rng().random(); 64];
    pub static ref CastleKeys: [u64; 16] = [rand::rng().random(); 16];
    pub static ref SideKey: u64 = rand::rng().random();
}

pub trait GameMoveTrait {
    fn make_move(&mut self, mv: &InternalMove) -> bool;
    fn undo_move(&mut self);
    fn generate_pos_key(&mut self);
    fn add_piece(&mut self, sq: usize, piece: Piece);
    fn clear_piece(&mut self, sq: usize);
    fn replace_piece(&mut self, from_sq: usize, to_sq: usize);
    fn quiet_mv(&mut self, from_sq: usize, to_sq: usize, piece: Piece);
}

impl GameMoveTrait for Game {
    fn make_move(&mut self, mv: &InternalMove) -> bool {
        match mv.flag {
            Flag::Quiet => self.quiet_mv(mv.from, mv.to, mv.piece),
            Flag::Capture(_) => self.replace_piece(mv.from, mv.to),
            Flag::EP(sq, _) => {
                self.replace_piece(mv.from, mv.to);
                self.clear_piece(sq);
            }
            Flag::Promotion(piece, _) => {
                self.clear_piece(mv.from);
                self.add_piece(mv.to, piece);
            }
            Flag::KingSideCastle(rook_from, rook_to) => {
                self.quiet_mv(mv.from, mv.to, mv.piece);
                self.quiet_mv(rook_from, rook_to, ROOK + mv.piece.color());
            }
            Flag::QueenSideCastle(rook_from, rook_to) => {
                self.quiet_mv(mv.from, mv.to, mv.piece);
                self.quiet_mv(rook_from, rook_to, ROOK + mv.piece.color());
            }
        }

        self.color.change_color();

        //If the castleRight is set, and if the king is on place and rook is on place than retain otherwise clear

        for c in &CASTLE_DATA {
            if !mv.castle.is_set(c.2)
                || !self.bitboard[(ROOK + c.3) as usize].is_set(c.0)
                || !self.bitboard[(KING + c.3) as usize].is_set(c.1)
            {
                self.castling.clear(c.2);
            }
        }

        if mv.piece.is_pawn() && mv.from.abs_diff(mv.to) == 16 {
            self.ep = Some(mv.to + 16 * mv.active_color.idx() - 8);
        } else {
            self.ep = None
        }

        if mv.piece.is_pawn() || matches!(mv.flag, Flag::Capture(_)) {
            self.half_move = 0
        } else {
            self.half_move = mv.half_move + 1;
        }

        if self.moves.len() % 2 == 0 {
            self.full_move += 1;
        }

        self.generate_pos_key();
        self.moves.push(*mv);

        let king_sq = self.bitboard[(KING + mv.active_color) as usize].get_lsb();

        if sq_attack(self, king_sq, mv.active_color) != 0 {
            self.undo_move();
            return false;
        }

        true
    }

    fn undo_move(&mut self) {
        let mv = match self.moves.pop() {
            Some(mv) => mv,
            None => return,
        };
        self.generate_pos_key();
        self.full_move -= if self.moves.len() % 2 == 0 { 1 } else { 0 };
        self.half_move = mv.half_move;
        self.ep = mv.ep;
        self.castling = mv.castle;
        self.color.change_color();

        match mv.flag {
            Flag::Quiet => self.quiet_mv(mv.to, mv.from, mv.piece),
            Flag::Capture(piece) => {
                self.replace_piece(mv.to, mv.from);
                self.add_piece(mv.to, piece);
            }
            Flag::EP(pos, piece) => {
                self.replace_piece(mv.to, mv.from);
                self.add_piece(pos, piece);
            }
            Flag::Promotion(_, cap_piece) => {
                self.clear_piece(mv.to);
                if let Some(piece) = cap_piece {
                    self.add_piece(mv.to, piece)
                }
                self.add_piece(mv.from, mv.piece);
            }
            Flag::KingSideCastle(rook_from, rook_to) => {
                self.quiet_mv(mv.to, mv.from, mv.piece);
                self.quiet_mv(rook_to, rook_from, ROOK + mv.piece.color());
            }
            Flag::QueenSideCastle(rook_from, rook_to) => {
                self.quiet_mv(mv.to, mv.from, mv.piece);
                self.quiet_mv(rook_to, rook_from, ROOK + mv.piece.color());
            }
        }
    }

    #[inline(always)]
    fn quiet_mv(&mut self, from_sq: usize, to_sq: usize, piece: Piece) {
        self.squares[from_sq] = Square::Empty;
        self.squares[to_sq] = Square::Occupied(piece);

        self.bitboard[piece.idx()] ^= (1u64 << to_sq) | (1u64 << from_sq);
        self.occupancy[piece.color().idx()] ^= (1u64 << to_sq) | (1u64 << from_sq);
        self.pos_key ^= PieceKeys[to_sq][piece.idx()] | PieceKeys[from_sq][piece.idx()];
    }

    #[inline(always)]
    fn add_piece(&mut self, sq: usize, piece: Piece) {
        match self.squares[sq] {
            Square::Empty => (),
            Square::Occupied(_) => self.clear_piece(sq),
        }
        self.squares[sq] = Square::Occupied(piece);
        self.bitboard[piece.idx()].set_bit(sq);
        self.occupancy[piece.color().idx()].set_bit(sq);
        self.pos_key ^= PieceKeys[sq][piece.idx()];
    }

    #[inline(always)]
    fn clear_piece(&mut self, sq: usize) {
        match self.squares[sq] {
            Square::Empty => panic!("Clearing a Peace that does not exist"),
            Square::Occupied(piece) => {
                self.squares[sq] = Square::Empty;
                self.bitboard[piece.idx()].clear_bit(sq);
                self.occupancy[piece.color().idx()].clear_bit(sq);
                self.pos_key ^= PieceKeys[sq][piece.idx()];
            }
        }
    }

    #[inline(always)]
    fn replace_piece(&mut self, from_sq: usize, to_sq: usize) {
        let piece = match self.squares[from_sq] {
            Square::Empty => {
                print_chess(self);
                panic!(
                    "There is no piece on square: {:#?}, \n other data: {:#?}",
                    from_sq, self.squares[from_sq]
                )
            }
            Square::Occupied(piece) => piece,
        };

        self.clear_piece(from_sq);
        self.add_piece(to_sq, piece);
    }

    #[inline(always)]
    fn generate_pos_key(&mut self) {
        self.pos_key ^= (*SideKey * self.color as u64) | CastleKeys[self.castling.idx()];

        if let Some(idx) = self.ep {
            self.pos_key ^= EpKeys[idx]
        }
    }
}
