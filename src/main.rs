use engine::game::{ self, Game };
pub mod engine;

fn main() {
    let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let not_alot = "5k2/8/8/4N3/8/8/8/5K2 w - - 0 1";
    let not_alot2 = "5k2/8/8/4N3/2N5/8/8/5K2 w - - 0 1";
    let game = Game::read_fen(&not_alot2);
    println!("{}", game.to_string());
    println!("{:?}, {:?} {}", game.active_color, game.en_passant, game.fullmove_number);

    // println!("{}, 'Hello World'");
    game.to_string();
}
