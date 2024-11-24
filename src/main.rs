use game::Game;

pub mod game;
pub mod piece;
pub mod square;
pub mod utils;
pub mod operations;
pub mod castling;
pub mod ray_attacks;
pub mod knight_attacks;
pub mod pawn_attacks;

fn main() {
    let game: Game = Game::initialize();
    // println!("{}, 'Hello World'");
    game.to_string();
}
