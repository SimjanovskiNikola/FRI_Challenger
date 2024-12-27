use std::vec;

fn perft(depth: u64) -> PerftPosParameters {
    // let move_list = vec![];
    // let params = PerftPosParameters::init();

    // if depth == 0 {
    //     return params;
    // }

    // let n_moves: Vec<u64> = GenerateMoves(move_list);
    // for i in 0..n_moves.len() {
    //     MakeMove(move_list[i]);
    //     if (!IsIncheck()) {
    //         params.nodes += perft(depth - 1).nodes;
    //     }
    //     UndoMove(move_list[i]);
    // }

    // return params;
    todo!();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PerftPosParameters {
    pub nodes: u64,
    pub captures: u64,
    pub ep: u64,
    pub castles: u64,
    pub promotions: u64,
    pub checks: u64,
    pub discovery_checks: u64,
    pub double_checks: u64,
    pub checkmates: u64,
}

impl PerftPosParameters {
    pub fn init() -> Self {
        return Self {
            nodes: 0,
            captures: 0,
            ep: 0,
            castles: 0,
            promotions: 0,
            checks: 0,
            discovery_checks: 0,
            double_checks: 0,
            checkmates: 0,
        };
    }

    pub fn init_params(
        nodes: u64,
        captures: u64,
        ep: u64,
        castles: u64,
        promotions: u64,
        checks: u64,
        discovery_checks: u64,
        double_checks: u64,
        checkmates: u64,
    ) -> Self {
        return Self {
            nodes: nodes,
            captures: captures,
            ep: ep,
            castles: castles,
            promotions: promotions,
            checks: checks,
            discovery_checks: discovery_checks,
            double_checks: double_checks,
            checkmates: checkmates,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{engine::game::Game, FEN_START};

    use super::*;

    pub const FEN_POS_TWO: &str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    pub const FEN_POS_THREE: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -";
    pub const FEN_POS_FOUR: &str =
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    pub const FEN_POS_FIVE: &str =
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    pub const FEN_POS_SIX: &str =
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

    // PERF TESTS FROM STARTING POSITION

    #[test]
    fn test_perft_init_pos_one() {
        let game = Game::read_fen(&FEN_START);
        assert_eq!(perft(1).nodes, 20)
    }

    #[test]
    fn test_perft_init_pos_two() {
        let game = Game::read_fen(&FEN_START);
        assert_eq!(perft(2).nodes, 400)
    }

    #[test]
    fn test_perft_init_pos_three() {
        let game = Game::read_fen(&FEN_START);
        assert_eq!(perft(3).nodes, 8902)
    }

    #[test]
    fn test_perft_init_pos_four() {
        let game = Game::read_fen(&FEN_START);
        assert_eq!(perft(4).nodes, 197281)
    }

    #[test]
    fn test_perft_init_pos_five() {
        let game = Game::read_fen(&FEN_START);
        assert_eq!(perft(5).nodes, 4865609)
    }

    #[test]
    fn test_perft_init_pos_six() {
        let game = Game::read_fen(&FEN_START);
        assert_eq!(perft(6).nodes, 119060324)
    }

    #[test]
    fn test_perft_init_pos_seven() {
        let game = Game::read_fen(&FEN_START);
        assert_eq!(perft(7).nodes, 3195901860)
    }

    #[test]
    fn test_perft_init_pos_eight() {
        let game = Game::read_fen(&FEN_START);
        assert_eq!(perft(8).nodes, 84998978956)
    }

    #[test]
    fn test_perft_init_pos_nine() {
        let game = Game::read_fen(&FEN_START);
        assert_eq!(perft(9).nodes, 2439530234167)
    }

    #[test]
    fn test_perft_pos_two_depth_1() {
        let game = Game::read_fen(&FEN_POS_TWO);
        let params = perft(1);
        assert_eq!(params.nodes, 48);
        assert_eq!(params.captures, 8);
        assert_eq!(params.ep, 0);
        assert_eq!(params.castles, 2);
        assert_eq!(params.promotions, 0);
        assert_eq!(params.checks, 0);
        assert_eq!(params.discovery_checks, 0);
        assert_eq!(params.double_checks, 0);
        assert_eq!(params.checkmates, 0);
    }
    #[test]
    fn test_perft_pos_two_depth_2() {
        let game = Game::read_fen(&FEN_POS_TWO);
        let params = perft(2);
        assert_eq!(params.nodes, 2039);
        assert_eq!(params.captures, 351);
        assert_eq!(params.ep, 1);
        assert_eq!(params.castles, 91);
        assert_eq!(params.promotions, 0);
        assert_eq!(params.checks, 3);
        assert_eq!(params.discovery_checks, 0);
        assert_eq!(params.double_checks, 0);
        assert_eq!(params.checkmates, 0);
    }
    #[test]
    fn test_perft_pos_two_depth_3() {
        let game = Game::read_fen(&FEN_POS_TWO);
        let params = perft(3);
        assert_eq!(params.nodes, 97862);
        assert_eq!(params.captures, 17102);
        assert_eq!(params.ep, 45);
        assert_eq!(params.castles, 3162);
        assert_eq!(params.promotions, 0);
        assert_eq!(params.checks, 993);
        assert_eq!(params.discovery_checks, 0);
        assert_eq!(params.double_checks, 0);
        assert_eq!(params.checkmates, 1);
    }
    #[test]
    fn test_perft_pos_two_depth_4() {
        let game = Game::read_fen(&FEN_POS_TWO);
        let params = perft(4);
        assert_eq!(params.nodes, 4085603);
        assert_eq!(params.captures, 757163);
        assert_eq!(params.ep, 1929);
        assert_eq!(params.castles, 128013);
        assert_eq!(params.promotions, 15172);
        assert_eq!(params.checks, 25523);
        assert_eq!(params.discovery_checks, 42);
        assert_eq!(params.double_checks, 6);
        assert_eq!(params.checkmates, 43);
    }
    #[test]
    fn test_perft_pos_two_depth_5() {
        let game = Game::read_fen(&FEN_POS_TWO);
        let params = perft(5);
        assert_eq!(params.nodes, 193690690);
        assert_eq!(params.captures, 35043416);
        assert_eq!(params.ep, 73365);
        assert_eq!(params.castles, 4993637);
        assert_eq!(params.promotions, 8392);
        assert_eq!(params.checks, 3309887);
        assert_eq!(params.discovery_checks, 19883);
        assert_eq!(params.double_checks, 2637);
        assert_eq!(params.checkmates, 30171);
    }
    #[test]
    fn test_perft_pos_two_depth_6() {
        let game = Game::read_fen(&FEN_POS_TWO);
        let params = perft(6);
        assert_eq!(params.nodes, 8031647685);
        assert_eq!(params.captures, 1558445089);
        assert_eq!(params.ep, 3577504);
        assert_eq!(params.castles, 184513607);
        assert_eq!(params.promotions, 56627920);
        assert_eq!(params.checks, 92238050);
        assert_eq!(params.discovery_checks, 568417);
        assert_eq!(params.double_checks, 54948);
        assert_eq!(params.checkmates, 54948);
    }

    // POSITION 3
    #[test]
    fn test_perft_pos_three_depth_1() {
        let game = Game::read_fen(&FEN_POS_THREE);
        let params = perft(1);
        assert_eq!(params.nodes, 14);
        assert_eq!(params.captures, 1);
        assert_eq!(params.ep, 0);
        assert_eq!(params.castles, 0);
        assert_eq!(params.promotions, 0);
        assert_eq!(params.checks, 2);
        assert_eq!(params.discovery_checks, 0);
        assert_eq!(params.double_checks, 0);
        assert_eq!(params.checkmates, 0);
    }
    #[test]
    fn test_perft_pos_three_depth_2() {
        let game = Game::read_fen(&FEN_POS_THREE);
        let params = perft(2);
        assert_eq!(params.nodes, 191);
        assert_eq!(params.captures, 14);
        assert_eq!(params.ep, 0);
        assert_eq!(params.castles, 0);
        assert_eq!(params.promotions, 0);
        assert_eq!(params.checks, 10);
        assert_eq!(params.discovery_checks, 0);
        assert_eq!(params.double_checks, 0);
        assert_eq!(params.checkmates, 0);
    }
    #[test]
    fn test_perft_pos_three_depth_3() {
        let game = Game::read_fen(&FEN_POS_THREE);
        let params = perft(3);
        assert_eq!(params.nodes, 2812);
        assert_eq!(params.captures, 209);
        assert_eq!(params.ep, 2);
        assert_eq!(params.castles, 0);
        assert_eq!(params.promotions, 0);
        assert_eq!(params.checks, 267);
        assert_eq!(params.discovery_checks, 3);
        assert_eq!(params.double_checks, 0);
        assert_eq!(params.checkmates, 0);
    }
    #[test]
    fn test_perft_pos_three_depth_4() {
        let game = Game::read_fen(&FEN_POS_THREE);
        let params = perft(4);
        assert_eq!(params.nodes, 43238);
        assert_eq!(params.captures, 3348);
        assert_eq!(params.ep, 123);
        assert_eq!(params.castles, 0);
        assert_eq!(params.promotions, 0);
        assert_eq!(params.checks, 1680);
        assert_eq!(params.discovery_checks, 106);
        assert_eq!(params.double_checks, 0);
        assert_eq!(params.checkmates, 17);
    }
    #[test]
    fn test_perft_pos_three_depth_5() {
        let game = Game::read_fen(&FEN_POS_THREE);
        let params = perft(5);
        assert_eq!(params.nodes, 674624);
        assert_eq!(params.captures, 52051);
        assert_eq!(params.ep, 1165);
        assert_eq!(params.castles, 0);
        assert_eq!(params.promotions, 0);
        assert_eq!(params.checks, 52950);
        assert_eq!(params.discovery_checks, 1292);
        assert_eq!(params.double_checks, 3);
        assert_eq!(params.checkmates, 0);
    }
    #[test]
    fn test_perft_pos_three_depth_6() {
        let game = Game::read_fen(&FEN_POS_THREE);
        let params = perft(6);
        assert_eq!(params.nodes, 11030083);
        assert_eq!(params.captures, 940350);
        assert_eq!(params.ep, 33325);
        assert_eq!(params.castles, 0);
        assert_eq!(params.promotions, 7552);
        assert_eq!(params.checks, 452473);
        assert_eq!(params.discovery_checks, 26067);
        assert_eq!(params.double_checks, 0);
        assert_eq!(params.checkmates, 2733);
    }
    #[test]
    fn test_perft_pos_three_depth_7() {
        let game = Game::read_fen(&FEN_POS_THREE);
        let params = perft(7);
        assert_eq!(params.nodes, 178633661);
        assert_eq!(params.captures, 14519036);
        assert_eq!(params.ep, 294874);
        assert_eq!(params.castles, 0);
        assert_eq!(params.promotions, 140024);
        assert_eq!(params.checks, 12797406);
        assert_eq!(params.discovery_checks, 370630);
        assert_eq!(params.double_checks, 3612);
        assert_eq!(params.checkmates, 87);
    }
    #[test]
    fn test_perft_pos_three_depth_8() {
        let game = Game::read_fen(&FEN_POS_THREE);
        let params = perft(8);
        assert_eq!(params.nodes, 3009794393);
        assert_eq!(params.captures, 267586558);
        assert_eq!(params.ep, 8009239);
        assert_eq!(params.castles, 0);
        assert_eq!(params.promotions, 6578076);
        assert_eq!(params.checks, 135626805);
        assert_eq!(params.discovery_checks, 7181487);
        assert_eq!(params.double_checks, 1630);
        assert_eq!(params.checkmates, 450410);
    }

    // POSITION 4
    #[test]
    fn test_perft_pos_four_depth_1() {
        let game = Game::read_fen(&FEN_POS_FOUR);
        assert_eq!(perft(1).nodes, 6);
    }
    #[test]
    fn test_perft_pos_four_depth_2() {
        let game = Game::read_fen(&FEN_POS_FOUR);
        assert_eq!(perft(2).nodes, 264);
    }
    #[test]
    fn test_perft_pos_four_depth_3() {
        let game = Game::read_fen(&FEN_POS_FOUR);
        assert_eq!(perft(3).nodes, 9467);
    }
    #[test]
    fn test_perft_pos_four_depth_4() {
        let game = Game::read_fen(&FEN_POS_FOUR);
        assert_eq!(perft(4).nodes, 422333);
    }
    #[test]
    fn test_perft_pos_four_depth_5() {
        let game = Game::read_fen(&FEN_POS_FOUR);
        assert_eq!(perft(5).nodes, 15833292);
    }
    #[test]
    fn test_perft_pos_four_depth_6() {
        let game = Game::read_fen(&FEN_POS_FOUR);
        assert_eq!(perft(6).nodes, 706045033);
    }

    // POSITION 5
    #[test]
    fn test_perft_pos_five_depth_1() {
        let game = Game::read_fen(&FEN_POS_FIVE);
        assert_eq!(perft(1).nodes, 44)
    }

    #[test]
    fn test_perft_pos_five_depth_2() {
        let game = Game::read_fen(&FEN_POS_FIVE);
        assert_eq!(perft(2).nodes, 1486)
    }

    #[test]
    fn test_perft_pos_five_depth_3() {
        let game = Game::read_fen(&FEN_POS_FIVE);
        assert_eq!(perft(3).nodes, 62379)
    }

    #[test]
    fn test_perft_pos_five_depth_4() {
        let game = Game::read_fen(&FEN_POS_FIVE);
        assert_eq!(perft(4).nodes, 2103487)
    }

    #[test]
    fn test_perft_pos_five_depth_5() {
        let game = Game::read_fen(&FEN_POS_FIVE);
        assert_eq!(perft(5).nodes, 89941194)
    }

    // POSITION 6

    #[test]
    fn test_perft_pos_six_depth_0() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(0).nodes, 1)
    }

    #[test]
    fn test_perft_pos_six_depth_1() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(1).nodes, 46)
    }

    #[test]
    fn test_perft_pos_six_depth_2() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(2).nodes, 2079)
    }

    #[test]
    fn test_perft_pos_six_depth_3() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(3).nodes, 89890)
    }

    #[test]
    fn test_perft_pos_six_depth_4() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(4).nodes, 3894594)
    }

    #[test]
    fn test_perft_pos_six_depth_5() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(5).nodes, 164075551)
    }

    #[test]
    fn test_perft_pos_six_depth_6() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(6).nodes, 6923051137)
    }

    #[test]
    fn test_perft_pos_six_depth_7() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(7).nodes, 287188994746)
    }

    #[test]
    fn test_perft_pos_six_depth_8() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(8).nodes, 11923589843526)
    }

    #[test]
    fn test_perft_pos_six_depth_9() {
        let game = Game::read_fen(&FEN_POS_SIX);
        assert_eq!(perft(9).nodes, 490154852788714)
    }
}
