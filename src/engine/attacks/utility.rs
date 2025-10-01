use std::arch::x86_64::_pdep_u64;
use std::arch::x86_64::_pext_u64;

// This Functions are used for sliding pieces (Rook, Bishop, Queen)
// And the creation of the BISHOP_LOOKUP and ROOK_LOOKUP tables

#[inline(always)]
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

    result
}

#[inline]
pub fn pext(bitboard: u64, mask: u64) -> u64 {
    unsafe { _pext_u64(bitboard, mask) }
}

#[inline(always)]
pub fn pdep(bitboard: u64, mask: u64) -> u64 {
    unsafe { _pdep_u64(bitboard, mask) }
}
