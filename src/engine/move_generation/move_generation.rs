use crate::engine::{
    attacks::{
        self,
        all_attacks::{Attacks, ATTACKS},
        ray_attacks::blocked_ray_attack,
    },
    game::Game,
    shared::{
        helper_func::{
            bit_pos_utility::*,
            bitboard::{Bitboard, BitboardTrait},
            const_utility::{File, Rank, SqPos},
            print_utility::{move_notation, print_bitboard, print_chess, sq_notation},
        },
        structures::{
            castling_struct::CastlingRights,
            internal_move::{Flag, InternalMove},
            piece_struct::{self, Color, Piece, PieceType},
            square_struct::Square,
        },
    },
};

macro_rules! get_attacks {
    ($rays:ident, $forward:expr, $piece:ident, $game:ident, $moves:ident) => {
        let idx = bit_scan_lsb($piece.pos);
        let attacks = &ATTACKS.ray_attacks.$rays;

        let (own_occ, enemy_occ) = get_occupancy($piece, $game);

        let ray_attacks = blocked_ray_attack(attacks[idx], &attacks, $forward, own_occ, enemy_occ);

        $moves |= ray_attacks
    };
}

pub fn gen_moves(color: Color, game: &Game) -> Vec<InternalMove> {
    // println!("{:?}", "Here");
    // print_chess(game);

    let mut positions: Vec<InternalMove> = vec![];

    for bitboard in game.piece_bitboard[color as usize] {
        for pos in extract_all_bits(bitboard) {
            let piece = match game.squares[pos] {
                Square::Occupied(piece) => piece,
                Square::Empty => panic!("Shouldn't be empty"),
            };

            let all_mov_att = get_all_moves(piece, game) | get_all_attacks(piece, game);
            positions.extend(get_internal_moves(all_mov_att, &piece, game));
        }
    }

    return positions;
}

pub fn gen_attacks(game: &Game, color: Color) -> Bitboard {
    let mut attacked_sq: Bitboard = 0;
    for bitboard in game.piece_bitboard[color as usize] {
        for square in extract_all_bits(bitboard) {
            if let Square::Occupied(piece) = game.squares[square] {
                attacked_sq.union(get_all_attacks(piece, game));
            }
        }
    }
    return attacked_sq;
}

fn get_all_moves(piece: Piece, game: &Game) -> u64 {
    match piece.p_type {
        PieceType::Pawn => return gen_pawn_mov(&piece, &game),
        PieceType::Knight => return gen_knight_mov_att(&piece, &game),
        PieceType::Bishop => return gen_bishop_mov_att(&piece, &game),
        PieceType::Rook => return gen_rook_mov_att(&piece, &game),
        PieceType::Queen => return gen_queen_mov_att(&piece, &game),
        PieceType::King => return gen_king_mov_att(&piece, &game),
    }
}

fn get_all_attacks(piece: Piece, game: &Game) -> u64 {
    match piece.p_type {
        PieceType::Pawn => return gen_pawn_att(&piece, game),
        PieceType::Knight => return gen_knight_mov_att(&piece, &game),
        PieceType::Bishop => return gen_bishop_mov_att(&piece, &game),
        PieceType::Rook => return gen_rook_mov_att(&piece, &game),
        PieceType::Queen => return gen_queen_mov_att(&piece, &game),
        PieceType::King => return gen_king_mov_att(&piece, &game),
    };
}

fn get_occupancy(piece: &Piece, game: &Game) -> (u64, u64) {
    let (white_idx, black_idx) = (Color::White as usize, Color::Black as usize);
    match piece.p_color {
        Color::White => return (game.occupancy[white_idx], game.occupancy[black_idx]),
        Color::Black => return (game.occupancy[black_idx], game.occupancy[white_idx]),
    };
}

