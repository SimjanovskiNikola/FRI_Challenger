use std::io::BufRead;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use std::{io, thread, u64};

use crate::engine::board::board::Board;
use crate::engine::board::color::ColorTrait;
use crate::engine::board::fen::FenTrait;
use crate::engine::board::moves::Move;
use crate::engine::misc::const_utility::FEN_START;
use crate::engine::misc::display::display_moves::{from_move_notation, move_notation};
use crate::engine::move_generator::make_move::BoardMoveTrait;
use crate::engine::search::iter_deepening::Search;
use crate::engine::search::transposition_table::{TT, TTTable};

use super::time::set_time_limit;

#[derive(Debug)]
pub struct UCITime {
    pub start_time: Instant,
    pub time_limit: Option<Duration>,
    pub moves_togo: usize,
    pub infinite: bool,
    pub max_depth: i8,
    pub quit: bool,
    pub stopped: bool,
}

impl UCITime {
    pub fn init() -> Self {
        Self {
            start_time: Instant::now(),
            time_limit: None,
            moves_togo: 0,
            infinite: false,
            max_depth: 63,
            quit: false,
            stopped: false,
        }
    }
}

#[derive()]
pub struct UCI {
    pub board: Board,
    pub uci: Arc<RwLock<UCITime>>,

    pub search_thread: Option<JoinHandle<()>>,
    pub is_searching: Arc<AtomicBool>,
}

impl UCI {
    pub fn init() -> UCI {
        UCI {
            board: Board::initialize(),
            uci: Arc::new(RwLock::new(UCITime::init())),
            search_thread: None,
            is_searching: Arc::new(AtomicBool::new(false)),
        }
    }

    // Main loop that processes UCI commands
    pub fn main(&mut self) {
        let (tx, rx) = mpsc::channel::<String>();

        // This thread is required for reading from stdin
        // It takes the line that is written and sends it to the main thread
        // so that the main thread can process it
        let _input_handle = thread::spawn(move || {
            let stdin = io::stdin();
            for line_result in stdin.lock().lines() {
                tx.send(line_result.expect("[Input thread]: Failed to read line from stdin !!!"))
                    .expect("[Input thread]: Failed to send line to receiver. Exiting...");
            }
        });

        // Creating an infinite loop that will keep running until the "quit" command is received
        // It processes the commands received from the input thread
        loop {
            match rx.try_recv() {
                Ok(cmd) => {
                    let args: Vec<&str> = cmd.trim().split_whitespace().collect();
                    if args.is_empty() {
                        continue;
                    }

                    match args[0] {
                        "uci" => self.uci_metadata(),
                        "quit" => {
                            self.abort_search();
                            break;
                        }
                        "stop" => self.uci_stop(),
                        "isready" => self.uci_is_ready(),
                        "ucinewgame" => self.uci_new_game(),
                        "position" => self.uci_position(&args[1..]),
                        "go" => self.uci_go(&args[1..]),
                        _ => eprintln!("[Main Loop Thread]: Unknown command: {}", args[0]),
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    eprintln!("[Main Loop Thread]: Channel disconnected. Exiting...");
                    self.abort_search();
                    break;
                }
            }

            // Sleep for a short duration to prevent busy-waiting
            thread::sleep(Duration::from_millis(5));
        }
    }

    // Metadata about the engine
    fn uci_metadata(&mut self) {
        println!("id name {}", "FRI Challenger 0.5.0");
        println!("id author Nikola Simjanovski");
        println!("uciok");
    }

    // Stop the current search
    fn uci_stop(&mut self) {
        self.stop_search();
    }

    // Engine is ready to receive commands
    fn uci_is_ready(&mut self) {
        println!("readyok");
    }

    // Start a new game
    fn uci_new_game(&mut self) {
        self.abort_search();

        self.board.reset();
        self.board.eval.full_reset();
        TT.write().unwrap().clear();
    }

    // Set up the board position from FEN or startpos and apply the given moves
    fn uci_position(&mut self, args: &[&str]) {
        self.board.eval.full_reset();
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

        // Apply FEN on to the board
        self.board = Board::read_fen(&fen.join(" "));

        for str_mv in moves {
            let mv = from_move_notation(str_mv, &mut self.board);
            self.board.make_move(&mv);
            // Must be removed every time so that it does not exceed ply (64 moves)
            self.board.moves.pop();
        }
    }

    fn uci_go(&mut self, args: &[&str]) {
        self.abort_search();

        let mut depth: Option<i8> = None;
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
        self.uci.write().unwrap().max_depth = depth.unwrap_or(63);

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

        self.is_searching.store(false, Ordering::Relaxed);

        self.create_search_thread();
    }

    ///
    /// Creates and starts a new search thread
    ///
    fn create_search_thread(&mut self) {
        let stop_flag_clone = Arc::clone(&self.is_searching);
        let board_clone = self.board.clone();
        let uci_clone = self.uci.clone();
        let mut search = Search::init(board_clone, uci_clone);

        let handle = thread::spawn(move || {
            let best_move: Option<Move> = search.iterative_deepening();

            if !stop_flag_clone.load(Ordering::Relaxed) || best_move.is_some() {
                if let Some(mv) = best_move {
                    let mv_notation = move_notation(mv.from, mv.to, mv.flag.get_promo_piece());
                    println!("bestmove {}", mv_notation);
                } else {
                    panic!("Search finished but no move found !!!");
                }
            } else {
                panic!("Search stopped externally before finding a best move !!!");
            }
        });

        self.search_thread = Some(handle);
    }

    ///
    /// Aborts the current search and joins the search and timer threads
    ///
    fn abort_search(&mut self) {
        self.stop_search();
        if let Some(search_handle) = self.search_thread.take() {
            search_handle.join().expect("Error while joining search thread");
        }

        self.uci.write().unwrap().stopped = false;
    }

    ///
    /// Stops the search by setting the stopped flag to true
    ///
    fn stop_search(&mut self) {
        self.uci.write().unwrap().stopped = true;

        if self.search_thread.is_some() {
            self.is_searching.store(true, Ordering::Relaxed);
        }
    }
}
