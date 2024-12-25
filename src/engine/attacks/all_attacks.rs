use super::{
    king_attacks::KingAttacks, knight_attacks::KnightAttacks, pawn_attacks::PawnAttacks,
    ray_attacks::Rays,
};
use lazy_static::lazy_static;

#[macro_export]
macro_rules! make_rays {
    ($ray_fn:ident) => {{
        let mut rays = vec![];

        for row in 0..8 {
            for col in 0..8 {
                rays.push($ray_fn(row, col));
            }
        }

        rays
    }};
}

lazy_static! {
    pub static ref ATTACKS: Attacks = Attacks::initialize();
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attacks {
    pub king_attacks: KingAttacks,
    pub knight_attacks: KnightAttacks,
    pub pawn_attacks: PawnAttacks,
    pub ray_attacks: Rays,
}

impl Attacks {
    pub fn initialize() -> Self {
        return Self {
            king_attacks: KingAttacks::init(),
            knight_attacks: KnightAttacks::init(),
            pawn_attacks: PawnAttacks::init(),
            ray_attacks: Rays::init(),
        };
    }
}