fn get_internal_moves(attacks: u64, piece: &Piece, game: &Game) -> Vec<InternalMove> {
    let potential_moves = extract_all_bits(attacks);
    let mut new_positions = vec![];
    for p_move in potential_moves {
        let mut new_move = InternalMove {
            position_key: 0, //FIXME:
            active_color: game.active_color,
            from: bit_scan_lsb(piece.pos),
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

        match new_move.piece.p_type {
            PieceType::Pawn => {
                add_ep_move(&mut new_move, game);
                new_positions.extend(add_promotion_move(&mut new_move, game));
            }
            _ => new_positions.push(new_move),
        }
    }
    if piece.p_type == PieceType::King {
        new_positions.extend(add_castling_moves(piece, game));
    }
    return new_positions;
}

#[rustfmt::skip]
pub fn add_castling_moves(piece: &Piece, game: &Game) -> Vec<InternalMove> {
    let mut new_positions = vec![];
    let mut mv = InternalMove {
            position_key: 0, 
            active_color: game.active_color,
            from: bit_scan_lsb(piece.pos),
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
        Color::White => {
            let attacked_sq = gen_attacks(game, Color::Black);
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
         Color::Black => {
            let attacked_sq = gen_attacks(game, Color::White);
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
    }

    // println!("{:#?}", new_positions);
    return new_positions;
}

pub fn add_ep_move(mv: &mut InternalMove, game: &Game) {
    // if game.en_passant == Some(33554432) {
    //     println!("{:?}", game.en_passant);
    // }
    match (mv.piece.p_type, game.en_passant) {
        (PieceType::Pawn, Some(bb)) => {
            if mv.to == bb.get_lsb() {
                mv.flag = Flag::EP;
                match mv.active_color {
                    Color::White => match game.squares[bb.get_lsb() - 8] {
                        Square::Empty => {
                            print_chess(game);
                            println!("EP Move: {:?}", bb.get_lsb());
                            println!("Move: {:?}", bb.get_lsb() - 8);
                            panic!("No Pawn on a specified place")
                        }
                        Square::Occupied(piece) => mv.captured = Some(piece),
                    },
                    Color::Black => match game.squares[bb.get_lsb() + 8] {
                        Square::Empty => {
                            print_chess(game);
                            println!("EP Move: {:?}", bb.get_lsb());
                            println!("Move: {:?}", bb.get_lsb() + 8);
                            panic!("No Pawn on a specified place")
                        }
                        Square::Occupied(piece) => mv.captured = Some(piece),
                    },
                }
            }
        }
        (_, _) => (),
    };
}
pub fn add_promotion_move(mv: &InternalMove, game: &Game) -> Vec<InternalMove> {
    let mut new_moves: Vec<InternalMove> = vec![];
    if (mv.piece.p_type == PieceType::Pawn)
        && ((mv.active_color == Color::White && get_bit_rank(mv.to) == Rank::Eight)
            || (mv.active_color == Color::Black && get_bit_rank(mv.to) == Rank::One))
    {
        new_moves.push(InternalMove {
            promotion: Some(Piece { p_type: PieceType::Queen, ..mv.piece }),
            flag: Flag::Promotion,
            ..*mv
        });
        new_moves.push(InternalMove {
            promotion: Some(Piece { p_type: PieceType::Rook, ..mv.piece }),
            flag: Flag::Promotion,
            ..*mv
        });
        new_moves.push(InternalMove {
            promotion: Some(Piece { p_type: PieceType::Bishop, ..mv.piece }),
            flag: Flag::Promotion,
            ..*mv
        });
        new_moves.push(InternalMove {
            promotion: Some(Piece { p_type: PieceType::Knight, ..mv.piece }),
            flag: Flag::Promotion,
            ..*mv
        });
    } else {
        new_moves.push(*mv);
    }

    return new_moves;
}

// TODO: IMPLEMNET
fn gen_pawn_att(piece: &Piece, game: &Game) -> u64 {
    let idx = bit_scan_lsb(piece.pos);

    let mut attacks = match piece.p_color {
        Color::Black => ATTACKS.pawn_attacks.black_diagonal_moves[idx],
        Color::White => ATTACKS.pawn_attacks.white_diagonal_moves[idx],
    };

    let (own_occupancy, enemy_occupancy) = get_occupancy(piece, game);

    attacks &= !own_occupancy;
    attacks &= match game.en_passant {
        Some(bitboard) => {
            let rez;
            let rank = get_bit_rank(bit_scan_lsb(bitboard));
            if (rank == Rank::Six && piece.p_color == Color::White)
                || (rank == Rank::Three && piece.p_color == Color::Black)
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

fn gen_pawn_mov(piece: &Piece, game: &Game) -> u64 {
    let idx = bit_scan_lsb(piece.pos);

    let mut attacks = match piece.p_color {
        Color::Black => ATTACKS.pawn_attacks.black_forward_moves[idx],
        Color::White => ATTACKS.pawn_attacks.white_forward_moves[idx],
    };

    let mut all_bits = extract_all_bits(attacks);
    if piece.p_color == Color::Black {
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

fn gen_knight_mov_att(piece: &Piece, game: &Game) -> u64 {
    let idx = bit_scan_lsb(piece.pos);
    let mut attacks = ATTACKS.knight_attacks.knight_attacks[idx];

    let (own_occupancy, _) = get_occupancy(piece, game);
    attacks &= !own_occupancy;

    return attacks;
}

fn gen_bishop_mov_att(piece: &Piece, game: &Game) -> u64 {
    let mut attacks: u64 = 0;
    get_attacks!(nw_rays, true, piece, game, attacks);
    get_attacks!(sw_rays, false, piece, game, attacks);
    get_attacks!(ne_rays, true, piece, game, attacks);
    get_attacks!(se_rays, false, piece, game, attacks);

    return attacks;
}

fn gen_rook_mov_att(piece: &Piece, game: &Game) -> u64 {
    let mut attacks: u64 = 0;
    get_attacks!(n_rays, true, piece, game, attacks);
    get_attacks!(s_rays, false, piece, game, attacks);
    get_attacks!(e_rays, true, piece, game, attacks);
    get_attacks!(w_rays, false, piece, game, attacks);

    return attacks;
}

fn gen_queen_mov_att(piece: &Piece, game: &Game) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= gen_bishop_mov_att(piece, game);
    attacks |= gen_rook_mov_att(piece, game);
    return attacks;
}

fn gen_king_mov_att(piece: &Piece, game: &Game) -> u64 {
    let idx = bit_scan_lsb(piece.pos);
    let mut attacks = ATTACKS.king_attacks.king_attacks[idx];

    let (own_occupancy, _) = get_occupancy(piece, game);
    attacks &= !own_occupancy;
    return attacks;
}

#[cfg(test)]
mod tests {

    use crate::engine::shared::helper_func::{
        bit_pos_utility::*,
        const_utility::{SqPos::*, FEN_CASTLE_TWO, FEN_PAWNS_BLACK, FEN_PAWNS_WHITE},
        print_utility::{print_bitboard, print_chess, print_move_list},
    };
    use super::*;

    fn test_mov_att(fen: &str, p_type: PieceType, p_color: Color, idx: usize) -> Vec<usize> {
        let game = Game::read_fen(&fen);
        // println!("{}", game.to_string());

        let allPieces = extract_all_bits(game.piece_bitboard[p_color as usize][p_type as usize]);
        let piece = match game.squares[allPieces[idx]] {
            Square::Empty => panic!("The Piece Must exist"),
            Square::Occupied(piece) => piece,
        };
        println!("{:?}", piece.p_type);
        return extract_all_bits(get_all_moves(piece, &game) | get_all_attacks(piece, &game));

        // print_bitboard(
        //     generate_knight_moves(&piece, &game),
        //     Some(bit_scan_lsb(piece.position) as i8),
        // );
    }

    #[test]
    fn test_white_pawns_mv_gen() {
        let game = Game::read_fen(&FEN_PAWNS_WHITE);
        let moves = gen_moves(Color::White, &game);
        assert_eq!(26, moves.len());
        print_move_list(moves);
    }

    #[test]
    fn test_mv_gen() {
        let game = Game::read_fen(&FEN_CASTLE_TWO);
        let moves = gen_moves(Color::White, &game);
        assert_eq!(48, moves.len());
        print_chess(&game);
        print_move_list(moves);
    }

    #[test]
    fn test_white_black_mv_gen() {
        let game = Game::read_fen(&FEN_PAWNS_BLACK);
        let moves = gen_moves(Color::Black, &game);
        assert_eq!(26, moves.len());
        print_move_list(moves);
    }

    #[test]
    fn test_attacks() {
        let fen = "8/8/2q5/3Q4/8/8/8/8 w - - 0 1";
        let game = Game::read_fen(&fen);
        print_bitboard(
            gen_attacks(&game, Color::Black),
            Some(bit_scan_lsb(game.piece_bitboard[1][4]) as i8),
        );
        print_bitboard(
            gen_attacks(&game, Color::White),
            Some(bit_scan_lsb(game.piece_bitboard[0][4]) as i8),
        );
    }

    // KNIGHT

    #[test]
    fn test_generate_knight_moves() {
        let fen = "8/8/8/4N3/2N5/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen, PieceType::Knight, Color::White, 1);
        let test_positions = vec![19, 21, 30, 42, 46, 51, 53];
        assert_eq!(test_positions, moves);
    }

    #[test]
    fn test_generate_3_knight_moves() {
        let fen_knight = "8/5N2/8/4N3/2N5/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_knight, PieceType::Knight, Color::White, 1);
        let test_positions = vec![19, 21, 30, 42, 46, 51];
        assert_eq!(test_positions, moves);
    }

    // BISHOP

    #[test]
    fn test_generate_bishop_moves_1_bishop() {
        let fen_one_bishop = "7B/8/8/8/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_one_bishop, PieceType::Bishop, Color::White, 0);
        let test_positions = vec![0, 9, 18, 27, 36, 45, 54];
        assert_eq!(test_positions, moves);
    }

    #[test]
    fn test_generate_bishop_moves_1_bishop_middle() {
        let fen_one_bishop = "8/8/8/4B3/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_one_bishop, PieceType::Bishop, Color::White, 0);

        let test_positions = vec![0, 9, 15, 18, 22, 27, 29, 43, 45, 50, 54, 57, 63];
        assert_eq!(test_positions, moves);
    }

    #[test]
    fn test_generate_bishop_moves_2_bishop() {
        let fen_one_bishop = "8/8/5B2/4B3/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_one_bishop, PieceType::Bishop, Color::White, 0);

        let test_positions = vec![0, 9, 15, 18, 22, 27, 29, 43, 50, 57];
        assert_eq!(test_positions, moves);
    }

    // ROOK

    #[test]
    fn test_generate_rook_moves_1_rook() {
        let fen_rook = "8/8/8/8/8/4R3/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_rook, PieceType::Rook, Color::White, 0);
        let test_positions = vec![4, 12, 16, 17, 18, 19, 21, 22, 23, 28, 36, 44, 52, 60];
        assert_eq!(test_positions, moves);
    }

    #[test]
    fn test_generate_rook_moves_1_rook_1enemy() {
        let fen_rook = "8/8/8/4r3/8/4R3/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_rook, PieceType::Rook, Color::White, 0);
        let test_positions = vec![4, 12, 16, 17, 18, 19, 21, 22, 23, 28, 36];
        assert_eq!(test_positions, moves);
    }

    // QUEEN

    #[test]
    fn test_generate_queen_moves() {
        let fen_queen = "8/8/8/3Q4/8/8/8/8 w - - 0 1";
        let moves = test_mov_att(&fen_queen, PieceType::Queen, Color::White, 0);
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
        let moves = test_mov_att(&fen_queen, PieceType::Queen, Color::White, 0);

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
        let moves = test_mov_att(&fen_king, PieceType::King, Color::White, 0);
        let test_positions = vec![26, 27, 28, 34, 36, 42, 43, 44];
        assert_eq!(test_positions, moves);
    }
}
