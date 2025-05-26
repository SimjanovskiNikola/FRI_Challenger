use crate::engine::board::make_move::BoardMoveTrait;
use crate::engine::board::mv_gen::BoardGenMoveTrait;
use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::moves::Move;

use std::sync::{atomic::AtomicU64, Mutex};

const MAX_TT_ENTRIES: usize = 140211;

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
    pub key: u64,
    pub mv: Move,
    pub score: i16,
    pub depth: u8,
    pub category: Bound,
}

impl TTEntry {
    pub fn init(key: u64, mv: Move, score: i16, depth: u8, category: Bound) -> Self {
        Self { key, mv, score, depth, category }
    }
}

#[derive(Debug)]
pub struct TTTable {
    pub table: Box<[Option<TTEntry>; MAX_TT_ENTRIES]>, //Vec<Option<TTEntry>>,
    pub lookups: u64,
    pub inserts: u64,
    pub hits: u64,
    pub collisions: u64,
}

impl TTTable {
    pub fn init() -> Self {
        Self {
            table: Box::new([None; MAX_TT_ENTRIES]),
            lookups: 0,
            inserts: 0,
            hits: 0,
            collisions: 0,
        }
    }

    pub fn idx(key: u64) -> usize {
        return (key % MAX_TT_ENTRIES as u64) as usize;
    }

    pub fn set(&mut self, key: u64, mv: Move, score: i16, depth: u8, category: Bound) {
        self.inserts += 1;
        if self.table[Self::idx(key)].is_some() {
            self.collisions += 1;
        }
        self.table[Self::idx(key)] = Some(TTEntry::init(key, mv, score, depth, category));
    }

    pub fn probe(
        &mut self,
        key: u64,
        depth: u8,
        mut alpha: i16,
        mut beta: i16,
    ) -> Option<(i16, Move)> {
        self.lookups += 1;
        let idx = Self::idx(key);
        if let Some(e) = self.table[idx] {
            if e.key == key && e.depth >= depth {
                match e.category {
                    Bound::Lower => alpha = alpha.max(e.score),
                    Bound::Exact => {
                        self.hits += 1;
                        return Some((e.score, e.mv));
                    }
                    Bound::Upper => beta = beta.min(e.score),
                }
                if alpha >= beta {
                    self.hits += 1;
                    return Some((e.score, e.mv));
                }
            }
        }

        return None;
    }

    pub fn get(&self, key: u64) -> Option<TTEntry> {
        let idx = Self::idx(key);
        if let Some(entry) = self.table.get(idx) {
            if let Some(e) = *entry {
                if e.key == key {
                    return Some(e);
                }
            }
        }

        return None;
    }

    pub fn print_stats(&self) {
        println!(
            "lookups: {}; inserts: {}; hits: {}; collisions: {};",
            self.lookups, self.inserts, self.hits, self.collisions
        );
    }

    pub fn clear(&mut self) {
        self.table.fill(None);
    }

    pub fn clear_stats(&mut self) {
        self.hits = 0;
        self.collisions = 0;
        self.inserts = 0;
        self.lookups = 0;
    }

    pub fn get_line(&self, board: &mut Board) -> Vec<Move> {
        let mut line: Vec<Move> = Vec::with_capacity(64); // TODO: Max Depth Add as a constant
        let mut moves_made = 0;

        while let Some(entry) = self.get(board.state.key) {
            if line.len() >= 64 {
                break;
            }

            line.push(entry.mv);

            if board.move_exists(&entry.mv) {
                board.make_move(&entry.mv);
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
