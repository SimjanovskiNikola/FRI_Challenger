use std::io::BufRead;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use std::{io, thread, u64};

use crate::engine::fen::fen::FenTrait;
use crate::engine::game::Game;
use crate::engine::move_generation::make_move::GameMoveTrait;
use crate::engine::search::searcher::iterative_deepening;
use crate::engine::shared::helper_func::const_utility::FEN_START;
use crate::engine::shared::helper_func::print_utility::{
    from_move_notation, move_notation, print_chess,
};
use crate::engine::shared::structures::color::ColorTrait;
use crate::engine::shared::structures::internal_move::{PositionIrr, PositionRev};
use crate::engine::{fen, game};

#[derive()]
pub struct UCI {
    pub game: Game,
    max_depth: usize,
    is_searching: Arc<AtomicBool>,
    search_thread: Option<JoinHandle<()>>,
}

impl UCI {
    pub fn init() -> UCI {
        UCI {
            game: Game::initialize(),
            max_depth: 64,
            is_searching: Arc::new(AtomicBool::new(false)),
            search_thread: None,
        }
    }

    fn engine_metadata() {
        println!("id name {}", "Challenger 1.0");
        println!("id author Nikola Simjanovski");
        println!("uciok");
    }

    pub fn main(&mut self) {
        let (tx, rx) = mpsc::channel::<String>();

        let _input_handle = thread::spawn(move || {
            let stdin = io::stdin();
            for line_result in stdin.lock().lines() {
                match line_result {
                    Ok(line) => {
                        if tx.send(line).is_err() {
                            eprintln!("info string Input thread: Main channel closed.");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("info string Input thread: Error reading stdin: {}", e);
                        break;
                    }
                }
            }
            eprintln!("info string Input thread terminating.");
        });

        loop {
            match rx.try_recv() {
                Ok(cmd) => {
                    let args: Vec<&str> = cmd.trim().split_whitespace().collect();
                    if args.is_empty() {
                        continue;
                    }

                    match args[0] {
                        "uci" => self.uci(),
                        "quit" => {
                            self.abort_search();
                            break;
                        }
                        "stop" => self.stop(),
                        "isready" => self.isready(),
                        "ucinewgame" => self.ucinewgame(),
                        "position" => self.position(&args[1..]),
                        "go" => self.go(&args[1..]),
                        _ => eprintln!("info string Unknown command: {}", args[0]),
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    eprintln!("info string Main loop: Input channel disconnected. Exiting.");
                    self.abort_search();
                    break;
                }
            }

            thread::sleep(Duration::from_millis(5));
        }
    }

    fn uci(&mut self) {
        println!("id name {}", "Challenger 1.0");
        println!("id author Nikola Simjanovski");
        println!("uciok");
    }

    fn stop(&mut self) {
        self.stop_search();
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn ucinewgame(&mut self) {
        self.abort_search();

        self.max_depth = 64;
        self.game.reset_board();
    }

    fn position(&mut self, args: &[&str]) {
        self.abort_search();

        let mut is_fen = false;
        let mut fen = Vec::with_capacity(args.len());
        let mut moves = Vec::with_capacity(args.len());

        let mut iter = args.iter();
        while let Some(&arg) = iter.next() {
            match arg {
                "startpos" => fen.push(FEN_START),
                "fen" => is_fen = true,
                "moves" => is_fen = false,
                _ => match is_fen {
                    true => fen.push(arg),
                    false => moves.push(arg),
                },
            }
        }

        self.game = Game::read_fen(&fen.join(" "));

        for s in moves {
            let (irr, rev) = from_move_notation(s, &self.game);
            self.game.make_move(&rev, &irr);
        }
        self.game.ply = 0;
        // print_chess(&self.game);
    }

    fn go(&mut self, args: &[&str]) {
        self.abort_search();

        let mut depth: Option<usize> = None;
        let mut infinite = false;
        let mut time_limit: Option<Duration> = None;

        let mut wtime: Option<usize> = None;
        let mut btime: Option<usize> = None;
        let mut winc: Option<usize> = None;
        let mut binc: Option<usize> = None;
        let mut moves_togo: Option<usize> = None;

        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            match *arg {
                "searchmoves" => (), // TODO:
                "ponder" => (),      // TODO:
                "nodes" => (),       // TODO:
                "mate" => (),        // TODO:
                "wtime" => wtime = iter.next().and_then(|v| v.parse().ok()),
                "btime" => btime = iter.next().and_then(|v| v.parse().ok()),
                "winc" => winc = iter.next().and_then(|v| v.parse().ok()),
                "binc" => binc = iter.next().and_then(|v| v.parse().ok()),
                "movestogo" => moves_togo = iter.next().and_then(|v| v.parse().ok()),
                "depth" => {
                    if let Some(d) = iter.next().and_then(|v| v.parse().ok()) {
                        depth = Some(d);
                        infinite = false;
                    }
                }
                "movetime" => {
                    if let Some(time) = iter.next().and_then(|v| v.parse::<u64>().ok()) {
                        time_limit = Some(Duration::from_millis(time));
                        infinite = false;
                    }
                }
                "infinite" => infinite = true,
                _ => {}
            }
        }

        self.is_searching.store(false, Ordering::Relaxed);

        let mut game_clone = self.game.clone();
        let stop_flag_clone = Arc::clone(&self.is_searching);

        let handle = thread::spawn(move || {
            game_clone.info.start_time = Instant::now();
            game_clone.info.infinite = infinite;
            game_clone.info.depth = depth.or(Some(12));
            game_clone.info.infinite = infinite;

            if !infinite && matches!(time_limit, None) && game_clone.color.is_white() {
                game_clone.info.time_limit = Some(Duration::from_millis(
                    ((wtime.unwrap_or(0) / moves_togo.unwrap_or(40)) + winc.unwrap_or(0)) as u64,
                ));
            } else if !infinite && matches!(time_limit, None) && game_clone.color.is_black() {
                game_clone.info.time_limit = Some(Duration::from_millis(
                    ((btime.unwrap_or(0) / moves_togo.unwrap_or(40)) + binc.unwrap_or(0)) as u64,
                ));
            } else {
                game_clone.info.time_limit = time_limit.or(Some(Duration::from_millis(u64::MAX)));
            }

            let best_move: Option<PositionRev> = iterative_deepening(&mut game_clone);

            if !stop_flag_clone.load(Ordering::Relaxed) || best_move.is_some() {
                if let Some(mv) = best_move {
                    println!(
                        "bestmove {}",
                        move_notation(mv.from, mv.to, mv.flag.get_promo_piece())
                    );
                } else {
                    if !stop_flag_clone.load(Ordering::Relaxed) {
                        eprintln!(
                            "info string Search finished but no move found (e.g., game over)."
                        );
                    } else {
                        eprintln!("info string Search stopped before finding a best move.");
                    }
                }
            } else {
                eprintln!("info string Search stopped externally before finding a best move.");
            }
        });

        self.search_thread = Some(handle);
    }

    // FIXME:
    fn start_search(&mut self) {
        self.game.info.stopped = false;
        let mv = iterative_deepening(&mut self.game);
        println!("{:?}", mv);
    }

    fn stop_search(&mut self) {
        self.game.info.stopped = true;

        if self.search_thread.is_some() {
            self.is_searching.store(true, Ordering::Relaxed);
        }
    }

    fn abort_search(&mut self) {
        self.stop_search();

        if let Some(handle) = self.search_thread.take() {
            eprintln!("info string Waiting for search thread to finish...");
            match handle.join() {
                Ok(_) => eprintln!("info string Search thread joined successfully."),
                Err(e) => eprintln!("info string Error joining search thread: {:?}", e),
            }
        }
    }
}
