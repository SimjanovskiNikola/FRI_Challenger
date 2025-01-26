use super::const_utility::{CLEAR_MASK, MOD67TABLE, SET_MASK};

pub type Bitboard = u64;

pub enum Shift {
    Up = 8,
    Right = 1,
    Down = -8,
    Left = -1,
    UpRight = 9,
    UpLeft = 7,
    DownRight = -7,
    DownLeft = -9,
}

pub trait BitboardTrait {
    //NOTE: Creating Bitboard
    /// Create Bitboard from Square.
    fn init(sq: usize) -> Bitboard;

    //NOTE: Bitboard Shifting, Rotating and other operations with Bitboard
    /** Checks if Bitboard is Empty.*/
    fn is_empty(self) -> bool;
    /** Checks if Bitboard is Empty.*/
    fn intersection(&mut self, bb: Bitboard) -> Bitboard;
    /** Checks if Bitboard is Empty.*/
    fn union(&mut self, bb: Bitboard) -> Bitboard;
    /** Checks if Bitboard is Empty.*/
    fn complement(&mut self);
    /** Checks if Bitboard is Empty.*/
    fn relative_complement(&mut self, bb: Bitboard);
    /** Checks if Bitboard is Empty.*/
    fn implication(&mut self, bb: Bitboard);
    /** Checks if Bitboard is Empty.*/
    fn exclusive_or(&mut self, bb: Bitboard);
    /** Checks if Bitboard is Empty.*/
    fn equivalent(&mut self, bb: Bitboard);
    /** Checks if Bitboard is Empty.*/
    fn shift(&mut self, shift: Shift) -> Bitboard;
    /** Checks if Bitboard is Empty.*/
    fn swap_n_bits(&mut self, i: usize, j: usize, n: usize);
    /** Checks if Bitboard is Empty.*/
    // TODO: fn rotate(self, rotate: Rotate);

    //NOTE: Operations with bits
    /** Checks if Bitboard is Empty.*/
    fn get_lsb(self) -> usize;

    /** Checks if Bitboard is Empty.*/
    fn get_msb(self) -> usize;

    /** Checks if Bitboard is Empty.*/
    fn get_bits(self) -> Vec<usize>;

    /** Checks if Bitboard is Empty.*/
    fn pop_lsb(&mut self) -> usize;

    /** Checks if Bitboard is Empty.*/
    fn set_bit(&mut self, sq: usize);

    /** Checks if Bitboard is Empty.*/
    fn clear_bit(&mut self, sq: usize);

    /** Checks if Bitboard is Empty.*/
    fn count(self) -> usize;

    /** Checks if Bitboard is Empty.*/
    fn is_set(self, sq: usize) -> bool;

    /** Checks if Bitboard is Empty.*/
    fn print(self, mark: Option<usize>);
}

impl BitboardTrait for Bitboard {
    fn init(sq: usize) -> Bitboard {
        return 1 << sq;
    }

    fn is_empty(self) -> bool {
        return self != 0;
    }

    fn intersection(&mut self, bb: Bitboard) -> Bitboard {
        *self &= bb;
        return *self;
    }

    fn union(&mut self, bb: Bitboard) -> Bitboard {
        *self |= bb;
        return *self;
    }

    fn complement(&mut self) {
        *self = !*self;
    }

    fn relative_complement(&mut self, bb: Bitboard) {
        *self = !*self & bb;
    }

    fn implication(&mut self, bb: Bitboard) {
        *self = !*self | bb;
    }

    fn exclusive_or(&mut self, bb: Bitboard) {
        *self ^= bb;
    }

    fn equivalent(&mut self, bb: Bitboard) {
        *self = !(*self ^ bb);
    }

    fn shift(&mut self, shift: Shift) -> Bitboard {
        let shift = shift as isize;
        if (shift as isize) > 0 {
            *self = *self << shift as usize
        } else {
            *self = *self >> (-shift as usize);
        };
        return *self;
    }

    fn swap_n_bits(&mut self, i: usize, j: usize, n: usize) {
        let m: u64 = (1 << n) - 1;
        let x: u64 = ((*self >> i) ^ (*self >> j)) & m;
        *self = *self ^ (x << i) ^ (x << j);
    }

    fn get_lsb(self) -> usize {
        let bit: u64 = self ^ (self - 1) ^ (!self & (self - 1));
        return MOD67TABLE[(bit % 67) as usize];
    }

    fn get_msb(self) -> usize {
        return (self as f64).log2().floor() as usize;
    }

    fn get_bits(self) -> Vec<usize> {
        let mut result = vec![];
        let mut bb = self;

        while bb != 0 {
            let next_bit = bb.get_lsb();
            bb ^= 1 << next_bit;
            result.push(next_bit);
        }

        return result;
    }

    fn pop_lsb(&mut self) -> usize {
        let idx = self.get_lsb();
        *self ^= 1 << idx;
        return idx;
    }

    fn set_bit(&mut self, sq: usize) {
        *self |= SET_MASK[sq];
    }

    fn clear_bit(&mut self, sq: usize) {
        *self &= CLEAR_MASK[sq];
    }

    fn count(self) -> usize {
        return self.get_bits().len();
    }

    fn is_set(self, sq: usize) -> bool {
        return Bitboard::is_empty(self & Bitboard::init(sq));
    }

    fn print(self, mark: Option<usize>) {
        let mut row = "".to_owned();
        let mut board = "".to_owned();

        for i in 0..64 {
            let value = (self >> i) & 1;
            let s = if value == 0 { ". ".to_owned() } else { format!("{} ", value.to_string()) };
            match mark {
                Some(idx) => {
                    if i == idx {
                        row.push_str("X");
                    } else {
                        row.push_str(&s);
                    }
                }
                None => row.push_str(&s),
            }

            if (i + 1) % 8 == 0 {
                row.push_str("\n");
                board.insert_str(0, &row);
                row.clear();
            }
        }
        println!("Bitboard: \n------Start------\n{}-------End-------", board);
    }
}
