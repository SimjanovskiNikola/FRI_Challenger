use std::{fs::File, time::Instant};
use super::fen::FenTrait;
use super::make_move::GameMoveTrait;
use crate::engine::game::Game;
use crate::engine::move_generation::move_generation::gen_moves;
use crate::engine::shared::structures::internal_move::*;

pub struct Stats {
    all_nodes: u64,
    nodes: u64,
    captures: u64,
    ep: u64,
    castles: u64,
    promotions: u64,
    checks: u64,
    checkmates: u64,
}

impl Stats {
    pub fn init() -> Stats {
        return Stats {
            all_nodes: 0,
            nodes: 0,
            captures: 0,
            ep: 0,
            castles: 0,
            promotions: 0,
            checks: 0,
            checkmates: 0,
        };
    }

    pub fn add_all_node(&mut self) {
        self.all_nodes += 1;
    }

    pub fn add_node(&mut self) {
        self.nodes += 1;
    }

    pub fn add_capture(&mut self) {
        self.nodes += 1;
        self.captures += 1;
    }

    pub fn add_ep(&mut self) {
        self.nodes += 1;
        self.captures += 1;
        self.ep += 1;
    }

    pub fn add_castle(&mut self) {
        self.nodes += 1;
        self.castles += 1;
    }

    pub fn add_promotion(&mut self) {
        self.nodes += 1;
        self.promotions += 1;
    }

    pub fn add_check(&mut self) {
        todo!();
    }

    pub fn add_checkmate(&mut self) {
        todo!();
    }

    pub fn print(&self) {
        println!("----------------------------");
        println!("All Nodes:    {}", self.all_nodes);
        println!("----------------------------");
        println!("Nodes:        {}", self.nodes);
        println!("----------------------------");
        println!("Captures:     {}", self.captures);
        println!("E.P:          {}", self.ep);
        println!("Castles:      {}", self.castles);
        println!("Promotions:   {}", self.promotions);
        println!("Checks:       {}", self.checks);
        println!("Checkmates:   {}", self.checkmates);
        println!("----------------------------");
    }
}

pub fn perft(depth: usize, game: &mut Game, stats: &mut Stats) -> u64 {
    let mut leaf_nodes: u64 = 0;
    stats.add_all_node();

    if depth == 0 {
        return 1;
    }

    let mut move_list: Vec<InternalMove> = gen_moves(game.color, game);
    for i in 0..move_list.len() {
        if !game.make_move(&mut move_list[i]) {
            continue;
        }

        if depth == 1 {
            match move_list[i].flag {
                Flag::Normal => stats.add_node(),
                Flag::Capture => stats.add_capture(),
                Flag::EP => stats.add_ep(),
                Flag::Promotion => stats.add_promotion(),
                Flag::KingSideCastle | Flag::QueenSideCastle => stats.add_castle(),
            }
        }

        leaf_nodes += perft(depth - 1, game, stats);
        game.undo_move();
    }
    return leaf_nodes;
}

pub fn init_test_func(fen: &str, depth: usize, dispaly_stats: bool) -> Stats {
    let mut game = Game::read_fen(&fen);
    let mut stats = Stats::init();
    let now = Instant::now();
    let nodes = perft(depth, &mut game, &mut stats);
    if dispaly_stats {
        println!("----------*Stats*-----------");
        println!("Time for {} nodes: {} ms", nodes, now.elapsed().as_millis());
        stats.print();
    }
    return stats;
}

pub fn profiler_init_test_func(fen: &str, depth: usize, dispaly_stats: bool) -> Stats {
    let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).build().unwrap();

    let stats = init_test_func(fen, depth, dispaly_stats);

    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };

    return stats;
}

#[cfg(test)]
mod tests {

    use crate::engine::shared::helper_func::const_utility::FEN_START;

    use super::*;

    pub const FEN_POS_TWO: &str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    pub const FEN_POS_THREE: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    pub const FEN_POS_FOUR: &str =
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    pub const FEN_POS_FIVE: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    pub const FEN_POS_SIX: &str =
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

    pub const FEN_BUG_2SQ_PAWN: &str = "8/8/8/K7/5p1k/8/4P3/8 w - - 0 1";

    // **** START: OTHER POSITIONS ****
    #[test]
    fn test_fen_bug_2_sq_pawn_dept_1() {
        let stats = init_test_func(&FEN_BUG_2SQ_PAWN, 1, true);
        assert_eq!(stats.nodes, 7);
    }

    #[test]
    fn test_fen_bug_2_sq_pawn_dept_2() {
        let stats = init_test_func(&FEN_BUG_2SQ_PAWN, 2, true);
        assert_eq!(stats.nodes, 44);
    }

    // **** START: STARTING POSITION ****
    #[test]
    fn test_perft_init_pos_one() {
        let stats = init_test_func(&FEN_START, 1, true);
        assert_eq!(stats.nodes, 20);
    }

