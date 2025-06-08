use ::engine::engine::protocols::uci::UCI;
use std::env;

use crate::engine::{
    misc::print_utility::print_bitboard, move_generator::generated::king::KING_RING,
};

pub mod engine;

fn main() {
    // FIXME: Needed to backtrace the call stack
    env::set_var("RUST_BACKTRACE", "1");
    let mut uci = UCI::init();
    uci.main();

    // let mut board = Board::read_fen(FEN_MATE_IN_4);
    // let mut board = Board::initialize();
    // play_chess(&mut board);
}
