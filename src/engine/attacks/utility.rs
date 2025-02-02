use std::arch::x86_64::{_pdep_u64, _pext_u64};

pub fn insert_bits(mask: u64, occupancy: u64) -> u64 {
    let mut result = 0;
    let mut bit = 0;
    for i in 0..64 {
        if (mask >> i) & 1 == 1 {
            if (occupancy >> bit) & 1 == 1 {
                result |= 1 << i;
            }
            bit += 1;
        }
    }
    return result;
}

pub fn pext(bitboard: u64, mask: u64) -> u64 {
    unsafe { _pext_u64(bitboard, mask) }
}

pub fn pdep(bitboard: u64, mask: u64) -> u64 {
    unsafe { _pdep_u64(bitboard, mask) }
}
