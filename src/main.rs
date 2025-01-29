use engine::game::Game;
use std::arch::x86_64::_pext_u64;

pub mod engine;

fn main() {
    let _game = Game::initialize();
}
