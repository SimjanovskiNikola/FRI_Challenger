use std::cmp;

use super::make_move::BoardMoveTrait;
use super::structures::board;
use super::structures::board::Board;
use super::structures::castling::*;
use super::structures::color;
use super::structures::color::*;
use super::structures::moves::*;
use super::structures::piece;
use super::structures::piece::*;
use super::structures::square::get_rank;
use super::structures::square::SqPos::*;
use crate::engine::board::structures::state;
use crate::engine::evaluation::eval_defs::CLR_SQ;
use crate::engine::evaluation::eval_defs::PSQT;
use crate::engine::misc::bit_pos_utility::*;
use crate::engine::misc::bitboard::Bitboard;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::misc::bitboard::Iterator;
use crate::engine::misc::bitboard::Shift;
use crate::engine::misc::const_utility::*;
use crate::engine::move_generator::bishop::*;
use crate::engine::move_generator::generated::knight;
use crate::engine::move_generator::king::*;
use crate::engine::move_generator::knight::*;
use crate::engine::move_generator::pawn::*;
use crate::engine::move_generator::queen::*;
use crate::engine::move_generator::rook::*;
use crate::engine::search::transposition_table::TT;

const PV_MV_SCORE: isize = 95000;
const TT_MV_SCORE: isize = 80000;
const SEE_MV_SCORE: isize = 2000;
const KILLER_MV_SCORE: [isize; 2] = [2000, 1950];
const HIS_MV_SCORE: isize = 1000;

pub trait BoardGenMoveTrait {
    // Generating -> Move as a struct
    fn gen_moves(&mut self) -> Vec<(Move, isize)>;
    fn gen_captures(&mut self) -> Vec<(Move, isize)>;

    // Converting Bitboard squares to Move struct
    fn add_quiet_moves(&mut self, bb: u64, piece: Piece, sq: usize);
    fn add_capture_moves(&mut self, bb: u64, piece: Piece, sq: usize);
    fn add_castling_moves(&mut self);
    fn add_ep_moves(&mut self);
    fn add_capture_promo_moves(&mut self, from_sq: u8, to_sq: u8, piece: Piece);
    fn add_quiet_promo_moves(&mut self, from_sq: u8, to_sq: u8, piece: Piece);

    // Pawn Moves and Captures
    fn pawn_moves(&mut self);
    fn pawn_quiet_moves(&mut self);
    fn pawn_capture_moves(&mut self);

    // Other Piece Moves and Captures
    fn piece_cap_moves(&mut self, piece: Piece);
    fn piece_quiet_moves(&mut self, piece: Piece);
    fn piece_all_moves(&mut self, piece: Piece);

    // Move Generator for all pieces
    fn get_mv_bb(piece: Piece, sq: usize, own_occ: u64, enemy_occ: u64) -> u64;

    // Move Ordering
    fn quiet_eval(&mut self, mv: &Move) -> isize;
    fn capture_eval(&mut self, mv: &Move) -> isize;
    fn see(&mut self, from: usize, to: usize) -> isize;

    // Is square Attacked
    fn sq_attack(&self, sq: usize, color: Color) -> u64;
    fn sq_attack_with_occ(&self, sq: usize, color: Color, occ: u64) -> u64;

    // Is repetition & does the move exist for current position
    fn is_repetition(&self) -> bool;
    fn move_exists(&mut self, mv: &Move) -> bool;
}

impl BoardGenMoveTrait for Board {
    #[inline(always)]
    fn gen_moves(&mut self) -> Vec<(Move, isize)> {
        self.pawn_moves();

        for piece in &PIECES_WITHOUT_PAWN {
            self.piece_all_moves(piece + self.color());
        }

        self.add_castling_moves();
        self.add_ep_moves();

        // self.gen_moves.sort_unstable_by_key(|&(_, score)| -score);

        // for (i, score) in &self.gen_moves {
        //     println!("{:?} {:?}", i, score);
        // }

        self.gen_moves.drain(..).collect()
    }

    #[inline(always)]
    fn gen_captures(&mut self) -> Vec<(Move, isize)> {
        self.pawn_capture_moves();

        for piece in &PIECES_WITHOUT_PAWN {
            self.piece_cap_moves(piece + self.color());
        }

        self.add_ep_moves();

        self.gen_moves.drain(..).collect()
    }

