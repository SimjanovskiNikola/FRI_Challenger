use crate::engine::{
    game::Game,
    move_generation::{make_move::GameMoveTrait, mv_gen::move_exists},
    shared::structures::internal_move::{PositionIrr, PositionRev},
};

const MAX_TT_ENTRIES: usize = 20000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TTEntry {
    pub key: u64,
    pub rev: PositionRev,
}

impl TTEntry {
    pub fn init(key: u64, rev: PositionRev) -> Self {
        Self { key, rev }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TTTable {
    pub table: [Option<TTEntry>; MAX_TT_ENTRIES],
}

impl TTTable {
    pub fn init() -> Self {
        Self { table: [None; MAX_TT_ENTRIES] }
    }

    pub fn idx(key: u64) -> usize {
        return (key % MAX_TT_ENTRIES as u64) as usize;
    }

    pub fn set(&mut self, key: u64, rev: PositionRev) {
        self.table[Self::idx(key)] = Some(TTEntry::init(key, rev));
    }

    pub fn get(&self, key: u64) -> Option<TTEntry> {
        let idx = Self::idx(key);
        if let Some(entry) = self.table[idx] {
            if entry.key == key {
                return Some(entry);
            }
        }
        return None;
    }

    pub fn clear(&mut self) {
        self.table.fill(None);
    }
}

pub fn get_line(game: &mut Game, pos_key: u64) -> Vec<PositionRev> {
    let mut line: Vec<PositionRev> = Vec::with_capacity(64); // TODO: Max Depth Add as a constant
    let mut optional_mv = TTTable::get(&game.tt, pos_key);
    let mut idx = 0;
    let mut irr: PositionIrr;

    while let Some(mv) = &optional_mv {
        if line.len() >= 64 {
            break;
        }

        line.push(mv.rev);

        if move_exists(game, &mv.rev) {
            idx += 1;
            irr = PositionIrr::init_with_game(game);
            game.make_move(&mv.rev, &irr);
        } else {
            break;
        }
        optional_mv = TTTable::get(&game.tt, game.key);
    }

    for _ in 0..idx {
        game.undo_move();
    }

    line
}
