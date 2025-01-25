use crate::engine::{
    attacks::all_attacks::{blocked_ray_att, ATTACKS},
    game::Game,
    shared::{
        helper_func::{
            bit_pos_utility::*,
            bitboard::{Bitboard, BitboardTrait},
            const_utility::{Rank, SqPos},
            print_utility::print_chess,
        },
        structures::{
            castling_struct::CastlingRights,
            color::{Color, ColorTrait, BLACK, WHITE},
            directions::Dir,
            internal_move::{Flag, InternalMove},
            piece::{Piece, PieceTrait, BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK},
            square::Square,
        },
    },
};

pub fn get_attacks(rays: Dir, piece: &Piece, pos: usize, game: &Game) -> u64 {
    let attacks = &ATTACKS.rays[rays.idx()];

    let (own_occ, enemy_occ) = get_occupancy(&piece, game);

    return blocked_ray_att(rays, attacks, attacks[pos], own_occ, enemy_occ);
}

pub fn gen_moves(color: Color, game: &Game) -> Vec<InternalMove> {
    let mut positions: Vec<InternalMove> = vec![];

    // for bitboard in game.piece_bitboard[color as usize] {
    for idx in ((color.idx())..game.bitboard.len()).step_by(2) {
        for pos in extract_all_bits(game.bitboard[idx]) {
            let piece = match game.squares[pos] {
                Square::Occupied(piece) => piece,
                Square::Empty => panic!("Shouldn't be empty"),
            };

            let all_mov_att = get_all_moves(piece, pos, game) | get_all_attacks(piece, pos, game);
            positions.extend(get_internal_moves(all_mov_att, &piece, pos, game));
        }
    }

    return positions;
}

pub fn gen_attacks(game: &Game, color: Color) -> Bitboard {
    let mut attacked_sq: Bitboard = 0;
    for idx in ((color.idx())..game.bitboard.len()).step_by(2) {
        for sq in extract_all_bits(game.bitboard[idx]) {
            if let Square::Occupied(piece) = game.squares[sq] {
                attacked_sq.union(get_all_attacks(piece, sq, game));
            }
        }
    }
    return attacked_sq;
}

fn get_all_moves(piece: Piece, pos: usize, game: &Game) -> u64 {
    match piece.kind() {
        PAWN => return gen_pawn_mov(&piece, pos, &game),
        KNIGHT => return gen_knight_mov_att(&piece, pos, &game),
        BISHOP => return gen_bishop_mov_att(&piece, pos, &game),
        ROOK => return gen_rook_mov_att(&piece, pos, &game),
        QUEEN => return gen_queen_mov_att(&piece, pos, &game),
        KING => return gen_king_mov_att(&piece, pos, &game),
        _ => panic!("Invalid Peace Type"),
    }
}

fn get_all_attacks(piece: Piece, pos: usize, game: &Game) -> u64 {
    match piece.kind() {
        PAWN => return gen_pawn_att(&piece, pos, game),
        KNIGHT => return gen_knight_mov_att(&piece, pos, &game),
        BISHOP => return gen_bishop_mov_att(&piece, pos, &game),
        ROOK => return gen_rook_mov_att(&piece, pos, &game),
        QUEEN => return gen_queen_mov_att(&piece, pos, &game),
        KING => return gen_king_mov_att(&piece, pos, &game),
        _ => panic!("Invalid Peace Type"),
    };
}

fn get_occupancy(piece: &Piece, game: &Game) -> (u64, u64) {
    let (white_idx, black_idx) = (WHITE.idx(), BLACK.idx());
    match piece.color() {
        WHITE => return (game.occupancy[white_idx], game.occupancy[black_idx]),
        BLACK => return (game.occupancy[black_idx], game.occupancy[white_idx]),
        _ => panic!("Invalid Color"),
    };
}

