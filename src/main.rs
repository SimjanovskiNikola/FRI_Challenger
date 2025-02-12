use engine::{game::Game, shared::helper_func::play_chess_utility::play_chess};

pub mod engine;

fn main() {
    let mut game = Game::initialize();
    play_chess(&mut game);
}