    #[inline(always)]
    fn get_mv_bb(piece: Piece, sq: usize, own_occ: u64, enemy_occ: u64) -> u64 {
        match piece.kind() {
            PAWN => {
                get_pawn_mv(sq, own_occ, enemy_occ, piece.color())
                    | get_pawn_att(sq, own_occ, enemy_occ, piece.color())
            }
            KNIGHT => get_knight_mv(sq, own_occ, enemy_occ, piece.color()),
            BISHOP => get_bishop_mv(sq, own_occ, enemy_occ, piece.color()),
            ROOK => get_rook_mv(sq, own_occ, enemy_occ, piece.color()),
            QUEEN => get_queen_mv(sq, own_occ, enemy_occ, piece.color()),
            KING => get_king_mv(sq, own_occ, enemy_occ, piece.color()),
            _ => panic!("Invalid Peace Type"),
        }
    }

    fn piece_quiet_moves(&mut self, piece: Piece) {
        let (own_occ, enemy_occ) = self.both_occ_bb(self.color());
        let mut bb = self.bb(piece);
        while let Some(sq) = bb.next() {
            let moves = Board::get_mv_bb(piece, sq, own_occ, enemy_occ);
            let quiet_moves = moves & !enemy_occ;
            self.add_quiet_moves(quiet_moves, piece, sq);
        }
    }

    fn piece_cap_moves(&mut self, piece: Piece) {
        let (own_occ, enemy_occ) = self.both_occ_bb(self.color());
        let mut bb = self.bb(piece);
        while let Some(sq) = bb.next() {
            let moves = Board::get_mv_bb(piece, sq, own_occ, enemy_occ);
            let capture_moves = moves & enemy_occ;
            self.add_capture_moves(capture_moves, piece, sq);
        }
    }

    fn piece_all_moves(&mut self, piece: Piece) {
        let (own_occ, enemy_occ) = self.both_occ_bb(self.color());
        let mut bb = self.bb(piece);
        while let Some(sq) = bb.next() {
            let moves = Board::get_mv_bb(piece, sq, own_occ, enemy_occ);
            let quiet_moves = moves & !enemy_occ;
            let capture_moves = moves & enemy_occ;
            self.add_quiet_moves(quiet_moves, piece, sq);
            self.add_capture_moves(capture_moves, piece, sq);
        }
    }

    fn pawn_moves(&mut self) {
        self.pawn_capture_moves();
        self.pawn_quiet_moves();
    }

    fn pawn_quiet_moves(&mut self) {
        let (own_occ, enemy_occ) = self.both_occ_bb(self.color());
        let both_occ = own_occ | enemy_occ;
        let piece = PAWN + self.color();
        if self.color().is_white() {
            let mv = (self.pawn_bb(self.color()) << 8) & !both_occ;
            let mut one_mv = mv & !RANK_BITBOARD[7];
            let mut one_promo = mv & RANK_BITBOARD[7];
            let mut two_mv = ((one_mv & RANK_BITBOARD[2]) << 8) & !both_occ;

            while let Some(to_sq) = one_mv.next() {
                let mv = Move::init((to_sq - 8) as u8, to_sq as u8, piece, Flag::Quiet);
                let eval = self.quiet_eval(&mv);
                self.gen_moves.push((mv, eval));
            }

            while let Some(to_sq) = one_promo.next() {
                self.add_quiet_promo_moves((to_sq - 8) as u8, to_sq as u8, piece);
            }

            while let Some(to_sq) = two_mv.next() {
                let mv = Move::init((to_sq - 16) as u8, to_sq as u8, piece, Flag::Quiet);
                let eval = self.quiet_eval(&mv);
                self.gen_moves.push((mv, eval));
            }
        } else {
            let mv = (self.pawn_bb(self.color()) >> 8) & !both_occ;
            let mut one_mv = mv & !RANK_BITBOARD[0];
            let mut one_promo = mv & RANK_BITBOARD[0];
            let mut two_mv = ((one_mv & RANK_BITBOARD[5]) >> 8) & !both_occ;

            while let Some(to_sq) = one_mv.next() {
                let mv = Move::init((to_sq + 8) as u8, to_sq as u8, piece, Flag::Quiet);
                let eval = self.quiet_eval(&mv);
                self.gen_moves.push((mv, eval));
            }

            while let Some(to_sq) = one_promo.next() {
                self.add_quiet_promo_moves((to_sq + 8) as u8, to_sq as u8, piece);
            }

            while let Some(to_sq) = two_mv.next() {
                let mv = Move::init((to_sq + 16) as u8, to_sq as u8, piece, Flag::Quiet);
                let eval = self.quiet_eval(&mv);
                self.gen_moves.push((mv, eval));
            }
        }
    }

