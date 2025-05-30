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
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub piece: Piece,
    pub flag: Flag,
}

impl Move {
    pub fn init(from: u8, to: u8, piece: Piece, flag: Flag) -> Self {
        Self { from, to, piece, flag }
    }
}
