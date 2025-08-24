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
        // FIXME: THIS IS WRONG !!! INVALID ZOBRIST KEY BECAUSE OF "|" !!!
        // self.state.key ^= PIECE_KEYS[to_sq][piece.idx()] | PIECE_KEYS[from_sq][piece.idx()];
    }

    fn zb_toggle_piece(&mut self, sq: usize, piece: Piece) {
        self.state.key ^= PIECE_KEYS[sq][piece.idx()]
    }

    fn zb_clr(&mut self) {
        self.state.key ^= SIDE_KEY * self.color() as u64;
    }

    fn zb_castling(&mut self) {
        self.state.key ^= CASTLE_KEYS[self.state.castling.idx()];
    }

    fn zb_ep(&mut self) {
        if let Some(idx) = self.state.ep {
            self.state.key ^= EP_KEYS[idx as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::{
        board::{fen::FenTrait, make_move::BoardMoveTrait, structures::board::Board},
        misc::{const_utility::FEN_START, display::display_moves::from_move_notation},
    };

    #[test]
    fn test_hash_v2() {
        let mut board = Board::read_fen(&FEN_START);
        let moves = ["e2e4", "e7e5", "b1c3", "b8c6", "c3b1", "c6b8"];
        let boards = [
            Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").key(),
            Board::read_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").key(),
            Board::read_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2").key(),
            Board::read_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 1 2").key(),
            Board::read_fen("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 2 3")
                .key(),
            Board::read_fen("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 3 3").key(),
            Board::read_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 4 4").key(),
        ];
        assert_eq!(boards[0], board.key());

        for (idx, notation) in moves.iter().enumerate() {
            let mv = from_move_notation(&notation, &mut board);
            board.make_move(&mv);
            assert_eq!(boards[idx + 1], board.key());
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_hash_v3() {
        let mut board = Board::read_fen(&FEN_START);
        let moves = ["e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "d7d6", "e1g1", 
                    "c8g4", "a2a4", "d8d7", "a4a5", "e8c8", "d1e2", "b7b5", "a5b6", 
                    "g4f3", "b6a7", "f3e2", "a7a8Q", "c6b8", "c4e2", "h7h5", "e2a6"  ];
        let boards = [
            Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").key(), 
            Board::read_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").key(),
            Board::read_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2").key(),
            Board::read_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2").key(),
            Board::read_fen("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3").key(),
            Board::read_fen("r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3").key(),
            Board::read_fen("r1bqkbnr/ppp2ppp/2np4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4").key(),
            Board::read_fen("r1bqkbnr/ppp2ppp/2np4/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 1 4").key(),
            Board::read_fen("r2qkbnr/ppp2ppp/2np4/4p3/2B1P1b1/5N2/PPPP1PPP/RNBQ1RK1 w kq - 2 5").key(),
            Board::read_fen("r2qkbnr/ppp2ppp/2np4/4p3/P1B1P1b1/5N2/1PPP1PPP/RNBQ1RK1 b kq a3 0 5").key(),
            Board::read_fen("r3kbnr/pppq1ppp/2np4/4p3/P1B1P1b1/5N2/1PPP1PPP/RNBQ1RK1 w kq - 1 6").key(),
            Board::read_fen("r3kbnr/pppq1ppp/2np4/P3p3/2B1P1b1/5N2/1PPP1PPP/RNBQ1RK1 b kq - 0 6").key(),
            Board::read_fen("2kr1bnr/pppq1ppp/2np4/P3p3/2B1P1b1/5N2/1PPP1PPP/RNBQ1RK1 w - - 1 7").key(),
            Board::read_fen("2kr1bnr/pppq1ppp/2np4/P3p3/2B1P1b1/5N2/1PPPQPPP/RNB2RK1 b - - 2 7").key(),
            Board::read_fen("2kr1bnr/p1pq1ppp/2np4/Pp2p3/2B1P1b1/5N2/1PPPQPPP/RNB2RK1 w - b6 0 8").key(),
            Board::read_fen("2kr1bnr/p1pq1ppp/1Pnp4/4p3/2B1P1b1/5N2/1PPPQPPP/RNB2RK1 b - - 0 8").key(),
            Board::read_fen("2kr1bnr/p1pq1ppp/1Pnp4/4p3/2B1P3/5b2/1PPPQPPP/RNB2RK1 w - - 0 9").key(),
            Board::read_fen("2kr1bnr/P1pq1ppp/2np4/4p3/2B1P3/5b2/1PPPQPPP/RNB2RK1 b - - 0 9").key(),
            Board::read_fen("2kr1bnr/P1pq1ppp/2np4/4p3/2B1P3/8/1PPPbPPP/RNB2RK1 w - - 0 10").key(),
            Board::read_fen("Q1kr1bnr/2pq1ppp/2np4/4p3/2B1P3/8/1PPPbPPP/RNB2RK1 b - - 0 10").key(),
            Board::read_fen("Qnkr1bnr/2pq1ppp/3p4/4p3/2B1P3/8/1PPPbPPP/RNB2RK1 w - - 1 11").key(),
            Board::read_fen("Qnkr1bnr/2pq1ppp/3p4/4p3/4P3/8/1PPPBPPP/RNB2RK1 b - - 0 11").key(),
            Board::read_fen("Qnkr1bnr/2pq1pp1/3p4/4p2p/4P3/8/1PPPBPPP/RNB2RK1 w - h6 0 12").key(),
            Board::read_fen("Qnkr1bnr/2pq1pp1/B2p4/4p2p/4P3/8/1PPP1PPP/RNB2RK1 b - - 1 12").key(),
        ];
        assert_eq!(boards[0], board.key());

        for (idx, notation) in moves.iter().enumerate() {
            let mv = from_move_notation(&notation, &mut board);
            board.make_move(&mv);
            // println!("{:?}. {:?}", idx, board.key());
            assert_eq!(boards[idx + 1], board.key());
        }
    }
}