    fn pawn_capture_moves(&mut self) {
        let (own_occ, enemy_occ) = self.both_occ_bb(self.color());
        let piece = PAWN + self.color();

        if self.color().is_white() {
            let left = ((self.bb(piece) << 9) & !FILE_BITBOARD[0]) & enemy_occ;
            let mut left_att = left & !RANK_BITBOARD[7];
            let mut left_promo = left & RANK_BITBOARD[7];

            while let Some(to_sq) = left_att.next() {
                let mv = Move::init(
                    (to_sq - 9) as u8,
                    to_sq as u8,
                    piece,
                    Flag::Capture(self.piece_sq(to_sq)),
                );
                let eval = self.capture_eval(&mv);
                self.gen_moves.push((mv, eval));
            }

            while let Some(to_sq) = left_promo.next() {
                self.add_capture_promo_moves((to_sq - 9) as u8, to_sq as u8, piece);
            }

            let right = ((self.bb(piece) << 7) & !FILE_BITBOARD[7]) & enemy_occ;
            let mut right_att = right & !RANK_BITBOARD[7];
            let mut right_promo = right & RANK_BITBOARD[7];

            while let Some(to_sq) = right_att.next() {
                let mv = Move::init(
                    (to_sq - 7) as u8,
                    to_sq as u8,
                    piece,
                    Flag::Capture(self.piece_sq(to_sq)),
                );
                let eval = self.capture_eval(&mv);
                self.gen_moves.push((mv, eval));
            }

            while let Some(to_sq) = right_promo.next() {
                self.add_capture_promo_moves((to_sq - 7) as u8, to_sq as u8, piece);
            }
        } else {
            let left = ((self.bb(piece) >> 9) & !FILE_BITBOARD[7]) & enemy_occ;
            let mut left_att = left & !RANK_BITBOARD[0];
            let mut left_promo = left & RANK_BITBOARD[0];

            while let Some(to_sq) = left_att.next() {
                let mv = Move::init(
                    (to_sq + 9) as u8,
                    to_sq as u8,
                    piece,
                    Flag::Capture(self.piece_sq(to_sq)),
                );
                let eval = self.capture_eval(&mv);
                self.gen_moves.push((mv, eval));
            }

            while let Some(to_sq) = left_promo.next() {
                self.add_capture_promo_moves((to_sq + 9) as u8, to_sq as u8, piece);
            }

            let right = ((self.bb(piece) >> 7) & !FILE_BITBOARD[0]) & enemy_occ;
            let mut right_att = right & !RANK_BITBOARD[0];
            let mut right_promo = right & RANK_BITBOARD[0];

            while let Some(to_sq) = right_att.next() {
                let mv = Move::init(
                    (to_sq + 7) as u8,
                    to_sq as u8,
                    piece,
                    Flag::Capture(self.piece_sq(to_sq)),
                );
                let eval = self.capture_eval(&mv);
                self.gen_moves.push((mv, eval));
            }

            while let Some(to_sq) = right_promo.next() {
                self.add_capture_promo_moves((to_sq + 7) as u8, to_sq as u8, piece);
            }
        }
    }

    #[inline(always)]
    fn add_capture_moves(&mut self, mut bb: u64, piece: Piece, from_sq: usize) {
        while let Some(to_sq) = bb.next() {
            let flag = Flag::Capture(self.piece_sq(to_sq));
            let mv = Move::init(from_sq as u8, to_sq as u8, piece, flag);
            let eval = self.capture_eval(&mv);
            self.gen_moves.push((mv, eval));
        }
    }

    #[inline(always)]
    fn add_quiet_moves(&mut self, mut bb: u64, piece: Piece, from_sq: usize) {
        while let Some(to_sq) = bb.next() {
            let mv = Move::init(from_sq as u8, to_sq as u8, piece, Flag::Quiet);
            let eval = self.quiet_eval(&mv);
            self.gen_moves.push((mv, eval));
        }
    }

