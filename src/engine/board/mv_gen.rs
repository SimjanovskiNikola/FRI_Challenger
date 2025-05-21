use super::make_move::BoardMoveTrait;
use super::structures::board::Board;
use super::structures::castling::*;
use super::structures::color;
use super::structures::color::*;
use super::structures::moves::*;
use super::structures::piece::*;
use super::structures::square::get_rank;
use super::structures::square::SqPos::*;
use crate::engine::misc::bit_pos_utility::*;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::misc::bitboard::Iterator;
use crate::engine::misc::bitboard::Shift;
use crate::engine::misc::const_utility::*;
use crate::engine::move_generator::bishop::*;
use crate::engine::move_generator::king::*;
use crate::engine::move_generator::knight::*;
use crate::engine::move_generator::pawn::*;
use crate::engine::move_generator::queen::*;
use crate::engine::move_generator::rook::*;

// pub trait BoardGenMoveTrait {
//     fn gen_moves(&self) -> Vec<Move>;
//     fn gen_captures(&self) -> Vec<Move>;
//     fn add_piece_moves(&self, moves: &mut Vec<Move>, piece: Piece);
//     fn add_piece_captures(&self, moves: &mut Vec<Move>, piece: Piece);

//     fn get_piece_mv_bb(&self, piece: Piece, pos: usize) -> u64;

//     fn add_basic_moves(&self, moves: &mut Vec<Move>, bb: u64, piece: Piece, from_sq: usize);
//     fn add_ep_moves();
//     fn add_promo_moves();
//     fn add_castling_moves();
// }

// impl BoardGenMoveTrait for Board {
//     fn gen_moves(&self) -> Vec<Move> {
//         let mut captures: Vec<(Move, isize)> = Vec::with_capacity(256);
//         let mut moves: Vec<Move> = Vec::with_capacity(256);

//         let mut queen_bb = self.queen_bb(self.color());
//         while let Some(sq) = queen_bb.next() {
//             let bb = get_queen_mv(pos, own, enemy);
//             let eval = self.eval_pos();
//             captures.push(value);
//         }

//         self.add_piece_moves(&mut captures, &mut moves, QUEEN + self.color());
//         self.add_piece_moves(&mut moves, QUEEN + self.color());
//         self.add_piece_moves(&mut moves, QUEEN + self.color());
//         self.add_piece_moves(&mut moves, QUEEN + self.color());
//         self.add_piece_moves(&mut moves, QUEEN + self.color());
//         self.add_pawn_moves(&mut moves, QUEEN + self.color());

//         self.add_castling_moves();
//         self.add_ep_moves();
//         self.add_promo_moves();
//         add_castling_moves(&(KING + color), board, &mut positions_rev);
//         // add_new_ep_move(board, &mut positions_rev);

//         positions_rev.sort_unstable_by(|a, b| eval_pos(b, &board).cmp(&eval_pos(a, &board)));
//         positions_rev
//     }

//     fn gen_captures(&self) -> Vec<Move> {
//         todo!()
//     }

//     fn add_piece_moves(&self, moves: &mut Vec<Move>, piece: Piece) {
//         let mut bb = self.bb(piece);
//         while let Some(from_sq) = bb.next() {
//             let bb = self.get_piece_mv_bb(piece, from_sq);
//             self.add_moves(moves, bb, piece, from_sq);
//         }
//     }

//     fn add_piece_captures(&self, moves: &Vec<Move>, piece: Piece) {
//         todo!()
//     }

//     fn get_piece_mv_bb(&self, piece: Piece, pos: usize) -> u64 {
//         let (own, enemy) = self.both_occ_bb(self.color());
//         match piece.kind() {
//             PAWN => {
//                 get_pawn_mv(piece.color(), pos, own, enemy)
//                     | get_pawn_att(piece.color(), pos, own, enemy, self.ep())
//             }
//             KNIGHT => get_knight_mv(pos, own, enemy),
//             BISHOP => get_bishop_mv(pos, own, enemy),
//             ROOK => get_rook_mv(pos, own, enemy),
//             QUEEN => get_queen_mv(pos, own, enemy),
//             KING => get_king_mv(pos, own, enemy),
//             _ => panic!("Invalid Peace Type"),
//         }
//     }

