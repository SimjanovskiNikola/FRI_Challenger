use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use crate::engine::board::make_move::BoardMoveTrait;
use crate::engine::board::mv_gen::gen_captures;
use crate::engine::board::mv_gen::gen_moves;
use crate::engine::board::mv_gen::is_repetition;
use crate::engine::board::mv_gen::sq_attack;
use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::moves::Move;
use crate::engine::board::structures::piece::PieceTrait;
use crate::engine::board::structures::piece::KING;
use crate::engine::evaluation::evaluation::Evaluation;
use crate::engine::misc::bitboard::BitboardTrait;
use crate::engine::misc::print_utility::get_move_list;
use crate::engine::protocols::time::safe_to_start_next_iter;
use crate::engine::protocols::time::time_over;
use crate::engine::protocols::uci::NewUCI;

use super::transposition_table::Bound;
use super::transposition_table::TTTable;

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
    pub board: Board,
    pub tt: Arc<Mutex<TTTable>>,
    pub uci: Arc<RwLock<NewUCI>>,
    pub info: SearchInfo,
}

impl Search {
    pub fn init(board: Board, tt: Arc<Mutex<TTTable>>, uci: Arc<RwLock<NewUCI>>) -> Self {
        Self { board, tt, uci, info: SearchInfo::init() }
    }
}

pub fn check_time_up() {
    todo!();
}

pub fn clear_search(search: &mut Search) {
    search.board.s_killers.iter_mut().for_each(|arr| arr.fill(None));
    search.board.s_history.iter_mut().for_each(|arr| arr.fill(0));
    // game.info.start_time = Instant::now();
    // game.info.stopped = false;
    search.info.nodes = 0;
    search.info.curr_key = search.board.state.key;
    search.info.curr_depth = 0;

    search.tt.lock().unwrap().clear();
    search.tt.lock().unwrap().clear_stats();
}

fn quiescence_search(mut alpha: isize, beta: isize, search: &mut Search) -> isize {
    let eval = search.board.evaluate_pos();
    if eval >= beta {
        return beta;
    }

    search.info.nodes += 1;

    alpha = alpha.max(eval);

    let board = &search.board;
    let mut pos_rev = gen_captures(board.state.color, board);

    for rev in &mut pos_rev {
        if (search.info.nodes & 2047) == 0 && time_over(&search) {
            break;
        }

        if !search.board.make_move(rev) {
            continue;
        }
        let score = -quiescence_search(-beta, -alpha, search);
        search.board.undo_move();

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
    pv: &mut Vec<Move>,
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
    if search.board.state.half_move >= 100 || is_repetition(&search.board) {
        return 0;
    }

    let mut tt_guard = search.tt.lock().unwrap();
    if let Some((score, rev)) =
        tt_guard.probe(search.board.state.key, depth, alpha as i16, beta as i16)
    {
        return score as isize;
    }
    drop(tt_guard);

    let mut best_mv = None;
    let mut best_score = alpha;
    let mut legal_mv_num = 0;
    let old_alpha: isize = alpha;

    let moves = gen_moves(search.board.state.color, &mut search.board);

    for mv in &moves {
        // Check Time every 2027 Nodes
        if (search.info.nodes & 2047) == 0 && time_over(&search) {
            return 0;
        }

        if !search.board.make_move(mv) {
            continue;
        }
        legal_mv_num += 1;
        let mut node_pv: Vec<Move> = Vec::new();
        let score = -alpha_beta(-beta, -alpha, depth - 1, &mut node_pv, search, true);
        search.board.undo_move();

        if score > alpha {
            if score >= beta {
                if !mv.flag.is_capture() {
                    search.board.s_killers[search.board.ply()][0] =
                        search.board.s_killers[search.board.ply()][1];
                    search.board.s_killers[search.board.ply()][1] = Some(*mv);
                }
                search.tt.lock().unwrap().set(
                    search.board.state.key,
                    *mv,
                    score as i16,
                    depth,
                    Bound::Upper,
                );

                return score;
            }

            pv.clear();
            pv.push(*mv);
            pv.append(&mut node_pv);
            alpha = score;
            best_score = score;
            best_mv = Some(*mv);

            if !mv.flag.is_capture() {
                search.board.s_history[mv.piece.idx()][mv.to as usize] += (depth * depth) as u64;
            }
        }
    }

    // Checking for if the position is draw or checkmate
    if legal_mv_num == 0 {
        let king_sq = search.board.bitboard[(KING + search.board.state.color) as usize].get_lsb();
        return match sq_attack(&search.board, king_sq, search.board.state.color) != 0 {
            true => -1000000 + (search.board.ply() as isize),
            false => 0,
        };
    }

    if let Some(mv) = best_mv {
        let bound = if best_score > old_alpha { Bound::Exact } else { Bound::Upper };
        search.tt.lock().unwrap().set(search.board.state.key, mv, alpha as i16, depth, bound);
    }

    alpha
}

pub fn iterative_deepening(mut search: Search) -> Option<Move> {
    clear_search(&mut search);

    let max_depth = search.uci.read().unwrap().max_depth;
    let mut alpha = MIN_INF;
    let mut beta = MAX_INF;
    let mut best_mv = None;
    let mut root_pv: Vec<Move> = Vec::new();

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

    use super::*;

    // NOTE: Uncomment In Cargo.toml the pprof to see the performance.
    //     #[test]
    //     fn test_fen_bug_2_sq_pawn_dept_1() {
    //         let mut board = Game::initialize();
    //         let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).build().unwrap();

    //         game.info.depth = Some(7);
    //         iterative_deepening(&mut game);

    //         if let Ok(report) = guard.report().build() {
    //             let file = File::create("flamegraph.svg").unwrap();
    //             report.flamegraph(file).unwrap();
    //         };
    //     }
}