    fn add_ep_moves(&mut self) {
        if let Some(mv) = self.state.ep {
            let color = self.color().opp();
            let (own_occ, enemy_occ) = self.both_occ_bb(color);

            let mut attack =
                get_pawn_att(mv as usize, own_occ, enemy_occ, color) & self.pawn_bb(color.opp());

            while let Some(sq) = attack.next() {
                let mv = Move::init(sq as u8, mv, PAWN + color.opp(), Flag::EP);
                let eval = self.quiet_eval(&mv);
                self.gen_moves.push((mv, eval));
            }
        }
    }

    fn add_capture_promo_moves(&mut self, from_sq: u8, to_sq: u8, piece: Piece) {
        let taken_piece = self.piece_sq(to_sq as usize); // squares[to_sq as usize];
        for promo_piece in &PROMO_PIECES {
            let flag = Flag::Promotion(*promo_piece + piece.color(), Some(taken_piece));
            let mv = Move::init(from_sq, to_sq, piece, flag);
            let eval = self.capture_eval(&mv);
            self.gen_moves.push((mv, eval));
        }
    }

    fn add_quiet_promo_moves(&mut self, from_sq: u8, to_sq: u8, piece: Piece) {
        for promo_piece in &PROMO_PIECES {
            let flag = Flag::Promotion(*promo_piece + piece.color(), None);
            let mv = Move::init(from_sq, to_sq, piece, flag);
            let eval = self.quiet_eval(&mv); // promo_piece.weight();
            self.gen_moves.push((mv, eval));
        }
    }

    #[inline(always)]
    fn add_castling_moves(&mut self) {
        let (own, enemy) = self.both_occ_bb(self.color());
        let piece = KING + self.color();
        match self.color() {
            WHITE => {
                if self.state.castling.valid(CastlingRights::WKINGSIDE, self, own, enemy) {
                    let mv = Move::init(E1 as u8, G1 as u8, piece, Flag::KingCastle);
                    let eval = self.quiet_eval(&mv);
                    self.gen_moves.push((mv, eval));
                }
                if self.state.castling.valid(CastlingRights::WQUEENSIDE, self, own, enemy) {
                    let mv = Move::init(E1 as u8, C1 as u8, piece, Flag::QueenCastle);
                    let eval = self.quiet_eval(&mv);
                    self.gen_moves.push((mv, eval));
                }
            }
            BLACK => {
                if self.state.castling.valid(CastlingRights::BKINGSIDE, self, own, enemy) {
                    let mv = Move::init(E8 as u8, G8 as u8, piece, Flag::KingCastle);
                    let eval = self.quiet_eval(&mv);
                    self.gen_moves.push((mv, eval));
                }
                if self.state.castling.valid(CastlingRights::BQUEENSIDE, self, own, enemy) {
                    let mv = Move::init(E8 as u8, C8 as u8, piece, Flag::QueenCastle);
                    let eval = self.quiet_eval(&mv);
                    self.gen_moves.push((mv, eval));
                }
            }
            _ => panic!("Invalid Castling"),
        }
    }

    fn quiet_eval(&mut self, mv: &Move) -> isize {
        if matches!(self.pv_moves[0][self.ply()], Some(pv_mv) if *mv == pv_mv) {
            return PV_MV_SCORE;
        }

        if matches!(TT.read().unwrap().get(self.key()), Some(tt_mv) if *mv == tt_mv.mv) {
            return TT_MV_SCORE;
        }

        if matches!(self.s_killers[self.ply()][0], Some(x) if x == *mv) {
            return KILLER_MV_SCORE[0];
        } else if matches!(self.s_killers[self.ply()][1], Some(x) if x == *mv) {
            return KILLER_MV_SCORE[1];
        }

        let his_score = self.s_history[mv.piece.idx()][mv.to as usize] as isize;
        return his_score + HIS_MV_SCORE;
    }

