use crate::engine::{
    game::Game,
    move_generation::{
        make_move::{GameMoveTrait},
        mv_gen::gen_moves,
    },
    shared::structures::internal_move::InternalMove,
};

const MAX_TT_ENTRIES: usize = 5000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PvEntry {
    pos_key: u64,
    mv: InternalMove,
}

impl PvEntry {
    fn init(pos_key: u64, mv: InternalMove) -> Self {
        Self { pos_key, mv }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PvTable {
    pub table: [Option<PvEntry>; MAX_TT_ENTRIES],
}

impl PvTable {
    pub fn init() -> Self {
        Self { table: [None; MAX_TT_ENTRIES] }
    }

    fn idx(pos_key: u64) -> usize {
        return (pos_key % MAX_TT_ENTRIES as u64) as usize;
    }

    pub fn set(&mut self, pos_key: u64, mv: InternalMove) {
        self.table[Self::idx(pos_key)] = Some(PvEntry::init(pos_key, mv));
    }

    pub fn get(&self, pos_key: u64) -> Option<InternalMove> {
        let idx = Self::idx(pos_key);
        if let Some(entry) = self.table[idx] {
            if entry.pos_key == pos_key {
                return Some(entry.mv);
            }
        }
        return None;
    }

    pub fn get_line(&mut self, game: &mut Game, pos_key: u64) -> usize {
        let mut mv = Self::get(&self, pos_key);
        let mut idx: usize = 0;
        let count: usize;

        while let Some(int_mv) = mv {
            if idx >= 64 {
                break;
            }

            if move_exists(game, &int_mv) {
                game.make_move(&int_mv);
                self.table[idx] = Some(PvEntry::init(pos_key, int_mv));
                idx += 1;
            } else {
                break;
            }

            mv = Self::get(&self, int_mv.position_key);
        }

        count = idx;

        while idx > 0 {
            game.undo_move();
            idx -= 1;
        }

        count
    }

    pub fn clear(&mut self) {
        self.table.fill(None);
    }
}

pub fn move_exists(game: &mut Game, internal_mv: &InternalMove) -> bool {
    let mut move_list: Vec<InternalMove> = gen_moves(game.color, game);

    for mv in &mut move_list {
        if *mv != *internal_mv && game.make_move(mv) {
            game.undo_move();
            return true;
        }
    }
    return false;
}
