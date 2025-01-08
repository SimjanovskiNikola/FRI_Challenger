// FRI - Chess Engine

use crate::engine::{game::Game, shared::structures::*};

use lazy_static::lazy_static;
use piece_struct::Color;
use square_struct::Square;
use rand::Rng;

use super::bit_pos_utility::bit_scan_lsb;

lazy_static! {
    pub static ref PieceKeys: [[[u64; 6]; 2]; 64] = [[[rand::thread_rng().gen(); 6]; 2]; 64];
    pub static ref EpKeys: [u64; 64] = [rand::thread_rng().gen(); 64];
    pub static ref CastleKeys: [u64; 16] = [rand::thread_rng().gen(); 16];
    pub static ref SideKey: u64 = rand::thread_rng().gen();
}

// const TERMINATION_MARKERS = ['1-0', '0-1', '1/2-1/2', '*'] NOTE: Maybe i will need this

fn generate_pos_key(game: &Game) -> u64 {
    let mut final_key: u64 = 0;

    for idx in 0..64 {
        let piece = match game.squares[idx] {
            Square::Empty => continue,
            Square::Occupied(piece) => piece,
        };

        if piece.pos != 0 {
            final_key ^= PieceKeys[idx][piece.p_color as usize][piece.p_type as usize];
        }
    }

    if game.active_color == Color::White {
        final_key ^= *SideKey;
    }

    match game.en_passant {
        Some(idx) => final_key ^= EpKeys[bit_scan_lsb(idx)],
        None => (),
    }

    if game.castling_rights.as_usize() < 16 {
        final_key ^= CastleKeys[game.castling_rights.as_usize()];
    }
    return final_key;
}

#[cfg(test)]
mod tests {
    use crate::engine::shared::helper_func::const_utility::FEN_START;

    use super::*;

    #[test]
    fn test_generate_key() {
        let game = Game::read_fen(FEN_START);
        println!("{:?}", generate_pos_key(&game));
        println!("{:?}", generate_pos_key(&game));
    }
}