    fn capture_eval(&mut self, mv: &Move) -> isize {
        if matches!(self.pv_moves[0][self.ply()], Some(pv_mv) if *mv == pv_mv) {
            return PV_MV_SCORE;
        }

        if matches!(TT.read().unwrap().get(self.key()), Some(tt_mv) if *mv == tt_mv.mv) {
            return TT_MV_SCORE;
        }

        let see = self.see(mv.from as usize, mv.to as usize);
        if see >= 0 {
            return see + SEE_MV_SCORE;
        } else {
            match mv.flag {
                Flag::Capture(cap) => cap.weight() - mv.piece as isize,
                _ => unreachable!("There is no flag capture"),
            }
        }
    }

    fn see(&mut self, from: usize, to: usize) -> isize {
        let mut occ = self.occ_bb(WHITE) | self.occ_bb(BLACK);
        let mut from_sq = from;
        let mut clr = self.color();
        // let mut clr = self.piece_sq(from).color();
        let mut gain = [0isize; 32];
        let mut depth = 0;

        let pce = self.piece_sq(to);
        // println!("Piece: {:?}", pce);
        gain[0] = pce.weight();

        // DEPRECATE:
        // let mut attackers = self.sq_attack(to, clr) | self.sq_attack(to, clr.opp());

        loop {
            depth += 1;
            clr.change_color();
            occ.clear_bit(from_sq);

            gain[depth] = self.piece_sq(from_sq).weight() - gain[depth - 1];

            let attacks = self.sq_attack_with_occ(to, clr.opp(), occ);

            let mut next_attacker = None;
            for &piece in &PIECES {
                let attackers: u64 = self.bb(piece + clr) & attacks & occ;
                // println!("Attackers: {:?}", piece);
                // print_bitboard(attackers, None);
                // print_bitboard(attacks, None);
                // print_bitboard(self.bb(piece + clr), None);
                // print_bitboard(occ, None);
                if attackers != 0 {
                    next_attacker = Some(attackers.get_lsb());
                    break;
                }
            }

            if next_attacker.is_none() {
                break;
            }
            from_sq = next_attacker.unwrap();
        }

        while {
            depth -= 1;
            depth > 0
        } {
            gain[depth - 1] = -cmp::max(-gain[depth - 1], gain[depth]);
        }

        // while depth > 0 {
        //     gain[depth - 1] = -cmp::max(-gain[depth - 1], gain[depth]);
        //     depth -= 1;
        // }

        gain[0]
    }

    #[inline(always)]
    fn sq_attack(&self, sq: usize, color: Color) -> u64 {
        let (own_occ, enemy_occ) = self.both_occ_bb(color);

        let op_pawns = self.bb(BLACK_PAWN - color);
        let op_knights = self.bb(BLACK_KNIGHT - color);
        let op_rq = self.bb(BLACK_QUEEN - color) | self.bb(BLACK_ROOK - color);
        let op_bq = self.bb(BLACK_QUEEN - color) | self.bb(BLACK_BISHOP - color);
        let op_king = self.bb(BLACK_KING - color);

        (get_pawn_att(sq, own_occ, enemy_occ, color) & op_pawns)
            | (get_knight_mv(sq, own_occ, enemy_occ, color) & op_knights)
            | (get_bishop_mv(sq, own_occ, enemy_occ, color) & op_bq)
            | (get_rook_mv(sq, own_occ, enemy_occ, color) & op_rq)
            | (get_king_mv(sq, own_occ, enemy_occ, color) & op_king)
    }

    #[inline(always)]
    fn sq_attack_with_occ(&self, sq: usize, color: Color, occ: u64) -> u64 {
        // let (own_occ, enemy_occ) = self.both_occ_bb(color);

        let op_pawns = self.bb(BLACK_PAWN - color);
        let op_knights = self.bb(BLACK_KNIGHT - color);
        let op_rq = self.bb(BLACK_QUEEN - color) | self.bb(BLACK_ROOK - color);
        let op_bq = self.bb(BLACK_QUEEN - color) | self.bb(BLACK_BISHOP - color);
        let op_king = self.bb(BLACK_KING - color);

        (get_pawn_att(sq, 0, occ, color) & op_pawns)
            | (get_knight_mv(sq, 0, occ, color) & op_knights)
            | (get_bishop_mv(sq, 0, occ, color) & op_bq)
            | (get_rook_mv(sq, 0, occ, color) & op_rq)
            | (get_king_mv(sq, 0, occ, color) & op_king)
    }

