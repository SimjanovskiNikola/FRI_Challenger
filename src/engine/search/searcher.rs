use crate::engine::{
    evaluation::evaluation::Evaluation,
    game::Game,
    move_generation::{
        make_move::GameMoveTrait,
        mv_gen::{gen_captures, gen_moves, is_repetition, sq_attack},
    },
    search::transposition_table::get_line,
    shared::{
        helper_func::{
            bitboard::BitboardTrait,
            print_utility::{get_move_list, move_notation, print_chess, print_move_list},
        },
        structures::{
            internal_move::{PositionIrr, PositionRev},
            piece::{PieceTrait, KING},
        },
    },
};
use std::{
    time::{Duration, Instant},
    u64,
};

use super::transposition_table::Bound;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInfo {
    pub start_time: Instant,
    pub time_limit: Option<Duration>,

    pub depth: Option<u8>,

    pub moves_togo: usize,
    pub infinite: bool,

    pub nodes: usize,
    pub curr_depth: u8,
    pub curr_key: u64,

    pub quit: bool,
    pub stopped: bool,

    pub fail_hard: usize,
    pub fail_hard_first: usize,
}

impl SearchInfo {
    pub fn init() -> Self {
        Self {
            start_time: Instant::now(),
            time_limit: None,
            depth: None,

            curr_depth: 0,
            curr_key: 0,

            moves_togo: 0,
            infinite: false,

            nodes: 0,

            quit: false,
            stopped: false,

            fail_hard: 0,
            fail_hard_first: 0,
        }
    }
}

pub fn check_time_up() {
    todo!();
}

pub fn clear_search(game: &mut Game) {
    game.s_killers.iter_mut().for_each(|arr| arr.fill(None));
    game.s_history.iter_mut().for_each(|arr| arr.fill(0));
    game.ply = 0;

    game.info.start_time = Instant::now();
    game.info.stopped = false;
    game.info.nodes = 0;
    game.info.curr_key = game.key;
    game.info.curr_depth = 0;

    game.tt.collisions = 0;
    game.tt.hits = 0;
    game.tt.inserts = 0;
    game.tt.lookups = 0;
}

fn quiescence_search(mut alpha: isize, beta: isize, game: &mut Game) -> isize {
    let eval = game.evaluate_pos();
    if eval >= beta {
        return beta;
    }

    game.info.nodes += 1;

    alpha = alpha.max(eval);

    let (irr, mut pos_rev) = gen_captures(game.color, game);

    for rev in &mut pos_rev {
        if (game.info.nodes & 2047) == 0 && time_over_or_stopped(game) {
            break;
        }

        if !game.make_move(rev, &irr) {
            continue;
        }
        let score = -quiescence_search(-beta, -alpha, game);
        game.undo_move();

        if score >= beta {
            return beta;
        }

        alpha = alpha.max(score);
    }

    alpha
}

fn alpha_beta(
    mut alpha: isize,
    mut beta: isize,
    depth: u8,
    pv: &mut Vec<PositionRev>,
    game: &mut Game,
    take_null: bool,
) -> isize {
    // If we reached the final depth than make sure there is no horizon effect
    if depth == 0 {
        return quiescence_search(alpha, beta, game);
    }

    game.info.nodes += 1;

    // Check if the position happened before or is draw
    if game.half_move >= 100 || is_repetition(game) {
        return 0;
    }

    if let Some(score) = game.tt.probe(game.key, depth, alpha as i16, beta as i16) {
        game.tt.hits += 1;
        return score as isize;
    }

    let mut best_mv = None;
    let mut best_score = alpha;
    let mut legal_mv_num = 0;
    let old_alpha: isize = alpha;

    let (irr, pos_rev) = gen_moves(game.color, game);

    for rev in &pos_rev {
        // Check Time every 2027 Nodes
        if (game.info.nodes & 2047) == 0 && time_over_or_stopped(game) {
            return 0;
        }

        if !game.make_move(rev, &irr) {
            continue;
        }
        legal_mv_num += 1;
        let mut node_pv: Vec<PositionRev> = Vec::new();
        let score = -alpha_beta(-beta, -alpha, depth - 1, &mut node_pv, game, true);
        game.undo_move();

        if score > alpha {
            if score >= beta {
                if !rev.flag.is_capture() {
                    game.s_killers[game.ply][0] = game.s_killers[game.ply][1];
                    game.s_killers[game.ply][1] = Some(*rev);
                }
                game.tt.set(game.key, *rev, score as i16, depth, Bound::Upper);

                return score;
            }

            pv.clear();
            pv.push(*rev);
            pv.append(&mut node_pv);
            alpha = score;
            best_score = score;
            best_mv = Some(rev);

            if !rev.flag.is_capture() {
                game.s_history[rev.piece.idx()][rev.to as usize] += (depth * depth) as u64;
            }
        }
    }

    // Checking for if the position is draw or checkmate
    if legal_mv_num == 0 {
        let king_sq = game.bitboard[(KING + game.color) as usize].get_lsb();
        return match sq_attack(game, king_sq, game.color) != 0 {
            true => -1000000 + (game.ply as isize),
            false => 0,
        };
    }

    if let Some(mv) = best_mv {
        let bound = if best_score > old_alpha { Bound::Exact } else { Bound::Upper };
        game.tt.set(game.key, *mv, alpha as i16, depth, bound);
    }

    alpha
}