//     fn add_moves(&self, moves: &mut Vec<Move>, mut bb: u64, piece: Piece, from_sq: usize) {
//         while let Some(to_sq) = bb.next() {
//             let flag = match self.squares[to_sq] {
//                 None => Flag::Quiet,
//                 Some(piece) => Flag::Capture(piece),
//             };
//             moves.push(Move::init(from_sq as u8, to_sq as u8, piece, flag));
//         }
//     }
// }

const PIECES_WITHOUT_PAWN: [u8; 5] = [KING, KNIGHT, BISHOP, ROOK, QUEEN];

#[inline(always)]
pub fn gen_moves(color: Color, board: &Board) -> Vec<Move> {
    let mut scored_moves: Vec<(Move, isize)> = Vec::with_capacity(256);
    let (own_occ, enemy_occ) = get_occupancy(&color, board);

    for piece in &PIECES_WITHOUT_PAWN {
        let mut bb = board.bb(piece + color);
        while let Some(sq) = bb.next() {
            let moves = get_all_moves(piece + color, sq, board, own_occ, enemy_occ);
            let quiet_moves = moves & !enemy_occ;
            let capture_moves = moves & enemy_occ;
            add_quiet_moves(quiet_moves, &(piece + color), sq, board, &mut scored_moves);
            add_capture_moves(capture_moves, &(piece + color), sq, board, &mut scored_moves);
        }
    }

    add_castling_moves(&(KING + color), board, &mut scored_moves);
    add_new_ep_move(board, &mut scored_moves);

    add_pawn_cap_moves(PAWN + color, color, board, &mut scored_moves);
    add_pawn_quiet_moves(PAWN + color, color, board, &mut scored_moves);

    // let mut scored: Vec<(isize, Move)> =
    //     positions_rev.drain(..).map(|mv| (eval_pos(&mv, &board), mv)).collect();

    scored_moves.sort_unstable_by_key(|&(_, score)| score);
    scored_moves.into_iter().map(|(mv, _)| mv).collect()

    // positions_rev.sort_unstable_by(|a, b| eval_pos(b, &board).cmp(&eval_pos(a, &board)));
    // positions_rev
}

fn add_pawn_quiet_moves(piece: Piece, color: Color, board: &Board, moves: &mut Vec<(Move, isize)>) {
    let (own_occ, enemy_occ) = get_occupancy(&color, board);
    let both_occ = own_occ | enemy_occ;
    if piece.color().is_white() {
        let mv = (board.bb(piece) << 8) & !both_occ;
        let mut one_mv = mv & !RANK_BITBOARD[7];
        let mut one_promo = mv & RANK_BITBOARD[7];
        let mut two_mv = ((one_mv & RANK_BITBOARD[2]) << 8) & !both_occ;

        while let Some(to_sq) = one_mv.next() {
            moves.push((Move::init((to_sq - 8) as u8, to_sq as u8, piece, Flag::Quiet), 0));
        }

        while let Some(to_sq) = one_promo.next() {
            add_promo((to_sq - 8) as u8, to_sq as u8, piece, board, moves);
        }

        while let Some(to_sq) = two_mv.next() {
            moves.push((Move::init((to_sq - 16) as u8, to_sq as u8, piece, Flag::Quiet), 0));
        }
    } else {
        let mv = (board.bb(piece) >> 8) & !both_occ;
        let mut one_mv = mv & !RANK_BITBOARD[0];
        let mut one_promo = mv & RANK_BITBOARD[0];
        let mut two_mv = ((one_mv & RANK_BITBOARD[5]) >> 8) & !both_occ;

        while let Some(to_sq) = one_mv.next() {
            moves.push((Move::init((to_sq + 8) as u8, to_sq as u8, piece, Flag::Quiet), 0));
        }

        while let Some(to_sq) = one_promo.next() {
            add_promo((to_sq + 8) as u8, to_sq as u8, piece, board, moves);
        }

        while let Some(to_sq) = two_mv.next() {
            moves.push((Move::init((to_sq + 16) as u8, to_sq as u8, piece, Flag::Quiet), 0));
        }
    }
}

