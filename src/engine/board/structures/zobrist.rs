use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color::ColorTrait;
use crate::engine::board::structures::piece::{Piece, PieceTrait};
use crate::engine::move_generator::generated::zobrist_keys::*;

pub trait ZobristKeysTrait {
    fn zb_reset_key(&mut self);
    fn zb_replace_piece(&mut self, from_sq: usize, to_sq: usize, piece: Piece);
    fn zb_toggle_piece(&mut self, sq: usize, piece: Piece);
    fn zb_clr(&mut self);
    fn zb_castling(&mut self);
    fn zb_ep(&mut self);
}

impl ZobristKeysTrait for Board {
    fn zb_reset_key(&mut self) {
        self.zb_clr();
        self.zb_castling();
        self.zb_ep();
    }

    fn zb_replace_piece(&mut self, from_sq: usize, to_sq: usize, piece: Piece) {
        self.state.key ^= PIECE_KEYS[to_sq][piece.idx()] | PIECE_KEYS[from_sq][piece.idx()];
    }

    fn zb_toggle_piece(&mut self, sq: usize, piece: Piece) {
        self.state.key ^= PIECE_KEYS[sq][piece.idx()]
    }

    fn zb_clr(&mut self) {
        self.state.key ^= SIDE_KEY * self.color() as u64;
    }

    fn zb_castling(&mut self) {
        //FIXME:
        self.state.key ^= CASTLE_KEYS[self.state.castling.idx()];
    }

    fn zb_ep(&mut self) {
        if let Some(idx) = self.state.ep {
            self.state.key ^= EP_KEYS[idx as usize]
        }
    }
}
