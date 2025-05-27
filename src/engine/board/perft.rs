use super::make_move::BoardMoveTrait;
use super::mv_gen::BoardGenMoveTrait;
use super::structures::board::Board;
use super::structures::moves::{Flag, Move};
use crate::engine::board::fen::FenTrait;
use std::fs::File;
use std::time::Instant;

pub struct Stats {
    pub all_nodes: u64,
    pub nodes: u64,
    pub captures: u64,
    pub ep: u64,
    pub castles: u64,
    pub promotions: u64,
    pub checks: u64,
    pub checkmates: u64,
}

impl Stats {
    pub fn init() -> Stats {
        Stats {
            all_nodes: 0,
            nodes: 0,
            captures: 0,
            ep: 0,
            castles: 0,
            promotions: 0,
            checks: 0,
            checkmates: 0,
        }
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

pub fn perft(depth: usize, board: &mut Board, stats: &mut Stats) -> u64 {
    let mut leaf_nodes: u64 = 0;
    stats.add_all_node();

    if depth == 0 {
        return 1;
    }

    let mut moves = board.gen_moves();
    for mv in &mut moves {
        if !board.make_move(mv) {
            continue;
        }

        if depth == 1 {
            match mv.flag {
                Flag::Quiet => stats.add_node(),
                Flag::Capture(_) => stats.add_capture(),
                Flag::EP => stats.add_ep(),
                Flag::Promotion(_, _) => stats.add_promotion(),
                Flag::KingCastle | Flag::QueenCastle => stats.add_castle(),
            }
        }

        leaf_nodes += perft(depth - 1, board, stats);
        board.undo_move();
    }

    leaf_nodes
}

pub fn init_test_func(fen: &str, depth: usize, dispaly_stats: bool) -> Stats {
    let mut board = Board::read_fen(fen);
    let mut stats = Stats::init();
    let now = Instant::now();
    let nodes = perft(depth, &mut board, &mut stats);
    if dispaly_stats {
        println!("----------*Stats*-----------");
        println!("Time for {} nodes: {} ms", nodes, now.elapsed().as_millis());
        stats.print();
    }

    stats
}


// It is commented Out because I can't cargo build --release my engine without going into the Dev Container
// NOTE: To use uncomment the pprof inside Cargo.toml
// pub fn profiler_init_test_func(fen: &str, depth: usize, dispaly_stats: bool) -> Stats {
//     let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).build().unwrap();

//     let stats = init_test_func(fen, depth, dispaly_stats);

//     if let Ok(report) = guard.report().build() {
//         let file = File::create("flamegraph.svg").unwrap();
//         report.flamegraph(file).unwrap();
//     };

//     stats
// }

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::engine::misc::const_utility::{
        FEN_BUG_2SQ_PAWN, FEN_POS_FIVE, FEN_POS_FOUR, FEN_POS_SIX, FEN_POS_THREE, FEN_POS_TWO,
        FEN_START,
    };

    use super::*;

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
        let stats = init_test_func(&FEN_START, 4, true);
        assert_eq!(stats.nodes, 197281);
    }

    #[test]
    fn test_perft_init_pos_five() {
        let stats = init_test_func(&FEN_START, 5, true);
        assert_eq!(stats.nodes, 4865609);
    }

    #[test]
    fn test_perft_init_pos_six() {
        let stats = init_test_func(&FEN_START, 6, true);
        assert_eq!(stats.nodes, 119060324);
    }

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

    #[test]
    fn test_perft_pos_two_depth_3() {
        let stats = init_test_func(&FEN_POS_TWO, 3, true);
        assert_eq!(stats.nodes, 97862);
    }

    #[test]
    fn test_perft_pos_two_depth_4() {
        let stats = init_test_func(&FEN_POS_TWO, 4, true);
        assert_eq!(stats.nodes, 4085603);
    }

    #[test]
    fn test_perft_pos_two_depth_5() {
        let stats = init_test_func(&FEN_POS_TWO, 5, true);
        assert_eq!(stats.nodes, 193690690);
    }

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

    #[test]
    fn test_perft_pos_three_depth_6() {
        let stats = init_test_func(&FEN_POS_THREE, 6, true);
        assert_eq!(stats.nodes, 11030083);
    }