    #[inline(always)]
    fn is_repetition(&self) -> bool {
        let mut threefold = 2;
        let his_len = self.history.len();
        let half_move = self.half_move();
        let start = his_len.abs_diff(half_move as usize);
        let end = his_len.max(0);
        for i in start..end {
            if self.history[i].key == self.key() {
                threefold -= 1;
            }
        }

        threefold <= 0
    }

    #[inline(always)]
    fn move_exists(&mut self, mv: &Move) -> bool {
        let mut moves = self.gen_moves();

        for temp_mv in &mut moves {
            if *mv == temp_mv.0 {
                if self.make_move(mv) {
                    self.undo_move();
                    return true;
                }
            }
        }
        return false;
    }
}

pub fn next_move(moves: &mut Vec<(Move, isize)>) -> Option<Move> {
    if moves.len() == 0 {
        return None;
    }

    for idx in 0..(moves.len() - 1) {
        if moves[idx].1 > moves[idx + 1].1 {
            moves.swap(idx, idx + 1);
        }
    }
    moves.pop().map(|(mv, _)| mv)
}

#[cfg(test)]
mod tests {

    use core::panic;

    use crate::engine::board::fen::FenTrait;
    use crate::engine::board::structures::zobrist::ZobristKeysTrait;
    use crate::engine::misc::display::display_board::*;
    use crate::engine::misc::display::display_moves::*;

    use super::*;

    fn test_mov_att(fen: &str, piece: Piece, idx: usize) -> Vec<usize> {
        let board = Board::read_fen(&fen);
        // println!("{}", game.to_string());
        let (own_occ, enemy_occ) = board.both_occ_bb(board.color()); // get_occupancy(&piece, &board);
        let all_pieces = extract_all_bits(board.bitboard[piece.idx()]);
        let piece = match board.squares[all_pieces[idx]] {
            None => panic!("The Piece Must exist"),
            Some(piece) => piece,
        };
        return extract_all_bits(Board::get_mv_bb(piece, all_pieces[idx], own_occ, enemy_occ));

        // print_bitboard(
        //     generate_knight_moves(&piece, &board),
        //     Some(bit_scan_lsb(piece.position) as i8),
        // );
    }

    #[test]
    fn test_white_pawns_mv_gen() {
        let mut board = Board::read_fen(&FEN_PAWNS_WHITE);
        let moves = board.gen_moves();
        print_move_list(&moves);
        assert_eq!(42, moves.len());
    }

    #[test]
    fn test_mv_gen() {
        let mut board = Board::read_fen(&FEN_CASTLE_TWO);
        let moves = board.gen_moves();
        print_chess(&board);
        print_move_list(&moves);
        assert_eq!(48, moves.len());
    }

    #[test]
    fn test_white_black_mv_gen() {
        let mut board = Board::read_fen(&FEN_PAWNS_BLACK);
        let moves = board.gen_moves();
        assert_eq!(42, moves.len());
        print_move_list(&moves);
    }

    #[test]
    fn test_attacks() {
        let fen = "8/8/2q5/3Q4/8/8/8/8 w - - 0 1";
        let board = Board::read_fen(&fen);
        print_bitboard(
            board.sq_attack(board.bitboard[BLACK_QUEEN.idx()].get_lsb(), BLACK),
            Some(board.bitboard[BLACK_QUEEN.idx()].get_msb() as i8),
        );
        print_bitboard(board.sq_attack(board.bitboard[WHITE_QUEEN.idx()].get_msb(), WHITE), None);
    }

    // KNIGHT

    #[test]
    fn test_generate_knight_moves() {
        let fen = "8/8/8/4N3/2N5/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen, WHITE_KNIGHT, 1);
        let test_positions = vec![19, 21, 30, 42, 46, 51, 53];
        assert_eq!(test_positions, moves);
    }

    #[test]
    fn test_generate_3_knight_moves() {
        let fen_knight = "8/5N2/8/4N3/2N5/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_knight, WHITE_KNIGHT, 1);
        let test_positions = vec![19, 21, 30, 42, 46, 51];
        assert_eq!(test_positions, moves);
    }

    // BISHOP

    #[test]
    fn test_generate_bishop_moves_1_bishop() {
        let fen_one_bishop = "7B/8/8/8/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_one_bishop, WHITE_BISHOP, 0);
        let test_positions = vec![0, 9, 18, 27, 36, 45, 54];
        assert_eq!(test_positions, moves);
    }

