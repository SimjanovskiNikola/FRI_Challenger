use std::cmp;

use crate::engine::board::board::Board;
use crate::engine::board::color::*;
use crate::engine::board::moves::*;
use crate::engine::board::piece::*;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::move_generator::mv_gen::BoardGenMoveTrait;

const PV_MV_SCORE: isize = 95000;
const TT_MV_SCORE: isize = 80000;
const SEE_MV_SCORE: isize = 2000;
const KILLER_MV_SCORE: [isize; 2] = [2000, 1950];
const HIS_MV_SCORE: isize = 1000;

pub trait MoveOrderingTrait {
    fn next_move(&mut self, moves: &mut Vec<(Move, isize)>) -> Option<Move>;
    fn score_moves(&mut self, moves: &mut Vec<(Move, isize)>);
    fn quiet_eval(&mut self, mv: &Move) -> isize;
    fn capture_eval(&mut self, mv: &Move) -> isize;
    fn see(&mut self, from: usize, to: usize) -> isize;
}

impl MoveOrderingTrait for Board {
    fn score_moves(&mut self, moves: &mut Vec<(Move, isize)>) {
        let pv_mv = self.pv_moves[0][self.ply()];
        // let tt_mv = self.tt.read().unwrap().get(self.key());
        for (mv, score) in moves.iter_mut() {
            if pv_mv == Some(*mv) {
                *score = PV_MV_SCORE;
            } else if matches!(self.tt.get(self.key()), Some(tt_move) if *mv == tt_move.mv) {
                *score = TT_MV_SCORE;
            }

            match mv.flag {
                Flag::Capture(_) | Flag::Promotion(_, Some(_)) => {
                    *score = self.capture_eval(mv);
                }
                Flag::EP => {
                    *score = 100;
                }
                Flag::NullMove => {
                    panic!("There should be no null move in the move list");
                }
                _ => {
                    *score = self.quiet_eval(mv);
                }
            }
        }
    }

    fn next_move(&mut self, moves: &mut Vec<(Move, isize)>) -> Option<Move> {
        if moves.len() == 0 {
            return None;
        }

        let best_idx =
            moves.iter().enumerate().max_by_key(|(_, (_, score))| score).map(|(idx, _)| idx)?;

        Some(moves.swap_remove(best_idx).0)
    }

    fn quiet_eval(&mut self, mv: &Move) -> isize {
        if Some(*mv) == self.s_killers[self.ply()][0] {
            return KILLER_MV_SCORE[0];
        } else if Some(*mv) == self.s_killers[self.ply()][1] {
            return KILLER_MV_SCORE[1];
        }

        return self.s_history[mv.piece.idx()][mv.to as usize] + HIS_MV_SCORE;
    }

    fn capture_eval(&mut self, mv: &Move) -> isize {
        assert!(self.squares[mv.to as usize] != 0, "There is no piece in the to square");

        self.see(mv.from as usize, mv.to as usize) + SEE_MV_SCORE
    }

    fn see(&mut self, mut from: usize, to: usize) -> isize {
        let mut occ = self.occ_bb(WHITE) | self.occ_bb(BLACK);
        let mut clr = self.color();
        let mut gain = [0isize; 32];
        let mut depth = 0;

        let pce = self.piece_sq(to);
        gain[0] = pce.weight();

        loop {
            depth += 1;
            clr.change_color();
            occ.clear_bit(from);

            gain[depth] = self.piece_sq(from).weight() - gain[depth - 1];

            let attacks = self.sq_attack_with_occ(to, clr.opp(), occ);

            let mut next_attacker = None;
            for &piece in &PIECES {
                let attackers: u64 = self.bb(piece + clr) & attacks & occ;
                if attackers != 0 {
                    next_attacker = Some(attackers.get_lsb());
                    break;
                }
            }

            if next_attacker.is_none() {
                break;
            }
            from = next_attacker.unwrap();
        }

        while {
            depth -= 1;
            depth > 0
        } {
            gain[depth - 1] = -cmp::max(-gain[depth - 1], gain[depth]);
        }

        gain[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::board::board::Board;
    use crate::engine::board::fen::FenTrait;
    use crate::engine::misc::display::display_board::print_chess;
    use crate::engine::move_generator::mv_oredering::MoveOrderingTrait;

    #[test]
    fn test_see_pos_1() {
        let fen = "1k1r4/1pp4p/p7/4p3/8/P5P1/1PP4P/2K1R3 w - - 0 1";
        let mut board = Board::read_fen(&fen);
        print_chess(&board);
        let see = board.see(4, 36);
        assert_eq!(see, 100);
    }

    #[test]
    fn test_see_pos_2() {
        let fen = "1k1r3q/1ppn3p/p4b2/4p3/8/P2N2P1/1PP1R1BP/2K1Q3 w - - 0 1";
        let mut board = Board::read_fen(&fen);
        print_chess(&board);
        let see = board.see(19, 36);
        assert_eq!(see, -225);
    }

    #[test]
    #[should_panic]
    fn test_see_pos_3() {
        let fen = "8/8/8/3b4/3B4/8/8/8 w - - 0 1";
        let mut board = Board::read_fen(&fen);
        print_chess(&board);
        let see = board.see(27, 36);
    }

    #[test]
    fn test_see_pos_4() {
        let fen = "2r4k/2r4p/p7/2b2p1b/4pP2/1BR5/P1R3PP/2Q4K w - - 0 1";
        let mut board = Board::read_fen(&fen);
        print_chess(&board);
        let see = board.see(18, 34);
        assert_eq!(see, 350);
    }

    #[test]
    fn test_see_pos_5() {
        let fen = "4q3/1p1pr1kb/1B2rp2/6p1/p3PP2/P3R1P1/1P2R1K1/4Q3 b - - 0 1";
        let mut board = Board::read_fen(&fen);
        print_chess(&board);
        let see = board.see(55, 28);
        assert_eq!(see, 100);
    }

    #[test]
    fn test_see_pos_6() {
        let fen = "2r2r1k/6bp/p7/2q2p1Q/3PpP2/1B6/P5PP/2RR3K b - - 0 1";
        let mut board = Board::read_fen(&fen);
        print_chess(&board);
        let see = board.see(34, 2);
        assert_eq!(see, 100);
    }

    #[test]
    fn test_see_pos_7() {
        let fen = "4R3/2r3p1/5bk1/1p1r3p/p2PR1P1/P1BK1P2/1P6/8 b - - 0 1";
        let mut board = Board::read_fen(&fen);
        print_chess(&board);
        let see = board.see(39, 30);
        assert_eq!(see, 0);
    }
}
