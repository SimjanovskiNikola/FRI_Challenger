use std::sync::{atomic::AtomicU64, Mutex};

use crossbeam::queue::ArrayQueue;

use crate::engine::{
    game::{self, Game},
    move_generation::{make_move::GameMoveTrait, mv_gen::move_exists},
    shared::structures::internal_move::{PositionIrr, PositionRev},
};

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
    pub rev: PositionRev,
    pub score: i16,
    pub depth: u8,
    pub category: Bound,
}

impl TTEntry {
    pub fn init(key: u64, rev: PositionRev, score: i16, depth: u8, category: Bound) -> Self {
        Self { key, rev, score, depth, category }
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

    pub fn set(&mut self, key: u64, rev: PositionRev, score: i16, depth: u8, category: Bound) {
        self.inserts += 1;
        if self.table[Self::idx(key)].is_some() {
            self.collisions += 1;
        }
        self.table[Self::idx(key)] = Some(TTEntry::init(key, rev, score, depth, category));
    }

    pub fn probe(
        &mut self,
        key: u64,
        depth: u8,
        mut alpha: i16,
        mut beta: i16,
    ) -> Option<(i16, PositionRev)> {
        self.lookups += 1;
        let idx = Self::idx(key);
        if let Some(e) = self.table[idx] {
            if e.key == key && e.depth >= depth {
                match e.category {
                    Bound::Lower => alpha = alpha.max(e.score),
                    Bound::Exact => {
                        self.hits += 1;
                        return Some((e.score, e.rev));
                    }
                    Bound::Upper => beta = beta.min(e.score),
                }
                if alpha >= beta {
                    self.hits += 1;
                    return Some((e.score, e.rev));
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

    pub fn get_line(&self, game: &mut Game) -> Vec<PositionRev> {
        let mut line: Vec<PositionRev> = Vec::with_capacity(64); // TODO: Max Depth Add as a constant
        let mut moves_made = 0;

        while let Some(mv) = self.get(game.key) {
            if line.len() >= 64 {
                break;
            }

            line.push(mv.rev);

            if move_exists(game, &mv.rev) {
                game.make_move(&mv.rev, &PositionIrr::init_with_game(game));
                moves_made += 1;
            } else {
                break;
            }
        }

        while moves_made > 0 {
            game.undo_move();
            moves_made -= 1;
        }

        line
    }
}
