use crate::engine::{
    game::Game,
    move_generation::{make_move::GameMoveTrait, mv_gen::move_exists},
    shared::structures::internal_move::InternalMove,
};

const MAX_TT_ENTRIES: usize = 5000;

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct TTEntry {
//     pub pos_key: u64,
//     pub mv: InternalMove,
// }

// impl TTEntry {
//     fn init(pos_key: u64, mv: InternalMove) -> Self {
//         Self { pos_key, mv }
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TTTable {
    pub table: [Option<InternalMove>; MAX_TT_ENTRIES],
}

impl TTTable {
    pub fn init() -> Self {
        Self { table: [None; MAX_TT_ENTRIES] }
    }

    pub fn idx(pos_key: u64) -> usize {
        return (pos_key % MAX_TT_ENTRIES as u64) as usize;
    }

    pub fn set(&mut self, pos_key: u64, mv: InternalMove) {
        self.table[Self::idx(pos_key)] = Some(mv);
    }

    pub fn get(&self, pos_key: u64) -> Option<InternalMove> {
        let idx = Self::idx(pos_key);
        if let Some(mv) = self.table[idx] {
            if mv.position_key == pos_key {
                return Some(mv);
            }
        }
        return None;
    }

    pub fn clear(&mut self) {
        self.table.fill(None);
    }
}

pub fn get_line(game: &mut Game, pos_key: u64) -> Vec<InternalMove> {
    let mut line: Vec<InternalMove> = Vec::with_capacity(64); // TODO: Max Depth Add as a constant
    let mut optional_mv = TTTable::get(&game.tt, pos_key);
    let mut idx = 0;

    while let Some(mv) = &optional_mv {
        if line.len() >= 64 {
            break;
        }

        line.push(*mv);

        if move_exists(game, mv) {
            idx += 1;
            game.make_move(mv);
        } else {
            break;
        }
        optional_mv = TTTable::get(&game.tt, game.pos_key);
    }

    for _ in 0..idx {
        game.undo_move();
    }

    line
}
