use engine::{
    attacks::pext::{gen_bishop_mov, gen_moves, BISHOP_MASKS},
    game::Game,
    shared::helper_func::{const_utility::SqPos, print_utility::print_bitboard},
};
use std::{arch::x86_64::_pext_u64, thread::Builder};

pub mod engine;

fn main() {
    let _game = Game::initialize();
}
