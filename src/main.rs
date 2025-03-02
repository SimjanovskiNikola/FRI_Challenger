use engine::fen::fen::FenTrait;
use engine::game::Game;
use engine::search::searcher::SearchInfo;
use engine::shared::helper_func::const_utility::FEN_MATE_IN_4;
use engine::shared::helper_func::play_chess_utility::play_chess;

pub mod engine;

fn main() {
    // let mut game = Game::read_fen(FEN_MATE_IN_4);
    let mut game = Game::initialize();
    let mut info = SearchInfo::init();
    play_chess(&mut game, &mut info);
}
