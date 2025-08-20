use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::moves::Move;
use crate::engine::misc::display::display_moves::get_move_list;
use crate::engine::misc::display::display_stats::DisplayStatsTrait;
use crate::engine::protocols::time::safe_to_start_next_iter;
use crate::engine::protocols::time::time_over;
use crate::engine::protocols::uci::NewUCI;
use crate::engine::search::transposition_table::TT;
use std::sync::Arc;
use std::sync::RwLock;

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

        self.info.nodes = 0;
        self.info.curr_key = self.board.state.key;
        self.info.curr_depth = 0;

        TT.write().unwrap().clear_stats();
        self.board.pv_clear();
    }

    pub fn set_curr_depth(&mut self, depth: u8) {
        self.info.curr_depth = depth;
    }
}

// Iterative Deepening
impl Search {
    pub fn iterative_deepening(&mut self) -> Option<Move> {
        self.clear_search();

        let max_depth = self.uci.read().unwrap().max_depth;
        let alpha = MIN_INF;
        let beta = MAX_INF;
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

    fn test_search(fen: &str, depth: u8, expected_pv: &str) {
        let uci = Arc::new(RwLock::new(NewUCI::init()));
        uci.write().unwrap().max_depth = depth;
        let board = Board::read_fen(fen);
        let mut search = Search::init(board, uci);

        search.iterative_deepening();
        let pv_line = get_move_list(&search.board.get_pv(), depth);
        assert_eq!(pv_line, expected_pv);
    }

    #[test]
    fn test_iter_deep_passed_pawn() {
        let depth = 6;
        let fen = "r4rk1/ppq3pp/2p1Pn2/4p1Q1/8/2N5/PP4PP/2KR1R2 w - - 0 1";
        let expected_pv = " f1f6 f8f6 d1d7 f6g6 g5e7 c7a5";
        // NOTE: Best Continuation after g5e7: c7c8 or c7b8
        test_search(fen, depth, expected_pv);
    }

    #[test]
    fn test_iter_deep_exchange() {
        let depth = 7;
        let fen = "8/2P1P3/b1B2p2/1pPRp3/2k3P1/P4pK1/nP3p1p/N7 w - - 0 1";
        // NOTE: Best Continuation after h2h1n: b5b4 or c3b2
        // let expected_pv = " b2b3 c4c3 d5d1 f2f1q d1f1 h2h1n"; // Depth 6
        let expected_pv = " d5d1 a2c3 b2c3 h2h1q d1h1"; // Depth 7
        test_search(fen, depth, expected_pv);
    }

    // NOTE: FIXME: Engine should look deeper before uncommenting the test
    // #[test]
    // fn test_iter_deep_fortress() {
    //     let depth = 8;
    //     let fen = "8/8/8/8/4kp2/1R6/P2q1PPK/8 w - - 0 1";
    //     let expected_pv = " b2b3 c4c3 d5d1 f2f1q d1f1 h2h1n";
    //     // NOTE: Best Continuation after h2h1n: b5b4 or c3b2
    //     test_search(fen, depth, expected_pv);
    // }

    #[test]
    fn test_iter_deep_fortress() {
        let depth = 6;
        let fen = "1r4k1/1nq3pp/pp1pp1r1/8/PPP2P2/6P1/5N1P/2RQR1K1 w - - 0 1";
        let expected_pv = " f4f5 e6f5 d1d5 c7f7 e1e7 f7d5";
        test_search(fen, depth, expected_pv);
    }

    #[test]
    fn test_iter_deep_queen_sac() {
        let depth = 6;
        let fen = "2k2Br1/p6b/Pq1r4/1p2p1b1/1Ppp2p1/Q1P3N1/5RPP/R3N1K1 b - - 0 1";
        let expected_pv = " g5e3 f8d6 e3f2 g1f2 b6d6 f2g1";
        test_search(fen, depth, expected_pv);
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