fn get_internal_moves(attacks: u64, piece: &Piece, pos: usize, game: &Game) -> Vec<InternalMove> {
    let potential_moves = extract_all_bits(attacks);
    let mut new_positions = vec![];
    for p_move in potential_moves {
        let mut new_move = InternalMove {
            position_key: 0,
            active_color: game.active_color,
            from: pos,
            to: p_move,
            piece: *piece,
            captured: match game.squares[p_move] {
                Square::Empty => None,
                Square::Occupied(piece) => Some(piece),
            },
            promotion: None,
            ep: game.en_passant,
            castle: game.castling_rights,
            half_move: game.halfmove_clock,
            flag: match game.squares[p_move] {
                Square::Empty => Flag::Normal,
                Square::Occupied(_) => Flag::Capture,
            },
        };
        if new_move.piece.is_pawn() {
            add_ep_move(&mut new_move, game);
            new_positions.extend(add_promotion_move(&mut new_move, game));
        } else {
            new_positions.push(new_move)
        }
    }
    if piece.is_king() {
        new_positions.extend(add_castling_moves(piece, pos, game));
    }
    return new_positions;
}

#[rustfmt::skip]
pub fn add_castling_moves(piece: &Piece, pos: usize, game: &Game) -> Vec<InternalMove> {
    let mut new_positions = vec![];
    let mut mv = InternalMove {
            position_key: 0, 
            active_color: game.active_color,
            from: pos,
            to: 0,
            piece: *piece,
            captured: None,
            promotion: None,
            ep: game.en_passant,
            castle: game.castling_rights,
            half_move: game.halfmove_clock,
            flag: Flag::Normal,
        };

    match mv.active_color {
        WHITE => {
            let attacked_sq = gen_attacks(game, BLACK);
            if (game.castling_rights.bits() & CastlingRights::WKINGSIDE.bits() != 0) && 
               (game.squares[SqPos::F1 as usize] == Square::Empty && game.squares[SqPos::G1 as usize] == Square::Empty) && 
               (!is_bit_set(attacked_sq, SqPos::E1 as usize) && !is_bit_set(attacked_sq, SqPos::F1 as usize) && !is_bit_set(attacked_sq, SqPos::G1 as usize)){
               new_positions.push(InternalMove {
                    to: SqPos::G1 as usize,
                    flag: Flag::KingSideCastle,
                    ..mv
                });
            }
            if (game.castling_rights.bits() & CastlingRights::WQUEENSIDE.bits() != 0) && 
               (game.squares[SqPos::D1 as usize] == Square::Empty && game.squares[SqPos::C1 as usize] == Square::Empty && game.squares[SqPos::B1 as usize] == Square::Empty) && 
               (!is_bit_set(attacked_sq, SqPos::E1 as usize) && !is_bit_set(attacked_sq, SqPos::D1 as usize) && !is_bit_set(attacked_sq, SqPos::C1 as usize)){
                new_positions.push(InternalMove {
                    to: SqPos::C1 as usize,
                    flag: Flag::QueenSideCastle,
                    ..mv
                });
            }
        }
         BLACK => {
            let attacked_sq = gen_attacks(game, WHITE);
            if (game.castling_rights.bits() & CastlingRights::BKINGSIDE.bits() != 0) && 
               (game.squares[SqPos::F8 as usize] == Square::Empty && game.squares[SqPos::G8 as usize] == Square::Empty) && 
               (!is_bit_set(attacked_sq, SqPos::E8 as usize) && !is_bit_set(attacked_sq, SqPos::F8 as usize) && !is_bit_set(attacked_sq, SqPos::G8 as usize)){
               new_positions.push(InternalMove {
                    to: SqPos::G8 as usize,
                    flag: Flag::KingSideCastle,
                    ..mv
                });
            }
            if (game.castling_rights.bits() & CastlingRights::BQUEENSIDE.bits() != 0) && 
               (game.squares[SqPos::D8 as usize] == Square::Empty && game.squares[SqPos::C8 as usize] == Square::Empty && game.squares[SqPos::B8 as usize] == Square::Empty) && 
               (!is_bit_set(attacked_sq, SqPos::E8 as usize) && !is_bit_set(attacked_sq, SqPos::D8 as usize) && !is_bit_set(attacked_sq, SqPos::C8 as usize)){
               new_positions.push(InternalMove {
                    to: SqPos::C8 as usize,
                    flag: Flag::QueenSideCastle,
                    ..mv
                });
            }
        }
        _ => panic!("Invalid Castling")
    }

    // println!("{:#?}", new_positions);
    return new_positions;
}

