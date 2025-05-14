use std::env;

// use ::engine::engine::engine_s::Engine;
use ::engine::engine::protocols::uci::UCI;
use engine::fen::fen::FenTrait;
use engine::game::Game;
use engine::search::searcher::SearchInfo;
use engine::search::transposition_table::TTTable;
use engine::shared::helper_func::const_utility::{FEN_MATE_IN_3, FEN_MATE_IN_4, FEN_MATE_IN_5};
use engine::shared::helper_func::play_chess_utility::play_chess;

pub mod engine;

fn main() {
    // FIXME: Needed to backtrace the call stack
    env::set_var("RUST_BACKTRACE", "1");
    let mut uci = UCI::init();
    uci.main();

    // let mut game = Game::read_fen(FEN_MATE_IN_4);
    // let mut game = Game::initialize();
    // play_chess(&mut game);
}
