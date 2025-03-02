use crate::engine::{
    evaluation::evaluation::Evaluation,
    game::Game,
    move_generation::{
        make_move::GameMoveTrait,
        mv_gen::{gen_moves, is_repetition, sq_attack},
    },
    search::transposition_table::get_line,
    shared::{
        helper_func::{bitboard::BitboardTrait, print_utility::print_move_list},
        structures::{
            internal_move::{Flag, InternalMove},
            piece::KING,
        },
    },
};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInfo {
    start_time: Instant,
    end_time: usize,
    time_limit: Duration,
    depth: usize,
    depth_set: usize,
    time_set: usize,
    moves_togo: usize,
    infinite: bool,

    nodes: usize,

    quit: bool,
    stopped: bool,

    fail_hard: usize,
    fail_hard_first: usize,
}

impl SearchInfo {
    pub fn init() -> Self {
        Self {
            start_time: Instant::now(),
            end_time: 0,
            time_limit: Duration::new(2, 0),
            depth: 0,
            depth_set: 0,
            time_set: 0,
            moves_togo: 0,
            infinite: false,

            nodes: 0,

            quit: false,
            stopped: false,

            fail_hard: 0,
            fail_hard_first: 0,
        }
    }
}

pub fn check_time_up() {
    todo!();
}

pub fn clear_search(game: &mut Game, info: &mut SearchInfo) {
    game.s_killers.iter_mut().for_each(|arr| arr.fill(0));
    game.s_history.iter_mut().for_each(|arr| arr.fill(0));
    game.tt.clear();
    game.ply = 0;

    info.start_time = Instant::now();
    info.stopped = false;
    info.nodes = 0;
}

fn quiescence_search(
    mut alpha: isize,
    beta: isize,
    game: &mut Game,
    info: &mut SearchInfo,
) -> isize {
    let stand_pat = game.evaluate_pos();

    if stand_pat >= beta {
        return beta;
    }

    alpha = alpha.max(stand_pat);

    // TODO: Order Moves with MVV-LVA
    let mut move_list = gen_moves(game.color, game);

    for mv in &mut move_list {
        match mv.flag {
            Flag::Capture(_) => (),
            Flag::EP(_, _) => (),
            Flag::Promotion(_, Some(_)) => (),
            _ => continue,
        }

        if !game.make_move(mv) {
            continue;
        }

        let score = -quiescence_search(-beta, -alpha, game, info);
        game.undo_move();

        if score >= beta {
            return beta;
        }

        alpha = alpha.max(score);
    }

    alpha
}

fn alpha_beta(
    mut alpha: isize,
    mut beta: isize,
    depth: usize,
    game: &mut Game,
    info: &mut SearchInfo,
    take_null: bool,
) -> isize {
    // || game.is_over()
    if depth == 0 {
        info.nodes += 1;
        // return quiescence_search(alpha, beta, game, info);
        return game.evaluate_pos();
    }

    info.nodes += 1;
    // info.start_time.elapsed() >= info.time_limit ||
    if is_repetition(game) || game.half_move >= 100 {
        return 0;
    }

    let mut best_mv = None;
    let mut best_score = MIN_INF;
    let mut legal_mv_num = 0;
    let old_alpha: isize = alpha;

    let mut move_list = gen_moves(game.color, game);
    // TODO: MVV-LVA

    for mv in &mut move_list {
        if !game.make_move(mv) {
            continue;
        }
        legal_mv_num += 1;
        let score = -alpha_beta(-beta, -alpha, depth - 1, game, info, true);

        game.undo_move();

        if score > best_score {
            best_score = score;
            best_mv = Some(mv);
        }

        alpha = alpha.max(score);
        if alpha >= beta {
            if legal_mv_num == 1 {
                info.fail_hard_first += 1;
            }
            info.fail_hard += 1;
            break;
        }
    }

    if legal_mv_num == 0 {
        let king_sq = game.bitboard[(KING + game.color) as usize].get_lsb();
        if sq_attack(&game, king_sq, game.color) != 0 {
            return -1000000;
        } else {
            return 0;
        }
    }

    if alpha != old_alpha {
        match best_mv {
            Some(mv) => game.tt.set(game.pos_key, *mv),
            None => println!("{:?}", "Best Move Was None"),
        }
    }

    best_score
}

pub fn iterative_deepening(game: &mut Game, info: &mut SearchInfo) -> Option<InternalMove> {
    info.start_time = Instant::now();
    info.time_limit = Duration::new(10, 0);
    let mut best_mv = None;
    let mut best_score = MIN_INF;
    clear_search(game, info);

    for depth in 1..8 {
        best_score = alpha_beta(MIN_INF, MAX_INF, depth, game, info, true);

        // if info.start_time.elapsed() >= info.time_limit {
        //     break;
        // }

        let line = get_line(game, game.pos_key);
        if line.len() > 0 {
            best_mv = Some(line[0]);
        }

        let mut mv = InternalMove::init();
        match best_mv {
            Some(m) => mv = m,
            None => (),
        };

        println!("-------------------------");
        println!("Depth: {:?}", depth);
        print_move_list(&[mv]);
        println!("Score {:?}", best_score);
        println!("-------------------------");
        println!("-------Best Line---------");
        print_move_list(&line);
        println!("");
        // println!("Fail Hard First: {:?}, Fail Hard: {:?}", info.fail_hard_first, info.fail_hard);
        println!("Ordering: {:.2}", ((info.fail_hard_first) as f64 / (info.fail_hard + 1) as f64));
        println!("");
    }

    best_mv
}

const MAX_INF: isize = isize::MAX / 2;
const MIN_INF: isize = isize::MIN / 2;
