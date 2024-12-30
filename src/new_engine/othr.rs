// FRI - Chess Engine

use lazy_static::lazy_static;
use rand::Rng;

pub type Bitboard = u64;

//DEPRECATE:
pub const BOARD_SQ_NUM: usize = 120;

//DEPRECATE:
pub const MAX_GAME_MOVES: usize = 2048;

//DEPRECATE:
#[rustfmt::skip]
pub static Sq120TOSq64: [isize; BOARD_SQ_NUM] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 0,  1,  2,  3,  4,  5,  6,  7,  -1,
    -1, 8,  9,  10, 11, 12, 13, 14, 15, -1,
    -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
    -1, 24, 25, 26, 27, 28, 29, 30, 31, -1,
    -1, 32, 33, 34, 35, 36, 37, 38, 39, -1,
    -1, 40, 41, 42, 43, 44, 45, 46, 47, -1,
    -1, 48, 49, 50, 51, 52, 53, 54, 55, -1,
    -1, 56, 57, 58, 59, 60, 61, 62, 63, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

//DEPRECATE:
#[rustfmt::skip]
pub static Sq64TOSq120: [usize; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98,
];

lazy_static! {
    pub static ref PieceKeys: [[u64; 13]; 120] = {
        let mut arr = [[0 as u64; 13]; 120];
        for i in 0..13 {
            for j in 0..120 {
                arr[i][j] = rand::thread_rng().gen();
            }
        }
        return arr;
    };
    pub static ref SideKey: u64 = rand::thread_rng().gen();
    pub static ref CastleKeys: [u64; 16] = {
        let mut arr = [0 as u64; 16];
        for idx in 0..16 {
            arr[idx] = rand::thread_rng().gen();
        }
        return arr;
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    WHITE = 0,
    BLACK = 1,
    BOTH = 2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    NONE,
    WPawn,
    WRook,
    WKnight,
    WBishop,
    WQueen,
    WKing,
    BPawn,
    BRook,
    BKnight,
    BBishop,
    BQueen,
    BKing,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum File {
    NONE = -1,
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rank {
    NONE = -1,
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

pub enum CastlingRights {
    WKingSide = 1,
    WQueenSide = 2,
    BKingSide = 4,
    BQueenSide = 8,
}

#[rustfmt::skip]
pub enum SqPos {
    A1 = 21, B1 = 22, C1 = 23, D1 = 24, E1 = 25, F1 = 26, G1 = 27, H1 = 28,
    A2 = 31, B2 = 32, C2 = 33, D2 = 34, E2 = 35, F2 = 36, G2 = 37, H2 = 38,
    A3 = 41, B3 = 42, C3 = 43, D3 = 44, E3 = 45, F3 = 46, G3 = 47, H3 = 48,
    A4 = 51, B4 = 52, C4 = 53, D4 = 54, E4 = 55, F4 = 56, G4 = 57, H4 = 58,
    A5 = 61, B5 = 62, C5 = 63, D5 = 64, E5 = 65, F5 = 66, G5 = 67, H5 = 68,
    A6 = 71, B6 = 72, C6 = 73, D6 = 74, E6 = 75, F6 = 76, G6 = 77, H6 = 78,
    A7 = 81, B7 = 82, C7 = 83, D7 = 84, E7 = 85, F7 = 86, G7 = 87, H7 = 88,
    A8 = 91, B8 = 92, C8 = 93, D8 = 94, E8 = 95, F8 = 96, G8 = 97, H8 = 98,
    NoSq = -1
}

pub struct Board {
    pieces: [u64; BOARD_SQ_NUM],
    pawns: [u64; 3],
    kings: [u64; 2],

    side: Color,
    en_passant: Option<u64>,
    fifty_move: u64,
    ply: u64,
    history_ply: u64,
    castle_permision: u8,
    positionKey: u64,

    piece_num: [u64; 13],
    big_pieces: [u64; 3],
    major_pieces: [u64; 3],
    minor_pieces: [u64; 3],

    history: [InternalMove; MAX_GAME_MOVES],

    piece_list: [[u64; 13]; 10],
}

pub struct InternalMove {
    move_pos: u64,
    castle_permision: u8,
    en_passant: Option<u64>,
    fifty_move: u64,
    positionKey: u64,
}

fn pos_to_sq(file: usize, rank: usize) -> usize {
    return (rank * 10) + (file + 21);
}

fn generate_pos_key(board: &Board) -> u64 {
    let mut final_key: u64 = 0;

    for idx in 0..BOARD_SQ_NUM {
        let piece = board.pieces[idx];
        if piece != 0 {
            final_key ^= PieceKeys[piece as usize][idx];
        }
    }

    if board.side == Color::WHITE {
        final_key ^= *SideKey;
    }

    match board.en_passant {
        Some(idx) => final_key ^= PieceKeys[0][idx as usize],
        None => (),
    }

    final_key ^= CastleKeys[board.castle_permision as usize];
    return final_key;
}