    #[test]
    fn test_perft_init_pos_two() {
        let stats = init_test_func(&FEN_START, 2, true);
        assert_eq!(stats.nodes, 400);
    }

    #[test]
    fn test_perft_init_pos_three() {
        let stats = init_test_func(&FEN_START, 3, true);
        assert_eq!(stats.nodes, 8902);
    }

    #[test]
    fn test_perft_init_pos_four() {
        let stats = profiler_init_test_func(&FEN_START, 4, true);
        assert_eq!(stats.nodes, 197281);
    }

    #[test]
    fn test_perft_init_pos_five() {
        let stats = init_test_func(&FEN_START, 5, true);
        assert_eq!(stats.nodes, 4865609);
    }

    // #[test]
    // fn test_perft_init_pos_six() {
    //     let game = Game::read_fen(&FEN_START);
    //     assert_eq!(perft(6).nodes, 119060324)
    // }

    // #[test]
    // fn test_perft_init_pos_seven() {
    //     let game = Game::read_fen(&FEN_START);
    //     assert_eq!(perft(7).nodes, 3195901860)
    // }

    // #[test]
    // fn test_perft_init_pos_eight() {
    //     let game = Game::read_fen(&FEN_START);
    //     assert_eq!(perft(8).nodes, 84998978956)
    // }

    // #[test]
    // fn test_perft_init_pos_nine() {
    //     let game = Game::read_fen(&FEN_START);
    //     assert_eq!(perft(9).nodes, 2439530234167)
    // }

    // **** START: POSITION 2 ****
    #[test]
    fn test_perft_pos_two_depth_1() {
        let stats = init_test_func(&FEN_POS_TWO, 1, true);
        assert_eq!(stats.nodes, 48);
    }

    #[test]
    fn test_perft_pos_two_depth_2() {
        let stats = init_test_func(&FEN_POS_TWO, 2, true);
        assert_eq!(stats.nodes, 2039);
    }

    // FIXME: Time Needed: 210 ms; Correct: No;  More castles than expected: Mine => 3198, Theirs => 3162
    // #[test]
    // fn test_perft_pos_two_depth_3() {
    //     let stats = init_test_func(&FEN_POS_TWO, 3, true);
    //     assert_eq!(stats.nodes, 97862);
    // }

    // FIXME: Time Needed: 8160 ms; Correct: No;  More castles than expected: Mine => 128166, Theirs => 128013
    // #[test]
    // fn test_perft_pos_two_depth_4() {
    //     let stats = init_test_func(&FEN_POS_TWO, 4, true);
    //     assert_eq!(stats.nodes, 4085603);
    // }

    // FIXME: Time Needed: ??? ms; Correct: ???;
    // #[test]
    // fn test_perft_pos_two_depth_5() {
    //     let stats = init_test_func(&FEN_POS_TWO, 5, true);
    //     assert_eq!(stats.nodes, 193690690);
    // }

    // FIXME: Time Needed: ??? ms; Correct: ???;
    // #[test]
    // fn test_perft_pos_two_depth_6() {
    //     let stats = init_test_func(&FEN_POS_TWO, 6, true);
    //     assert_eq!(stats.nodes, 8031647685);
    // }

    // // POSITION 3
    #[test]
    fn test_perft_pos_three_depth_1() {
        let stats = init_test_func(&FEN_POS_THREE, 1, true);
        assert_eq!(stats.nodes, 14);
    }

    #[test]
    fn test_perft_pos_three_depth_2() {
        let stats = init_test_func(&FEN_POS_THREE, 2, true);
        assert_eq!(stats.nodes, 191);
    }
    #[test]
    fn test_perft_pos_three_depth_3() {
        let stats = init_test_func(&FEN_POS_THREE, 3, true);
        assert_eq!(stats.nodes, 2812);
    }
    #[test]
    fn test_perft_pos_three_depth_4() {
        let stats = init_test_func(&FEN_POS_THREE, 4, true);
        assert_eq!(stats.nodes, 43238);
    }
    #[test]
    fn test_perft_pos_three_depth_5() {
        let stats = init_test_func(&FEN_POS_THREE, 5, true);
        assert_eq!(stats.nodes, 674624);
    }

    // FIXME: Time Needed: 15194 ms; Correct: Yes;
    // #[test]
    // fn test_perft_pos_three_depth_6() {
    //     let stats = init_test_func(&FEN_POS_THREE, 6, true);
    //     assert_eq!(stats.nodes, 11030083);
    // }

    // #[test]
    // fn test_perft_pos_three_depth_7() {
    //     let stats = init_test_func(&FEN_POS_THREE, 7, true);
    //     assert_eq!(stats.nodes, 178633661);
    // }

    // #[test]
    // fn test_perft_pos_three_depth_8() {
    //     let stats = init_test_func(&FEN_POS_THREE, 8, true);
    //     assert_eq!(stats.nodes, 3009794393);
    // }

