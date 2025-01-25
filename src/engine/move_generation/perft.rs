use std::{os::linux::raw::stat, vec};

use crate::engine::{
    game::Game,
    move_generation::move_generation::gen_moves,
    shared::{
        helper_func::print_utility::{move_notation, print_chess, print_move_list},
        structures::internal_move::{Flag, InternalMove},
    },
};
use lazy_static::lazy_static;

use super::make_move::GameMoveTrait;
//        O        | 1
//    O       O    | 2
//  O   O   O   O  | 4
// O O O O O O O O | 8

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

    let mut move_list: Vec<InternalMove> = gen_moves(game.active_color, game);
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

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::engine::{
        game::Game,
        shared::helper_func::{const_utility::FEN_START, print_utility::print_move_list},
    };

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

    // PERF TESTS FROM STARTING POSITION

    fn init_test_func(fen: &str, depth: usize) -> Stats {
        let mut game = Game::read_fen(&fen);
        let mut stats = Stats::init();
        let now = Instant::now();
        let nodes = perft(depth, &mut game, &mut stats);
        println!("----------*Stats*-----------");
        println!("Time for {} nodes: {} ms", nodes, now.elapsed().as_millis());
        stats.print();

        return stats;
    }

    #[test]
    fn test_fen_bug_2_sq_pawn_dept_1() {
        let stats = init_test_func(&FEN_BUG_2SQ_PAWN, 1);
        assert_eq!(stats.nodes, 7);
    }

    #[test]
    fn test_fen_bug_2_sq_pawn_dept_2() {
        let stats = init_test_func(&FEN_BUG_2SQ_PAWN, 2);
        assert_eq!(stats.nodes, 44);
    }

    #[test]
    fn test_perft_init_pos_one() {
        let stats = init_test_func(&FEN_START, 1);
        assert_eq!(stats.nodes, 20);
    }

    #[test]
    fn test_perft_init_pos_two() {
        let stats = init_test_func(&FEN_START, 2);
        assert_eq!(stats.nodes, 400);
    }

    #[test]
    fn test_perft_init_pos_three() {
        let stats = init_test_func(&FEN_START, 3);
        assert_eq!(stats.nodes, 8902);
    }

    #[test]
    fn test_perft_init_pos_four() {
        let stats = init_test_func(&FEN_START, 4);
        assert_eq!(stats.nodes, 197281);
    }

    #[test]
    fn test_perft_init_pos_five() {
        let stats = init_test_func(&FEN_START, 5);
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

    #[test]
    fn test_perft_pos_two_depth_1() {
        let stats = init_test_func(&FEN_POS_TWO, 1);
        assert_eq!(stats.nodes, 48);
    }

    #[test]
    fn test_perft_pos_two_depth_2() {
        let stats = init_test_func(&FEN_POS_TWO, 2);
        assert_eq!(stats.nodes, 2039);
    }

    //FIXME: More castles than expected
    #[test]
    fn test_perft_pos_two_depth_3() {
        let stats = init_test_func(&FEN_POS_TWO, 3);
        assert_eq!(stats.nodes, 97862);
    }

    // #[test]
    // fn test_perft_pos_two_depth_4() {
    //     let game = Game::read_fen(&FEN_POS_TWO);
    //     let params = perft(4);
    //     assert_eq!(params.nodes, 4085603);
    //     assert_eq!(params.captures, 757163);
    //     assert_eq!(params.ep, 1929);
    //     assert_eq!(params.castles, 128013);
    //     assert_eq!(params.promotions, 15172);
    //     assert_eq!(params.checks, 25523);
    //     assert_eq!(params.discovery_checks, 42);
    //     assert_eq!(params.double_checks, 6);
    //     assert_eq!(params.checkmates, 43);
    // }
    // #[test]
    // fn test_perft_pos_two_depth_5() {
    //     let game = Game::read_fen(&FEN_POS_TWO);
    //     let params = perft(5);
    //     assert_eq!(params.nodes, 193690690);
    //     assert_eq!(params.captures, 35043416);
    //     assert_eq!(params.ep, 73365);
    //     assert_eq!(params.castles, 4993637);
    //     assert_eq!(params.promotions, 8392);
    //     assert_eq!(params.checks, 3309887);
    //     assert_eq!(params.discovery_checks, 19883);
    //     assert_eq!(params.double_checks, 2637);
    //     assert_eq!(params.checkmates, 30171);
    // }
    // #[test]
    // fn test_perft_pos_two_depth_6() {
    //     let game = Game::read_fen(&FEN_POS_TWO);
    //     let params = perft(6);
    //     assert_eq!(params.nodes, 8031647685);
    //     assert_eq!(params.captures, 1558445089);
    //     assert_eq!(params.ep, 3577504);
    //     assert_eq!(params.castles, 184513607);
    //     assert_eq!(params.promotions, 56627920);
    //     assert_eq!(params.checks, 92238050);
    //     assert_eq!(params.discovery_checks, 568417);
    //     assert_eq!(params.double_checks, 54948);
    //     assert_eq!(params.checkmates, 54948);
    // }

    // // POSITION 3
    #[test]
    fn test_perft_pos_three_depth_1() {
        let stats = init_test_func(&FEN_POS_THREE, 1);
        assert_eq!(stats.nodes, 14);
    }

    // FIXME: Same problem with pawn pushed 2 times.
    #[test]
    fn test_perft_pos_three_depth_2() {
        let stats = init_test_func(&FEN_POS_THREE, 2);
        assert_eq!(stats.nodes, 191);
    }
    #[test]
    fn test_perft_pos_three_depth_3() {
        let stats = init_test_func(&FEN_POS_THREE, 3);
        assert_eq!(stats.nodes, 2812);
    }
    #[test]
    fn test_perft_pos_three_depth_4() {
        let stats = init_test_func(&FEN_POS_THREE, 4);
        assert_eq!(stats.nodes, 43238);
    }
    #[test]
    fn test_perft_pos_three_depth_5() {
        let stats = init_test_func(&FEN_POS_THREE, 5);
        assert_eq!(stats.nodes, 674624);
    }

    // #[test]
    // fn test_perft_pos_three_depth_6() {
    //     let game = Game::read_fen(&FEN_POS_THREE);
    //     let params = perft(6);
    //     assert_eq!(params.nodes, 11030083);
    //     assert_eq!(params.captures, 940350);
    //     assert_eq!(params.ep, 33325);
    //     assert_eq!(params.castles, 0);
    //     assert_eq!(params.promotions, 7552);
    //     assert_eq!(params.checks, 452473);
    //     assert_eq!(params.discovery_checks, 26067);
    //     assert_eq!(params.double_checks, 0);
    //     assert_eq!(params.checkmates, 2733);
    // }
    // #[test]
    // fn test_perft_pos_three_depth_7() {
    //     let game = Game::read_fen(&FEN_POS_THREE);
    //     let params = perft(7);
    //     assert_eq!(params.nodes, 178633661);
    //     assert_eq!(params.captures, 14519036);
    //     assert_eq!(params.ep, 294874);
    //     assert_eq!(params.castles, 0);
    //     assert_eq!(params.promotions, 140024);
    //     assert_eq!(params.checks, 12797406);
    //     assert_eq!(params.discovery_checks, 370630);
    //     assert_eq!(params.double_checks, 3612);
    //     assert_eq!(params.checkmates, 87);
    // }
    // #[test]
    // fn test_perft_pos_three_depth_8() {
    //     let game = Game::read_fen(&FEN_POS_THREE);
    //     let params = perft(8);
    //     assert_eq!(params.nodes, 3009794393);
    //     assert_eq!(params.captures, 267586558);
    //     assert_eq!(params.ep, 8009239);
    //     assert_eq!(params.castles, 0);
    //     assert_eq!(params.promotions, 6578076);
    //     assert_eq!(params.checks, 135626805);
    //     assert_eq!(params.discovery_checks, 7181487);
    //     assert_eq!(params.double_checks, 1630);
    //     assert_eq!(params.checkmates, 450410);
    // }

    // // POSITION 4
    #[test]
    fn test_perft_pos_four_depth_1() {
        let stats = init_test_func(&FEN_POS_FOUR, 1);
        assert_eq!(stats.nodes, 6);
    }

    #[test]
    fn test_perft_pos_four_depth_2() {
        let stats = init_test_func(&FEN_POS_FOUR, 2);
        assert_eq!(stats.nodes, 264);
    }

    #[test]
    fn test_perft_pos_four_depth_3() {
        let stats = init_test_func(&FEN_POS_FOUR, 3);
        assert_eq!(stats.nodes, 9467);
    }
    #[test]
    fn test_perft_pos_four_depth_4() {
        let stats = init_test_func(&FEN_POS_FOUR, 4);
        assert_eq!(stats.nodes, 422333);
    }

    #[test]
    fn test_perft_pos_four_depth_5() {
        let stats = init_test_func(&FEN_POS_FOUR, 5);
        assert_eq!(stats.nodes, 15833292);
    }
    // #[test]
    // fn test_perft_pos_four_depth_6() {
    //     let game = Game::read_fen(&FEN_POS_FOUR);
    //     assert_eq!(perft(6).nodes, 706045033);
    // }

    // // POSITION 5
    #[test]
    fn test_perft_pos_five_depth_1() {
        let mut game = Game::read_fen(&FEN_POS_FIVE);
        let mut stats = Stats::init();
        let nodes = perft(1, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 44);
    }

    #[test]
    fn test_perft_pos_five_depth_2() {
        let mut game = Game::read_fen(&FEN_POS_FIVE);
        let mut stats = Stats::init();
        let nodes = perft(2, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 1486);
    }

    #[test]
    fn test_perft_pos_five_depth_3() {
        let mut game = Game::read_fen(&FEN_POS_FIVE);
        let mut stats = Stats::init();
        let nodes = perft(3, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 62379);
    }

    #[test]
    fn test_perft_pos_five_depth_4() {
        let mut game = Game::read_fen(&FEN_POS_FIVE);
        let mut stats = Stats::init();
        let nodes = perft(4, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 2103487);
    }

    // FIXME: Same problem with 2 sq pawn push and el passant on some square near it.
    #[test]
    fn test_perft_pos_five_depth_5() {
        let mut game = Game::read_fen(&FEN_POS_FIVE);
        let mut stats = Stats::init();
        let nodes = perft(5, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 89941194);
    }

    // // POSITION 6

    #[test]
    fn test_perft_pos_six_depth_0() {
        let mut game = Game::read_fen(&FEN_POS_SIX);
        let mut stats = Stats::init();
        let nodes = perft(0, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 1);
    }

    #[test]
    fn test_perft_pos_six_depth_1() {
        let mut game = Game::read_fen(&FEN_POS_SIX);
        let mut stats = Stats::init();
        let nodes = perft(1, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 46);
    }

    #[test]
    fn test_perft_pos_six_depth_2() {
        let mut game = Game::read_fen(&FEN_POS_SIX);
        let mut stats = Stats::init();
        let nodes = perft(2, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 2079);
    }

    #[test]
    fn test_perft_pos_six_depth_3() {
        let mut game = Game::read_fen(&FEN_POS_SIX);
        let mut stats = Stats::init();
        let nodes = perft(3, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 89890);
    }

    #[test]
    fn test_perft_pos_six_depth_4() {
        let mut game = Game::read_fen(&FEN_POS_SIX);
        let mut stats = Stats::init();
        let nodes = perft(4, &mut game, &mut stats);
        stats.print();
        assert_eq!(nodes, 3894594);
    }

    // FIXME: Needed more than 4 minutes: Maybe I need to optimize the code if i want to run the following test.
    // #[test]
    // fn test_perft_pos_six_depth_5() {
    //     let mut game = Game::read_fen(&FEN_POS_SIX);
    //     let mut stats = Stats::init();
    //     let nodes = perft(5, &mut game, &mut stats);
    //     stats.print();
    //     assert_eq!(nodes, 164075551);
    // }

    // #[test]
    // fn test_perft_pos_six_depth_6() {
    //     let game = Game::read_fen(&FEN_POS_SIX);
    //     assert_eq!(perft(6).nodes, 6923051137)
    // }

    // #[test]
    // fn test_perft_pos_six_depth_7() {
    //     let game = Game::read_fen(&FEN_POS_SIX);
    //     assert_eq!(perft(7).nodes, 287188994746)
    // }

    // #[test]
    // fn test_perft_pos_six_depth_8() {
    //     let game = Game::read_fen(&FEN_POS_SIX);
    //     assert_eq!(perft(8).nodes, 11923589843526)
    // }

    // #[test]
    // fn test_perft_pos_six_depth_9() {
    //     let game = Game::read_fen(&FEN_POS_SIX);
    //     assert_eq!(perft(9).nodes, 490154852788714)
    // }
}
