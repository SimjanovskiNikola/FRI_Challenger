use bitflags::bitflags;

use super::color::*;
use super::square::SqPos::*;
use crate::engine::board::board::Board;
use crate::engine::board::piece::PieceTrait;
use crate::engine::move_generator::mv_gen::BoardGenMoveTrait;

pub const CASTLE_DATA: [(usize, usize, CastlingRights, Color); 4] = [
    (H1 as usize, E1 as usize, CastlingRights::WKINGSIDE, WHITE),
    (A1 as usize, E1 as usize, CastlingRights::WQUEENSIDE, WHITE),
    (H8 as usize, E8 as usize, CastlingRights::BKINGSIDE, BLACK),
    (A8 as usize, E8 as usize, CastlingRights::BQUEENSIDE, BLACK),
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

pub const CLR_LONG_SHORT_CASTLE_MASK: [[u8; 2]; 2] = [
    [CastlingRights::WKINGSIDE.bits(), CastlingRights::WQUEENSIDE.bits()],
    [CastlingRights::BKINGSIDE.bits(), CastlingRights::BQUEENSIDE.bits()],
];

pub const CLR_CASTLE_MASK: [u8; 2] = [
    CastlingRights::WQUEENSIDE.bits() | CastlingRights::WKINGSIDE.bits(),
    CastlingRights::BQUEENSIDE.bits() | CastlingRights::BKINGSIDE.bits(),
];

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct CastlingRights: u8 {
        const NONE = 0;
        const WKINGSIDE = 1 << 0;
        const WQUEENSIDE = 1 << 1;
        const BKINGSIDE = 1 << 2;
        const BQUEENSIDE = 1 << 3;
        const ALL = 15;
    }
}

impl CastlingRights {
    #[inline(always)]
    pub fn val(&self) -> u8 {
        self.bits()
    }

    #[inline(always)]
    pub fn idx(&self) -> usize {
        self.bits() as usize
    }

    #[inline(always)]
    pub fn add(&mut self, castle: CastlingRights) {
        *self |= castle
    }

    #[inline(always)]
    pub fn clear(&mut self, castle: CastlingRights) {
        *self &= !castle
    }

    #[inline(always)]
    pub fn all_set(&self) -> bool {
        self.idx() == 15
    }

    #[inline(always)]
    pub fn is_set(&self, castle: CastlingRights) -> bool {
        self.val() & castle.val() != 0
    }

    fn get_mask(&self, clr: Color) -> u8 {
        CLR_CASTLE_MASK[clr.idx()] & self.val()
    }

    #[inline(always)]
    pub fn long(&self, clr: Color) -> u8 {
        CLR_LONG_SHORT_CASTLE_MASK[clr.idx()][1] & self.val()
    }

    #[inline(always)]
    pub fn short(&self, clr: Color) -> u8 {
        CLR_LONG_SHORT_CASTLE_MASK[clr.idx()][0] & self.val()
    }

    #[inline(always)]
    pub fn sq_empty(&self, castling: CastlingRights, own: u64, enemy: u64) -> bool {
        let occ = own | enemy;
        let resp = match castling {
            CastlingRights::WKINGSIDE => occ & ((1 << F1.idx()) | (1 << G1.idx())),
            CastlingRights::WQUEENSIDE => {
                occ & ((1 << D1.idx()) | (1 << C1.idx()) | (1 << B1.idx()))
            }
            CastlingRights::BKINGSIDE => occ & ((1 << F8.idx()) | (1 << G8.idx())),
            CastlingRights::BQUEENSIDE => {
                occ & ((1 << D8.idx()) | (1 << C8.idx()) | (1 << B8.idx()))
            }
            _ => panic!("Invalid Castling Rights"),
        };

        resp == 0
    }

    #[inline(always)]
    pub fn sq_att(&self, castle: CastlingRights, board: &Board, _own: u64, _enemy: u64) -> bool {
        let resp = match castle {
            CastlingRights::WKINGSIDE => {
                board.sq_attack(E1.idx(), WHITE)
                    | board.sq_attack(F1.idx(), WHITE)
                    | board.sq_attack(G1.idx(), WHITE)
            }
            CastlingRights::WQUEENSIDE => {
                board.sq_attack(E1.idx(), WHITE)
                    | board.sq_attack(D1.idx(), WHITE)
                    | board.sq_attack(C1.idx(), WHITE)
            }
            CastlingRights::BKINGSIDE => {
                board.sq_attack(E8.idx(), BLACK)
                    | board.sq_attack(F8.idx(), BLACK)
                    | board.sq_attack(G8.idx(), BLACK)
            }
            CastlingRights::BQUEENSIDE => {
                board.sq_attack(E8.idx(), BLACK)
                    | board.sq_attack(D8.idx(), BLACK)
                    | board.sq_attack(C8.idx(), BLACK)
            }
            _ => panic!("Invalid Castling Rights"),
        };

        resp != 0
    }

    #[inline(always)]
    pub fn valid(&self, castle: CastlingRights, board: &Board, own: u64, enemy: u64) -> bool {
        self.is_set(castle)
            && self.sq_empty(castle, own, enemy)
            && !self.sq_att(castle, board, own, enemy)
    }
}
