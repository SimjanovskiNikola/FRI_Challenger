use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::sync::atomic::AtomicI16;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use crate::engine::board::board::Board;
use crate::engine::board::moves::ExtendedMove;
use crate::engine::board::moves::Move;
use crate::engine::move_generator::make_move::BoardMoveTrait;
use crate::engine::move_generator::mv_gen::BoardGenMoveTrait;

pub static TT: Lazy<RwLock<TTTable>> = Lazy::new(|| RwLock::new(TTTable::init()));

const MAX_TT_ENTRIES: usize = 403139;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bound {
    Lower,
    Exact,
    Upper,
}

// NOTE: 64 + 32 + 16 + 8 + 8 = 128 BITS = 16 Bytes
// NOTE: 1Mb = 1000000 Bytes = 166,666 Entries
// NOTE: Currently Around 15Mb
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TTEntry {
    pub key: u64,        // 8 Bytes Max
    pub mv: Move,        // 6 Bytes Max
    pub score: i16,      // 2 Bytes
    pub depth: i8,       // 1 Byte
    pub category: Bound, // 1 Byte
    pub age: i16,        // 2 Bytes
}

impl TTEntry {
    pub fn init(key: u64, mv: Move, score: i16, depth: i8, category: Bound, age: i16) -> Self {
        Self { key, mv, score, depth, category, age }
    }
}

#[derive(Debug)]
pub struct TTTable {
    pub table: Box<[Option<TTEntry>]>,
    pub lookups: AtomicU64,
    pub inserts: AtomicU64,
    pub hits: AtomicU64,
    pub collisions: AtomicU64,
    pub curr_age: AtomicI16,
}

impl TTTable {
    pub fn init() -> Self {
        Self {
            table: vec![None; MAX_TT_ENTRIES].into_boxed_slice(), //Box::new([None; MAX_TT_ENTRIES]),
            lookups: AtomicU64::new(0),
            inserts: AtomicU64::new(0),
            hits: AtomicU64::new(0),
            collisions: AtomicU64::new(0),
            curr_age: AtomicI16::new(0),
        }
    }

    #[inline(always)]
    pub fn idx(key: u64) -> usize {
        return (key % MAX_TT_ENTRIES as u64) as usize;
    }

    pub fn set(&mut self, key: u64, mv: Move, score: i16, depth: i8, category: Bound) {
        self.inserts.fetch_add(1, Ordering::Relaxed);

        if let Some(entry) = self.table[Self::idx(key)] {
            self.collisions.fetch_add(1, Ordering::Relaxed);
            if (entry.age < self.curr_age.load(Ordering::Relaxed)) || (entry.depth <= depth) {
                self.table[Self::idx(key)] = Some(TTEntry::init(
                    key,
                    mv,
                    score,
                    depth,
                    category,
                    self.curr_age.load(Ordering::Relaxed),
                ));
            }
            return;
        }

        self.table[Self::idx(key)] = Some(TTEntry::init(
            key,
            mv,
            score,
            depth,
            category,
            self.curr_age.load(Ordering::Relaxed),
        ));
    }

    pub fn probe(&self, key: u64, depth: i8, mut alpha: i16, mut beta: i16) -> Option<(i16, Move)> {
        self.lookups.fetch_add(1, Ordering::Relaxed);
        let idx = Self::idx(key);
        if let Some(e) = self.table[idx] {
            if e.key == key
                && (e.depth as i16 + e.age)
                    >= (depth as i16 + self.curr_age.load(Ordering::Relaxed))
            {
                match e.category {
                    Bound::Lower => alpha = alpha.max(e.score),
                    Bound::Exact => {
                        self.hits.fetch_add(1, Ordering::Relaxed);
                        return Some((e.score, e.mv));
                    }
                    Bound::Upper => beta = beta.min(e.score),
                }
                if alpha >= beta {
                    self.hits.fetch_add(1, Ordering::Relaxed);
                    return Some((e.score, e.mv));
                }
            }
        }

        return None;
    }

    pub fn get(&self, key: u64) -> Option<TTEntry> {
        if let Some(entry) = self.table[Self::idx(key)] {
            if entry.key == key {
                return Some(entry);
            }
        }

        return None;
    }

    pub fn print_stats(&self) {
        println!(
            "TT -> lookups: {}; inserts: {}; hits: {}; collisions: {};",
            self.lookups.load(Ordering::Relaxed),
            self.inserts.load(Ordering::Relaxed),
            self.hits.load(Ordering::Relaxed),
            self.collisions.load(Ordering::Relaxed)
        );
    }

    pub fn clear(&mut self) {
        self.table.fill(None);
        self.clear_stats();
        self.curr_age.store(0, Ordering::Relaxed);
    }

    pub fn clear_stats(&self) {
        self.hits.store(0, Ordering::Relaxed);
        self.collisions.store(0, Ordering::Relaxed);
        self.inserts.store(0, Ordering::Relaxed);
        self.lookups.store(0, Ordering::Relaxed);
        self.curr_age.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_line(&self, board: &mut Board) -> Vec<ExtendedMove> {
        let mut line: Vec<ExtendedMove> = Vec::with_capacity(64); // TODO: Max Depth Add as a constant
        let mut moves_made = 0;

        while let Some(entry) = self.get(board.state.key) {
            if line.len() >= 64 {
                break;
            }

            if board.move_exists(&entry.mv) {
                // println!("Before Key: {:?}", board.state.key);
                board.make_move(&entry.mv);
                line.push(ExtendedMove { mv: entry.mv, key: board.state.key });
                // println!("After Key: {:?}", board.state.key);
                moves_made += 1;
            } else {
                break;
            }
        }

        while moves_made > 0 {
            board.undo_move();
            moves_made -= 1;
        }

        line
    }
}
