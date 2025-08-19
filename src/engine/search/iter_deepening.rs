use super::transposition_table::TTTable;
use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::moves::Move;
use crate::engine::misc::print_utility::get_move_list;
use crate::engine::protocols::time::safe_to_start_next_iter;
use crate::engine::protocols::time::time_over;
use crate::engine::protocols::uci::NewUCI;
use crate::engine::search::transposition_table::TT;
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

    pub beta_cut_count: [usize; 64],
    pub beta_cut_index_sum: [usize; 64],
    pub alpha_raise_count: [usize; 64],
    pub alpha_raise_index_sum: [usize; 64],
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
            beta_cut_count: [0; 64],
            beta_cut_index_sum: [0; 64],
            alpha_raise_count: [0; 64],
            alpha_raise_index_sum: [0; 64],
        }
    }
}

#[derive(Debug)]
pub struct Search {
    pub board: Board,
    pub uci: Arc<RwLock<NewUCI>>,
    pub info: SearchInfo,
}

// Common Search Function
impl Search {
    pub fn init(board: Board, uci: Arc<RwLock<NewUCI>>) -> Self {
        Self { board, uci, info: SearchInfo::init() }
    }

    pub fn clear_search(&mut self) {
        self.board.s_killers.iter_mut().for_each(|arr| arr.fill(None));
        self.board.s_history.iter_mut().for_each(|arr| arr.fill(0));
        // game.info.start_time = Instant::now();
        // game.info.stopped = false;
        self.info.nodes = 0;
        self.info.curr_key = self.board.state.key;
        self.info.curr_depth = 0;

        TT.write().unwrap().clear_stats();
        self.board.pv_clear();
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

    pub fn print_ordering_info(&self, depth: u8) {
        let avg_beta_idx = self.info.beta_cut_index_sum[depth as usize] as f64
            / (self.info.beta_cut_count[depth as usize] + 1) as f64;

        let avg_alpha_idx = self.info.alpha_raise_index_sum[depth as usize] as f64
            / (self.info.alpha_raise_count[depth as usize] + 1) as f64;

        let fhf = self.info.fail_hard_first as f64 / (self.info.fail_hard + 1) as f64;
        println!(
            "Depth: {:?}, Avg Beta Index: {:.4}, Avg Alpha Index: {:.4}, Fail Hard First: {:.4}",
            depth, avg_beta_idx, avg_alpha_idx, fhf
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

            // Get Best Line from current position and print info
            let pv_line = self.board.get_pv();
            if !pv_line.is_empty() {
                best_mv = Some(pv_line[0]);
            }

            self.print_info(score, get_move_list(&pv_line, self.info.curr_depth));
            self.print_ordering_info(depth);
            // search.tt.lock().unwrap().print_stats();
        }

        best_mv
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::engine::board::fen::FenTrait;

    use super::*;

    #[test]
    fn test_search() {
        let uci = Arc::new(RwLock::new(NewUCI::init()));
        uci.write().unwrap().max_depth = 6;
        let board = Board::read_fen("r4rk1/ppq3pp/2p1Pn2/4p1Q1/8/2N5/PP4PP/2KR1R2 w - - 0 1");
        let mut search = Search::init(board, uci);

        search.iterative_deepening();
        let pv_line = get_move_list(&search.board.get_pv(), 0);
        assert_eq!(pv_line, " f1f6 f8f6 d1d7 f6g6 g5e7 c7a5");
        // NOTE: Best Continuation after g5e7: c7c8 or c7b8
    }

    // NOTE: Uncomment In Cargo.toml the pprof to see the performance.
    // #[test]
    // fn test_search() {
    //     let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).build().unwrap();

    //     let uci = Arc::new(RwLock::new(NewUCI::init()));
    //     uci.write().unwrap().max_depth = 6;
    //     // let board = Board::read_fen("2kr3r/pppq1pp1/3np2p/8/2pP4/4P2P/PP1N1PP1/2RQK2R b K - 1 13");
    //     // let board = Board::read_fen("r4rk1/ppq3pp/2p1Pn2/4p1Q1/8/2N5/PP4PP/2KR1R2 w - - 0 1");
    //     // let board =
    //     // Board::read_fen("2r1r3/ppqn1kp1/3b1n1p/3P1b2/Q2Pp1P1/7P/PP1N1PB1/R1B1R1K1 b - - 0 0");
    //     // let board = Board::read_fen("5rk1/ppq3pp/2p1rn2/4p1Q1/8/2N4P/PP4P1/2KRR3 w - - 0 3");
    //     let board = Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    //     let mut search = Search::init(board, uci);

    //     let mv = search.iterative_deepening();

    //     if let Ok(report) = guard.report().build() {
    //         let file = File::create("flamegraph.svg").unwrap();
    //         report.flamegraph(file).unwrap();
    //     };
    // }
}
