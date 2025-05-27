use crate::engine::search::iter_deepening::Search;
use std::time::Duration;

pub fn set_time_limit(movestogo: usize, time: usize, inc: usize) -> Duration {
    let time_per_move = time as f64 / movestogo.max(1) as f64;

    let safe_time = time_per_move + (inc as f64 * 0.8);
    let max_allowed = time as f64 * 0.9;

    let alloc = safe_time.min(max_allowed);
    Duration::from_millis(alloc as u64)
}

pub fn safe_to_start_next_iter(search: &Search) -> bool {
    if time_over(search) {
        return false;
    }
    let uci = search.uci.read().unwrap();

    let elapsed = uci.start_time.elapsed();
    let total_time = uci.time_limit.unwrap_or(Duration::from_millis(u64::MAX));
    let remaining_time = total_time - elapsed;
    let threshold = remaining_time.mul_f32(0.4); // 40% of remaining time

    // Allow another iteration only if less than 40% of remaining time has been used
    elapsed < (total_time - threshold)
}

pub fn time_over(search: &Search) -> bool {
    let uci = search.uci.read().unwrap();
    let elapsed = uci.start_time.elapsed();
    let limit = uci.time_limit.unwrap_or(Duration::from_millis(u64::MAX));
    let stopped = uci.stopped;
    drop(uci);
    // println!("{:?}", elapsed);
    // println!("{:?}", limit);
    // println!("{:?}", stopped);
    // println!("{:?}", elapsed >= limit || stopped);
    elapsed >= limit || stopped
}
