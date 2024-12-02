use crate::engine::{
    attacks::{
        all_attacks::ATTACKS,
        knight_attacks::KnightAttacks,
        ray_attacks::blocked_ray_attack,
    },
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
            let position;
            match piece.piece_type {
                PieceType::Knight => {
                    position = generate_knight_moves(&piece, &game);
                }
                PieceType::Bishop => {
                    position = generate_bishop_moves(&piece, &game);
                }
                piece_type =>
                    panic!("Piece Type {:?} is not yet supported", piece.piece_type),

                // PieceType::Pawn => panic!("Piece Type {} is not yet supported"),
                // PieceType::Rook => panic!("Piece Type {} is not yet supported"),
                // PieceType::Queen => panic!("Piece Type {} is not yet supported"),
                // PieceType::King => panic!("Piece Type {} is not yet supported"),
            }
            positions.extend(position);
        }
    }

    return positions;
}

fn generate_knight_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let idx = bit_scan_lsb(piece.position);
    let mut attacks = ATTACKS.knight_attacks.knight_rays[idx];

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

    return new_positions;
}

fn generate_bishop_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let idx = bit_scan_lsb(piece.position);
    let attacks = &ATTACKS.ray_attacks;

    let (own_occupancy, enemy_occupancy) = match piece.piece_color {
        PieceColor::White => (game.white_occupancy, game.black_occupancy),
        PieceColor::Black => (game.black_occupancy, game.white_occupancy),
    };

    let mut new_positions = vec![];

    macro_rules! get_attacks {
        ($rays:ident, $forward:expr) => {
            let ray_attacks = blocked_ray_attack(
                attacks.$rays[idx],
                &attacks.$rays,
                $forward,
                own_occupancy,
                enemy_occupancy
            );
            let potential_moves = extract_all_bits(ray_attacks);
            for pmove in potential_moves {
                let mut new_position = game.clone();
                new_position.move_peace(piece.position, pmove);
                new_positions.push(new_position);
            }
        };
    }

    get_attacks!(nw_rays, true);
    get_attacks!(sw_rays, false);
    get_attacks!(ne_rays, true);
    get_attacks!(se_rays, false);

    return new_positions;
}

#[cfg(test)]
mod tests {
    use std::usize;

    use crate::engine::shared::helper_func::print_utility::print_bitboard;

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

    #[test]
    fn test_generate_bishop_moves_1_bishop() {
        let fen_one_bishop = "7B/8/8/8/8/8/8/8 w - - 0 1";
        let game = Game::read_fen(fen_one_bishop);
        println!("{}", game.to_string());

        let moves = generate_bishop_moves(&game.pieces[0], &game);
        let test_positions = [0, 9, 18, 27, 36, 45, 54];
        assert_eq!(moves.len(), test_positions.len());

        for one_move in moves {
            let piece = &one_move.pieces[0];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
        }
    }

    #[test]
    fn test_generate_bishop_moves_1_bishop_middle() {
        let fen_one_bishop = "8/8/8/4B3/8/8/8/8 w - - 0 1";
        let game = Game::read_fen(fen_one_bishop);
        println!("{}", game.to_string());

        let moves = generate_bishop_moves(&game.pieces[0], &game);
        let test_positions = [45, 54, 63, 27, 18, 9, 0, 43, 50, 57, 29, 22, 15];
        assert_eq!(moves.len(), test_positions.len());

        for one_move in moves {
            let piece = &one_move.pieces[0];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
        }
    }

    #[test]
    fn test_generate_bishop_moves_2_bishop() {
        let fen_one_bishop = "8/8/5B2/4B3/8/8/8/8 w - - 0 1";
        let game = Game::read_fen(fen_one_bishop);
        println!("{}", game.to_string());

        let moves = generate_bishop_moves(&game.pieces[1], &game);
        let test_positions = [27, 18, 9, 0, 43, 50, 57, 29, 22, 15];
        assert_eq!(moves.len(), test_positions.len());

        for one_move in moves {
            let piece = &one_move.pieces[1];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
        }
    }
}
