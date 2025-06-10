use ::engine::engine::protocols::uci::UCI;
use std::env;

use crate::engine::{
    board::{fen::FenTrait, structures::board::Board},
    evaluation::evaluation::EvaluationTrait,
    misc::print_utility::print_bitboard,
    move_generator::generated::king::KING_RING,
};

pub mod engine;

fn main() {
    // FIXME: Needed to backtrace the call stack
    // env::set_var("RUST_BACKTRACE", "1");
    // let mut uci = UCI::init();
    // uci.main();

    // let mut board = Board::read_fen("8/2p1k1p1/p3p3/2n1N3/4P2P/8/4K1P1/8 w - - 0 0"); // Endgame Good position for black -0.7
    let mut board =
        Board::read_fen("r2q1rk1/1b3ppp/p5n1/1p1pPN2/4n3/4b2P/PPB2PP1/R2QRNK1 w - - 0 0"); //
    let eval = board.evaluation();
    println!("Evaluation: {:?}", eval);
}
