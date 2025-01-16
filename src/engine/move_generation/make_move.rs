use std::vec;

use crate::engine::{
    game::Game,
    shared::{
        helper_func::{
            bit_pos_utility::{bit_scan_lsb, is_bit_set},
            bitboard::{Bitboard, BitboardTrait},
            const_utility::SqPos::*,
            print_utility::{print_bitboard, print_chess},
        },
        structures::{
            castling_struct::CastlingRights,
            internal_move::{Flag, InternalMove},
            piece_struct::{Color, Piece, PieceType},
            square_struct::Square,
        },
    },
};

// const TERMINATION_MARKERS = ['1-0', '0-1', '1/2-1/2', '*'] NOTE: Maybe i will need this

use lazy_static::lazy_static;
use rand::Rng;

use super::{fen::FenTrait, move_generation::gen_attacks};

lazy_static! {
    pub static ref PieceKeys: [[[u64; 6]; 2]; 64] = [[[rand::thread_rng().gen(); 6]; 2]; 64];
    pub static ref EpKeys: [u64; 64] = [rand::thread_rng().gen(); 64];
    pub static ref CastleKeys: [u64; 16] = [rand::thread_rng().gen(); 16];
    pub static ref SideKey: u64 = rand::thread_rng().gen();
}

pub trait GameMoveTrait {
    fn make_move(&mut self, mv: InternalMove) -> bool;
    fn undo_move(&mut self);
    fn generate_pos_key(&self) -> u64;
    fn add_piece(&mut self, sq: usize, piece: Piece);
    fn clear_piece(&mut self, sq: usize);
    fn replace_piece(&mut self, from_sq: usize, to_sq: usize);
}

impl GameMoveTrait for Game {
    fn make_move(&mut self, mv: InternalMove) -> bool {
        // if mv.from == 51 && mv.to == 35 && mv.piece.p_type == PieceType::Pawn {
        //     println!("{:?}", "MAKE MOVE START");
        //     print_chess(self);
        // }

        match mv.flag {
            Flag::Normal => match self.squares[mv.from] {
                Square::Empty => {
                    print_chess(self);
                    panic!("Panic at Normal Flag {:#?}", mv);
                }
                Square::Occupied(_) => self.replace_piece(mv.from, mv.to),
            },
            Flag::Capture => self.replace_piece(mv.from, mv.to),
            Flag::EP => {
                self.replace_piece(mv.from, mv.to);
                match mv.active_color {
                    Color::White => self.clear_piece(mv.to - 8),
                    Color::Black => self.clear_piece(mv.to + 8),
                }
            }
            Flag::Promotion => {
                if let Some(piece) = mv.promotion {
                    self.clear_piece(mv.from);
                    self.add_piece(mv.to, piece);
                } else {
                    panic!("Something is invalid with promotion peace")
                }
            }
            Flag::KingSideCastle => {
                self.replace_piece(mv.from, mv.to);
                match mv.active_color {
                    Color::White => self.replace_piece(H1 as usize, F1 as usize),
                    Color::Black => self.replace_piece(H8 as usize, F8 as usize),
                }
            }
            Flag::QueenSideCastle => {
                // println!("{:?}", mv.active_color);
                // print_chess(self);

                self.replace_piece(mv.from, mv.to);
                match mv.active_color {
                    Color::White => self.replace_piece(A1 as usize, D1 as usize),
                    Color::Black => self.replace_piece(A8 as usize, D8 as usize),
                }

                // print_chess(self);
            }
        }

        self.set_occupancy(Color::White);
        self.set_occupancy(Color::Black);

        self.change_active_color();

        //If the castleRight is set, and if the king is on place and rook is on place than retain otherwise clear
        let castle_tuple: [(usize, usize, CastlingRights, Color); 4] = [
            (H1 as usize, E1 as usize, CastlingRights::WKINGSIDE, Color::White),
            (A1 as usize, E1 as usize, CastlingRights::WQUEENSIDE, Color::White),
            (H8 as usize, E8 as usize, CastlingRights::BKINGSIDE, Color::Black),
            (A8 as usize, E8 as usize, CastlingRights::BQUEENSIDE, Color::Black),
        ];
        for c in castle_tuple {
            let mut clear_castle = false;
            if mv.castle.bits() & c.2.bits() == 0 {
                clear_castle = true;
            }
            match self.squares[c.1] {
                Square::Empty => clear_castle = true,
                Square::Occupied(piece) => {
                    if piece.p_type != PieceType::King {
                        clear_castle = true
                    }
                }
            };
            match self.squares[c.0] {
                Square::Empty => clear_castle = true,
                Square::Occupied(piece) => {
                    if piece.p_type != PieceType::Rook {
                        clear_castle = true
                    }
                }
            };

            if clear_castle {
                self.castling_rights.clear(c.2);
            }
        }

        if mv.piece.p_type == PieceType::Pawn && mv.from.abs_diff(mv.to) == 16 {
            self.en_passant = match mv.active_color {
                Color::White => Some(Bitboard::init(mv.to - 8)),
                Color::Black => Some(Bitboard::init(mv.to + 8)),
            }
        } else {
            self.en_passant = None
        }

        if mv.piece.p_type == PieceType::Pawn || mv.captured != None {
            self.halfmove_clock = 0
        } else {
            self.halfmove_clock = mv.half_move + 1;
        }

        if self.moves.len() % 2 == 0 {
            self.fullmove_number += 1;
        }

        self.moves.push(InternalMove { position_key: self.generate_pos_key(), ..mv });

        let king_sq =
            self.piece_bitboard[mv.active_color as usize][PieceType::King as usize].get_lsb();

        // if mv.from == 51 && mv.to == 35 && mv.piece.p_type == PieceType::Pawn {
        //     print_chess(self);
        //     println!("{:?}", "MAKE MOVE END");
        //     println!(
        //         "King is attacked: {:?}",
        //         gen_attacks(self, self.active_color).is_set(king_sq)
        //     );
        // }

        if gen_attacks(self, self.active_color).is_set(king_sq) {
            self.undo_move();
            return false;
        }

        return true;
    }

