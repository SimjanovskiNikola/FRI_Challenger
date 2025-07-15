use ::engine::engine::protocols::uci::UCI;
use std::env;

use crate::engine::{
    board::{
        fen::FenTrait,
        structures::{
            board::Board,
            color::{BLACK, WHITE},
        },
    },
    evaluation::evaluation::EvaluationTrait,
    misc::print_utility::print_bitboard,
    move_generator::{
        bishop::{has_bishop_pair, BLACK_SQUARES, WHITE_SQUARES},
        generated::king::KING_RING,
    },
};

pub mod engine;

fn main() {
    // FIXME: Needed to backtrace the call stack
    // env::set_var("RUST_BACKTRACE", "1");
    // let mut uci = UCI::init();
    // uci.main();

    let mut between = [[0u64; 64]; 64];

    for sq1 in 0..64 {
        for sq2 in 0..64 {
            if sq1 == sq2 {
                between[sq1][sq2] = 1u64 << sq1;
                continue;
            }

            let r1 = sq1 / 8;
            let f1 = sq1 % 8;
            let r2 = sq2 / 8;
            let f2 = sq2 % 8;

            let dr = (r2 as isize - r1 as isize).signum();
            let df = (f2 as isize - f1 as isize).signum();

            let aligned = (r1 == r2)
                || (f1 == f2)
                || ((r1 as isize - r2 as isize).abs() == (f1 as isize - f2 as isize).abs());

            if aligned {
                let mut bb = 0u64;
                let mut rr = r1 as isize;
                let mut ff = f1 as isize;

                loop {
                    let sq = (rr * 8 + ff) as usize;
                    bb |= 1u64 << sq;
                    if sq == sq2 {
                        break;
                    }
                    rr += dr;
                    ff += df;
                }
                between[sq1][sq2] = bb;
            }
        }
    }

    println!("pub const BETWEEN_BB: [[u64; 64]; 64] = [");
    for row in &between {
        print!("    [");
        for (j, val) in row.iter().enumerate() {
            print!("{:#018x}", val);
            if j != row.len() - 1 {
                print!(", ");
            }
        }
        println!("],");
    }
    println!("];");

    // let mut board = Board::read_fen("8/2p1k1p1/p3p3/2n1N3/4P2P/8/4K1P1/8 w - - 0 0"); // Endgame Good position for black -0.7
    // let mut board = Board::read_fen("8/8/2KB4/3Pb3/1r2k3/8/2R5/8 b - - 0 0"); // Endgame Good position for black 0.63
    // let mut board = Board::read_fen("6k1/4bppp/8/2P1P3/1p3B2/1B1b3P/5PP1/6K1 b - - 0 0");
    // let mut board = Board::read_fen("6k1/4bppp/8/2P1P3/1p3B2/1B1b3P/5PP1/6K1 b - - 0 0");
    // let mut board =
    //     Board::read_fen("r1bq1rk1/2p2ppp/p1n2n2/2b1p3/Pp2P3/1B3N2/1PPN1PPP/R1BQR1K1 w - - 0 0");
    // let mut board =
    //     Board::read_fen("1rb1r1k1/2q2pp1/1b1p2np/1pp5/3Pn3/1B2BNNP/1P1Q1PP1/R3R1K1 w - - 0 0");
    // Board::read_fen("6k1/5p2/7p/4p1p1/pn2P3/2K1BP1P/6P1/8 b - - 2 45");

    // let eval = board.evaluation();
    // println!("Evaluation: {:?}", eval);
}