fn add_pawn_cap_moves(piece: Piece, color: Color, board: &Board, moves: &mut Vec<(Move, isize)>) {
    let (own_occ, enemy_occ) = get_occupancy(&color, board);

    if piece.color().is_white() {
        let left = ((board.bb(piece) << 9) & !FILE_BITBOARD[0]) & enemy_occ;
        let mut left_att = left & !RANK_BITBOARD[7];
        let mut left_promo = left & RANK_BITBOARD[7];

        while let Some(to_sq) = left_att.next() {
            moves.push((
                Move::init(
                    (to_sq - 9) as u8,
                    to_sq as u8,
                    piece,
                    Flag::Capture(board.cap_piece(to_sq)),
                ),
                0,
            ));
        }

        while let Some(to_sq) = left_promo.next() {
            add_promo((to_sq - 9) as u8, to_sq as u8, piece, board, moves);
        }

        let right = ((board.bb(piece) << 7) & !FILE_BITBOARD[7]) & enemy_occ;
        let mut right_att = right & !RANK_BITBOARD[7];
        let mut right_promo = right & RANK_BITBOARD[7];

        while let Some(to_sq) = right_att.next() {
            moves.push((
                Move::init(
                    (to_sq - 7) as u8,
                    to_sq as u8,
                    piece,
                    Flag::Capture(board.cap_piece(to_sq)),
                ),
                0,
            ));
        }

        while let Some(to_sq) = right_promo.next() {
            add_promo((to_sq - 9) as u8, to_sq as u8, piece, board, moves);
        }
    } else {
        let left = ((board.bb(piece) >> 9) & !FILE_BITBOARD[7]) & enemy_occ;
        let mut left_att = left & !RANK_BITBOARD[0];
        let mut left_promo = left & RANK_BITBOARD[0];

        while let Some(to_sq) = left_att.next() {
            moves.push((
                Move::init(
                    (to_sq + 9) as u8,
                    to_sq as u8,
                    piece,
                    Flag::Capture(board.cap_piece(to_sq)),
                ),
                0,
            ));
        }

        while let Some(to_sq) = left_promo.next() {
            add_promo((to_sq + 9) as u8, to_sq as u8, piece, board, moves);
        }

        let right = ((board.bb(piece) >> 7) & !FILE_BITBOARD[0]) & enemy_occ;
        let mut right_att = right & !RANK_BITBOARD[0];
        let mut right_promo = right & RANK_BITBOARD[0];

        while let Some(to_sq) = right_att.next() {
            moves.push((
                Move::init(
                    (to_sq + 7) as u8,
                    to_sq as u8,
                    piece,
                    Flag::Capture(board.cap_piece(to_sq)),
                ),
                0,
            ));
        }

        while let Some(to_sq) = right_promo.next() {
            add_promo((to_sq + 7) as u8, to_sq as u8, piece, board, moves);
        }
    }
}

fn add_promo(from_sq: u8, to_sq: u8, piece: Piece, board: &Board, moves: &mut Vec<(Move, isize)>) {
    moves.push((
        Move::init(
            from_sq,
            to_sq,
            piece,
            Flag::Promotion(QUEEN + piece.color(), board.squares[to_sq as usize]),
        ),
        0,
    ));
    moves.push((
        Move::init(
            from_sq,
            to_sq,
            piece,
            Flag::Promotion(ROOK + piece.color(), board.squares[to_sq as usize]),
        ),
        0,
    ));
    moves.push((
        Move::init(
            from_sq,
            to_sq,
            piece,
            Flag::Promotion(BISHOP + piece.color(), board.squares[to_sq as usize]),
        ),
        0,
    ));
    moves.push((
        Move::init(
            from_sq,
            to_sq,
            piece,
            Flag::Promotion(KNIGHT + piece.color(), board.squares[to_sq as usize]),
        ),
        0,
    ));
}

