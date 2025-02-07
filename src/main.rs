use engine::{
    game::Game, move_generation::perft::init_test_func,
    shared::helper_func::const_utility::FEN_START,
};

pub mod engine;

fn main() {
    let _game = Game::initialize();
    let stats = init_test_func(&FEN_START, 5, true);
    assert_eq!(stats.nodes, 4865609);
}