    #[test]
    fn test_generate_bishop_moves_1_bishop_middle() {
        let fen_one_bishop = "8/8/8/4B3/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_one_bishop, WHITE_BISHOP, 0);

        let test_positions = vec![0, 9, 15, 18, 22, 27, 29, 43, 45, 50, 54, 57, 63];
        assert_eq!(test_positions, moves);
    }

    #[test]
    fn test_generate_bishop_moves_2_bishop() {
        let fen_one_bishop = "8/8/5B2/4B3/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_one_bishop, WHITE_BISHOP, 0);

        let test_positions = vec![0, 9, 15, 18, 22, 27, 29, 43, 50, 57];
        assert_eq!(test_positions, moves);
    }

    // ROOK
    #[test]
    fn test_generate_rook_moves_1_rook() {
        let fen_rook = "8/8/8/8/8/4R3/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_rook, WHITE_ROOK, 0);
        let test_positions = vec![4, 12, 16, 17, 18, 19, 21, 22, 23, 28, 36, 44, 52, 60];
        assert_eq!(test_positions, moves);
    }

    #[test]
    fn test_generate_rook_moves_1_rook_1enemy() {
        let fen_rook = "8/8/8/4r3/8/4R3/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_rook, WHITE_ROOK, 0);
        let test_positions = vec![4, 12, 16, 17, 18, 19, 21, 22, 23, 28, 36];
        assert_eq!(test_positions, moves);
    }

    // QUEEN

    #[test]
    fn test_generate_queen_moves() {
        let fen_queen = "8/8/8/3Q4/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_queen, WHITE_QUEEN, 0);
        let test_positions = vec![
            D1 as usize,
            H1 as usize,
            A2 as usize,
            D2 as usize,
            G2 as usize,
            B3 as usize,
            D3 as usize,
            F3 as usize,
            C4 as usize,
            D4 as usize,
            E4 as usize,
            A5 as usize,
            B5 as usize,
            C5 as usize,
            E5 as usize,
            F5 as usize,
            G5 as usize,
            H5 as usize,
            C6 as usize,
            D6 as usize,
            E6 as usize,
            B7 as usize,
            D7 as usize,
            F7 as usize,
            A8 as usize,
            D8 as usize,
            G8 as usize,
        ];
        assert_eq!(test_positions, moves);
    }

    #[test]
    fn test_generate_queen_moves_one_enemy() {
        let fen_queen = "8/8/2q5/3Q4/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_queen, WHITE_QUEEN, 0);

        let test_positions = vec![
            D1 as usize,
            H1 as usize,
            A2 as usize,
            D2 as usize,
            G2 as usize,
            B3 as usize,
            D3 as usize,
            F3 as usize,
            C4 as usize,
            D4 as usize,
            E4 as usize,
            A5 as usize,
            B5 as usize,
            C5 as usize,
            E5 as usize,
            F5 as usize,
            G5 as usize,
            H5 as usize,
            C6 as usize,
            D6 as usize,
            E6 as usize,
            D7 as usize,
            F7 as usize,
            D8 as usize,
            G8 as usize,
        ];
        assert_eq!(test_positions, moves);
    }

    // KING

    #[test]
    fn test_generate_king_moves() {
        let fen_king = "8/8/8/3K4/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_king, WHITE_KING, 0);
        let test_positions = vec![26, 27, 28, 34, 36, 42, 43, 44];
        assert_eq!(test_positions, moves);
    }

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

    #[test]
    fn test_is_repetition() {
        let mut board = Board::read_fen(&FEN_START);
        let moves = ["b1c3", "b8c6", "c3b1", "c6b8"];

        for i in moves.iter() {
            let mv = from_move_notation(i, &mut board);
            board.make_move(&mv);
        }

        for i in board.history.iter() {
            println!("History: {:?}", i.key);
        }

        println!("Curr Key: {:?}", board.key());

        assert_eq!(board.is_repetition(), true);
    }

    #[test]
    fn test_is_repetition_v2() {
        let mut board = Board::read_fen(&FEN_START);
        let moves = ["e2e4", "e7e5", "b1c3", "b8c6", "c3b1", "c6b8"];

        println!("{:?}", board.key());

        for (idx, notation) in moves.iter().enumerate() {
            let mv = from_move_notation(&notation, &mut board);
            board.make_move(&mv);
            println!("{:?}", board.key());
        }

        assert_eq!(board.is_repetition(), false);
    }

