const MAX_TT_ENTRIES: u64 = 500211;

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
    pub key: u64,                       // 8Bytes
    pub pawn_behind_masks: [u64; 2],    // 16Bytes
    pub pawn_att_span: [u64; 2],        // 16Bytes
    pub king_pawn_dx: [u8; 2],          // 2Bytes
    pub open_file: [u64; 2],            // 16Bytes
    pub shelter: [(i16, i16, i16); 2],  // 12Bytes
    pub pawn_eval: [(isize, isize); 2], // 32Bytes
    pub candidate_passed: [u64; 2],     // 16Bytes
    // Candidate Passed Pawns,
    pub age: i8, // 1Byte
} // Total:  103 Bytes (Padding to 71 Bytes on 64-bit)                                 

impl PawnEntry {
    pub fn init(
        key: u64,
        pawn_behind_masks: [u64; 2],
        pawn_att_span: [u64; 2],
        king_pawn_dx: [u8; 2],
        open_file: [u64; 2],
        shelter: [(i16, i16, i16); 2],
        pawn_eval: [(isize, isize); 2],
        candidate_passed: [u64; 2],
        age: i8,
    ) -> Self {
        Self {
            key,
            pawn_behind_masks,
            pawn_att_span,
            king_pawn_dx,
            open_file,
            shelter,
            pawn_eval,
            candidate_passed,
            age,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PawnHashTable {
    pub table: Box<[Option<PawnEntry>]>,
    pub lookups: u64,
    pub inserts: u64,
    pub hits: u64,
    pub collisions: u64,
    pub curr_age: i8,
}

impl PawnHashTable {
    pub fn init() -> Self {
        Self {
            table: vec![None; MAX_TT_ENTRIES as usize].into_boxed_slice(), //Box::new([None; MAX_TT_ENTRIES]),
            lookups: 0,
            inserts: 0,
            hits: 0,
            collisions: 0,
            curr_age: 0,
        }
    }

    #[inline(always)]
    pub fn idx(key: u64) -> usize {
        return (key % MAX_TT_ENTRIES) as usize;
    }

    pub fn set(
        &mut self,
        key: u64,
        pawn_behind_masks: [u64; 2],
        pawn_att_span: [u64; 2],
        king_pawn_dx: [u8; 2],
        open_file: [u64; 2],
        shelter: [(i16, i16, i16); 2],
        pawn_eval: [(isize, isize); 2],
        candidate_passed: [u64; 2],
    ) {
        self.inserts += 1;

        if let Some(entry) = self.table[Self::idx(key)] {
            self.collisions += 1;
            if entry.age < self.curr_age {
                self.table[Self::idx(key)] = Some(PawnEntry::init(
                    key,
                    pawn_behind_masks,
                    pawn_att_span,
                    king_pawn_dx,
                    open_file,
                    shelter,
                    pawn_eval,
                    candidate_passed,
                    self.curr_age,
                ));
            }
            return;
        }

        self.table[Self::idx(key)] = Some(PawnEntry::init(
            key,
            pawn_behind_masks,
            pawn_att_span,
            king_pawn_dx,
            open_file,
            shelter,
            pawn_eval,
            candidate_passed,
            self.curr_age,
        ));
    }

    pub fn get(&mut self, key: u64) -> Option<PawnEntry> {
        if let Some(entry) = self.table[Self::idx(key)] {
            if entry.key == key {
                self.hits += 1;
                return Some(entry);
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
        self.clear_stats();
        self.curr_age = 0;
    }

    pub fn clear_stats(&mut self) {
        self.hits = 0;
        self.collisions = 0;
        self.inserts = 0;
        self.lookups = 0;
        self.curr_age += 1;
    }
}
