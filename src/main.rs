use engine::game::Game;
pub mod engine;

fn main() {
    let game: Game = Game::initialize();
    // println!("{}, 'Hello World'");
    game.to_string();
}
