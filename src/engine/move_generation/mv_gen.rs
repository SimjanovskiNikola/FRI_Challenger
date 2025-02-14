use crate::engine::attacks::bishop::*;
use crate::engine::attacks::king::*;
use crate::engine::attacks::knight::*;
use crate::engine::attacks::pawn::*;
use crate::engine::attacks::queen::*;
use crate::engine::attacks::rook::*;
use crate::engine::game::Game;
use crate::engine::shared::helper_func::bit_pos_utility::*;
use crate::engine::shared::helper_func::bitboard::BitboardTrait;
use crate::engine::shared::helper_func::const_utility::*;
use crate::engine::shared::structures::castling_struct::*;
use crate::engine::shared::structures::color::*;
use crate::engine::shared::structures::internal_move::*;
use crate::engine::shared::structures::piece::*;
use crate::engine::shared::structures::square::*;
use crate::engine::shared::structures::square::SqPos::*;

#[inline(always)]
pub fn gen_moves(color: Color, game: &Game) -> Vec<InternalMove> {
    let mut positions: Vec<InternalMove> = Vec::with_capacity(256);
    let (own_occ, enemy_occ) = get_occupancy(&color, game);

    for piece in &PIECES {
        let mut bb = game.bitboard[(piece + color) as usize];
        while bb != 0 {
            let pos = bb.pop_lsb();
            let moves = get_all_moves(piece + color, pos, game, own_occ, enemy_occ);
            get_internal_moves(moves, &(piece + color), pos, game, &mut positions);
        }
    }

    positions
}