fn get_cap_piece(sq: usize, board: &Board) -> Piece {
    match board.squares[sq] {
        Some(piece) => piece,
        None => unreachable!("There is no piece to be captured at this location"),
    }
}

#[inline(always)]
fn add_capture_moves(
    mut bb: u64,
    piece: &Piece,
    from_sq: usize,
    board: &Board,
    moves: &mut Vec<(Move, isize)>,
) {
    while let Some(to_sq) = bb.next() {
        let flag = match board.squares[to_sq] {
            Some(piece) => Flag::Capture(piece),
            None => unreachable!("There is no piece to be captured at this location"),
        };
        let mv = Move::init(from_sq as u8, to_sq as u8, *piece, flag);
        let eval = eval_pos(&mv, board); //eval_cap_move()
        moves.push((mv, eval));
    }
}

#[inline(always)]
fn add_quiet_moves(
    mut bb: u64,
    piece: &Piece,
    from_sq: usize,
    board: &Board,
    moves: &mut Vec<(Move, isize)>,
) {
    while let Some(to_sq) = bb.next() {
        let mv = Move::init(from_sq as u8, to_sq as u8, *piece, Flag::Quiet);
        let eval = eval_pos(&mv, board); //eval_quiet_move()
        moves.push((mv, eval));
    }
}

#[inline(always)]
pub fn gen_captures(color: Color, board: &Board) -> Vec<Move> {
    let mut positions_rev: Vec<Move> = Vec::with_capacity(256);
    let (own_occ, enemy_occ) = get_occupancy(&color, board);

    for piece in &PIECES {
        let mut bb = board.bitboard[(piece + color) as usize];
        while let Some(sq) = bb.next() {
            let moves = match piece.kind() {
                PAWN => get_pawn_att(color, sq, own_occ, enemy_occ, None),
                KNIGHT => get_knight_mv(sq, own_occ, enemy_occ) & enemy_occ,
                BISHOP => get_bishop_mv(sq, own_occ, enemy_occ) & enemy_occ,
                ROOK => get_rook_mv(sq, own_occ, enemy_occ) & enemy_occ,
                QUEEN => get_queen_mv(sq, own_occ, enemy_occ) & enemy_occ,
                KING => get_king_mv(sq, own_occ, enemy_occ) & enemy_occ,
                _ => panic!("Invalid Peace Type"),
            };
            get_positions_rev(moves, &(piece + color), sq, board, &mut positions_rev);
        }
    }

    positions_rev.sort_unstable_by(|a, b| eval_pos(b, &board).cmp(&eval_pos(a, &board)));
    positions_rev
}

fn eval_pos(mv: &Move, board: &Board) -> isize {
    // FIXME: I will need to update this based on pv table that should be located inside the game
    // if matches!(game.tt.get(game.key), Some(x) if x.rev == *pos) {
    //     return 95000;
    // }

    match mv.flag {
        Flag::Quiet => {
            if matches!(board.s_killers[board.ply()][0], Some(x) if x == *mv) {
                90000
            } else if matches!(board.s_killers[board.ply()][1], Some(x) if x == *mv) {
                80000
            } else {
                board.s_history[mv.piece.idx()][mv.to as usize] as isize
            }
        }
        Flag::KingCastle => 20,
        Flag::QueenCastle => 20,
        Flag::Capture(cap) => cap.weight() - mv.piece as isize,
        Flag::EP => PAWN.weight(),
        Flag::Promotion(promo, Some(cap)) => cap.weight() - mv.piece as isize + promo.weight(),
        Flag::Promotion(promo, None) => promo.weight(),
    }
}

