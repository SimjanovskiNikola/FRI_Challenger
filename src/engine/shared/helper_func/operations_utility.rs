// Checks if bitboard is 0 or not.
fn is_empty(bitboard: u64) -> bool {
    return bitboard == 0;
}

/**  ```
|===============|
| a | b | a & b |
| 0 | 0 |   0   |
| 0 | 1 |   0   |
| 1 | 0 |   0   |
| 1 | 1 |   1   |
|===============|
// Intersection is: Idempotent, Commutative, Associative
``` */
fn intersection(bitboard_1: u64, bitboard_2: u64) -> u64 {
    return bitboard_1 & bitboard_2;
}

/**  ```
|===============|
| a | b | a & b |
| 0 | 0 |   0   |
| 0 | 1 |   1   |
| 1 | 0 |   1   |
| 1 | 1 |   1   |
|===============|
// Union is: Idempotent, Commutative, Associative
``` */
fn union(bitboard_1: u64, bitboard_2: u64) -> u64 {
    return bitboard_1 | bitboard_2;
}

/**  ```
|=========|
| a | !a  |
| 0 |  1  |
| 1 |  0  |
|=========|
``` */
fn complement(bitboard: u64) -> u64 {
    return !bitboard;
}

/**  ```
|===============|
| a | b | a & b |
| 0 | 0 |   0   |
| 0 | 1 |   1   |
| 1 | 0 |   0   |
| 1 | 1 |   0   |
|===============|
``` */
fn relative_complement(bitboard_1: u64, bitboard_2: u64) -> u64 {
    return complement(bitboard_1) & bitboard_2;
}

/**  ```
|===============|
| a | b | a & b |
| 0 | 0 |   1   |
| 0 | 1 |   1   |
| 1 | 0 |   0   |
| 1 | 1 |   1   |
|===============|
``` */
fn implication(bitboard_1: u64, bitboard_2: u64) -> u64 {
    return complement(bitboard_1) | bitboard_2;
}

/**  ```
|===============|
| a | b | a & b |
| 0 | 0 |   0   |
| 0 | 1 |   1   |
| 1 | 0 |   1   |
| 1 | 1 |   0   |
|===============|
``` */
fn exclusive_or(a: u64, b: u64) -> u64 {
    return a ^ b;
}

/**  ```
|===============|
| a | b | a & b |
| 0 | 0 |   1   |
| 0 | 1 |   0   |
| 1 | 0 |   0   |
| 1 | 1 |   1   |
|===============|
``` */
fn equivalent(a: u64, b: u64) -> u64 {
    return !exclusive_or(a, b);
}

// //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// //****************************************** SHIFTING****************************************************** */
// //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
fn shift(bitboard: u64, num: i64) -> u64 {
    return if num > 0 { bitboard << num } else { bitboard >> (-num as usize) };
}

fn shift_up_right(bitboard: u64) -> u64 {
    return shift(bitboard, 9);
}

fn shift_up(bitboard: u64) -> u64 {
    return shift(bitboard, 8);
}

fn shift_up_left(bitboard: u64) -> u64 {
    return shift(bitboard, 7);
}

fn shift_left(bitboard: u64) -> u64 {
    return shift(bitboard, -1);
}

fn shift_down_left(bitboard: u64) -> u64 {
    return shift(bitboard, -9);
}

fn shift_down(bitboard: u64) -> u64 {
    return shift(bitboard, -8);
}

fn shift_down_right(bitboard: u64) -> u64 {
    return shift(bitboard, -7);
}

fn shift_right(bitboard: u64) -> u64 {
    return shift(bitboard, 1);
}

fn swap_N_Bits(b: u64, i: usize, j: usize, n: usize) -> u64 {
    let m: u64 = (1 << n) - 1;
    let x: u64 = ((b >> i) ^ (b >> j)) & m;
    return b ^ (x << i) ^ (x << j);
}