    #[test]
    fn test_perft_pos_three_depth_7() {
        let stats = init_test_func(&FEN_POS_THREE, 7, true);
        assert_eq!(stats.nodes, 178633661);
    }

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

    #[test]
    fn test_perft_pos_four_depth_5() {
        let stats = init_test_func(&FEN_POS_FOUR, 5, true);
        assert_eq!(stats.nodes, 15833292);
    }

    // #[test]
    // fn test_perft_pos_four_depth_6() {
    //     let stats = init_test_func(&FEN_POS_FOUR, 6, true);
    //     assert_eq!(stats.nodes, 706045033);
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

    #[test]
    fn test_perft_pos_five_depth_5() {
        let stats = init_test_func(&FEN_POS_FIVE, 5, true);
        assert_eq!(stats.nodes, 89941194);
    }

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

    #[test]
    fn test_perft_pos_six_depth_5() {
        let stats = init_test_func(&FEN_POS_SIX, 5, true);
        assert_eq!(stats.nodes, 164075551);
    }

    // FIXME: Time Needed: ??? ms; Correct: ???;
    // #[test]
    // fn test_perft_pos_six_depth_6() {
    //     let stats = init_test_func(&FEN_POS_SIX, 6, true);
    //     assert_eq!(stats.nodes, 6923051137);
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

    #[test]
    fn test_all_perft_test() {
        for full_line in &ALL_PERFT_TESTS {
            
            let mut parts = full_line.split(" ;");
            let fen = parts.next().expect("test_line must contain at least a FEN").trim();
            for part in parts {
                let mut depth_node = part.split(" ");

                let depth = depth_node.next().unwrap().replace("D", ""); 
                let depth_num = depth.parse::<usize>().unwrap();

                let nodes = depth_node.next().unwrap(); 
                let nodes_num = nodes.parse::<u64>().unwrap(); 
                
                let stats = init_test_func(&fen, depth_num, false);
                assert_eq!(stats.nodes, nodes_num);
                println!("Fen: {:?}, depth: {:?}, nodes: {:?}", fen, depth_num, nodes_num ); 
            }
        }
    }
}

