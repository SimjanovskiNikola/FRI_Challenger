use crate::engine::shared::{helper_func::bit_pos_utility::*, structures::directions::*};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rays {
    pub rays: [[u64; 64]; 8],
}

impl Rays {}

#[cfg(test)]
mod tests {
    use crate::engine::{
        attacks::all_attacks::Attacks,
        shared::helper_func::{bitboard::BitboardTrait, print_utility::print_bitboard},
    };

    use super::*;
}
