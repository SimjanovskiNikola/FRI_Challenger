use crate::engine::{
    attacks::{ all_attacks::ATTACKS, knight_attacks::KnightAttacks },
    game::Game,
    shared::{
        helper_func::{
            bit_pos_utility::{ bit_scan_lsb, extract_all_bits },
            print_utility::bitboard_to_string,
        },
        structures::piece_struct::{ Piece, PieceColor, PieceType },
    },
};

fn generate_moves(game: &Game) -> Vec<Game> {
    let mut positions = vec![];

    for piece in &game.pieces {
        if piece.piece_color == game.active_color {
            match piece.piece_type {
                PieceType::Knight => {
                    let position = generate_knight_moves(&piece, &game);
                    positions.extend(position);
                }
                piece_type =>
                    panic!("Piece Type {:?} is not yet supported", piece.piece_type),

                // PieceType::Pawn => panic!("Piece Type {} is not yet supported"),
                // PieceType::Rook => panic!("Piece Type {} is not yet supported"),
                // PieceType::Knight => panic!("Piece Type {} is not yet supported"),
                // PieceType::Bishop => panic!("Piece Type {} is not yet supported"),
                // PieceType::Queen => panic!("Piece Type {} is not yet supported"),
                // PieceType::King => panic!("Piece Type {} is not yet supported"),
            }
        }
    }

    return positions;
}

fn generate_knight_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let idx = bit_scan_lsb(piece.position);
    let mut attacks = ATTACKS.knight_attacks.knight_rays[idx];
    println!("{}", bitboard_to_string(attacks, Some(idx as i8)));

    let own_occupancy = match piece.piece_color {
        PieceColor::White => game.white_occupancy,
        PieceColor::Black => game.black_occupancy,
    };

    attacks &= !own_occupancy;

    let mut new_positions = vec![];
    let potential_moves = extract_all_bits(attacks);
    for pmove in potential_moves {
        let mut new_position = game.clone();
        new_position.move_peace(piece.position, pmove);
        new_positions.push(new_position);
    }

    println!("{}", bitboard_to_string(attacks, Some(idx as i8)));

    return new_positions;
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    #[test]
    fn test_generate_knight_moves() {
        let not_alot = "8/8/8/4N3/2N5/8/8/8 w - - 0 1";

        let game = Game::read_fen(not_alot);
        println!("{}", game.to_string());

        let moves = generate_knight_moves(&game.pieces[0], &game);
        assert_eq!(moves.len(), 7);

        let test_positions = [19, 21, 30, 42, 46, 51, 53];
        for one_move in moves {
            assert_eq!(one_move.pieces.len(), 2);
            let piece = &one_move.pieces[0];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
            println!("{:?}", bit_scan_lsb(piece.position));
        }
    }

    #[test]
    fn test_generate_3_knight_moves() {
        let not_alot = "8/5N2/8/4N3/2N5/8/8/8 w - - 0 1";

        let game = Game::read_fen(not_alot);
        println!("{}", game.to_string());

        let moves = generate_knight_moves(&game.pieces[1], &game);
        assert_eq!(moves.len(), 6);

        let test_positions = [19, 21, 30, 42, 46, 51];
        for one_move in moves {
            assert_eq!(one_move.pieces.len(), 3);
            let piece = &one_move.pieces[1];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
            println!("{:?}", bit_scan_lsb(piece.position));
        }
    }
}