pub fn add_ep_move(mv: &mut InternalMove, game: &Game) {
    // if game.en_passant == Some(33554432) {
    //     println!("{:?}", game.en_passant);
    // }
    match (mv.piece.kind(), game.en_passant) {
        (PAWN, Some(bb)) => {
            if mv.to == bb.get_lsb() {
                mv.flag = Flag::EP;
                match mv.active_color {
                    WHITE => match game.squares[bb.get_lsb() - 8] {
                        Square::Empty => {
                            print_chess(game);
                            println!("EP Move: {:?}", bb.get_lsb());
                            println!("Move: {:?}", bb.get_lsb() - 8);
                            panic!("No Pawn on a specified place")
                        }
                        Square::Occupied(piece) => mv.captured = Some(piece),
                    },
                    BLACK => match game.squares[bb.get_lsb() + 8] {
                        Square::Empty => {
                            print_chess(game);
                            println!("EP Move: {:?}", bb.get_lsb());
                            println!("Move: {:?}", bb.get_lsb() + 8);
                            panic!("No Pawn on a specified place")
                        }
                        Square::Occupied(piece) => mv.captured = Some(piece),
                    },
                    _ => panic!("Invalid El Passant"),
                }
            }
        }
        (_, _) => (),
    };
}
pub fn add_promotion_move(mv: &InternalMove, game: &Game) -> Vec<InternalMove> {
    let mut new_moves: Vec<InternalMove> = vec![];
    if (mv.piece.is_pawn())
        && ((mv.active_color == WHITE && get_bit_rank(mv.to) == Rank::Eight)
            || (mv.active_color == BLACK && get_bit_rank(mv.to) == Rank::One))
    {
        new_moves.push(InternalMove {
            promotion: Some(QUEEN + mv.piece.color()),
            flag: Flag::Promotion,
            ..*mv
        });
        new_moves.push(InternalMove {
            promotion: Some(ROOK + mv.piece.color()),
            flag: Flag::Promotion,
            ..*mv
        });
        new_moves.push(InternalMove {
            promotion: Some(BISHOP + mv.piece.color()),
            flag: Flag::Promotion,
            ..*mv
        });
        new_moves.push(InternalMove {
            promotion: Some(KNIGHT + mv.piece.color()),
            flag: Flag::Promotion,
            ..*mv
        });
    } else {
        new_moves.push(*mv);
    }

    return new_moves;
}

// TODO: IMPLEMNET
fn gen_pawn_att(piece: &Piece, pos: usize, game: &Game) -> u64 {
    let mut attacks = match piece.color() {
        BLACK => ATTACKS.pawn.black_diagonal_moves[pos],
        WHITE => ATTACKS.pawn.white_diagonal_moves[pos],
        _ => panic!("Invalid Color"),
    };

    let (own_occupancy, enemy_occupancy) = get_occupancy(piece, game);

    attacks &= !own_occupancy;
    attacks &= match game.en_passant {
        Some(bitboard) => {
            let rez;
            let rank = get_bit_rank(bit_scan_lsb(bitboard));
            if (rank == Rank::Six && piece.is_white()) || (rank == Rank::Three && piece.is_black())
            {
                rez = enemy_occupancy | bitboard;
            } else {
                rez = enemy_occupancy;
            }
            rez
        }
        None => enemy_occupancy,
    };

    return attacks;
}

fn gen_pawn_mov(piece: &Piece, pos: usize, game: &Game) -> u64 {
    let mut attacks = match piece.color() {
        BLACK => ATTACKS.pawn.black_forward_moves[pos],
        WHITE => ATTACKS.pawn.white_forward_moves[pos],
        _ => panic!("Invalid Color"),
    };

    let mut all_bits = extract_all_bits(attacks);
    if piece.is_black() {
        all_bits.reverse();
    }

    for (i, attack) in all_bits.iter().enumerate() {
        match game.squares[*attack] {
            Square::Empty => continue,
            Square::Occupied(piece) => {
                if i == 0 {
                    return 0;
                } else if i == 1 {
                    attacks = clear_bit(attacks, *attack)
                }
            }
        }
    }

    // TODO: Somewhere I need the promotions
    return attacks;
}

