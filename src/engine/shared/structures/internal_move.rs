use super::castling_struct::*;
use super::color::*;
use super::piece::*;

// Check about BigPawn Flag and what it does
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flag {
    Quiet,
    Capture(Piece),
    EP(usize, Piece),
    Promotion(Piece, Option<Piece>),
    KingSideCastle(usize, usize),
    QueenSideCastle(usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InternalMove {
    pub position_key: u64,
    pub active_color: Color,
    pub from: usize,
    pub to: usize,
    pub piece: Piece,
    pub ep: Option<usize>,
    pub castle: CastlingRights,
    pub flag: Flag,
    pub half_move: usize,
    //TODO: Add Score
}

impl InternalMove {
    pub fn init() -> Self {
        Self {
            position_key: 0u64,
            active_color: WHITE,
            from: 0,
            to: 0,
            piece: 0,
            ep: None,
            castle: CastlingRights::NONE,
            flag: Flag::Quiet,
            half_move: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PositionIrr {
    pub key: u64,
    pub side: Color,
    pub ep: Option<u8>,
    pub castle: CastlingRights,
    pub half_move: u8,
    pub full_move: u16,
    pub score: isize,
}

impl PositionIrr {
    pub fn init(
        key: u64,
        side: Color,
        ep: Option<u8>,
        castle: CastlingRights,
        half_move: u8,
        full_move: u16,
        score: isize,
    ) -> Self {
        Self { key, side, ep, castle, half_move, full_move, score }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PosFlag {
    Quiet = 0,
    DoublePushPawn = 1,
    KingCastle = 2,
    QueenCastle = 3,
    Capture = 4,
    EP = 5,
    KnightPromo = 8,
    BishopPromo = 9,
    RookPromo = 10,
    QueenPromo = 11,
    KnightPromoCapture = 12,
    BishopPromoCapture = 13,
    RookPromoCapture = 14,
    QueenPromoCapture = 15,
}

pub const PROMO: u8 = 0b1000;
pub const CAPTURE: u8 = 0b0100;

impl PosFlag {
    pub fn get_promo_piece(&self) -> Piece {
        match self {
            PosFlag::KnightPromo | PosFlag::KnightPromoCapture => KNIGHT,
            PosFlag::BishopPromo | PosFlag::BishopPromoCapture => KNIGHT,
            PosFlag::RookPromo | PosFlag::RookPromoCapture => ROOK,
            PosFlag::QueenPromo | PosFlag::QueenPromoCapture => QUEEN,
            _ => panic!("Getting a promo piece when there is no promo flag"),
        }
    }

    pub fn is_promo(&self) -> bool {
        (*self as u8) & PROMO != 0
    }

    pub fn is_capture(&self) -> bool {
        (*self as u8) & CAPTURE != 0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PositionRev {
    pub from: u8,
    pub to: u8,
    pub piece: Piece,
    pub capture: Piece,
    pub flag: PosFlag,
}

impl PositionRev {
    pub fn init(from: u8, to: u8, piece: Piece, capture: Piece, flag: PosFlag) -> Self {
        Self { from, to, piece, capture, flag }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position(PositionRev, PositionIrr);

impl Position {
    pub fn get_rev(&self) -> PositionRev {
        self.0
    }

    pub fn get_irr(&self) -> PositionIrr {
        self.1
    }
}