pub const ALL_PERFT_TESTS: [&str; 172] = [
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ;D1 20 ;D2 400 ;D3 8902 ;D4 197281 ;D5 4865609",
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1 ;D1 20 ;D2 400 ;D3 8902 ;D4 197281 ;D5 4865609",
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ;D1 48 ;D2 2039 ;D3 97862 ;D4 4085603",
    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ;D1 14 ;D2 191 ;D3 2812 ;D4 43238 ;D5 674624",
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1 ;D1 6 ;D2 264 ;D3 9467 ;D4 422333",
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ;D1 46 ;D2 2079 ;D3 89890 ;D4 3894594",
    "4k3/8/8/8/8/8/8/4K2R w K - 0 1 ;D1 15 ;D2 66 ;D3 1197 ;D4 7059 ;D5 133987",
    "4k3/8/8/8/8/8/8/R3K3 w Q - 0 1 ;D1 16 ;D2 71 ;D3 1287 ;D4 7626 ;D5 145232",
    "4k2r/8/8/8/8/8/8/4K3 w k - 0 1 ;D1 5 ;D2 75 ;D3 459 ;D4 8290 ;D5 47635",
    "r3k3/8/8/8/8/8/8/4K3 w q - 0 1 ;D1 5 ;D2 80 ;D3 493 ;D4 8897 ;D5 52710",
    "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1 ;D1 26 ;D2 112 ;D3 3189 ;D4 17945 ;D5 532933",
    "r3k2r/8/8/8/8/8/8/4K3 w kq - 0 1 ;D1 5 ;D2 130 ;D3 782 ;D4 22180 ;D5 118882",
    "8/8/8/8/8/8/6k1/4K2R w K - 0 1 ;D1 12 ;D2 38 ;D3 564 ;D4 2219 ;D5 37735",
    "8/8/8/8/8/8/1k6/R3K3 w Q - 0 1 ;D1 15 ;D2 65 ;D3 1018 ;D4 4573 ;D5 80619",
    "4k2r/6K1/8/8/8/8/8/8 w k - 0 1 ;D1 3 ;D2 32 ;D3 134 ;D4 2073 ;D5 10485",
    "r3k3/1K6/8/8/8/8/8/8 w q - 0 1 ;D1 4 ;D2 49 ;D3 243 ;D4 3991 ;D5 20780",
    "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1 ;D1 26 ;D2 568 ;D3 13744 ;D4 314346",
    "r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1 ;D1 25 ;D2 567 ;D3 14095 ;D4 328965",
    "r3k2r/8/8/8/8/8/8/2R1K2R w Kkq - 0 1 ;D1 25 ;D2 548 ;D3 13502 ;D4 312835",
    "r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq - 0 1 ;D1 25 ;D2 547 ;D3 13579 ;D4 316214",
    "1r2k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1 ;D1 26 ;D2 583 ;D3 14252 ;D4 334705",
    "2r1k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1 ;D1 25 ;D2 560 ;D3 13592 ;D4 317324",
    "r3k1r1/8/8/8/8/8/8/R3K2R w KQq - 0 1 ;D1 25 ;D2 560 ;D3 13607 ;D4 320792",
    "4k3/8/8/8/8/8/8/4K2R b K - 0 1 ;D1 5 ;D2 75 ;D3 459 ;D4 8290 ;D5 47635",
    "4k3/8/8/8/8/8/8/R3K3 b Q - 0 1 ;D1 5 ;D2 80 ;D3 493 ;D4 8897 ;D5 52710",
    "4k2r/8/8/8/8/8/8/4K3 b k - 0 1 ;D1 15 ;D2 66 ;D3 1197 ;D4 7059 ;D5 133987",
    "r3k3/8/8/8/8/8/8/4K3 b q - 0 1 ;D1 16 ;D2 71 ;D3 1287 ;D4 7626 ;D5 145232",
    "4k3/8/8/8/8/8/8/R3K2R b KQ - 0 1 ;D1 5 ;D2 130 ;D3 782 ;D4 22180 ;D5 118882",
    "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1 ;D1 26 ;D2 112 ;D3 3189 ;D4 17945 ;D5 532933",
    "8/8/8/8/8/8/6k1/4K2R b K - 0 1 ;D1 3 ;D2 32 ;D3 134 ;D4 2073 ;D5 10485",
    "8/8/8/8/8/8/1k6/R3K3 b Q - 0 1 ;D1 4 ;D2 49 ;D3 243 ;D4 3991 ;D5 20780",
    "4k2r/6K1/8/8/8/8/8/8 b k - 0 1 ;D1 12 ;D2 38 ;D3 564 ;D4 2219 ;D5 37735",
    "r3k3/1K6/8/8/8/8/8/8 b q - 0 1 ;D1 15 ;D2 65 ;D3 1018 ;D4 4573 ;D5 80619",
    "r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1 ;D1 26 ;D2 568 ;D3 13744 ;D4 314346",
    "r3k2r/8/8/8/8/8/8/1R2K2R b Kkq - 0 1 ;D1 26 ;D2 583 ;D3 14252 ;D4 334705",
    "r3k2r/8/8/8/8/8/8/2R1K2R b Kkq - 0 1 ;D1 25 ;D2 560 ;D3 13592 ;D4 317324",
    "r3k2r/8/8/8/8/8/8/R3K1R1 b Qkq - 0 1 ;D1 25 ;D2 560 ;D3 13607 ;D4 320792",
    "1r2k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1 ;D1 25 ;D2 567 ;D3 14095 ;D4 328965",
    "2r1k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1 ;D1 25 ;D2 548 ;D3 13502 ;D4 312835",
    "r3k1r1/8/8/8/8/8/8/R3K2R b KQq - 0 1 ;D1 25 ;D2 547 ;D3 13579 ;D4 316214",
    "8/1n4N1/2k5/8/8/5K2/1N4n1/8 w - - 0 1 ;D1 14 ;D2 195 ;D3 2760 ;D4 38675 ;D5 570726",
    "8/1k6/8/5N2/8/4n3/8/2K5 w - - 0 1 ;D1 11 ;D2 156 ;D3 1636 ;D4 20534 ;D5 223507",
    "8/8/4k3/3Nn3/3nN3/4K3/8/8 w - - 0 1 ;D1 19 ;D2 289 ;D3 4442 ;D4 73584 ;D5 1198299",
    "K7/8/2n5/1n6/8/8/8/k6N w - - 0 1 ;D1 3 ;D2 51 ;D3 345 ;D4 5301 ;D5 38348",
    "k7/8/2N5/1N6/8/8/8/K6n w - - 0 1 ;D1 17 ;D2 54 ;D3 835 ;D4 5910 ;D5 92250",
    "8/1n4N1/2k5/8/8/5K2/1N4n1/8 b - - 0 1 ;D1 15 ;D2 193 ;D3 2816 ;D4 40039",
    "8/1k6/8/5N2/8/4n3/8/2K5 b - - 0 1 ;D1 16 ;D2 180 ;D3 2290 ;D4 24640 ;D5 288141",
    "8/8/3K4/3Nn3/3nN3/4k3/8/8 b - - 0 1 ;D1 4 ;D2 68 ;D3 1118 ;D4 16199 ;D5 281190",
    "K7/8/2n5/1n6/8/8/8/k6N b - - 0 1 ;D1 17 ;D2 54 ;D3 835 ;D4 5910 ;D5 92250",
    "k7/8/2N5/1N6/8/8/8/K6n b - - 0 1 ;D1 3 ;D2 51 ;D3 345 ;D4 5301 ;D5 38348",
    "B6b/8/8/8/2K5/4k3/8/b6B w - - 0 1 ;D1 17 ;D2 278 ;D3 4607 ;D4 76778 ;D5 1320507",
    "8/8/1B6/7b/7k/8/2B1b3/7K w - - 0 1 ;D1 21 ;D2 316 ;D3 5744 ;D4 93338 ;D5 1713368",
    "k7/B7/1B6/1B6/8/8/8/K6b w - - 0 1 ;D1 21 ;D2 144 ;D3 3242 ;D4 32955 ;D5 787524",
    "K7/b7/1b6/1b6/8/8/8/k6B w - - 0 1 ;D1 7 ;D2 143 ;D3 1416 ;D4 31787 ;D5 310862",
    "B6b/8/8/8/2K5/5k2/8/b6B b - - 0 1 ;D1 6 ;D2 106 ;D3 1829 ;D4 31151 ;D5 530585",
    "8/8/1B6/7b/7k/8/2B1b3/7K b - - 0 1 ;D1 17 ;D2 309 ;D3 5133 ;D4 93603 ;D5 1591064",
    "k7/B7/1B6/1B6/8/8/8/K6b b - - 0 1 ;D1 7 ;D2 143 ;D3 1416 ;D4 31787 ;D5 310862",
    "K7/b7/1b6/1b6/8/8/8/k6B b - - 0 1 ;D1 21 ;D2 144 ;D3 3242 ;D4 32955 ;D5 787524",
    "7k/RR6/8/8/8/8/rr6/7K w - - 0 1 ;D1 19 ;D2 275 ;D3 5300 ;D4 104342 ;D5 2161211",
    "R6r/8/8/2K5/5k2/8/8/r6R w - - 0 1 ;D1 36 ;D2 1027 ;D3 29215 ;D4 771461",
    "7k/RR6/8/8/8/8/rr6/7K b - - 0 1 ;D1 19 ;D2 275 ;D3 5300 ;D4 104342 ;D5 2161211",
    "R6r/8/8/2K5/5k2/8/8/r6R b - - 0 1 ;D1 36 ;D2 1027 ;D3 29227 ;D4 771368",
    "6kq/8/8/8/8/8/8/7K w - - 0 1 ;D1 2 ;D2 36 ;D3 143 ;D4 3637 ;D5 14893",
    "6KQ/8/8/8/8/8/8/7k b - - 0 1 ;D1 2 ;D2 36 ;D3 143 ;D4 3637 ;D5 14893",
    "K7/8/8/3Q4/4q3/8/8/7k w - - 0 1 ;D1 6 ;D2 35 ;D3 495 ;D4 8349 ;D5 166741",
    "6qk/8/8/8/8/8/8/7K b - - 0 1 ;D1 22 ;D2 43 ;D3 1015 ;D4 4167 ;D5 105749",
    "6KQ/8/8/8/8/8/8/7k b - - 0 1 ;D1 2 ;D2 36 ;D3 143 ;D4 3637 ;D5 14893",
    "K7/8/8/3Q4/4q3/8/8/7k b - - 0 1 ;D1 6 ;D2 35 ;D3 495 ;D4 8349 ;D5 166741",
    "8/8/8/8/8/K7/P7/k7 w - - 0 1 ;D1 3 ;D2 7 ;D3 43 ;D4 199 ;D5 1347",
    "8/8/8/8/8/7K/7P/7k w - - 0 1 ;D1 3 ;D2 7 ;D3 43 ;D4 199 ;D5 1347",
    "K7/p7/k7/8/8/8/8/8 w - - 0 1 ;D1 1 ;D2 3 ;D3 12 ;D4 80 ;D5 342",
    "7K/7p/7k/8/8/8/8/8 w - - 0 1 ;D1 1 ;D2 3 ;D3 12 ;D4 80 ;D5 342",
    "8/2k1p3/3pP3/3P2K1/8/8/8/8 w - - 0 1 ;D1 7 ;D2 35 ;D3 210 ;D4 1091 ;D5 7028",
    "8/8/8/8/8/K7/P7/k7 b - - 0 1 ;D1 1 ;D2 3 ;D3 12 ;D4 80 ;D5 342",
    "8/8/8/8/8/7K/7P/7k b - - 0 1 ;D1 1 ;D2 3 ;D3 12 ;D4 80 ;D5 342",
    "K7/p7/k7/8/8/8/8/8 b - - 0 1 ;D1 3 ;D2 7 ;D3 43 ;D4 199 ;D5 1347",
    "7K/7p/7k/8/8/8/8/8 b - - 0 1 ;D1 3 ;D2 7 ;D3 43 ;D4 199 ;D5 1347",
    "8/2k1p3/3pP3/3P2K1/8/8/8/8 b - - 0 1 ;D1 5 ;D2 35 ;D3 182 ;D4 1091 ;D5 5408",
    "8/8/8/8/8/4k3/4P3/4K3 w - - 0 1 ;D1 2 ;D2 8 ;D3 44 ;D4 282 ;D5 1814",
    "4k3/4p3/4K3/8/8/8/8/8 b - - 0 1 ;D1 2 ;D2 8 ;D3 44 ;D4 282 ;D5 1814",
    "8/8/7k/7p/7P/7K/8/8 w - - 0 1 ;D1 3 ;D2 9 ;D3 57 ;D4 360 ;D5 1969",
    "8/8/k7/p7/P7/K7/8/8 w - - 0 1 ;D1 3 ;D2 9 ;D3 57 ;D4 360 ;D5 1969",
    "8/8/3k4/3p4/3P4/3K4/8/8 w - - 0 1 ;D1 5 ;D2 25 ;D3 180 ;D4 1294 ;D5 8296",
    "8/3k4/3p4/8/3P4/3K4/8/8 w - - 0 1 ;D1 8 ;D2 61 ;D3 483 ;D4 3213 ;D5 23599",
    "8/8/3k4/3p4/8/3P4/3K4/8 w - - 0 1 ;D1 8 ;D2 61 ;D3 411 ;D4 3213 ;D5 21637",
    "k7/8/3p4/8/3P4/8/8/7K w - - 0 1 ;D1 4 ;D2 15 ;D3 90 ;D4 534 ;D5 3450",
    "8/8/7k/7p/7P/7K/8/8 b - - 0 1 ;D1 3 ;D2 9 ;D3 57 ;D4 360 ;D5 1969",
    "8/8/k7/p7/P7/K7/8/8 b - - 0 1 ;D1 3 ;D2 9 ;D3 57 ;D4 360 ;D5 1969",
    "8/8/3k4/3p4/3P4/3K4/8/8 b - - 0 1 ;D1 5 ;D2 25 ;D3 180 ;D4 1294 ;D5 8296",
    "8/3k4/3p4/8/3P4/3K4/8/8 b - - 0 1 ;D1 8 ;D2 61 ;D3 411 ;D4 3213 ;D5 21637",
    "8/8/3k4/3p4/8/3P4/3K4/8 b - - 0 1 ;D1 8 ;D2 61 ;D3 483 ;D4 3213 ;D5 23599",
    "k7/8/3p4/8/3P4/8/8/7K b - - 0 1 ;D1 4 ;D2 15 ;D3 89 ;D4 537 ;D5 3309",
    "7k/3p4/8/8/3P4/8/8/K7 w - - 0 1 ;D1 4 ;D2 19 ;D3 117 ;D4 720 ;D5 4661",
    "7k/8/8/3p4/8/8/3P4/K7 w - - 0 1 ;D1 5 ;D2 19 ;D3 116 ;D4 716 ;D5 4786",
    "k7/8/8/7p/6P1/8/8/K7 w - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112",
    "k7/8/7p/8/8/6P1/8/K7 w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354",
    "k7/8/8/6p1/7P/8/8/K7 w - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112",
    "k7/8/6p1/8/8/7P/8/K7 w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354",
    "k7/8/8/3p4/4p3/8/8/7K w - - 0 1 ;D1 3 ;D2 15 ;D3 84 ;D4 573 ;D5 3013",
    "k7/8/3p4/8/8/4P3/8/7K w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4271",
    "7k/3p4/8/8/3P4/8/8/K7 b - - 0 1 ;D1 5 ;D2 19 ;D3 117 ;D4 720 ;D5 5014",
    "7k/8/8/3p4/8/8/3P4/K7 b - - 0 1 ;D1 4 ;D2 19 ;D3 117 ;D4 712 ;D5 4658",
    "k7/8/8/7p/6P1/8/8/K7 b - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112",
    "k7/8/7p/8/8/6P1/8/K7 b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354",
    "k7/8/8/6p1/7P/8/8/K7 b - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112",
    "k7/8/6p1/8/8/7P/8/K7 b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354",
    "k7/8/8/3p4/4p3/8/8/7K b - - 0 1 ;D1 5 ;D2 15 ;D3 102 ;D4 569 ;D5 4337",
    "k7/8/3p4/8/8/4P3/8/7K b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4271",
    "7k/8/8/p7/1P6/8/8/7K w - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112",
    "7k/8/8/p7/1P6/8/8/7K b - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112",
    "7k/8/8/1p6/P7/8/8/7K w - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112",
    "7k/8/8/1p6/P7/8/8/7K b - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112",
    "7k/8/p7/8/8/1P6/8/7K w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354",
    "7k/8/p7/8/8/1P6/8/7K b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354",
    "7k/8/1p6/8/8/P7/8/7K w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354",
    "7k/8/1p6/8/8/P7/8/7K b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354",
    "k7/7p/8/8/8/8/6P1/K7 w - - 0 1 ;D1 5 ;D2 25 ;D3 161 ;D4 1035 ;D5 7574",
    "k7/7p/8/8/8/8/6P1/K7 b - - 0 1 ;D1 5 ;D2 25 ;D3 161 ;D4 1035 ;D5 7574",
    "k7/6p1/8/8/8/8/7P/K7 w - - 0 1 ;D1 5 ;D2 25 ;D3 161 ;D4 1035 ;D5 7574",
    "k7/6p1/8/8/8/8/7P/K7 b - - 0 1 ;D1 5 ;D2 25 ;D3 161 ;D4 1035 ;D5 7574",
    "8/Pk6/8/8/8/8/6Kp/8 w - - 0 1 ;D1 11 ;D2 97 ;D3 887 ;D4 8048 ;D5 90606",
    "8/Pk6/8/8/8/8/6Kp/8 b - - 0 1 ;D1 11 ;D2 97 ;D3 887 ;D4 8048 ;D5 90606",
    "3k4/3pp3/8/8/8/8/3PP3/3K4 w - - 0 1 ;D1 7 ;D2 49 ;D3 378 ;D4 2902 ;D5 24122",
    "3k4/3pp3/8/8/8/8/3PP3/3K4 b - - 0 1 ;D1 7 ;D2 49 ;D3 378 ;D4 2902 ;D5 24122",
    "8/PPPk4/8/8/8/8/4Kppp/8 w - - 0 1 ;D1 18 ;D2 270 ;D3 4699 ;D4 79355 ;D5 1533145",
    "8/PPPk4/8/8/8/8/4Kppp/8 b - - 0 1 ;D1 18 ;D2 270 ;D3 4699 ;D4 79355 ;D5 1533145",
    "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N w - - 0 1 ;D1 24 ;D2 421 ;D3 7421 ;D4 124608 ;D5 2193768",
    "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N b - - 0 1 ;D1 24 ;D2 421 ;D3 7421 ;D4 124608 ;D5 2193768",
    "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N w - - 0 1 ;D1 24 ;D2 496 ;D3 9483 ;D4 182838 ;D5 3605103",
    "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1 ;D1 24 ;D2 496 ;D3 9483 ;D4 182838 ;D5 3605103",

    // // Specials
	"3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1 ;D6 1134888",
	"r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1 ;D4 1274206",
	"8/8/8/8/k1p4R/8/3P4/3K4 w - - 0 1 ;D6 1134888",
	"8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1 ;D6 1440467",
	"8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1 ;D6 1440467",
	"8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1 ;D6 1015133",
	"8/b2p2k1/8/2P5/8/4K3/8/8 b - - 0 1 ;D6 1015133",
	"5k2/8/8/8/8/8/8/4K2R w K - 0 1 ;D6 661072",
	"4k2r/8/8/8/8/8/8/5K2 b k - 0 1 ;D6 661072",
	"3k4/8/8/8/8/8/8/R3K3 w Q - 0 1 ;D6 803711",
	"r3k3/8/8/8/8/8/8/3K4 b q - 0 1 ;D6 803711",

    // // en passant capture checks opponent
	"8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1 ;D6 1440467",
	"8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1 ;D6 1440467",

    // // avoid illegal ep(thanks to Steve Maughan)
	"3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1 ;D6 1134888",
	"8/8/8/8/k1p4R/8/3P4/3K4 w - - 0 1 ;D6 1134888",
    
    // // avoid illegal ep #2
	"8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1 ;D6 1015133",
    "8/b2p2k1/8/2P5/8/4K3/8/8 b - - 0 1 ;D6 1015133",
    
	// // short castling gives check
	"5k2/8/8/8/8/8/8/4K2R w K - 0 1 ;D6 661072",
    "4k2r/8/8/8/8/8/8/5K2 b k - 0 1 ;D6 661072",
    
	// // long castling gives check
	"3k4/8/8/8/8/8/8/R3K3 w Q - 0 1 ;D6 803711",
	"r3k3/8/8/8/8/8/8/3K4 b q - 0 1 ;D6 803711",
    
    // // castling(including losing cr due to rook capture)
	"r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1 ;D4 1274206",
	"r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1 ;D4 1274206",

    // // castling prevented
	"r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1 ;D4 1720476",
	"r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1 ;D4 1720476",
	// //  promote out of check
	"2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1 ;D6 3821001",
	"3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1 ;D6 3821001",

    // // "# discovered check
	"8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1 ;D5 1004658",
	"5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1 ;D5 1004658",

    // // "# promote to give check
	"4k3/1P6/8/8/8/8/K7/8 w - - 0 1 ;D6 217342",
	"8/k7/8/8/8/8/1p6/4K3 b - - 0 1 ;D6 217342",

    // // "# underpromote to check
	"8/P1k5/K7/8/8/8/8/8 w - - 0 1 ;D6 92683",
	"8/8/8/8/8/k7/p1K5/8 b - - 0 1 ;D6 92683",

    // // "# self stalemate
	"K1k5/8/P7/8/8/8/8/8 w - - 0 1 ;D6 2217",
	"8/8/8/8/8/p7/8/k1K5 b - - 0 1 ;D6 2217",

    // // stalemate/checkmate:
	"8/k1P5/8/1K6/8/8/8/8 w - - 0 1 ;D7 567584",
	"8/8/8/8/1k6/8/K1p5/8 b - - 0 1 ;D7 567584",

    // // double check
	"8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1 ;D4 23527",

    // // short castling impossible although the rook never moved away from its corner
	"1k6/1b6/8/8/7R/8/8/4K2R b K - 0 1 ;D5 1063513",
	"4k2r/8/8/7r/8/8/1B6/1K6 w k - 0 1 ;D5 1063513",

    // // long castling impossible although the rook never moved away from its corner
	"1k6/8/8/8/R7/1n6/8/R3K3 b Q - 0 1 ;D5 346695",
    "r3k3/8/1N6/r7/8/8/8/1K6 w q - 0 1 ;D5 346695",
];
