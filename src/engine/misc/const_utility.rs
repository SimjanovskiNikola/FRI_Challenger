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

pub const FEN_POS_TWO: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
pub const FEN_POS_THREE: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
pub const FEN_POS_FOUR: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
pub const FEN_POS_FIVE: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
pub const FEN_POS_SIX: &str =
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

pub const FEN_BUG_2SQ_PAWN: &str = "8/8/8/K7/5p1k/8/4P3/8 w - - 0 1";

pub const FEN_MATE_IN_3: &str = "2rr3k/pp3pp1/1nnqbN1p/3pN3/2pP4/2P3Q1/PPB4P/R4RK1 w - - 0 1";
pub const FEN_MATE_IN_4: &str = "r1b1Rbk1/1p3p1p/p2q2pQ/3P4/3Nn3/1P1n1PP1/PB4BP/R5K1 w - - 0 1";
pub const FEN_MATE_IN_5: &str = "N1bk3r/P5pp/3b1p2/3B4/R2nP1nq/3P3N/1BP3KP/4Q2R b - - 0 1";
// **** END: FEN STRINGS ****

/* A Chess board has files and ranks */
/* Rank (Row) - horizontal from A to H */
/* Files (Columns) - vertical from 1 to 8*/

pub static FILE_LETTERS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

pub static FILE_BITBOARD: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

pub static RANK_BITBOARD: [u64; 8] = [
    0x00000000000000FF,
    0x000000000000FF00,
    0x0000000000FF0000,
    0x00000000FF000000,
    0x000000FF00000000,
    0x0000FF0000000000,
    0x00FF000000000000,
    0xFF00000000000000,
];