    // **** START: POSITION 4 ****
    #[test]
    fn test_perft_pos_four_depth_1() {
        let stats = init_test_func(&FEN_POS_FOUR, 1, true);
        assert_eq!(stats.nodes, 6);
    }

    #[test]
    fn test_perft_pos_four_depth_2() {
        let stats = init_test_func(&FEN_POS_FOUR, 2, true);
        assert_eq!(stats.nodes, 264);
    }

    #[test]
    fn test_perft_pos_four_depth_3() {
        let stats = init_test_func(&FEN_POS_FOUR, 3, true);
        assert_eq!(stats.nodes, 9467);
    }

    #[test]
    fn test_perft_pos_four_depth_4() {
        let stats = init_test_func(&FEN_POS_FOUR, 4, true);
        assert_eq!(stats.nodes, 422333);
    }

    // FIXME: Time Needed: 35346 ms; Improved To: 10439ms Correct: Yes;
    #[test]
    fn test_perft_pos_four_depth_5() {
        let stats = init_test_func(&FEN_POS_FOUR, 5, true);
        assert_eq!(stats.nodes, 15833292);
    }

    // FIXME:
    // #[test]
    // fn test_perft_pos_four_depth_6() {
    //     let game = Game::read_fen(&FEN_POS_FOUR);
    //     assert_eq!(perft(6).nodes, 706045033);
    // }

    // **** START: POSITION 5 ****

    #[test]
    fn test_perft_pos_five_depth_1() {
        let stats = init_test_func(&FEN_POS_FIVE, 1, true);
        assert_eq!(stats.nodes, 44);
    }

    #[test]
    fn test_perft_pos_five_depth_2() {
        let stats = init_test_func(&FEN_POS_FIVE, 2, true);
        assert_eq!(stats.nodes, 1486);
    }

    #[test]
    fn test_perft_pos_five_depth_3() {
        let stats = init_test_func(&FEN_POS_FIVE, 3, true);
        assert_eq!(stats.nodes, 62379);
    }

    #[test]
    fn test_perft_pos_five_depth_4() {
        let stats = init_test_func(&FEN_POS_FIVE, 4, true);
        assert_eq!(stats.nodes, 2103487);
    }

    // FIXME: Time Needed: 185666 ms; Correct: Yes;
    // #[test]
    // fn test_perft_pos_five_depth_5() {
    //     let stats = init_test_func(&FEN_POS_FIVE, 5, true);
    //     assert_eq!(stats.nodes, 89941194);
    // }

    // **** START: POSITION 6 ****

    // FIXME: Time Needed: 0 ms; Correct: No; Fails because of where is stats placed
    // #[test]
    // fn test_perft_pos_six_depth_0() {
    //     let stats = init_test_func(&FEN_POS_SIX, 0, true);
    //     assert_eq!(stats.nodes, 1);
    // }

    #[test]
    fn test_perft_pos_six_depth_1() {
        let stats = init_test_func(&FEN_POS_SIX, 1, true);
        assert_eq!(stats.nodes, 46);
    }

    #[test]
    fn test_perft_pos_six_depth_2() {
        let stats = init_test_func(&FEN_POS_SIX, 2, true);
        assert_eq!(stats.nodes, 2079);
    }

    #[test]
    fn test_perft_pos_six_depth_3() {
        let stats = init_test_func(&FEN_POS_SIX, 3, true);
        assert_eq!(stats.nodes, 89890);
    }

    #[test]
    fn test_perft_pos_six_depth_4() {
        let stats = init_test_func(&FEN_POS_SIX, 4, true);
        assert_eq!(stats.nodes, 3894594);
    }

    // FIXME: Time Needed: ??? ms; Correct: ???;
    // #[test]
    // fn test_perft_pos_six_depth_5() {
    //     let stats = init_test_func(&FEN_POS_SIX, 5, true);
    //     assert_eq!(stats.nodes, 164075551);
    // }

    // FIXME: Time Needed: ??? ms; Correct: ???;
    // #[test]
    // fn test_perft_pos_six_depth_6() {
    //     let game = Game::read_fen(&FEN_POS_SIX);
    //     assert_eq!(perft(6).nodes, 6923051137)
    // }

    // FIXME: Time Needed: ??? ms; Correct: ???;
    // #[test]
    // fn test_perft_pos_six_depth_7() {
    //     let game = Game::read_fen(&FEN_POS_SIX);
    //     assert_eq!(perft(7).nodes, 287188994746)
    // }

    // FIXME: Time Needed: ??? ms; Correct: ???;
    // #[test]
    // fn test_perft_pos_six_depth_8() {
    //     let game = Game::read_fen(&FEN_POS_SIX);
    //     assert_eq!(perft(8).nodes, 11923589843526)
    // }

    // FIXME: Time Needed: ??? ms; Correct: ???;
    // #[test]
    // fn test_perft_pos_six_depth_9() {
    //     let game = Game::read_fen(&FEN_POS_SIX);
    //     assert_eq!(perft(9).nodes, 490154852788714)
    // }
}