    fn undo_move(&mut self) {
        // println!("Before Taking move: {:#?}", self.moves);
        // print_chess(self);

        let mv = match self.moves.pop() {
            Some(mv) => mv,
            None => return,
        };

        // println!("After Taking move: {:#?}", self.moves);
        // print_chess(self);

        self.fullmove_number -= if self.moves.len() % 2 == 0 { 1 } else { 0 };
        self.halfmove_clock = mv.half_move;
        self.en_passant = mv.ep;
        self.castling_rights = mv.castle;
        self.change_active_color();

        match mv.flag {
            // DEPRECATE: Here should be one liner -> Only self.replace
            Flag::Normal => match self.squares[mv.to] {
                Square::Empty => {
                    print_chess(self);
                    panic!("Panic at Normal Flag {:#?}", mv);
                }
                Square::Occupied(_) => self.replace_piece(mv.to, mv.from),
            },
            Flag::Capture => {
                self.replace_piece(mv.to, mv.from);
                match mv.captured {
                    Some(piece) => self.add_piece(mv.to, piece),
                    None => panic!("Error regarding placing back capture piece"),
                }
            }
            Flag::EP => {
                self.replace_piece(mv.to, mv.from);
                match (mv.active_color, mv.captured) {
                    (Color::White, Some(piece)) => self.add_piece(mv.to - 8, piece),
                    (Color::Black, Some(piece)) => self.add_piece(mv.to + 8, piece),
                    (Color::Black, None) => panic!("Error regarding placing back ep move"),
                    (Color::White, None) => panic!("Error regarding placing back ep move"),
                }
            }
            Flag::Promotion => {
                self.clear_piece(mv.to);
                match mv.captured {
                    Some(piece) => self.add_piece(mv.to, piece),
                    None => (),
                }
                self.add_piece(mv.from, mv.piece);
            }
            Flag::KingSideCastle => {
                self.replace_piece(mv.to, mv.from);
                match mv.active_color {
                    Color::White => self.replace_piece(F1 as usize, H1 as usize),
                    Color::Black => self.replace_piece(F8 as usize, H8 as usize),
                }
            }
            Flag::QueenSideCastle => {
                self.replace_piece(mv.to, mv.from);
                match mv.active_color {
                    Color::White => self.replace_piece(D1 as usize, A1 as usize),
                    Color::Black => self.replace_piece(D8 as usize, A8 as usize),
                }
            }
        }

        self.set_occupancy(Color::White);
        self.set_occupancy(Color::Black);
    }

    fn add_piece(&mut self, sq: usize, piece: Piece) {
        match self.squares[sq] {
            Square::Empty => (),
            Square::Occupied(_) => self.clear_piece(sq),
        }
        self.squares[sq] = Square::Occupied(Piece { pos: 1 << sq, ..piece });
        self.piece_bitboard[piece.p_color as usize][piece.p_type as usize].set_bit(sq);
    }

    fn clear_piece(&mut self, sq: usize) {
        match self.squares[sq] {
            Square::Empty => {
                panic!("Clearing a Peace that does not exist")
            }
            Square::Occupied(piece) => {
                self.piece_bitboard[piece.p_color as usize][piece.p_type as usize].clear_bit(sq);
                self.squares[sq] = Square::Empty;
            }
        }
    }

    fn replace_piece(&mut self, from_sq: usize, to_sq: usize) {
        let piece = match self.squares[from_sq] {
            Square::Empty => {
                print_chess(&self);
                panic!(
                    "There is no piece on square: {:#?}, \n other data: {:#?}",
                    from_sq, self.squares[from_sq]
                )
            }
            Square::Occupied(piece) => piece,
        };

        self.clear_piece(from_sq);
        self.add_piece(to_sq, piece);
    }

    fn generate_pos_key(&self) -> u64 {
        let mut final_key: u64 = 0;

        for idx in 0..64 {
            if let Square::Occupied(piece) = self.squares[idx] {
                // if piece.pos == 0 {
                //     panic!("Something wrong with the position of the peace")
                // }
                final_key ^= PieceKeys[idx][piece.p_color as usize][piece.p_type as usize];
            }
        }

        if self.active_color == Color::White {
            final_key ^= *SideKey;
        }

        match self.en_passant {
            Some(idx) => final_key ^= EpKeys[bit_scan_lsb(idx)],
            None => (),
        }

        if self.castling_rights.as_usize() < 16 {
            final_key ^= CastleKeys[self.castling_rights.as_usize()];
        }
        return final_key;
    }
}
