use std::ops::Add;

use super::castling_struct::*;
use super::piece;
use super::piece::*;
use super::color::*;

// Check about BigPawn Flag and what it does
// DEPRECATE:
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flag {
    Normal = 1,
    Capture = 2,
    EP = 4,
    Promotion = 8,
    KingSideCastle = 16,
    QueenSideCastle = 32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InternalMove {
    pub position_key: u64,
    pub active_color: Color,
    pub from: usize,
    pub to: usize,
    pub piece: Piece,
    pub captured: Option<Piece>,
    pub promotion: Option<Piece>,
    pub ep: Option<u64>,
    pub castle: CastlingRights,
    pub flag: Flag,
    pub half_move: usize,
    //TODO: Add Score
}

impl InternalMove {
    pub fn init() -> Self {
        return Self {
            position_key: 0u64,
            active_color: WHITE,
            from: 0,
            to: 0,
            piece: 0,
            captured: None,
            promotion: None,
            ep: None,
            castle: CastlingRights::NONE,
            flag: Flag::Normal,
            half_move: 0,
        };
    }
}

// pub type Position = u128;

// trait PositionGetTrait {
//     fn parse(&self, shift: usize, bits: usize) -> u64;
//     fn get_from(&self) -> u8;
//     fn get_to(&self) -> u8;
//     fn get_piece(&self) -> u8;
//     fn get_color(&self) -> u8;
//     fn get_captured(&self) -> Option<Piece>;
//     fn get_promotion(&self) -> Option<Piece>;
//     fn get_ep(&self) -> Option<usize>;
//     fn get_castle(&self) -> CastlingRights;
//     fn get_flag(&self) -> Flag;
//     fn get_half_move(&self) -> usize;
//     fn get_key(&self) -> u64;
//     #[rustfmt::skip]
//     fn get_all(&self) -> (u8, u8, u8, u8, Option<Piece>, Option<Piece>, Option<usize>, CastlingRights, Flag, usize, u64);
// }

// impl PositionGetTrait for Position {
//     fn parse(&self, shift: usize, bits: usize) -> u64 {
//         return ((self >> shift) & (((1 << bits) as u128) - 1)) as u64;
//     }

//     fn get_from(&self) -> u8 {
//         return self.parse(0, 7) as u8;
//     }

//     fn get_to(&self) -> u8 {
//         return self.parse(7, 7) as u8;
//     }

//     fn get_piece(&self) -> u8 {
//         return self.parse(14, 7) as u8;
//     }

//     fn get_color(&self) -> u8 {
//         return self.get_piece().color();
//     }

//     fn get_captured(&self) -> Option<Piece> {
//         match self.parse(21, 7) as u8 {
//             0 => return None,
//             piece => return Some(piece),
//         }
//     }

//     fn get_promotion(&self) -> Option<Piece> {
//         match self.parse(28, 7) as u8 {
//             0 => return None,
//             piece => return Some(piece),
//         }
//     }

//     fn get_ep(&self) -> Option<usize> {
//         match self.parse(35, 7) as u8 {
//             0 => return None,
//             sq => return Some(sq as usize),
//         }
//     }

//     fn get_castle(&self) -> CastlingRights {
//         let castle = self.parse(42, 4) as u8;
//         let mut rez: CastlingRights = CastlingRights::NONE;
//         if castle & 1 != 0 {
//             rez.add(CastlingRights::WKINGSIDE);
//         }
//         if castle & 2 != 0 {
//             rez.add(CastlingRights::WQUEENSIDE);
//         }
//         if castle & 4 != 0 {
//             rez.add(CastlingRights::BKINGSIDE);
//         }
//         if castle & 8 != 0 {
//             rez.add(CastlingRights::BQUEENSIDE);
//         }
//         return rez;
//     }

//     fn get_flag(&self) -> Flag {
//         match self.parse(46, 6) as u8 {
//             1 => return Flag::Normal,
//             2 => return Flag::Capture,
//             4 => return Flag::EP,
//             8 => return Flag::Promotion,
//             16 => return Flag::KingSideCastle,
//             32 => return Flag::QueenSideCastle,
//             _ => panic!(""),
//         }
//     }

//     fn get_half_move(&self) -> usize {
//         return self.parse(52, 6) as usize;
//     }

//     fn get_key(&self) -> u64 {
//         return self.parse(58, 64);
//     }

//     fn get_all(
//         &self,
//     ) -> (
//         u8,
//         u8,
//         u8,
//         u8,
//         Option<Piece>,
//         Option<Piece>,
//         Option<usize>,
//         CastlingRights,
//         Flag,
//         usize,
//         u64,
//     ) {
//         return (
//             self.get_from(),
//             self.get_to(),
//             self.get_piece(),
//             self.get_color(),
//             self.get_captured(),
//             self.get_promotion(),
//             self.get_ep(),
//             self.get_castle(),
//             self.get_flag(),
//             self.get_half_move(),
//             self.get_key(),
//         );
//     }
// }

// trait PositionSetTrait {
//     fn set(&mut self, shift: usize, bits: u128);
//     fn set_from(&mut self, from: u8);
//     fn set_to(&mut self, to: u8);
//     fn set_piece(&mut self, piece: u8);
//     fn set_color(&mut self, color: u8);
//     fn set_captured(&mut self, captured: Option<Piece>);
//     fn set_promotion(&mut self, promotion: Option<Piece>);
//     fn set_ep(&mut self, ep: Option<usize>);
//     fn set_castle(&mut self, castle: CastlingRights);
//     fn set_flag(&mut self, flag: Flag);
//     fn set_half_move(&mut self, half_move: usize);
//     fn set_key(&mut self, key: u64);
// }

// impl PositionSetTrait for Position {
//     fn set(&mut self, shift: usize, bits: u128) {
//         *self |= bits << shift;
//     }

//     fn set_from(&mut self, from: u8) {
//         self.set(0, from as u128);
//     }

//     fn set_to(&mut self, to: u8) {
//         self.set(7, to as u128);
//     }

//     fn set_piece(&mut self, piece: u8) {
//         self.set(14, piece as u128);
//     }

//     fn set_color(&mut self, color: u8) {
//         self.set(14, color as u128);
//     }

//     fn set_captured(&mut self, captured: Option<Piece>) {
//         match captured {
//             Some(piece) => self.set(21, piece as u128),
//             None => (),
//         }
//     }

//     fn set_promotion(&mut self, promotion: Option<Piece>) {
//         match promotion {
//             Some(piece) => self.set(28, piece as u128),
//             None => (),
//         }
//     }

//     fn set_ep(&mut self, ep: Option<usize>) {
//         match ep {
//             Some(piece) => self.set(35, piece as u128),
//             None => (),
//         }
//     }

//     fn set_castle(&mut self, castle: CastlingRights) {
//         if castle.is_set(CastlingRights::WKINGSIDE) {
//             self.set(14, 1 as u128)
//         }
//         if castle.is_set(CastlingRights::WQUEENSIDE) {
//             self.set(14, 1 as u128)
//         }
//         if castle.is_set(CastlingRights::BKINGSIDE) {
//             self.set(14, 1 as u128)
//         }
//         if castle.is_set(CastlingRights::BQUEENSIDE) {
//             self.set(14, 1 as u128)
//         }
//     }

//     fn set_flag(&mut self, flag: Flag) {
//         match flag {
//             Flag::Normal => self.set(14, 1 as u128),
//             Flag::Capture => self.set(14, 1 as u128),
//             Flag::EP => self.set(14, 1 as u128),
//             Flag::Promotion => self.set(14, 1 as u128),
//             Flag::KingSideCastle => self.set(14, 1 as u128),
//             Flag::QueenSideCastle => self.set(14, 1 as u128),
//         }
//     }

//     fn set_half_move(&mut self, half_move: usize) {
//         self.set(14, 1 as u128)
//     }

//     fn set_key(&mut self, key: u64) {
//         self.set(14, key as u128)
//     }
// }

// NOTE: If Performance is better without enum and struct
// Change InternalMove from struct to u128 where you will store all relevant information as one integer
// 64 bits -> position key (IF NECESSARY)
// 8 bits -> color and piece
// 7 bits -> from position
// 7 bits -> to position
// 8 bits -> color and piece captured
// 4 bits -> promotion piece (knight, bishop, rook, queen)
// 4 bits -> Castling Rights (wKingSIde, wQueenSide, bKingSIde, bQueenSide)
// 7 bits -> ep position
// other bits -> maybe for score

// TODO: Generate position key every time a move is created

// 0 from (7 bits) ->
// 7 to (7 bits) ->
// 14 piece | color (7 bits) ->
// 21 captured (7 bits) ->
// 28 promotion (7 bits) ->
// 35 ep( 7 bits) ->
// 42 castle (4 bits) ->
// 46 flag (6 bits) ->
// 52 half_move (7 bits) ->
// 59 key (64 bits) ->
// 123 final