fn gen_knight_mov_att(piece: &Piece, pos: usize, game: &Game) -> u64 {
    let (own_occupancy, _) = get_occupancy(piece, game);
    let attacks = ATTACKS.knight[pos];

    return attacks & !own_occupancy;
}

fn gen_bishop_mov_att(piece: &Piece, pos: usize, game: &Game) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= get_attacks(Dir::NORTHWEST, piece, pos, game);
    attacks |= get_attacks(Dir::SOUTHWEST, piece, pos, game);
    attacks |= get_attacks(Dir::NORTHEAST, piece, pos, game);
    attacks |= get_attacks(Dir::SOUTHEAST, piece, pos, game);

    return attacks;
}

fn gen_rook_mov_att(piece: &Piece, pos: usize, game: &Game) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= get_attacks(Dir::NORTH, piece, pos, game);
    attacks |= get_attacks(Dir::SOUTH, piece, pos, game);
    attacks |= get_attacks(Dir::EAST, piece, pos, game);
    attacks |= get_attacks(Dir::WEST, piece, pos, game);

    return attacks;
}

fn gen_queen_mov_att(piece: &Piece, pos: usize, game: &Game) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= gen_bishop_mov_att(piece, pos, game);
    attacks |= gen_rook_mov_att(piece, pos, game);
    return attacks;
}

fn gen_king_mov_att(piece: &Piece, pos: usize, game: &Game) -> u64 {
    let attacks = ATTACKS.king[pos];
    let (own_occupancy, _) = get_occupancy(piece, game);
    return attacks & !own_occupancy;
}

#[cfg(test)]
mod tests {

    use crate::engine::shared::{
        helper_func::{
            bit_pos_utility::*,
            const_utility::{SqPos::*, FEN_CASTLE_TWO, FEN_PAWNS_BLACK, FEN_PAWNS_WHITE},
            print_utility::{print_bitboard, print_chess, print_move_list},
        },
        structures::piece::{
            BLACK_QUEEN, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_QUEEN, WHITE_ROOK,
        },
    };
    use super::*;

    fn test_mov_att(fen: &str, piece: Piece, idx: usize) -> Vec<usize> {
        let game = Game::read_fen(&fen);
        // println!("{}", game.to_string());

        let allPieces = extract_all_bits(game.bitboard[piece.idx()]);
        let piece = match game.squares[allPieces[idx]] {
            Square::Empty => panic!("The Piece Must exist"),
            Square::Occupied(piece) => piece,
        };
        return extract_all_bits(
            get_all_moves(piece, allPieces[idx], &game)
                | get_all_attacks(piece, allPieces[idx], &game),
        );

        // print_bitboard(
        //     generate_knight_moves(&piece, &game),
        //     Some(bit_scan_lsb(piece.position) as i8),
        // );
    }

    #[test]
    fn test_white_pawns_mv_gen() {
        let game = Game::read_fen(&FEN_PAWNS_WHITE);
        let moves = gen_moves(WHITE, &game);
        assert_eq!(26, moves.len());
        print_move_list(moves);
    }

    #[test]
    fn test_mv_gen() {
        let game = Game::read_fen(&FEN_CASTLE_TWO);
        let moves = gen_moves(WHITE, &game);
        assert_eq!(48, moves.len());
        print_chess(&game);
        print_move_list(moves);
    }

    #[test]
    fn test_white_black_mv_gen() {
        let game = Game::read_fen(&FEN_PAWNS_BLACK);
        let moves = gen_moves(BLACK, &game);
        assert_eq!(26, moves.len());
        print_move_list(moves);
    }

    #[test]
    fn test_attacks() {
        let fen = "8/8/2q5/3Q4/8/8/8/8 w - - 0 1";
        let game = Game::read_fen(&fen);
        print_bitboard(
            gen_attacks(&game, BLACK),
            Some(bit_scan_lsb(game.bitboard[BLACK_QUEEN.idx()]) as i8),
        );
        print_bitboard(
            gen_attacks(&game, WHITE),
            Some(bit_scan_lsb(game.bitboard[WHITE_QUEEN.idx()]) as i8),
        );
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
