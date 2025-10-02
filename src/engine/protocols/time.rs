use crate::engine::search::iter_deepening::Search;
use std::{sync::atomic::Ordering, time::Duration};

#[inline(always)]
pub fn set_time_limit(movestogo: usize, time: usize, inc: usize) -> Duration {
    let time_per_move = time as f64 / movestogo.max(1) as f64;

    let safe_time = time_per_move + (inc as f64 * 0.8);
    let max_allowed = time as f64 * 0.9;

    let alloc = safe_time.min(max_allowed);
    Duration::from_millis(alloc as u64)
}

#[inline(always)]
pub fn safe_to_start_next_iter(search: &Search) -> bool {
    if time_over(search) {
        return false;
    }

    let elapsed = search.uci.start_time.elapsed();
    let total_time = search.uci.time_limit.unwrap_or(Duration::from_millis(u64::MAX));
    let remaining_time = total_time - elapsed;
    let threshold = remaining_time.mul_f32(0.4); // 40% of remaining time

    // Allow another iteration only if less than 40% of remaining time has been used
    elapsed < (total_time - threshold)
}

#[inline(always)]
pub fn time_over(search: &Search) -> bool {
    let elapsed = search.uci.start_time.elapsed();
    let limit = search.uci.time_limit.unwrap_or(Duration::from_millis(u64::MAX));
    let stopped = search.uci.stopped.load(Ordering::Relaxed);
    elapsed >= limit || stopped
}
