use crate::engine::{
    attacks::knight_attacks::KnightAttacks,
    game::Game,
    shared::{
        helper_func::utils::{ bit_scan, bitboard_to_string, extract_bits },
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
    let idx = bit_scan(piece.position);
    let mut attacks = game.allAttacks.knight_attacks.knight_rays[idx];
    println!("{:?}", attacks);
    println!("{}", bitboard_to_string(attacks, Some(idx)));

    let own_occupancy = match piece.piece_color {
        PieceColor::White => game.white_occupancy,
        PieceColor::Black => game.black_occupancy,
    };

    let enemy_occupancy = match piece.piece_color {
        PieceColor::White => game.black_occupancy,
        PieceColor::Black => game.white_occupancy,
    };
    let potential_moves = extract_bits(attacks);
    for pmove in potential_moves {
        let mut new_position = game.clone();
        new_position.move_peace(piece.position, pmove);
    }
    attacks &= !own_occupancy;
    println!("{}", bitboard_to_string(attacks, Some(idx)));

    todo!()
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
    }
}
