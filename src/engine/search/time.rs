use std::time::Duration;

use crate::engine::game::Game;

pub fn set_time_limit(mv_played: usize, time: usize, inc: usize) -> Duration {
    let nMoves = mv_played.min(10);
    let factor = 2 - nMoves / 10;
    let target = (time + inc) / (60 - mv_played);
    let limit: u64 = (factor * target) as u64;
    println!("{:?} {:?} {:?} {:?}", nMoves, factor, target, limit);
    Duration::from_millis(limit)
}

pub fn safe_to_start_next_iter(game: &Game) -> bool {
    if time_over(game) {
        return false;
    }

    let elapsed = game.info.start_time.elapsed();
    let total_time = game.info.time_limit.unwrap_or(Duration::from_millis(u64::MAX));
    let remaining_time = total_time - elapsed;
    let threshold = remaining_time.mul_f32(0.4); // 40% of remaining time

    // Allow another iteration only if less than 40% of remaining time has been used
    elapsed < (total_time - threshold)
}

pub fn time_over(game: &Game) -> bool {
    game.info.start_time.elapsed()
        >= game.info.time_limit.unwrap_or(Duration::from_millis(u64::MAX))
        || game.info.stopped
}
