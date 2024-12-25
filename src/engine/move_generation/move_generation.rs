use crate::engine::{
    attacks::{all_attacks::ATTACKS, ray_attacks::blocked_ray_attack},
    game::Game,
    shared::{
        helper_func::bit_pos_utility::*,
        structures::piece_struct::{Piece, PieceColor, PieceType},
    },
};

macro_rules! get_attacks {
    ($rays:ident, $forward:expr, $piece:ident, $game:ident, $moves:ident) => {
        let idx = bit_scan_lsb($piece.position);
        let attacks = &ATTACKS.ray_attacks.$rays;

        let (own_occ, enemy_occ) = get_occupancy($piece, $game);

        let ray_attacks =
            blocked_ray_attack(attacks[idx], &attacks, $forward, own_occ, enemy_occ);

        $moves.extend(get_new_positions(ray_attacks, $piece, $game))
    };
}

fn generate_moves(game: &Game) -> Vec<Game> {
    let mut positions = vec![];

    for piece in &game.pieces {
        if piece.piece_color == game.active_color {
            let position;
            match piece.piece_type {
                PieceType::Knight => position = generate_knight_moves(&piece, &game),
                PieceType::Bishop => position = generate_bishop_moves(&piece, &game),
                PieceType::Rook => position = generate_bishop_moves(&piece, &game),
                PieceType::Queen => position = generate_queen_moves(&piece, &game),
                PieceType::King => position = generate_king_moves(&piece, &game),
                _ => panic!("Piece Type {:?} is not yet supported", piece.piece_type),
            }
            positions.extend(position);
        }
    }
    return positions;
}

fn generate_knight_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let idx = bit_scan_lsb(piece.position);
    let mut attacks = ATTACKS.knight_attacks.knight_attacks[idx];

    let (own_occupancy, _) = get_occupancy(piece, game);
    attacks &= !own_occupancy;

    return get_new_positions(attacks, piece, game);
}

fn generate_bishop_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let mut new_positions = vec![];
    get_attacks!(nw_rays, true, piece, game, new_positions);
    get_attacks!(sw_rays, false, piece, game, new_positions);
    get_attacks!(ne_rays, true, piece, game, new_positions);
    get_attacks!(se_rays, false, piece, game, new_positions);

    return new_positions;
}

fn generate_rook_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let mut new_positions = vec![];
    get_attacks!(n_rays, true, piece, game, new_positions);
    get_attacks!(s_rays, false, piece, game, new_positions);
    get_attacks!(e_rays, true, piece, game, new_positions);
    get_attacks!(w_rays, false, piece, game, new_positions);

    return new_positions;
}

fn generate_queen_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let mut new_positions = vec![];
    new_positions.extend(generate_bishop_moves(piece, game));
    new_positions.extend(generate_rook_moves(piece, game));
    return new_positions;
}

fn generate_king_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let idx = bit_scan_lsb(piece.position);
    let mut attacks = ATTACKS.king_attacks.king_attacks[idx];

    let (own_occupancy, _) = get_occupancy(piece, game);
    attacks &= !own_occupancy;

    return get_new_positions(attacks, piece, game);
}

fn get_occupancy(piece: &Piece, game: &Game) -> (u64, u64) {
    match piece.piece_color {
        PieceColor::White => return (game.white_occupancy, game.black_occupancy),
        PieceColor::Black => return (game.black_occupancy, game.white_occupancy),
    };
}

fn get_new_positions(attacks: u64, piece: &Piece, game: &Game) -> Vec<Game> {
    let potential_moves = extract_all_bits(attacks);
    let mut new_positions = vec![];
    for pmove in potential_moves {
        let mut new_position = game.clone();
        new_position.move_peace(piece.position, pmove);
        new_positions.push(new_position);
    }
    return new_positions;
}

#[cfg(test)]
mod tests {
    use crate::engine::shared::helper_func::bit_pos_utility::notation_to_idx;
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

    #[test]
    fn test_generate_rook_moves_1_rook() {
        let fen_one_bishop = "8/8/8/8/8/4R3/8/8 w - - 0 1";
        let game = Game::read_fen(fen_one_bishop);
        println!("{}", game.to_string());

        let moves = generate_rook_moves(&game.pieces[0], &game);
        let test_positions = [28, 36, 44, 52, 60, 12, 4, 19, 18, 17, 16, 21, 22, 23];
        assert_eq!(moves.len(), test_positions.len());

        for one_move in moves {
            let piece = &one_move.pieces[0];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
        }
    }

    #[test]
    fn test_generate_rook_moves_1_rook_1enemy() {
        let fen_one_bishop = "8/8/8/4r3/8/4R3/8/8 w - - 0 1";
        let game = Game::read_fen(fen_one_bishop);
        println!("{}", game.to_string());

        let moves = generate_rook_moves(&game.pieces[1], &game);
        let test_positions = [28, 36, 12, 4, 19, 18, 17, 16, 21, 22, 23];
        assert_eq!(moves.len(), test_positions.len());

        for one_move in moves {
            let piece = &one_move.pieces[0];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
        }
    }

    #[test]
    fn test_generate_queen_moves() {
        let fen_one_bishop = "8/8/8/3Q4/8/8/8/8 w - - 0 1";
        let game = Game::read_fen(fen_one_bishop);
        println!("{}", game.to_string());

        let moves = generate_queen_moves(&game.pieces[0], &game);
        let test_positions = notation_to_idx(&[
            "a2", "b3", "c4", "e6", "f7", "g8", "a8", "b7", "c6", "e4", "f3", "g2", "h1",
            "d1", "d2", "d3", "d4", "d6", "d7", "d8", "a5", "b5", "c5", "e5", "f5", "g5",
            "h5",
        ]);
        assert_eq!(moves.len(), test_positions.len());

        for one_move in moves {
            let piece = &one_move.pieces[0];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
        }
    }

    #[test]
    fn test_generate_queen_moves_one_enemy() {
        let fen_one_bishop = "8/8/2q5/3Q4/8/8/8/8 w - - 0 1";
        let game = Game::read_fen(fen_one_bishop);
        println!("{}", game.to_string());

        let moves = generate_queen_moves(&game.pieces[1], &game);
        let test_positions = notation_to_idx(&[
            "a2", "b3", "c4", "e6", "f7", "g8", "c6", "e4", "f3", "g2", "h1", "d1", "d2",
            "d3", "d4", "d6", "d7", "d8", "a5", "b5", "c5", "e5", "f5", "g5", "h5",
        ]);
        assert_eq!(moves.len(), test_positions.len());

        for one_move in moves {
            let piece = &one_move.pieces[0];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
        }
    }

    #[test]
    fn test_generate_king_moves() {
        let fen_one_bishop = "8/8/8/3K4/8/8/8/8 w - - 0 1";
        let game = Game::read_fen(fen_one_bishop);
        println!("{}", game.to_string());

        let moves = generate_king_moves(&game.pieces[0], &game);
        let test_positions =
            notation_to_idx(&["c4", "c5", "c6", "d4", "d6", "e4", "e5", "e6"]);
        assert_eq!(moves.len(), test_positions.len());

        for one_move in moves {
            let piece = &one_move.pieces[0];
            let idx = bit_scan_lsb(piece.position);
            assert!(test_positions.contains(&idx));
        }
    }
    // FIXME: Not covered that the kings (Black and White) should be 1 square appart from each other. They make wall one another
}
