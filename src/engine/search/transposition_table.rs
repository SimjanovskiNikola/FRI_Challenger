use crate::engine::{
    game::{self, Game},
    move_generation::{make_move::GameMoveTrait, mv_gen::move_exists},
    shared::structures::internal_move::{PositionIrr, PositionRev},
};

const MAX_TT_ENTRIES: usize = 2440211;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TTTable {
    pub table: Vec<Option<TTEntry>>,
    pub lookups: u64,
    pub inserts: u64,
    pub hits: u64,
    pub collisions: u64,
}

impl TTTable {
    pub fn init() -> Self {
        Self { table: vec![None; MAX_TT_ENTRIES], lookups: 0, inserts: 0, hits: 0, collisions: 0 }
    }

    pub fn idx(key: u64) -> usize {
        return (key % MAX_TT_ENTRIES as u64) as usize;
    }

    pub fn set(&mut self, key: u64, rev: PositionRev, score: i16, depth: u8, category: Bound) {
        self.table[Self::idx(key)] = Some(TTEntry::init(key, rev, score, depth, category));
    }

    pub fn probe(&self, key: u64, depth: u8, mut alpha: i16, mut beta: i16) -> Option<i16> {
        let idx = Self::idx(key);
        if let Some(entry) = self.table.get(idx) {
            if let Some(e) = *entry {
                if e.key == key && e.depth >= depth {
                    match e.category {
                        Bound::Lower => alpha = alpha.max(e.score),
                        Bound::Exact => return Some(e.score),
                        Bound::Upper => beta = beta.min(e.score),
                    }
                    if alpha >= beta {
                        return Some(e.score);
                    }
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

    pub fn clear(&mut self) {
        self.table.fill(None);
    }
}

pub fn get_line(game: &mut Game, mut pos_key: u64) -> Vec<PositionRev> {
    let mut line: Vec<PositionRev> = Vec::with_capacity(64); // TODO: Max Depth Add as a constant

    while let Some(mv) = game.tt.get(pos_key) {
        if line.len() >= 64 {
            break;
        }

        line.push(mv.rev);

        if move_exists(game, &mv.rev) {
            game.make_move(&mv.rev, &PositionIrr::init_with_game(game));
            pos_key = game.key;
        } else {
            break;
        }
    }

    while game.ply > 0 {
        game.undo_move();
    }

    line
}
