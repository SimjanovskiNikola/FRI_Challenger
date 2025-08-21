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

    fn add_piece(&mut self, sq: usize, piece: Piece);
    fn clear_piece(&mut self, sq: usize);
    fn replace_piece(&mut self, from_sq: usize, to_sq: usize);
    fn quiet_mv(&mut self, from_sq: usize, to_sq: usize, piece: Piece);
}

impl BoardMoveTrait for Board {
    fn make_move(&mut self, mv: &Move) -> bool {
        self.history.push(self.state);
        self.moves.push(*mv);

        match mv.flag {
            Flag::Quiet => self.quiet_mv(mv.from as usize, mv.to as usize, mv.piece),
            Flag::Capture(_) => self.replace_piece(mv.from as usize, mv.to as usize),
            Flag::EP => {
                self.replace_piece(mv.from as usize, mv.to as usize);
                self.clear_piece((mv.to + 16 * mv.piece.color() - 8) as usize);
            }
            Flag::Promotion(piece, _) => {
                self.clear_piece(mv.from as usize);
                self.add_piece(mv.to as usize, piece);
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
        // self.zb_reset_key();
        // self.zb_clr();
        // self.zb_castling();
        // self.zb_ep();
        // self.generate_pos_key();

        match mv.flag {
            Flag::Quiet => self.quiet_mv(mv.to as usize, mv.from as usize, mv.piece),
            Flag::Capture(piece) => {
                self.replace_piece(mv.to as usize, mv.from as usize);
                self.add_piece(mv.to as usize, piece);
            }
            Flag::EP => {
                self.replace_piece(mv.to as usize, mv.from as usize);
                self.add_piece(
                    (mv.to + 16 * mv.piece.color() - 8) as usize,
                    PAWN + mv.piece.color().opp(),
                );
            }
            Flag::Promotion(_, cap_piece) => {
                self.clear_piece(mv.to as usize);
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
        self.squares[from_sq] = None;
        self.squares[to_sq] = Some(piece);

        self.bitboard[piece.idx()] ^= (1u64 << to_sq) | (1u64 << from_sq);
        self.bitboard[piece.color().idx()] ^= (1u64 << to_sq) | (1u64 << from_sq);
        self.state.key ^= PIECE_KEYS[from_sq][piece.idx()];
        self.state.key ^= PIECE_KEYS[to_sq][piece.idx()];
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
        self.state.key ^= PIECE_KEYS[sq][piece.idx()];
    }

    #[inline(always)]
    fn clear_piece(&mut self, sq: usize) {
        match self.squares[sq] {
            None => {
                // println!("Square to remove peace: {:?}", sq_notation(sq as u8));
                // print_chess(&self);
                // print_move_list(&self.gen_moves());
                // self.undo_move();
                // print_chess(&self);
                // print_move_list(&self.gen_moves());
                panic!("Clearing a Peace that does not exist")
            }
            Some(piece) => {
                self.squares[sq] = None;
                self.bitboard[piece.idx()].clear_bit(sq);
                self.bitboard[piece.color().idx()].clear_bit(sq);
                self.state.key ^= PIECE_KEYS[sq][piece.idx()];
            }
        }
    }

    #[inline(always)]
    fn replace_piece(&mut self, from_sq: usize, to_sq: usize) {
        let piece = match self.squares[from_sq] {
            None => {
                // print_chess(self);
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

    fn make_null_move(&mut self) -> bool {
        todo!()
    }

    fn undo_null_move(&mut self) {
        todo!()
    }

    fn make_state(&mut self, mv: &Move) {
        self.zb_reset_key();

        // Switch the color
        self.state.color.change_color();
        self.zb_clr();

        //If the castleRight is set, and if the king is on place and rook is on place than retain otherwise clear
        for c in &CASTLE_DATA {
            if !self.state.castling.is_set(c.2)
                || !self.bitboard[(ROOK + c.3) as usize].is_set(c.0)
                || !self.bitboard[(KING + c.3) as usize].is_set(c.1)
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
        if mv.piece.is_pawn() || mv.flag.is_capture() || mv.flag.is_promo() {
            self.state.half_move = 0
        } else {
            self.state.half_move += 1;
        }

        // Full Move should be half of the moves
        if self.history.len() % 2 == 0 {
            self.state.full_move += 1;
        }

        // self.generate_pos_key();
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
