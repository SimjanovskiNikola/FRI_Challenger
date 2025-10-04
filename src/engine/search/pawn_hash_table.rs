use once_cell::sync::Lazy;
use std::sync::{
    RwLock,
    atomic::{AtomicI8, AtomicU64, Ordering},
};

// const MAX_TT_ENTRIES: u64 = 400211;
const MAX_TT_ENTRIES: u64 = 403139;

pub static PAWN_TT: Lazy<RwLock<PawnHashTable>> = Lazy::new(|| RwLock::new(PawnHashTable::init()));

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
pub struct PawnEntry {
    pub key: u64,                      // 8Bytes
    pub shelter: [(i16, i16, i16); 2], // 12Bytes
    pub pawn_eval: [(i16, i16); 2],    // 8Bytes // Can be one i32 max
    pub candidate_passed: [u64; 2],    // 16Bytes
    pub age: i8,                       // 1Byte
} // Total:  103 Bytes (Padding to 71 Bytes on 64-bit)                                 

impl PawnEntry {
    pub fn init(
        key: u64,
        shelter: [(i16, i16, i16); 2],
        pawn_eval: [(i16, i16); 2],
        candidate_passed: [u64; 2],
        age: i8,
    ) -> Self {
        Self { key, shelter, pawn_eval, candidate_passed, age }
    }
}

#[derive(Debug)]
pub struct PawnHashTable {
    pub table: Box<[Option<PawnEntry>]>,
    pub lookups: AtomicU64,
    pub inserts: AtomicU64,
    pub hits: AtomicU64,
    pub collisions: AtomicU64,
    pub curr_age: AtomicI8,
}

impl PawnHashTable {
    pub fn init() -> Self {
        Self {
            table: vec![None; MAX_TT_ENTRIES as usize].into_boxed_slice(), //Box::new([None; MAX_TT_ENTRIES]),
            lookups: AtomicU64::new(0),
            inserts: AtomicU64::new(0),
            hits: AtomicU64::new(0),
            collisions: AtomicU64::new(0),
            curr_age: AtomicI8::new(0),
        }
    }

    #[inline(always)]
    pub fn idx(key: u64) -> usize {
        return (key % MAX_TT_ENTRIES) as usize;
    }

    pub fn set(
        &mut self,
        key: u64,
        shelter: [(i16, i16, i16); 2],
        pawn_eval: [(i16, i16); 2],
        candidate_passed: [u64; 2],
    ) {
        self.inserts.fetch_add(1, Ordering::Relaxed);

        if let Some(entry) = self.table[Self::idx(key)] {
            self.collisions.fetch_add(1, Ordering::Relaxed);
            if entry.age < self.curr_age.load(Ordering::Relaxed) {
                self.table[Self::idx(key)] = Some(PawnEntry::init(
                    key,
                    shelter,
                    pawn_eval,
                    candidate_passed,
                    self.curr_age.load(Ordering::Relaxed),
                ));
            }
            return;
        }

        self.table[Self::idx(key)] = Some(PawnEntry::init(
            key,
            shelter,
            pawn_eval,
            candidate_passed,
            self.curr_age.load(Ordering::Relaxed),
        ));
    }

    pub fn get(&self, key: u64) -> Option<PawnEntry> {
        self.lookups.fetch_add(1, Ordering::Relaxed);

        if let Some(entry) = self.table[Self::idx(key)] {
            if entry.key == key {
                self.hits.fetch_add(1, Ordering::Relaxed);
                return Some(entry);
            }
        }

        return None;
    }

    pub fn print_stats(&self) {
        println!(
            "PAWN_TT -> lookups: {}; inserts: {}; hits: {}; collisions: {};",
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

    pub fn clear_stats(&mut self) {
        self.hits.store(0, Ordering::Relaxed);
        self.collisions.store(0, Ordering::Relaxed);
        self.inserts.store(0, Ordering::Relaxed);
        self.lookups.store(0, Ordering::Relaxed);
        self.curr_age.fetch_add(1, Ordering::Relaxed);
    }
}