    #[test]
    fn test_is_repetition_v4() {
        let mut board = Board::read_fen(&FEN_START);
        let moves = [
            "e2e4", "e7e5", "b1c3", "b8c6", "c3b1", "c6b8", "b1c3", "b8c6", "c3b1", "c6b8", "b1c3",
        ];

        println!("{:?}", board.key());

        for (idx, notation) in moves.iter().enumerate() {
            let mv = from_move_notation(&notation, &mut board);
            board.make_move(&mv);
            println!("{:?}", board.key());
        }

        assert_eq!(board.is_repetition(), true);
    }

    #[test]
    #[rustfmt::skip]
    fn test_is_repetition_v3() {
        let mut board = Board::read_fen(&FEN_START);
        let moves = ["e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "d7d6", "e1g1", 
                    "c8g4", "a2a4", "d8d7", "a4a5", "e8c8", "d1e2", "b7b5", "a5b6", 
                    "g4f3", "b6a7", "f3e2", "a7a8Q", "c6b8", "c4e2", "h7h5", "e2a6"  ];
       
      

        for (idx, notation) in moves.iter().enumerate() {
            assert_eq!(board.is_repetition(), false);
            let mv = from_move_notation(&notation, &mut board);
            board.make_move(&mv);
        }
    }

    #[test]
    fn test_mv_gen_v2() {
        let mut board = Board::read_fen(&FEN_START);
        let moves = [
            "e2e4", "e7e5", "g1f3", "b8c6", "f1b5", "a7a6", "b5a4", "g8f6", "d2d3", "d7d6", "c2c3",
            "g7g6", "c1e3", "c8d7", "b1d2", "f8e7", "e1g1", "e8g8", "h2h3", "f8e8", "a4b3", "d8c8",
            "d3d4", "e5d4", "c3d4", "c6b4", "a2a3", "b4c6", "e4e5", "d6e5", "d4e5", "f6h5", "d2e4",
            "h5g7", "e4f6", "e7f6", "e5f6", "g7f5", "f3g5", "c6d8", "d1d2", "f5e3", "f2e3", "d8e6",
            "g5f3", "d7c6", "f3e5", "c6e4", "a1c1", "e8d8", "d2b4", "e4d5", "b3c2", "c7c5", "b4e1",
            "c8c7", "e1c3", "e6g5", "c1d1", "b7b6", "h3h4", "g5e4", "c2e4", "d5e4", "d1d4", "c7b7",
            "d4d2", "d8d2", "c3d2", "b7c7", "e5g4", "a8d8", "g4h6", "g8f8", "d2e1", "f8e8", "f1f4",
            "c7c6", "e1e2", "c6b7", "g1h2", "e4d3", "e2g4", "b7d5", "f4f2", "d3e4", "a3a4", "d5d6",
            "f2f4", "e4c6", "a4a5", "b6a5", "g4g5", "c6a8", "h6f7", "e8f7", "g5h6", "f7e6", "h6h7",
            "a8e4", "h7e7", "d6e7", "f6e7", "e6e7", "f4e4", "e7f6", "e4c4", "d8c8", "g2g4", "f6e6",
            "h2g3", "e6e5", "h4h5", "e5d5", "c4a4", "g6h5", "g4h5", "c8g8", "g3f4", "g8f8", "f4g5",
            "f8b8", "h5h6", "b8b2", "h6h7", "b2b8", "a4h4", "b8h8", "g5f6", "c5c4", "f6g7", "h8c8",
            "e3e4", "d5e5", "h7h8Q", "c8h8", "h4h8", "e5e4", "h8h5", "c4c3", "h5h3", "e4d4",
            "h3h4", "d4d3", "h4h8", "c3c2", "h8d8", "d3c3", "d8c8", "c3d2", "c8d8", "d2c3",
        ];

        for (idx, notation) in moves.iter().enumerate() {
            let mv = from_move_notation(&notation, &mut board);
            board.make_move(&mv);
            board.moves.clear();
        }

        println!("{:?}", board.ply());

        let moves = board.gen_moves();
        println!("{:?}", moves);
    }
}
