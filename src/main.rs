use std::env;

use engine::fen::fen::FenTrait;
use engine::game::Game;
use engine::search::searcher::SearchInfo;
use engine::shared::helper_func::const_utility::{FEN_MATE_IN_3, FEN_MATE_IN_4, FEN_MATE_IN_5};
use engine::shared::helper_func::play_chess_utility::play_chess;

pub mod engine;

fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // FIXME: Needed to backtrace the call stack
    let mut game = Game::read_fen(FEN_MATE_IN_5);
    // let mut game = Game::initialize();
    let mut info = SearchInfo::init();
    play_chess(&mut game, &mut info);
}
