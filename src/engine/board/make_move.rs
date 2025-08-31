use super::mv_gen::BoardGenMoveTrait;
use super::structures::board::Board;
use super::structures::castling::*;
use super::structures::color::ColorTrait;
use super::structures::moves::*;
use super::structures::piece::*;
use crate::engine::board::structures::zobrist::ZobristKeysTrait;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::move_generator::generated::zobrist_keys::*;
use core::panic;

pub trait BoardMoveTrait {
    fn make_move(&mut self, mv: &Move) -> bool;
    fn undo_move(&mut self);
    fn make_null_move(&mut self) -> bool;
    fn undo_null_move(&mut self);
    fn make_state(&mut self, mv: &Move);
    fn undo_state(&mut self);

    fn clear_piece(&mut self, sq: usize, piece: Piece);
    fn add_piece(&mut self, sq: usize, piece: Piece);
    fn quiet_mv(&mut self, from_sq: usize, to_sq: usize, piece: Piece);
}

impl BoardMoveTrait for Board {
    fn make_move(&mut self, mv: &Move) -> bool {
        self.history.push(self.state);
        self.moves.push(*mv);
        self.zb_reset_key();

        match mv.flag {
            Flag::Quiet => self.quiet_mv(mv.from as usize, mv.to as usize, mv.piece),
            Flag::Capture(piece) => {
                self.clear_piece(mv.to as usize, piece);
                self.quiet_mv(mv.from as usize, mv.to as usize, mv.piece);
            }
            Flag::EP => {
                self.quiet_mv(mv.from as usize, mv.to as usize, mv.piece);
                self.clear_piece(
                    (mv.to + 16 * mv.piece.color() - 8) as usize,
                    PAWN + mv.piece.color().opp(),
                );
            }
            Flag::Promotion(promotion, cap_piece) => {
                self.clear_piece(mv.from as usize, mv.piece);
                if let Some(piece) = cap_piece {
                    self.clear_piece(mv.to as usize, piece)
                }
                self.add_piece(mv.to as usize, promotion);
            }
            Flag::KingCastle => {
                let sq = &ROOK_SQ[mv.piece.color().idx()][0];
                self.quiet_mv(mv.from as usize, mv.to as usize, mv.piece);
                self.quiet_mv(sq.0, sq.1, ROOK + mv.piece.color());
            }
            Flag::QueenCastle => {
                let sq = &ROOK_SQ[mv.piece.color().idx()][1];
                self.quiet_mv(mv.from as usize, mv.to as usize, mv.piece);
                self.quiet_mv(sq.0, sq.1, ROOK + mv.piece.color());
            }
        }

        self.make_state(mv);

        if self.sq_attack(self.king_sq(self.color().opp()), mv.piece.color()) != 0 {
            self.undo_move();
            return false;
        }
        true
    }

    fn undo_move(&mut self) {
        let (mv, st) = match (self.moves.pop(), self.history.pop()) {
            (Some(m), Some(s)) => (m, s),
            (None, None) => return,
            (_, _) => panic!("There is something wrong"),
        };

        match mv.flag {
            Flag::Quiet => self.quiet_mv(mv.to as usize, mv.from as usize, mv.piece),
            Flag::Capture(piece) => {
                self.quiet_mv(mv.to as usize, mv.from as usize, mv.piece);
                self.add_piece(mv.to as usize, piece);
            }
            Flag::EP => {
                self.quiet_mv(mv.to as usize, mv.from as usize, mv.piece);
                self.add_piece(
                    (mv.to + 16 * mv.piece.color() - 8) as usize,
                    PAWN + mv.piece.color().opp(),
                );
            }
            Flag::Promotion(promotion, cap_piece) => {
                self.clear_piece(mv.to as usize, promotion);
                if let Some(piece) = cap_piece {
                    self.add_piece(mv.to as usize, piece)
                }
                self.add_piece(mv.from as usize, mv.piece);
            }
            Flag::KingCastle => {
                let sq = &ROOK_SQ[mv.piece.color().idx()][0];
                self.quiet_mv(mv.to as usize, mv.from as usize, mv.piece);
                self.quiet_mv(sq.1, sq.0, ROOK + mv.piece.color());
            }
            Flag::QueenCastle => {
                let sq = &ROOK_SQ[mv.piece.color().idx()][1];
                self.quiet_mv(mv.to as usize, mv.from as usize, mv.piece);
                self.quiet_mv(sq.1, sq.0, ROOK + mv.piece.color());
            }
        }

        self.state = st;
    }

