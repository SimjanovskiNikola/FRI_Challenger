use crate::engine::game::Game;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInfo {
    start_time: usize,
    end_time: usize,
    depth: usize,
    depth_set: usize,
    time_set: usize,
    moves_togo: usize,
    infinite: bool,

    nodes: usize,

    quit: bool,
    stopped: bool,
}

// TODO: Find Better Place For This
pub fn is_repetition(game: &Game) -> bool {
    for i in (game.moves.len() - game.half_move)..game.moves.len() {
        if game.moves[i].position_key == game.pos_key {
            return true;
        }
    }

    false
}

pub fn check_time_up() {
    todo!();
}

pub fn clear_search(game: &Game, info: &SearchInfo) {
    todo!()
}

pub fn alpha_beta_search(
    alpha: usize,
    beta: usize,
    depth: usize,
    game: &Game,
    info: &SearchInfo,
    take_null: bool,
) -> usize {
    return 0;
}

pub fn quiescence_search(alpha: usize, beta: usize, game: &Game, info: &SearchInfo) -> usize {
    return 0;
}

pub fn search_position(game: &Game, info: &SearchInfo) {
    todo!()
}
