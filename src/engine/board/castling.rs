use super::color::*;
use super::square::SqPos::*;
use crate::engine::board::board::Board;
use crate::engine::board::piece::PieceTrait;
use crate::engine::move_generator::mv_gen::BoardGenMoveTrait;

pub type Castling = u8;

pub const CASTLING_NONE: Castling = 0;
pub const CASTLING_WKINGSIDE: Castling = 1 << 0;
pub const CASTLING_WQUEENSIDE: Castling = 1 << 1;
pub const CASTLING_BKINGSIDE: Castling = 1 << 2;
pub const CASTLING_BQUEENSIDE: Castling = 1 << 3;
pub const CASTLING_ALL: Castling = 15;

pub const CASTLE_DATA: [(usize, usize, Castling, Color); 4] = [
    (H1 as usize, E1 as usize, CASTLING_WKINGSIDE, WHITE),
    (A1 as usize, E1 as usize, CASTLING_WQUEENSIDE, WHITE),
    (H8 as usize, E8 as usize, CASTLING_BKINGSIDE, BLACK),
    (A8 as usize, E8 as usize, CASTLING_BQUEENSIDE, BLACK),
];

pub const ROOK_SQ: [[(usize, usize); 2]; 2] = [
    [(H1 as usize, F1 as usize), (A1 as usize, D1 as usize)],
    [(H8 as usize, F8 as usize), (A8 as usize, D8 as usize)],
];

pub const CASTLE_PAWN_SHIELD: [u64; 4] = [
    0b0000000000000000000000000000000000000000111000001110000000000000,
    0b0000000000000000000000000000000000000000000001110000011100000000,
    0b0000000011100000111000000000000000000000000000000000000000000000,
    0b0000000000000111000001110000000000000000000000000000000000000000,
];

pub const CLR_LONG_SHORT_CASTLE_MASK: [[Castling; 2]; 2] =
    [[CASTLING_WKINGSIDE, CASTLING_WQUEENSIDE], [CASTLING_BKINGSIDE, CASTLING_BQUEENSIDE]];

pub const CLR_CASTLE_MASK: [Castling; 2] =
    [CASTLING_WQUEENSIDE | CASTLING_WKINGSIDE, CASTLING_BQUEENSIDE | CASTLING_BKINGSIDE];

pub trait CastlingRightsTrait {
    fn val(&self) -> Castling;

    fn add(&mut self, castle: Castling);
    fn clear(&mut self, castle: Castling);

    fn all_set(&self) -> bool;
    fn is_set(&self, castle: Castling) -> bool;

    fn get_mask(&self, clr: Color) -> Castling;
    fn long(&self, clr: Color) -> Castling;
    fn short(&self, clr: Color) -> Castling;

    fn sq_empty(&self, castling: Castling, own: u64, enemy: u64) -> bool;
    fn sq_att(&self, castle: Castling, board: &Board, own: u64, enemy: u64) -> bool;

    fn valid(&self, castle: Castling, board: &Board, own: u64, enemy: u64) -> bool;
}

impl CastlingRightsTrait for Castling {
    #[inline(always)]
    fn val(&self) -> Castling {
        *self
    }

    #[inline(always)]
    fn add(&mut self, castle: Castling) {
        *self |= castle
    }

    #[inline(always)]
    fn clear(&mut self, castle: Castling) {
        *self &= !castle
    }

    #[inline(always)]
    fn all_set(&self) -> bool {
        *self == CASTLING_ALL
    }

    #[inline(always)]
    fn is_set(&self, castle: Castling) -> bool {
        self.val() & castle.val() != 0
    }

    #[inline(always)]
    fn get_mask(&self, clr: Color) -> Castling {
        CLR_CASTLE_MASK[clr.idx()] & self.val()
    }

    #[inline(always)]
    fn long(&self, clr: Color) -> Castling {
        CLR_LONG_SHORT_CASTLE_MASK[clr.idx()][1] & self.val()
    }

    #[inline(always)]
    fn short(&self, clr: Color) -> Castling {
        CLR_LONG_SHORT_CASTLE_MASK[clr.idx()][0] & self.val()
    }

    #[inline(always)]
    fn sq_empty(&self, castling: Castling, own: u64, enemy: u64) -> bool {
        let occ = own | enemy;
        let resp = match castling {
            CASTLING_WKINGSIDE => occ & ((1 << F1.idx()) | (1 << G1.idx())),
            CASTLING_WQUEENSIDE => occ & ((1 << D1.idx()) | (1 << C1.idx()) | (1 << B1.idx())),
            CASTLING_BKINGSIDE => occ & ((1 << F8.idx()) | (1 << G8.idx())),
            CASTLING_BQUEENSIDE => occ & ((1 << D8.idx()) | (1 << C8.idx()) | (1 << B8.idx())),
            _ => panic!("Invalid Castling Rights"),
        };

        resp == 0
    }

    #[inline(always)]
    fn sq_att(&self, castle: Castling, board: &Board, _own: u64, _enemy: u64) -> bool {
        let resp = match castle {
            CASTLING_WKINGSIDE => {
                board.sq_attack(E1.idx(), WHITE)
                    | board.sq_attack(F1.idx(), WHITE)
                    | board.sq_attack(G1.idx(), WHITE)
            }
            CASTLING_WQUEENSIDE => {
                board.sq_attack(E1.idx(), WHITE)
                    | board.sq_attack(D1.idx(), WHITE)
                    | board.sq_attack(C1.idx(), WHITE)
            }
            CASTLING_BKINGSIDE => {
                board.sq_attack(E8.idx(), BLACK)
                    | board.sq_attack(F8.idx(), BLACK)
                    | board.sq_attack(G8.idx(), BLACK)
            }
            CASTLING_BQUEENSIDE => {
                board.sq_attack(E8.idx(), BLACK)
                    | board.sq_attack(D8.idx(), BLACK)
                    | board.sq_attack(C8.idx(), BLACK)
            }
            _ => panic!("Invalid Castling Rights"),
        };

        resp != 0
    }

    // #[inline(always)]
    fn valid(&self, castle: Castling, board: &Board, own: u64, enemy: u64) -> bool {
        self.is_set(castle)
            && self.sq_empty(castle, own, enemy)
            && !self.sq_att(castle, board, own, enemy)
    }
}
