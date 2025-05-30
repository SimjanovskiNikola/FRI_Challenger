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
use crate::engine::misc::bit_pos_utility::*;
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

pub trait BoardGenMoveTrait {
    // Generating -> Move as a struct
    fn gen_moves(&mut self) -> Vec<Move>;
    fn gen_captures(&mut self) -> Vec<Move>;

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
    fn quiet_eval(&self, mv: &Move) -> isize;
    fn capture_eval(&self, mv: &Move) -> isize;

    // Is square Attacked
    fn sq_attack(&self, sq: usize, color: Color) -> u64;

    // Is repetition & does the move exist for current position
    fn is_repetition(&self) -> bool;
    fn move_exists(&mut self, mv: &Move) -> bool;
}

impl BoardGenMoveTrait for Board {
    #[inline(always)]
    fn gen_moves(&mut self) -> Vec<Move> {
        self.pawn_moves();

        for piece in &PIECES_WITHOUT_PAWN {
            self.piece_all_moves(piece + self.color());
        }

        self.add_castling_moves();
        self.add_ep_moves();

        self.gen_moves.sort_unstable_by_key(|&(_, score)| -score);
        self.gen_moves.drain(..).map(|(mv, _)| mv).collect()
    }

    #[inline(always)]
    fn gen_captures(&mut self) -> Vec<Move> {
        self.pawn_capture_moves();

        for piece in &PIECES_WITHOUT_PAWN {
            self.piece_cap_moves(piece + self.color());
        }

        self.add_ep_moves();

        self.gen_moves.sort_unstable_by_key(|&(_, score)| -score);
        self.gen_moves.drain(..).map(|(mv, _)| mv).collect()
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
                self.gen_moves
                    .push((Move::init((to_sq - 8) as u8, to_sq as u8, piece, Flag::Quiet), 0));
            }

            while let Some(to_sq) = one_promo.next() {
                self.add_quiet_promo_moves((to_sq - 8) as u8, to_sq as u8, piece);
            }

