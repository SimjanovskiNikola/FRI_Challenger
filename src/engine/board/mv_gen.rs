use super::structures::board::Board;
use super::structures::castling::*;
use super::structures::color::*;
use super::structures::moves::*;
use super::structures::piece::*;
use super::structures::square::SqPos::*;
use crate::engine::misc::bit_pos_utility::*;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::misc::bitboard::Iterator;
use crate::engine::misc::const_utility::*;
use crate::engine::move_generator::bishop::*;
use crate::engine::move_generator::king::*;
use crate::engine::move_generator::knight::*;
use crate::engine::move_generator::pawn::*;
use crate::engine::move_generator::queen::*;
use crate::engine::move_generator::rook::*;

use super::make_move::GameMoveTrait;

#[inline(always)]
pub fn gen_moves(color: Color, board: &Board) -> Vec<Move> {
    let mut positions_rev: Vec<Move> = Vec::with_capacity(256);
    let (own_occ, enemy_occ) = get_occupancy(&color, board);

    for piece in &PIECES {
        let mut bb = board.bitboard(piece + color);
        while let Some(sq) = bb.next() {
            let moves = get_all_moves(piece + color, sq, board, own_occ, enemy_occ);
            get_positions_rev(moves, &(piece + color), sq, board, &mut positions_rev);
        }
    }

    add_castling_moves(&(KING + color), board, &mut positions_rev);

    positions_rev.sort_unstable_by(|a, b| eval_pos(b, &board).cmp(&eval_pos(a, &board)));
    positions_rev
}

#[inline(always)]
pub fn gen_captures(color: Color, board: &Board) -> Vec<Move> {
    let mut positions_rev: Vec<Move> = Vec::with_capacity(256);
    let (own_occ, enemy_occ) = get_occupancy(&color, board);

    for piece in &PIECES {
        let mut bb = board.bitboard[(piece + color) as usize];
        while let Some(sq) = bb.next() {
            let moves = match piece.kind() {
                PAWN => get_pawn_att(color, sq, own_occ, enemy_occ, board.state.ep),
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
                | get_pawn_att(piece.color(), pos, own_occ, enemy_occ, board.state.ep)
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
    (board.bitboard(WHITE + piece.color()), board.bitboard(BLACK - piece.color()))
}

#[inline(always)]
pub fn sq_attack(game: &Board, sq: usize, color: Color) -> u64 {
    let (own_occ, enemy_occ) = get_occupancy(&color, game);

    let op_pawns = game.bitboard(BLACK_PAWN - color);
    let op_knights = game.bitboard(BLACK_KNIGHT - color);
    let op_rq = game.bitboard(BLACK_QUEEN - color) | game.bitboard(BLACK_ROOK - color);
    let op_bq = game.bitboard(BLACK_QUEEN - color) | game.bitboard(BLACK_BISHOP - color);
    let op_king = game.bitboard(BLACK_KING - color);

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
pub fn add_castling_moves(piece: &Piece, board: &Board, positions: &mut Vec<Move>) {
    let (own, enemy) = get_occupancy(piece, board);
    match piece.color() {
        WHITE => {
            if board.state.castling.valid(CastlingRights::WKINGSIDE, board, own, enemy) {
                positions.push(Move::init(E1 as u8, G1 as u8, *piece, Flag::KingCastle));
            }
            if board.state.castling.valid(CastlingRights::WQUEENSIDE, board, own, enemy) {
                positions.push(Move::init(E1 as u8, C1 as u8, *piece, Flag::QueenCastle));
            }
        }
        BLACK => {
            if board.state.castling.valid(CastlingRights::BKINGSIDE, board, own, enemy) {
                positions.push(Move::init(E8 as u8, G8 as u8, *piece, Flag::KingCastle));
            }
            if board.state.castling.valid(CastlingRights::BQUEENSIDE, board, own, enemy) {
                positions.push(Move::init(E8 as u8, C8 as u8, *piece, Flag::QueenCastle));
            }
        }
        _ => panic!("Invalid Castling"),
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
