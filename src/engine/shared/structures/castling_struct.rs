use bitflags::bitflags;

use super::color::*;
use super::square::SqPos;
use crate::engine::game::Game;
use crate::engine::move_generation::mv_gen::*;
use crate::engine::shared::structures::square::SqPos::*;

pub const CASTLE_DATA: [(usize, usize, CastlingRights, Color); 4] = [
    (H1 as usize, E1 as usize, CastlingRights::WKINGSIDE, WHITE),
    (A1 as usize, E1 as usize, CastlingRights::WQUEENSIDE, WHITE),
    (H8 as usize, E8 as usize, CastlingRights::BKINGSIDE, BLACK),
    (A8 as usize, E8 as usize, CastlingRights::BQUEENSIDE, BLACK),
];

pub const ROOK_SQ: [[(usize, usize); 2]; 2] = [
    [(SqPos::H1 as usize, SqPos::F1 as usize), (SqPos::A1 as usize, SqPos::D1 as usize)],
    [(SqPos::H8 as usize, SqPos::F8 as usize), (SqPos::A8 as usize, SqPos::D8 as usize)],
];

pub const CASTLE_PAWN_SHIELD: [u64; 4] = [
    0b0000000000000000000000000000000000000000111000001110000000000000,
    0b0000000000000000000000000000000000000000000001110000011100000000,
    0b0000000011100000111000000000000000000000000000000000000000000000,
    0b0000000000000111000001110000000000000000000000000000000000000000,
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
    pub fn sq_att(&self, castle: CastlingRights, game: &Game, _own: u64, _enemy: u64) -> bool {
        let resp = match castle {
            CastlingRights::WKINGSIDE => {
                sq_attack(game, E1.idx(), WHITE)
                    | sq_attack(game, F1.idx(), WHITE)
                    | sq_attack(game, G1.idx(), WHITE)
            }
            CastlingRights::WQUEENSIDE => {
                sq_attack(game, E1.idx(), WHITE)
                    | sq_attack(game, D1.idx(), WHITE)
                    | sq_attack(game, C1.idx(), WHITE)
            }
            CastlingRights::BKINGSIDE => {
                sq_attack(game, E8.idx(), BLACK)
                    | sq_attack(game, F8.idx(), BLACK)
                    | sq_attack(game, G8.idx(), BLACK)
            }
            CastlingRights::BQUEENSIDE => {
                sq_attack(game, E8.idx(), BLACK)
                    | sq_attack(game, D8.idx(), BLACK)
                    | sq_attack(game, C8.idx(), BLACK)
            }
            _ => panic!("Invalid Castling Rights"),
        };

        resp != 0
    }

    #[inline(always)]
    pub fn valid(&self, castle: CastlingRights, game: &Game, own: u64, enemy: u64) -> bool {
        self.is_set(castle)
            && self.sq_empty(castle, own, enemy)
            && !self.sq_att(castle, game, own, enemy)
    }
}
