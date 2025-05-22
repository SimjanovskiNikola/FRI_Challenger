use std::io::BufRead;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use std::{io, thread, u64};

use crate::engine::board::fen::FenTrait;
use crate::engine::board::make_move::BoardMoveTrait;
use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color::ColorTrait;
use crate::engine::board::structures::moves::Move;
use crate::engine::misc::const_utility::FEN_START;
use crate::engine::misc::print_utility::{from_move_notation, move_notation};
use crate::engine::search::searcher::{iterative_deepening, Search};
use crate::engine::search::transposition_table::TTTable;

use super::time::set_time_limit;

#[derive(Debug)]
pub struct NewUCI {
    pub start_time: Instant,
    pub time_limit: Option<Duration>,
    pub moves_togo: usize,
    pub infinite: bool,
    pub max_depth: u8,
    pub moves_played: usize,
    pub quit: bool,
    pub stopped: bool,
    pub is_searching: Arc<AtomicBool>, // FIXME: Maybe Here should be used stopped ???
    pub search_thread: Option<JoinHandle<()>>,
}

impl NewUCI {
    pub fn init() -> Self {
        Self {
            start_time: Instant::now(),
            time_limit: None,
            moves_togo: 0,
            infinite: false,
            max_depth: 64,
            moves_played: 0,
            quit: false,
            stopped: false,
            is_searching: Arc::new(AtomicBool::new(false)),
            search_thread: None,
        }
    }
}

#[derive()]
pub struct UCI {
    pub board: Board,
    pub uci: Arc<RwLock<NewUCI>>,
    pub tt: Arc<Mutex<TTTable>>,
}

impl UCI {
    pub fn init() -> UCI {
        UCI {
            board: Board::initialize(),
            uci: Arc::new(RwLock::new(NewUCI::init())),
            tt: Arc::new(Mutex::new(TTTable::init())),
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

        self.board.reset();
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

        self.uci.write().unwrap().moves_played = 0;
        self.board = Board::read_fen(&fen.join(" "));

        for str_mv in moves {
            let mv = from_move_notation(str_mv, &mut self.board);
            self.board.make_move(&mv);
            self.uci.write().unwrap().moves_played += 1;
        }
        self.board.moves.clear();
    }

    fn go(&mut self, args: &[&str]) {
        self.abort_search();

        let mut depth: Option<u8> = None;
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

        self.uci.write().unwrap().start_time = Instant::now();
        self.uci.write().unwrap().infinite = infinite;
        self.uci.write().unwrap().max_depth = depth.unwrap_or(64);

        if !infinite && matches!(time_limit, None) && self.board.state.color.is_white() {
            self.uci.write().unwrap().time_limit = Some(set_time_limit(
                moves_togo.unwrap_or(30),
                wtime.unwrap_or(0),
                winc.unwrap_or(0),
            ));
        } else if !infinite && matches!(time_limit, None) && self.board.state.color.is_black() {
            self.uci.write().unwrap().time_limit = Some(set_time_limit(
                moves_togo.unwrap_or(30),
                btime.unwrap_or(0),
                binc.unwrap_or(0),
            ));
        } else {
            self.uci.write().unwrap().time_limit =
                time_limit.or(Some(Duration::from_millis(u64::MAX)));
        }

        self.uci.write().unwrap().is_searching.store(false, Ordering::Relaxed);

        let stop_flag_clone = Arc::clone(&self.uci.read().unwrap().is_searching);

        let mut board_clone = self.board.clone();
        let mut tt_clone = self.tt.clone();
        let mut uci_clone = self.uci.clone();

        let search = Search::init(board_clone, tt_clone, uci_clone);

        let handle = thread::spawn(move || {
            let best_move: Option<Move> = iterative_deepening(search);

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

        self.uci.write().unwrap().search_thread = Some(handle);
    }

    // fn start_search(&mut self) {
    //     self.uci.write().unwrap().stopped = false;
    //     let mv = iterative_deepening(&mut self.game);
    //     println!("{:?}", mv);
    // }

    fn stop_search(&mut self) {
        self.uci.write().unwrap().stopped = true;

        if self.uci.read().unwrap().search_thread.is_some() {
            self.uci.write().unwrap().is_searching.store(true, Ordering::Relaxed);
        }
    }

    fn abort_search(&mut self) {
        self.stop_search();

        if let Some(handle) = self.uci.write().unwrap().search_thread.take() {
            eprintln!("info string Waiting for search thread to finish...");
            match handle.join() {
                Ok(_) => eprintln!("info string Search thread joined successfully."),
                Err(e) => eprintln!("info string Error joining search thread: {:?}", e),
            }
        }

        self.uci.write().unwrap().stopped = false;
    }
}
