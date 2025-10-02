use super::piece::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flag {
    Quiet,
    KingCastle,
    QueenCastle,
    Capture(Piece),
    EP,
    Promotion(Piece, Option<Piece>),
    NullMove,
}

impl Flag {
    #[inline(always)]
    pub const fn is_capture(&self) -> bool {
        match *self {
            Flag::Capture(_) | Flag::EP | Flag::Promotion(_, Some(_)) => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub const fn is_promo(&self) -> bool {
        match *self {
            Flag::Promotion(_, _) => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub const fn get_promo_piece(&self) -> Option<Piece> {
        match *self {
            Flag::Promotion(piece, _) => Some(piece),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub piece: Piece,
    pub flag: Flag,
}

impl Move {
    pub const fn init(from: u8, to: u8, piece: Piece, flag: Flag) -> Self {
        Self { from, to, piece, flag }
    }

    pub const fn null_move() -> Self {
        Self { from: 0, to: 0, piece: 0, flag: Flag::NullMove }
    }
}

pub struct ExtendedMove {
    pub mv: Move,
    pub key: u64,
}