    #[inline(always)]
    fn quiet_mv(&mut self, from_sq: usize, to_sq: usize, piece: Piece) {
        self.squares[from_sq] = 0;
        self.squares[to_sq] = piece;

        self.bitboard[piece.idx()] ^= (1u64 << to_sq) | (1u64 << from_sq);
        self.bitboard[piece.color().idx()] ^= (1u64 << to_sq) | (1u64 << from_sq);
        self.state.key ^= PIECE_KEYS[from_sq][piece.idx()];
        self.state.key ^= PIECE_KEYS[to_sq][piece.idx()];
    }

    #[inline(always)]
    fn add_piece(&mut self, sq: usize, piece: Piece) {
        assert!(self.squares[sq] == 0, "Adding a Piece on a square, where a peace already exists");
        self.squares[sq] = piece;
        self.bitboard[piece.idx()].set_bit(sq);
        self.bitboard[piece.color().idx()].set_bit(sq);
        self.state.key ^= PIECE_KEYS[sq][piece.idx()];
    }

    #[inline(always)]
    fn clear_piece(&mut self, sq: usize, piece: Piece) {
        assert!(self.squares[sq] != 0, "Clearing a Peace that does not exist");
        self.squares[sq] = 0;
        self.bitboard[piece.idx()].clear_bit(sq);
        self.bitboard[piece.color().idx()].clear_bit(sq);
        self.state.key ^= PIECE_KEYS[sq][piece.idx()];
    }

    fn make_null_move(&mut self) -> bool {
        todo!()
    }

    fn undo_null_move(&mut self) {
        todo!()
    }

    fn make_state(&mut self, mv: &Move) {
        // Switch the color
        self.state.color.change_color();
        self.zb_clr();

        //If the castleRight is set, and if the king is on place and rook is on place than retain otherwise clear
        for c in &CASTLE_DATA {
            if !(self.state.castling.is_set(c.2)
                && self.bitboard[(ROOK + c.3) as usize].is_set(c.0)
                && self.bitboard[(KING + c.3) as usize].is_set(c.1))
            {
                self.state.castling.clear(c.2);
            }
        }
        self.zb_castling();

        // Setting the En passant
        if mv.piece.is_pawn() && mv.from.abs_diff(mv.to) == 16 {
            self.state.ep = Some(mv.to + 16 * mv.piece.color() - 8);
        } else {
            self.state.ep = None
        }
        self.zb_ep();

        // If the move is pawn or if there is a capture, reset the halfmove
        let keep_half_move =
            !(mv.piece.is_pawn() || mv.flag.is_capture() || mv.flag.is_promo()) as u8;
        self.state.half_move = (self.state.half_move + 1) * keep_half_move;

        // Full Move should be half of the moves
        self.state.full_move = self.history.len() as u16 / 2 + 1;
    }

    fn undo_state(&mut self) {
        assert!(self.history.len() > 0, "Can't Pop states from empty history");
        self.history.pop();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn key_test() {
        let mut board = Board::initialize();
        let key = board.state.key;
        println!("{:?}", board.state.key);

        let moves = board.gen_moves();
        board.make_move(&moves[0].0);
        board.undo_move();

        assert_eq!(board.state.key, key, "Key should be same after undoing the move");
    }
}
