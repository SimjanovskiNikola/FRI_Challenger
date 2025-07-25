use super::transposition_table::TTTable;
use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::moves::Move;
use crate::engine::misc::print_utility::get_move_list;
use crate::engine::misc::print_utility::get_pv_move_list;
use crate::engine::protocols::time::safe_to_start_next_iter;
use crate::engine::protocols::time::time_over;
use crate::engine::protocols::uci::NewUCI;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

const MIN_ASP_WINDOW_DEPTH: u8 = 6;
const MAX_INF: isize = isize::MAX / 2;
const MIN_INF: isize = isize::MIN / 2;

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

// Common Search Function
impl Search {
    pub fn init(board: Board, tt: Arc<Mutex<TTTable>>, uci: Arc<RwLock<NewUCI>>) -> Self {
        Self { board, tt, uci, info: SearchInfo::init() }
    }

    pub fn clear_search(&mut self) {
        self.board.s_killers.iter_mut().for_each(|arr| arr.fill(None));
        self.board.s_history.iter_mut().for_each(|arr| arr.fill(0));
        // game.info.start_time = Instant::now();
        // game.info.stopped = false;
        self.info.nodes = 0;
        self.info.curr_key = self.board.state.key;
        self.info.curr_depth = 0;

        // self.tt.lock().unwrap().clear();
        self.tt.lock().unwrap().clear_stats();
    }

    pub fn set_curr_depth(&mut self, depth: u8) {
        self.info.curr_depth = depth;
    }

    pub fn print_info(&self, score: isize, line: String) {
        let time = self.uci.read().unwrap().start_time.elapsed().as_millis();
        println!(
            "info depth {} nodes {} time {} score cp {} pv{}",
            self.info.curr_depth, self.info.nodes, time, score, line
        );
    }

    pub fn print_pruning_info(&self, score: isize) {
        println!(
            "Fail Hard First: {:?}, Fail Hard: {:?}",
            self.info.fail_hard_first, self.info.fail_hard
        );
    }

    pub fn print_ordering_info(&self) {
        println!(
            "Ordering: {:.4}",
            ((self.info.fail_hard_first) as f64 / (self.info.fail_hard + 1) as f64)
        );
    }
}

// Iterative Deepening
impl Search {
    pub fn iterative_deepening(&mut self) -> Option<Move> {
        self.clear_search();

        let max_depth = self.uci.read().unwrap().max_depth;
        let mut alpha = MIN_INF;
        let mut beta = MAX_INF;
        let mut best_mv = None;

        for depth in 1..max_depth + 1 {
            if !safe_to_start_next_iter(&self) {
                break;
            }

            self.set_curr_depth(depth);
            let score = self.alpha_beta(alpha, beta, depth, true);

            if time_over(&self) {
                break;
            }

            // Adjust alpha and beta using aspiration window
            (alpha, beta) = Self::aspiration_window(alpha, beta, score, depth);

            // Get Best Line from current position and print info

            let root_pv = self.tt.lock().unwrap().get_line(&mut self.board);
            if root_pv.len() > 0 {
                best_mv = Some(root_pv[0]);
            }
            self.print_info(score, get_move_list(&root_pv, self.info.curr_depth));
            self.print_ordering_info();
            // search.tt.lock().unwrap().print_stats();
        }

        best_mv
    }

    pub fn aspiration_window(alpha: isize, beta: isize, score: isize, depth: u8) -> (isize, isize) {
        match depth < MIN_ASP_WINDOW_DEPTH || (score <= alpha) || (score >= beta) {
            true => (MIN_INF, MAX_INF),
            false => (score - 30, score + 30),
        }
        // (MIN_INF, MAX_INF)
    }
}

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