pub fn iterative_deepening(game: &mut Game) -> Option<PositionRev> {
    clear_search(game);

    let mut alpha = MIN_INF;
    let mut beta = MAX_INF;
    let mut best_mv = None;
    let mut root_pv: Vec<PositionRev> = Vec::new();

    for depth in 1..game.info.depth.unwrap_or(9) + 1 {
        set_curr_depth(game, depth);
        let score = alpha_beta(alpha, beta, depth, &mut root_pv, game, true);

        if time_over_or_stopped(game) {
            break;
        }

        // Adjust alpha and beta using aspiration window
        (alpha, beta) = aspiration_window(alpha, beta, score, depth);

        // Get Best Line from current position and print info
        if root_pv.len() > 0 {
            best_mv = Some(root_pv[0]);
        }

        print_info(game, score, get_move_list(&root_pv));
    }

    best_mv
}

fn aspiration_window(alpha: isize, beta: isize, score: isize, depth: u8) -> (isize, isize) {
    match depth < MIN_ASP_WINDOW_DEPTH || (score <= alpha) || (score >= beta) {
        true => (MIN_INF, MAX_INF),
        false => (score - 30, score + 30),
    }
    // (MIN_INF, MAX_INF)
}

// FIXME: NOTE: Some useful small functions

fn set_curr_depth(game: &mut Game, depth: u8) {
    game.info.curr_depth = depth;
}

fn print_info(game: &mut Game, score: isize, line: String) {
    println!(
        "info depth {} nodes {} score cp {} pv{}",
        game.info.curr_depth, game.info.nodes, score, line
    );
}

fn print_pruning_info(game: &mut Game, score: isize) {
    println!(
        "Fail Hard First: {:?}, Fail Hard: {:?}",
        game.info.fail_hard_first, game.info.fail_hard
    );
}

fn print_ordering_info(game: &mut Game) {
    println!(
        "Ordering: {:.4}",
        ((game.info.fail_hard_first) as f64 / (game.info.fail_hard + 1) as f64)
    );
}

fn time_over_or_stopped(game: &Game) -> bool {
    game.info.start_time.elapsed()
        >= game.info.time_limit.unwrap_or(Duration::from_millis(u64::MAX))
        || game.info.stopped
}

const MIN_ASP_WINDOW_DEPTH: u8 = 6;
const MAX_INF: isize = isize::MAX / 2;
const MIN_INF: isize = isize::MIN / 2;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::engine::game::Game;

    use super::*;

    // NOTE: Uncomment In Cargo.toml the pprof to see the performance.
    //     #[test]
    //     fn test_fen_bug_2_sq_pawn_dept_1() {
    //         let mut game = Game::initialize();
    //         let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).build().unwrap();

    //         game.info.depth = Some(7);
    //         iterative_deepening(&mut game);

    //         if let Ok(report) = guard.report().build() {
    //             let file = File::create("flamegraph.svg").unwrap();
    //             report.flamegraph(file).unwrap();
    //         };
    //     }
}
