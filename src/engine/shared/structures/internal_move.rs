use crate::engine::game::Game;

use super::castling_struct::*;
use super::color::*;
use super::piece::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flag {
    Quiet,
    KingCastle,
    QueenCastle,
    Capture(Piece),
    EP,
    Promotion(Piece, Option<Piece>),
}

impl Flag {
    pub fn is_capture(&self) -> bool {
        match *self {
            Flag::Capture(_) | Flag::EP | Flag::Promotion(_, Some(_)) => true,
            _ => false,
        }
    }

    pub fn get_promo_piece(&self) -> Option<Piece> {
        match *self {
            Flag::Promotion(piece, _) => Some(piece),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PositionRev {
    pub from: u8,
    pub to: u8,
    pub piece: Piece,
    pub flag: Flag,
}

impl PositionRev {
    pub fn init(from: u8, to: u8, piece: Piece, flag: Flag) -> Self {
        Self { from, to, piece, flag }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PositionIrr {
    pub key: u64,
    pub color: Color,
    pub ep: Option<u8>,
    pub castle: CastlingRights,
    pub half_move: u8,
    pub full_move: u16,
    pub score: isize,
}

impl PositionIrr {
    pub fn init(
        key: u64,
        color: Color,
        ep: Option<u8>,
        castle: CastlingRights,
        half_move: u8,
        full_move: u16,
        score: isize,
    ) -> Self {
        Self { key, color, ep, castle, half_move, full_move, score }
    }

    pub fn init_with_game(game: &Game) -> Self {
        Self {
            key: game.key,
            color: game.color,
            ep: game.ep,
            castle: game.castling,
            half_move: game.half_move,
            full_move: game.full_move,
            score: 0,
        }
    }
}