#[inline(always)]
pub fn get_all_moves(piece: Piece, pos: usize, board: &Board, own_occ: u64, enemy_occ: u64) -> u64 {
    match piece.kind() {
        PAWN => {
            get_pawn_mv(piece.color(), pos, own_occ, enemy_occ)
                | get_pawn_att(piece.color(), pos, own_occ, enemy_occ, None)
        }
        KNIGHT => get_knight_mv(pos, own_occ, enemy_occ),
        BISHOP => get_bishop_mv(pos, own_occ, enemy_occ),
        ROOK => get_rook_mv(pos, own_occ, enemy_occ),
        QUEEN => get_queen_mv(pos, own_occ, enemy_occ),
        KING => get_king_mv(pos, own_occ, enemy_occ),
        _ => panic!("Invalid Peace Type"),
    }
}

#[inline(always)]
pub fn get_occupancy(piece: &Piece, board: &Board) -> (u64, u64) {
    (board.bb(WHITE + piece.color()), board.bb(BLACK - piece.color()))
    // board.both_occ_bb(color);
}

#[inline(always)]
pub fn sq_attack(game: &Board, sq: usize, color: Color) -> u64 {
    let (own_occ, enemy_occ) = get_occupancy(&color, game);

    let op_pawns = game.bb(BLACK_PAWN - color);
    let op_knights = game.bb(BLACK_KNIGHT - color);
    let op_rq = game.bb(BLACK_QUEEN - color) | game.bb(BLACK_ROOK - color);
    let op_bq = game.bb(BLACK_QUEEN - color) | game.bb(BLACK_BISHOP - color);
    let op_king = game.bb(BLACK_KING - color);

    (get_pawn_att(color, sq, own_occ, enemy_occ, None) & op_pawns)
        | (get_knight_mv(sq, own_occ, enemy_occ) & op_knights)
        | (get_bishop_mv(sq, own_occ, enemy_occ) & op_bq)
        | (get_rook_mv(sq, own_occ, enemy_occ) & op_rq)
        | (get_king_mv(sq, own_occ, enemy_occ) & op_king)
}

#[inline(always)]
fn get_positions_rev(
    mut attacks: u64,
    piece: &Piece,
    from_sq: usize,
    board: &Board,
    new_positions: &mut Vec<Move>,
) {
    while attacks != 0 {
        let to_sq = attacks.pop_lsb();
        let mut new_move = Move {
            from: from_sq as u8,
            to: to_sq as u8,
            piece: *piece,
            flag: match board.squares[to_sq] {
                None => Flag::Quiet,
                Some(piece) => Flag::Capture(piece),
            },
        };

        if new_move.piece.is_pawn() {
            add_ep_move(&mut new_move, board);
            add_promotion_move(&new_move, board, new_positions);
        } else {
            new_positions.push(new_move)
        }
    }
}

pub fn is_repetition(board: &Board) -> bool {
    assert!(
        board.history.len() >= board.state.half_move as usize,
        "It is Negative {:?} {:?}",
        board.history.len(),
        board.state.half_move
    );
    for i in (board.history.len() - board.state.half_move as usize)..board.history.len() {
        if board.history[i].key == board.state.key {
            return true;
        }
    }

    false
}

pub fn move_exists(board: &mut Board, mv: &Move) -> bool {
    let mut moves = gen_moves(board.state.color, board);

    for temp_mv in &mut moves {
        if mv == temp_mv {
            if board.make_move(mv) {
                board.undo_move();
                return true;
            }
        }
    }
    false
}

#[inline(always)]
pub fn add_castling_moves(piece: &Piece, board: &Board, moves: &mut Vec<(Move, isize)>) {
    let (own, enemy) = get_occupancy(piece, board);
    match piece.color() {
        WHITE => {
            if board.state.castling.valid(CastlingRights::WKINGSIDE, board, own, enemy) {
                moves.push((Move::init(E1 as u8, G1 as u8, *piece, Flag::KingCastle), 10));
            }
            if board.state.castling.valid(CastlingRights::WQUEENSIDE, board, own, enemy) {
                moves.push((Move::init(E1 as u8, C1 as u8, *piece, Flag::QueenCastle), 10));
            }
        }
        BLACK => {
            if board.state.castling.valid(CastlingRights::BKINGSIDE, board, own, enemy) {
                moves.push((Move::init(E8 as u8, G8 as u8, *piece, Flag::KingCastle), 10));
            }
            if board.state.castling.valid(CastlingRights::BQUEENSIDE, board, own, enemy) {
                moves.push((Move::init(E8 as u8, C8 as u8, *piece, Flag::QueenCastle), 10));
            }
        }
        _ => panic!("Invalid Castling"),
    }
}

