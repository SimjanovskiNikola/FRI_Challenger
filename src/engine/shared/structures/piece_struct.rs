use num_enum::{IntoPrimitive, TryFromPrimitive};

/**
fn main() {
    let num: u8 = 1;
    match MyEnum::try_from(num) {
        Ok(my_enum) => println!("{:?}", my_enum),
        Err(e) => println!("Error: {:?}", e),
    }
*/

/** Determines the color of the peace. */
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(usize)]
pub enum Color {
    White = 0,
    Black = 1,
}

/** Determines the Type of the peace. */
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(usize)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub pos: u64,
    pub p_color: Color,
    pub p_type: PieceType,
}

impl Piece {
    pub fn init(p_color: Color, p_type: PieceType, pos: Option<u64>) -> Piece {
        return Piece {
            p_color,
            p_type,
            pos: match pos {
                Some(position) => position,
                None => 0,
            },
        };
    }

    pub fn to_string(&self) -> String {
        let result = match self.p_type {
            PieceType::Pawn => "p ",
            PieceType::Rook => "r ",
            PieceType::Knight => "n ",
            PieceType::Bishop => "b ",
            PieceType::Queen => "q ",
            PieceType::King => "k ",
        };

        if self.p_color == Color::White {
            return result.to_ascii_uppercase();
        } else {
            return result.to_string();
        }
    }

    pub fn chess_figure(&self) -> &str {
        return match self.p_color {
            Color::White => match self.p_type {
                PieceType::Pawn => "♙",
                PieceType::Knight => "♘",
                PieceType::Bishop => "♗",
                PieceType::Rook => "♖",
                PieceType::Queen => "♕",
                PieceType::King => "♔",
            },

            Color::Black => match self.p_type {
                PieceType::Pawn => "♟",
                PieceType::Knight => "♞",
                PieceType::Bishop => "♝",
                PieceType::Rook => "♜",
                PieceType::Queen => "♛",
                PieceType::King => "♚",
            },
        };
    }
}