            while let Some(to_sq) = two_mv.next() {
                self.gen_moves
                    .push((Move::init((to_sq - 16) as u8, to_sq as u8, piece, Flag::Quiet), 0));
            }
        } else {
            let mv = (self.pawn_bb(self.color()) >> 8) & !both_occ;
            let mut one_mv = mv & !RANK_BITBOARD[0];
            let mut one_promo = mv & RANK_BITBOARD[0];
            let mut two_mv = ((one_mv & RANK_BITBOARD[5]) >> 8) & !both_occ;

            while let Some(to_sq) = one_mv.next() {
                self.gen_moves
                    .push((Move::init((to_sq + 8) as u8, to_sq as u8, piece, Flag::Quiet), 0));
            }

            while let Some(to_sq) = one_promo.next() {
                self.add_quiet_promo_moves((to_sq + 8) as u8, to_sq as u8, piece);
            }

            while let Some(to_sq) = two_mv.next() {
                self.gen_moves
                    .push((Move::init((to_sq + 16) as u8, to_sq as u8, piece, Flag::Quiet), 0));
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
                self.gen_moves.push((
                    Move::init(
                        (to_sq - 9) as u8,
                        to_sq as u8,
                        piece,
                        Flag::Capture(self.cap_piece(to_sq)),
                    ),
                    0,
                ));
            }

            while let Some(to_sq) = left_promo.next() {
                self.add_capture_promo_moves((to_sq - 9) as u8, to_sq as u8, piece);
            }

            let right = ((self.bb(piece) << 7) & !FILE_BITBOARD[7]) & enemy_occ;
            let mut right_att = right & !RANK_BITBOARD[7];
            let mut right_promo = right & RANK_BITBOARD[7];

            while let Some(to_sq) = right_att.next() {
                self.gen_moves.push((
                    Move::init(
                        (to_sq - 7) as u8,
                        to_sq as u8,
                        piece,
                        Flag::Capture(self.cap_piece(to_sq)),
                    ),
                    0,
                ));
            }

            while let Some(to_sq) = right_promo.next() {
                self.add_capture_promo_moves((to_sq - 7) as u8, to_sq as u8, piece);
            }
        } else {
            let left = ((self.bb(piece) >> 9) & !FILE_BITBOARD[7]) & enemy_occ;
            let mut left_att = left & !RANK_BITBOARD[0];
            let mut left_promo = left & RANK_BITBOARD[0];

            while let Some(to_sq) = left_att.next() {
                self.gen_moves.push((
                    Move::init(
                        (to_sq + 9) as u8,
                        to_sq as u8,
                        piece,
                        Flag::Capture(self.cap_piece(to_sq)),
                    ),
                    0,
                ));
            }

            while let Some(to_sq) = left_promo.next() {
                self.add_capture_promo_moves((to_sq + 9) as u8, to_sq as u8, piece);
            }

            let right = ((self.bb(piece) >> 7) & !FILE_BITBOARD[0]) & enemy_occ;
            let mut right_att = right & !RANK_BITBOARD[0];
            let mut right_promo = right & RANK_BITBOARD[0];

            while let Some(to_sq) = right_att.next() {
                self.gen_moves.push((
                    Move::init(
                        (to_sq + 7) as u8,
                        to_sq as u8,
                        piece,
                        Flag::Capture(self.cap_piece(to_sq)),
                    ),
                    0,
                ));
            }

            while let Some(to_sq) = right_promo.next() {
                self.add_capture_promo_moves((to_sq + 7) as u8, to_sq as u8, piece);
            }
        }
    }

    #[inline(always)]
    fn add_capture_moves(&mut self, mut bb: u64, piece: Piece, from_sq: usize) {
        while let Some(to_sq) = bb.next() {
            let flag = Flag::Capture(self.cap_piece(to_sq));
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
                self.gen_moves.push((Move::init(sq as u8, mv, PAWN + color.opp(), Flag::EP), 100));
            }
        }
    }

    fn add_capture_promo_moves(&mut self, from_sq: u8, to_sq: u8, piece: Piece) {
        let taken_piece = self.cap_piece(to_sq as usize); // squares[to_sq as usize];
        for promo_piece in &PROMO_PIECES {
            let eval = taken_piece.weight() - piece as isize + promo_piece.weight();
            let flag = Flag::Promotion(*promo_piece + piece.color(), Some(taken_piece));
            self.gen_moves.push((Move::init(from_sq, to_sq, piece, flag), eval));
        }
    }

    fn add_quiet_promo_moves(&mut self, from_sq: u8, to_sq: u8, piece: Piece) {
        for promo_piece in &PROMO_PIECES {
            let eval = promo_piece.weight();
            let flag = Flag::Promotion(*promo_piece + piece.color(), None);
            self.gen_moves.push((Move::init(from_sq, to_sq, piece, flag), eval));
        }
    }

    #[inline(always)]
    fn add_castling_moves(&mut self) {
        let (own, enemy) = self.both_occ_bb(self.color());
        let piece = KING + self.color();
        match self.color() {
            WHITE => {
                if self.state.castling.valid(CastlingRights::WKINGSIDE, self, own, enemy) {
                    self.gen_moves
                        .push((Move::init(E1 as u8, G1 as u8, piece, Flag::KingCastle), 0));
                }
                if self.state.castling.valid(CastlingRights::WQUEENSIDE, self, own, enemy) {
                    self.gen_moves
                        .push((Move::init(E1 as u8, C1 as u8, piece, Flag::QueenCastle), 0));
                }
            }
            BLACK => {
                if self.state.castling.valid(CastlingRights::BKINGSIDE, self, own, enemy) {
                    self.gen_moves
                        .push((Move::init(E8 as u8, G8 as u8, piece, Flag::KingCastle), 0));
                }
                if self.state.castling.valid(CastlingRights::BQUEENSIDE, self, own, enemy) {
                    self.gen_moves
                        .push((Move::init(E8 as u8, C8 as u8, piece, Flag::QueenCastle), 0));
                }
            }
            _ => panic!("Invalid Castling"),
        }
    }

    fn quiet_eval(&self, mv: &Move) -> isize {
        if matches!(self.tt_mv, Some(x) if x.mv == *mv) {
            return 95000;
        }

        if matches!(self.s_killers[self.ply()][0], Some(x) if x == *mv) {
            90000
        } else if matches!(self.s_killers[self.ply()][1], Some(x) if x == *mv) {
            80000
        } else {
            self.s_history[mv.piece.idx()][mv.to as usize] as isize
        }
    }

    fn capture_eval(&self, mv: &Move) -> isize {
        if matches!(self.tt_mv, Some(x) if x.mv == *mv) {
            return 95000;
        }

        match mv.flag {
            Flag::Capture(cap) => cap.weight() - mv.piece as isize,
            _ => unreachable!("There is no flag capture"),
        }
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
    fn is_repetition(&self) -> bool {
        let his_len = self.history.len();
        let half_move = self.half_move();
        assert!(his_len >= half_move as usize, "It is Negative {:?} {:?}", his_len, half_move);
        for i in (his_len - half_move as usize)..his_len {
            if self.history[i].key == self.key() {
                return true;
            }
        }

        false
    }

    #[inline(always)]
    fn move_exists(&mut self, mv: &Move) -> bool {
        let mut moves = self.gen_moves();

        for temp_mv in &mut moves {
            if mv == temp_mv {
                if self.make_move(mv) {
                    self.undo_move();
                    return true;
                }
            }
        }
        return false;
    }
}

// NOTE: DO NOT REMOVE THIS !!!!
// fn eval_pos(mv: &Move, board: &Board) -> isize {
// FIXME: I will need to update this based on pv table that should be located inside the game
// if matches!(game.tt.get(game.key), Some(x) if x.rev == *pos) {
//     return 95000;
// }

//     match mv.flag {
//         Flag::Quiet => {
//             if matches!(board.s_killers[board.ply()][0], Some(x) if x == *mv) {
//                 90000
//             } else if matches!(board.s_killers[board.ply()][1], Some(x) if x == *mv) {
//                 80000
//             } else {
//                 board.s_history[mv.piece.idx()][mv.to as usize] as isize
//             }
//         }
//         Flag::KingCastle => 20,
//         Flag::QueenCastle => 20,
//         Flag::Capture(cap) => cap.weight() - mv.piece as isize,
//         Flag::EP => PAWN.weight(),
//         Flag::Promotion(promo, Some(cap)) => cap.weight() - mv.piece as isize + promo.weight(),
//         Flag::Promotion(promo, None) => promo.weight(),
//     }
// }

//

#[cfg(test)]
mod tests {

    use crate::engine::board::fen::FenTrait;
    use crate::engine::misc::print_utility::print_bitboard;
    use crate::engine::misc::print_utility::print_chess;
    use crate::engine::misc::print_utility::print_move_list;

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
}
