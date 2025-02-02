use std::{array, collections::HashMap};
use num_enum::TryFromPrimitive;
use lazy_static::lazy_static;

// **** START: FEN STRINGS ****
pub const FEN_START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN1: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
pub const FEN2: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
pub const FEN3: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
pub const FEN_MIDDLE_GAME: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
pub const FEN_PAWNS_WHITE: &str =
    "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
pub const FEN_PAWNS_BLACK: &str =
    "rnbqkbnr/p1p1p3/3p3p/1p1p4/2P1Pp2/8/PP1P1PpP/RNBQKB1R b KQkq e3 0 1";
pub const FEN_CASTLE_ONE: &str = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
pub const FEN_CASTLE_TWO: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
pub const FEN_2KING_1WKNIGHT: &str = "5k2/8/8/4N3/8/8/8/5K2 w - - 0 1";
pub const FEN_2KING_2WKNIGHT: &str = "5k2/8/8/4N3/2N5/8/8/5K2 w - - 0 1";
// **** END: FEN STRINGS ****

/* A Chess board has files and ranks */
/* Rank (Row) - horizontal from A to H */
/* Files (Columns) - vertical from 1 to 8*/

pub static FILE_LETTERS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

lazy_static! {
// DEPRECATE: UGLY
    pub static ref RANK_MAP: HashMap<char, usize> = {
        let mut map = HashMap::new();
        map.insert('a', 0);
        map.insert('b', 1);
        map.insert('c', 2);
        map.insert('d', 3);
        map.insert('e', 4);
        map.insert('f', 5);
        map.insert('g', 6);
        map.insert('h', 7);
        return map;
    };
    pub static ref SET_MASK: [u64; 64] = array::from_fn(|idx| 1 << idx);
    pub static ref CLEAR_MASK: [u64; 64] = array::from_fn(|idx| !(1 << idx));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
pub enum Rank {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

pub const FILE_BITBOARD: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

pub const RANK_BITBOARD: [u64; 8] = [
    0x00000000000000FF,
    0x000000000000FF00,
    0x0000000000FF0000,
    0x00000000FF000000,
    0x000000FF00000000,
    0x0000FF0000000000,
    0x00FF000000000000,
    0xFF00000000000000,
];

pub static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23, 3, 12, 16, 59, 41, 19, 24, 54, 4, 64, 13, 10, 17, 62, 60, 28, 42,
    30, 20, 51, 25, 44, 55, 47, 5, 32, 64, 38, 14, 22, 11, 58, 18, 53, 63, 9, 61, 27, 29, 50, 43,
    46, 31, 37, 21, 57, 52, 8, 26, 49, 45, 36, 56, 7, 48, 35, 6, 34, 33,
];

pub const STACK_SIZE: usize = 4 * 1024 * 1024;
