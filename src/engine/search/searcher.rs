use crate::engine::{
    evaluation::evaluation::Evaluation,
    game::{self, Game},
    move_generation::{
        make_move::GameMoveTrait,
        mv_gen::{gen_captures, gen_moves, is_repetition, sq_attack},
    },
    protocols::uci::NewUCI,
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
    sync::{atomic::AtomicU64, Arc, Mutex, RwLock},
    time::{Duration, Instant},
    u64,
};

use super::{
    time::{safe_to_start_next_iter, time_over},
    transposition_table::{Bound, TTTable},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInfo {
    pub nodes: usize,
    pub curr_depth: u8,
    pub curr_key: u64,

    pub fail_hard: usize,
    pub fail_hard_first: usize,
}

impl SearchInfo {
    pub fn init() -> Self {
        Self {
            curr_depth: 0,
            // DEPRECATE: It is not used
            curr_key: 0,
            nodes: 0,
            fail_hard: 0,
            fail_hard_first: 0,
        }
    }
}

#[derive(Debug)]
pub struct Search {
    pub game: Game,
    pub tt: Arc<Mutex<TTTable>>,
    pub uci: Arc<RwLock<NewUCI>>,
    pub info: SearchInfo,
}

impl Search {
    pub fn init(game: Game, tt: Arc<Mutex<TTTable>>, uci: Arc<RwLock<NewUCI>>) -> Self {
        Self { game, tt, uci, info: SearchInfo::init() }
    }
}

pub fn check_time_up() {
    todo!();
}

pub fn clear_search(search: &mut Search) {
    search.game.s_killers.iter_mut().for_each(|arr| arr.fill(None));
    search.game.s_history.iter_mut().for_each(|arr| arr.fill(0));
    search.game.ply = 0;
    // game.info.start_time = Instant::now();
    // game.info.stopped = false;
    search.info.nodes = 0;
    search.info.curr_key = search.game.key;
    search.info.curr_depth = 0;

    search.tt.lock().unwrap().clear();
    search.tt.lock().unwrap().clear_stats();
}

fn quiescence_search(mut alpha: isize, beta: isize, search: &mut Search) -> isize {
    let eval = search.game.evaluate_pos();
    if eval >= beta {
        return beta;
    }

    search.info.nodes += 1;

    alpha = alpha.max(eval);

    let game = &search.game;
    let (irr, mut pos_rev) = gen_captures(game.color, game);

    for rev in &mut pos_rev {
        if (search.info.nodes & 2047) == 0 && time_over(&search) {
            break;
        }

        if !search.game.make_move(rev, &irr) {
            continue;
        }
        let score = -quiescence_search(-beta, -alpha, search);
        search.game.undo_move();

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
    search: &mut Search,
    take_null: bool,
) -> isize {
    // If we reached the final depth than make sure there is no horizon effect
    if depth == 0 {
        return quiescence_search(alpha, beta, search);
    }

    search.info.nodes += 1;

    // Check if the position happened before or is draw
    // TODO: There is some bug regarding repetition
    if search.game.half_move >= 100 || is_repetition(&search.game) {
        return 0;
    }

    let mut tt_guard = search.tt.lock().unwrap();
    if let Some((score, rev)) = tt_guard.probe(search.game.key, depth, alpha as i16, beta as i16) {
        return score as isize;
    }
    drop(tt_guard);

    let mut best_mv = None;
    let mut best_score = alpha;
    let mut legal_mv_num = 0;
    let old_alpha: isize = alpha;

    let (irr, pos_rev) = gen_moves(search.game.color, &search.game);

    for rev in &pos_rev {
        // Check Time every 2027 Nodes
        if (search.info.nodes & 2047) == 0 && time_over(&search) {
            return 0;
        }

        if !search.game.make_move(rev, &irr) {
            continue;
        }
        legal_mv_num += 1;
        let mut node_pv: Vec<PositionRev> = Vec::new();
        let score = -alpha_beta(-beta, -alpha, depth - 1, &mut node_pv, search, true);
        search.game.undo_move();

        if score > alpha {
            if score >= beta {
                if !rev.flag.is_capture() {
                    search.game.s_killers[search.game.ply][0] =
                        search.game.s_killers[search.game.ply][1];
                    search.game.s_killers[search.game.ply][1] = Some(*rev);
                }
                search.tt.lock().unwrap().set(
                    search.game.key,
                    *rev,
                    score as i16,
                    depth,
                    Bound::Upper,
                );

                return score;
            }

            pv.clear();
            pv.push(*rev);
            pv.append(&mut node_pv);
            alpha = score;
            best_score = score;
            best_mv = Some(*rev);

            if !rev.flag.is_capture() {
                search.game.s_history[rev.piece.idx()][rev.to as usize] += (depth * depth) as u64;
            }
        }
    }

    // Checking for if the position is draw or checkmate
    if legal_mv_num == 0 {
        let king_sq = search.game.bitboard[(KING + search.game.color) as usize].get_lsb();
        return match sq_attack(&search.game, king_sq, search.game.color) != 0 {
            true => -1000000 + (search.game.ply as isize),
            false => 0,
        };
    }

    if let Some(mv) = best_mv {
        let bound = if best_score > old_alpha { Bound::Exact } else { Bound::Upper };
        search.tt.lock().unwrap().set(search.game.key, mv, alpha as i16, depth, bound);
    }

    alpha
}

pub fn iterative_deepening(mut search: Search) -> Option<PositionRev> {
    clear_search(&mut search);

    let max_depth = search.uci.read().unwrap().max_depth;
    let mut alpha = MIN_INF;
    let mut beta = MAX_INF;
    let mut best_mv = None;
    let mut root_pv: Vec<PositionRev> = Vec::new();

    for depth in 1..max_depth + 1 {
        if !safe_to_start_next_iter(&search) {
            break;
        }

        set_curr_depth(&mut search.info, depth);
        let score = alpha_beta(alpha, beta, depth, &mut root_pv, &mut search, true);

        if time_over(&search) {
            break;
        }

        // Adjust alpha and beta using aspiration window
        (alpha, beta) = aspiration_window(alpha, beta, score, depth);

        // Get Best Line from current position and print info
        if root_pv.len() > 0 {
            best_mv = Some(root_pv[0]);
        }
        print_info(&search.info, score, get_move_list(&root_pv));
        root_pv.clear();
        // search.tt.lock().unwrap().print_stats();
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

fn set_curr_depth(info: &mut SearchInfo, depth: u8) {
    info.curr_depth = depth;
}

fn print_info(info: &SearchInfo, score: isize, line: String) {
    println!("info depth {} nodes {} score cp {} pv{}", info.curr_depth, info.nodes, score, line);
}

fn print_pruning_info(info: &SearchInfo, score: isize) {
    println!("Fail Hard First: {:?}, Fail Hard: {:?}", info.fail_hard_first, info.fail_hard);
}

fn print_ordering_info(info: &mut SearchInfo) {
    println!("Ordering: {:.4}", ((info.fail_hard_first) as f64 / (info.fail_hard + 1) as f64));
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
