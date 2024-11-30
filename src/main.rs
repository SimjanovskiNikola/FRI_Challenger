use engine::{ game::{ Game }, shared::helper_func::const_utility::* };
pub mod engine;

// This is where everything starts, In the main function of the project.
fn main() {
    let game = Game::read_fen(&FEN_2KING_2WKNIGHT);
    println!("{}", game.to_string());
    // println!("{:?}, {:?} {}", game.active_color, game.en_passant, game.fullmove_number);

    game.to_string();
}