#[inline(always)]
fn get_all_moves(piece: Piece, pos: usize, game: &Game, own_occ: u64, enemy_occ: u64) -> u64 {
    match piece.kind() {
        PAWN => {
            get_pawn_mv(piece.color(), pos, own_occ, enemy_occ)
                | get_pawn_att(piece.color(), pos, own_occ, enemy_occ, game.ep)
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
fn get_occupancy(piece: &Piece, game: &Game) -> (u64, u64) {
    (game.occupancy[(WHITE + piece.color()).idx()], game.occupancy[(BLACK - piece.color()).idx()])
}

#[inline(always)]
pub fn sq_attack(game: &Game, sq: usize, color: Color) -> u64 {
    let (own_occ, enemy_occ) = get_occupancy(&color, game);

    let op_pawns = game.bitboard[(BLACK_PAWN - color) as usize];
    let op_knights = game.bitboard[(BLACK_KNIGHT - color) as usize];
    let op_rq = game.bitboard[(BLACK_QUEEN - color) as usize]
        | game.bitboard[(BLACK_ROOK - color) as usize];
    let op_bq = game.bitboard[(BLACK_QUEEN - color) as usize]
        | game.bitboard[(BLACK_BISHOP - color) as usize];
    let op_king = game.bitboard[(BLACK_KING - color) as usize];

    (get_pawn_att(color, sq, own_occ, enemy_occ, None) & op_pawns)
        | (get_knight_mv(sq, own_occ, enemy_occ) & op_knights)
        | (get_bishop_mv(sq, own_occ, enemy_occ) & op_bq)
        | (get_rook_mv(sq, own_occ, enemy_occ) & op_rq)
        | (get_king_mv(sq, own_occ, enemy_occ) & op_king)
}

#[inline(always)]
fn get_internal_moves(
    mut attacks: u64,
    piece: &Piece,
    pos: usize,
    game: &Game,
    new_positions: &mut Vec<InternalMove>,
) {
    while attacks != 0 {
        let p_move = attacks.pop_lsb();
        let mut new_move = InternalMove {
            position_key: game.pos_key,
            active_color: game.color,
            from: pos,
            to: p_move,
            piece: *piece,
            ep: game.ep,
            castle: game.castling,
            half_move: game.half_move,
            flag: match game.squares[p_move] {
                Square::Empty => Flag::Quiet,
                Square::Occupied(piece) => Flag::Capture(piece),
            },
        };
        if new_move.piece.is_pawn() {
            add_ep_move(&mut new_move, game);
            add_promotion_move(&new_move, game, new_positions);
        } else {
            new_positions.push(new_move)
        }
    }

    if piece.is_king() {
        add_castling_moves(piece, pos, game, new_positions);
    }
}

#[rustfmt::skip]
#[inline(always)]
pub fn add_castling_moves(piece: &Piece, pos: usize, game: &Game, positions: &mut Vec<InternalMove>) {
    
    let mv = InternalMove {
            position_key: 0, 
            active_color: game.color,
            from: pos,
            to: 0,
            piece: *piece,
            ep: game.ep,
            castle: game.castling,
            half_move: game.half_move,
            flag: Flag::Quiet,
        };

    let (own, enemy) = get_occupancy(piece, game);
    match mv.active_color {
        WHITE => {
            if game.castling.valid(CastlingRights::WKINGSIDE, game, own, enemy) {
               positions.push(InternalMove { to: SqPos::G1.idx(), flag: Flag::KingSideCastle(H1 as usize, F1 as usize), ..mv });
            }
            if game.castling.valid(CastlingRights::WQUEENSIDE, game, own, enemy) {
               positions.push(InternalMove { to: SqPos::C1.idx(), flag: Flag::QueenSideCastle(A1 as usize, D1 as usize), ..mv });
            }
        }
         BLACK => {
            if game.castling.valid(CastlingRights::BKINGSIDE, game, own, enemy) {
               positions.push(InternalMove { to: SqPos::G8.idx(), flag: Flag::KingSideCastle(H8 as usize, F8 as usize), ..mv });
            }
            if game.castling.valid(CastlingRights::BQUEENSIDE, game, own, enemy) {
               positions.push(InternalMove { to: SqPos::C8.idx(), flag: Flag::QueenSideCastle(A8 as usize, D8 as usize), ..mv });
            }
        }
        _ => panic!("Invalid Castling")
    }

}

#[inline(always)]
pub fn add_ep_move(mv: &mut InternalMove, game: &Game) {
    if Some(mv.to) == game.ep {
        let sq = mv.to + 16 * mv.active_color.idx() - 8;
        mv.flag = Flag::EP(sq, PAWN + BLACK - mv.active_color);
    }
}

#[inline(always)]
pub fn add_promotion_move(mv: &InternalMove, _game: &Game, positions: &mut Vec<InternalMove>) {
    if (mv.active_color.is_white() && get_bit_rank(mv.to) == Rank::Eight)
        || (mv.active_color.is_black() && get_bit_rank(mv.to) == Rank::One)
    {
        let color = mv.piece.color();
        let cap_piece: Option<Piece> = match mv.flag {
            Flag::Capture(piece) => Some(piece),
            _ => None,
        };

        positions.push(InternalMove { flag: Flag::Promotion(QUEEN + color, cap_piece), ..*mv });
        positions.push(InternalMove { flag: Flag::Promotion(ROOK + color, cap_piece), ..*mv });
        positions.push(InternalMove { flag: Flag::Promotion(BISHOP + color, cap_piece), ..*mv });
        positions.push(InternalMove { flag: Flag::Promotion(KNIGHT + color, cap_piece), ..*mv });
    } else {
        positions.push(*mv);
    }
}

pub fn is_repetition(game: &Game) -> bool {
    for i in (game.moves.len() - game.half_move)..game.moves.len() {
        if game.moves[i].position_key == game.pos_key {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use crate::engine::{
        move_generation::fen::FenTrait,
        shared::{
            helper_func::{
                bit_pos_utility::*,
                const_utility::{FEN_CASTLE_TWO, FEN_PAWNS_BLACK, FEN_PAWNS_WHITE},
                print_utility::{print_bitboard, print_chess, print_move_list},
            },
            structures::piece::{
                BLACK_QUEEN, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_QUEEN, WHITE_ROOK,
            },
        },
    };
    use super::*;

    fn test_mov_att(fen: &str, piece: Piece, idx: usize) -> Vec<usize> {
        let game = Game::read_fen(&fen);
        // println!("{}", game.to_string());
        let (own_occ, enemy_occ) = get_occupancy(&piece, &game);
        let all_pieces = extract_all_bits(game.bitboard[piece.idx()]);
        let piece = match game.squares[all_pieces[idx]] {
            Square::Empty => panic!("The Piece Must exist"),
            Square::Occupied(piece) => piece,
        };
        return extract_all_bits(get_all_moves(piece, all_pieces[idx], &game, own_occ, enemy_occ));

        // print_bitboard(
        //     generate_knight_moves(&piece, &game),
        //     Some(bit_scan_lsb(piece.position) as i8),
        // );
    }

    #[test]
    fn test_white_pawns_mv_gen() {
        let game = Game::read_fen(&FEN_PAWNS_WHITE);
        let moves = gen_moves(WHITE, &game);
        assert_eq!(42, moves.len());
        print_move_list(&moves);
    }

    #[test]
    fn test_mv_gen() {
        let game = Game::read_fen(&FEN_CASTLE_TWO);
        let moves = gen_moves(WHITE, &game);
        print_chess(&game);
        print_move_list(&moves);
        assert_eq!(48, moves.len());
    }

    #[test]
    fn test_white_black_mv_gen() {
        let game = Game::read_fen(&FEN_PAWNS_BLACK);
        let moves = gen_moves(BLACK, &game);
        assert_eq!(42, moves.len());
        print_move_list(&moves);
    }

    #[test]
    fn test_attacks() {
        let fen = "8/8/2q5/3Q4/8/8/8/8 w - - 0 1";
        let game = Game::read_fen(&fen);
        print_bitboard(
            sq_attack(&game, game.bitboard[BLACK_QUEEN.idx()].get_lsb(), BLACK),
            Some(game.bitboard[BLACK_QUEEN.idx()].get_msb() as i8),
        );
        print_bitboard(sq_attack(&game, game.bitboard[WHITE_QUEEN.idx()].get_msb(), WHITE), None);
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
