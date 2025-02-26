use std::time::Instant;

use iai_callgrind::bincode::de;

use crate::engine::{
    game::Game,
    search::transposition_table::get_line,
    shared::{helper_func::print_utility::move_notation, structures::internal_move::InternalMove},
};

use super::{evaluation::evaluate_pos, transposition_table::PvEntry};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInfo {
    start_time: Instant,
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

pub fn clear_search(game: &mut Game, info: &mut SearchInfo) {
    game.s_killers.iter_mut().for_each(|arr| arr.fill(0));
    game.s_history.iter_mut().for_each(|arr| arr.fill(0));
    game.pv.clear();
    game.ply = 0;

    info.start_time = Instant::now();
    info.stopped = false;
    info.nodes = 0;
}

pub fn alpha_beta_search(
    alpha: isize,
    beta: isize,
    depth: usize,
    game: &mut Game,
    info: &mut SearchInfo,
    take_null: bool,
) -> isize {
    // if depth == 0 {
    //     info.nodes += 1;
    //     return evaluate_pos(game, color);
    // }

    // if is_repetition(&game) || game.half_move >= 100 {
    //     return 0;
    // }

    // if (game.ply > 100) {
    //     return evaluate_pos(game, color);
    // }

    // let mut move_list: Vec<InternalMove> = gen_moves(game.color, game);
    // for mv in &mut move_list {
    //     if !game.make_move(mv) {
    //         continue;
    //     }

    //     leaf_nodes += perft(depth - 1, game, stats);
    //     game.undo_move();
    // }
    return 0;
    // leaf_nodes
}

pub fn quiescence_search(alpha: usize, beta: usize, game: &Game, info: &SearchInfo) -> usize {
    return 0;
}

pub fn search_position(game: &mut Game, info: &mut SearchInfo) {
    let mut best_pos: Option<PvEntry> = None;
    let mut best_score = isize::MIN;
    let mut pv_moves = 0;

    clear_search(game, info);

    for depth in 1..8 {
        best_score = alpha_beta_search(isize::MIN, isize::MAX, depth, game, info, true);
        best_pos = game.pv.table[0];

        match best_pos {
            Some(mv) => pv_moves = get_line(game, mv.pos_key),
            None => break,
        }

        println!(
            "Depth: {:?}, Score: {:?}, Position: {:?}, Nodes: {:?},",
            depth, best_score, best_pos, info.nodes
        );

        // for pv_num in 0..pv_moves {
        //     let mv = game.pv.get(pos_key);
        //     print!("{:?}",);
        // }
        // println!()
    }
}
