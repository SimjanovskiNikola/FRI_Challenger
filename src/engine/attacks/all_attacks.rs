use super::{
    knight_attacks::KnightAttacks,
    pawn_attacks::PawnAttacks,
    ray_attacks::Rays,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attacks {
    pub knight_attacks: KnightAttacks,
    pub pawn_attacks: PawnAttacks,
    pub ray_attacks: Rays,
}

impl Attacks {
    pub fn initialize() -> Self {
        return Self {
            knight_attacks: KnightAttacks::initialize(),
            pawn_attacks: PawnAttacks::initialize(),
            ray_attacks: Rays::initialize(),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    #[test]
    fn test_pawn_attacks_initialize() {}
}
