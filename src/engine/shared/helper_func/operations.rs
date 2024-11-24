pub static notAFile: u64 = 0xfefefefefefefefe;
pub static notHFile: u64 = 0x7f7f7f7f7f7f7f7f;

fn is_empty(a: u64) -> bool {
    return a == 0;
}

fn intersection(a: u64, b: u64) -> u64 {
    return a & b;
}

fn union(a: u64, b: u64) -> u64 {
    return a | b;
}

fn complement(a: u64) -> u64 {
    return !a;
}

fn relative_complement(a: u64, b: u64) -> u64 {
    return complement(a) & b;
}

fn implication(a: u64, b: u64) -> u64 {
    return complement(a) | b;
}

fn exclusive_or(a: u64, b: u64) -> u64 {
    return a ^ b;
}

fn equivalent(a: u64, b: u64) -> u64 {
    return !exclusive_or(a, b);
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//****************************************** SHIFTING****************************************************** */
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//TODO: Learn Inline If
fn shift(bitboard: u64, num: i64) -> u64 {
    if num > 0 {
        return bitboard << num;
    }
    return bitboard >> (-num as usize);
}

fn shift_north_east(bitboard: u64) -> u64 {
    return shift(bitboard, 9);
}

fn shift_north(bitboard: u64) -> u64 {
    return shift(bitboard, 8);
}

fn shift_north_west(bitboard: u64) -> u64 {
    return shift(bitboard, 7);
}

fn shift_west(bitboard: u64) -> u64 {
    return shift(bitboard, -1);
}

fn shift_south_west(bitboard: u64) -> u64 {
    return shift(bitboard, -9);
}

fn shift_south(bitboard: u64) -> u64 {
    return shift(bitboard, -8);
}

fn shift_south_east(bitboard: u64) -> u64 {
    return shift(bitboard, -7);
}

fn shift_east(bitboard: u64) -> u64 {
    return shift(bitboard, 1);
}
