/* Enums for the peaces */
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}
#[derive(Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Piece {
    pub position: u64,
    pub piece_color: PieceColor,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn to_string(&self) -> String {
        let result = match self.piece_type {
            PieceType::Pawn => "p ",
            PieceType::Rook => "r ",
            PieceType::Knight => "n ",
            PieceType::Bishop => "b ",
            PieceType::Queen => "q ",
            PieceType::King => "k ",
        };

        if self.piece_color == PieceColor::White {
            return result.to_ascii_uppercase();
        } else {
            return result.to_string();
        }
    }
}