#[inline(always)]
pub fn add_new_ep_move(board: &Board, moves: &mut Vec<(Move, isize)>) {
    if let Some(mv) = board.state.ep {
        let color = board.color().opp();
        let (own_occ, enemy_occ) = get_occupancy(&color, board);

        let mut attack =
            get_pawn_att(color, mv as usize, own_occ, enemy_occ, None) & board.pawn_bb(color.opp());

        while let Some(sq) = attack.next() {
            moves.push((Move::init(sq as u8, mv, PAWN + color.opp(), Flag::EP), 100));
        }
    }
}

#[inline(always)]
pub fn add_ep_move(mv: &mut Move, board: &Board) {
    if Some(mv.to) == board.state.ep {
        mv.flag = Flag::EP;
    }
}

#[inline(always)]
pub fn add_promotion_move(mv: &Move, board: &Board, moves: &mut Vec<Move>) {
    if (mv.piece.is_white() && get_bit_rank(mv.to as usize) == Rank::Eight)
        || (mv.piece.is_black() && get_bit_rank(mv.to as usize) == Rank::One)
    {
        let color = mv.piece.color();
        let cap_piece: Option<Piece> = board.squares[mv.to as usize];

        moves.push(Move { flag: Flag::Promotion(QUEEN + color, cap_piece), ..*mv });
        moves.push(Move { flag: Flag::Promotion(ROOK + color, cap_piece), ..*mv });
        moves.push(Move { flag: Flag::Promotion(BISHOP + color, cap_piece), ..*mv });
        moves.push(Move { flag: Flag::Promotion(KNIGHT + color, cap_piece), ..*mv });
    } else {
        moves.push(*mv);
    }
}

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
        let (own_occ, enemy_occ) = get_occupancy(&piece, &board);
        let all_pieces = extract_all_bits(board.bitboard[piece.idx()]);
        let piece = match board.squares[all_pieces[idx]] {
            None => panic!("The Piece Must exist"),
            Some(piece) => piece,
        };
        return extract_all_bits(get_all_moves(piece, all_pieces[idx], &board, own_occ, enemy_occ));

        // print_bitboard(
        //     generate_knight_moves(&piece, &board),
        //     Some(bit_scan_lsb(piece.position) as i8),
        // );
    }

    #[test]
    fn test_white_pawns_mv_gen() {
        let board = Board::read_fen(&FEN_PAWNS_WHITE);
        let moves = gen_moves(WHITE, &board);
        assert_eq!(42, moves.len());
        print_move_list(&moves);
    }

    #[test]
    fn test_mv_gen() {
        let board = Board::read_fen(&FEN_CASTLE_TWO);
        let moves = gen_moves(WHITE, &board);
        print_chess(&board);
        print_move_list(&moves);
        assert_eq!(48, moves.len());
    }

    #[test]
    fn test_white_black_mv_gen() {
        let board = Board::read_fen(&FEN_PAWNS_BLACK);
        let moves = gen_moves(BLACK, &board);
        assert_eq!(42, moves.len());
        print_move_list(&moves);
    }

    #[test]
    fn test_attacks() {
        let fen = "8/8/2q5/3Q4/8/8/8/8 w - - 0 1";
        let board = Board::read_fen(&fen);
        print_bitboard(
            sq_attack(&board, board.bitboard[BLACK_QUEEN.idx()].get_lsb(), BLACK),
            Some(board.bitboard[BLACK_QUEEN.idx()].get_msb() as i8),
        );
        print_bitboard(sq_attack(&board, board.bitboard[WHITE_QUEEN.idx()].get_msb(), WHITE), None);
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
