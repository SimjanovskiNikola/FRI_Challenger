use crate::engine::board::color::Color;

use super::bishop::get_bishop_mask;
use super::bishop::get_bishop_mv;
use super::rook::get_rook_mask;
use super::rook::get_rook_mv;

#[inline(always)]
pub fn get_queen_mv(sq: usize, own: u64, enemy: u64, color: Color) -> u64 {
    get_bishop_mv(sq, own, enemy, color) | get_rook_mv(sq, own, enemy, color)
}

#[inline(always)]
pub fn get_queen_mask(sq: usize, own: u64, enemy: u64, color: Color) -> u64 {
    get_bishop_mask(sq, own, enemy, color) | get_rook_mask(sq, own, enemy, color)
}
