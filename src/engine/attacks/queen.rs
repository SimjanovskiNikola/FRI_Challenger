use super::bishop::get_bishop_mv;
use super::rook::get_rook_mv;
use crate::engine::attacks::bishop::{get_bishop_lookup, get_bishop_mask};
use crate::engine::attacks::rook::{get_rook_lookup, get_rook_mask};
use crate::engine::board::color::Color;

#[inline(always)]
/// Gets Queen moves considering other pieces on the board and excluding own pieces
pub fn get_queen_mv(sq: usize, own: u64, enemy: u64, color: Color) -> u64 {
    get_bishop_mv(sq, own, enemy, color) | get_rook_mv(sq, own, enemy, color)
}

#[inline(always)]
/// Gets Queen moves considering other pieces on the board
pub fn get_queen_mask(sq: usize, own: u64, enemy: u64, color: Color) -> u64 {
    get_bishop_mask(sq, own, enemy, color) | get_rook_mask(sq, own, enemy, color)
}

#[inline(always)]
/// Gets only the mask of possible moves, ignoring other pieces on the board
pub const fn get_queen_lookup(sq: usize, own: u64, enemy: u64, color: Color) -> u64 {
    get_bishop_lookup(sq, own, enemy, color) | get_rook_lookup(sq, own, enemy, color)
}
