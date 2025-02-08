use crate::engine::{
    game::Game,
    shared::{
        helper_func::{
            bit_pos_utility::bit_scan_lsb,
            bitboard::{Bitboard, BitboardTrait},
            print_utility::print_chess,
        },
        structures::{
            castling_struct::CastlingRights,
            color::{Color, BLACK, WHITE},
            internal_move::{Flag, InternalMove},
            piece::{Piece, PieceTrait, KING, ROOK},
            square::{SqPos::*, Square},
        },
    },
};

// const TERMINATION_MARKERS = ['1-0', '0-1', '1/2-1/2', '*'] NOTE: Maybe i will need this

use lazy_static::lazy_static;
use rand::Rng;

use super::move_generation::sq_attack;

lazy_static! {
    pub static ref PieceKeys: [[u64; 14]; 64] = [[rand::rng().random(); 14]; 64];
    pub static ref EpKeys: [u64; 64] = [rand::rng().random(); 64];
    pub static ref CastleKeys: [u64; 16] = [rand::rng().random(); 16];
    pub static ref SideKey: u64 = rand::rng().random();
}

pub trait GameMoveTrait {
    fn make_move(&mut self, mv: &mut InternalMove) -> bool;
    fn undo_move(&mut self);
    fn generate_pos_key(&self) -> u64;
    fn add_piece(&mut self, sq: usize, piece: Piece);
    fn clear_piece(&mut self, sq: usize);
    fn replace_piece(&mut self, from_sq: usize, to_sq: usize);
}

impl GameMoveTrait for Game {
    fn make_move(&mut self, mv: &mut InternalMove) -> bool {
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
                    WHITE => self.clear_piece(mv.to - 8),
                    BLACK => self.clear_piece(mv.to + 8),
                    _ => panic!("Invalid Color"),
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
                    WHITE => self.replace_piece(H1 as usize, F1 as usize),
                    BLACK => self.replace_piece(H8 as usize, F8 as usize),
                    _ => panic!("Invalid Color"),
                }
            }
            Flag::QueenSideCastle => {
                // println!("{:?}", mv.active_color);
                // print_chess(self);

                self.replace_piece(mv.from, mv.to);
                match mv.active_color {
                    WHITE => self.replace_piece(A1 as usize, D1 as usize),
                    BLACK => self.replace_piece(A8 as usize, D8 as usize),
                    _ => panic!("Invalid Color"),
                }

                // print_chess(self);
            }
        }

        self.color.change_color();

        //If the castleRight is set, and if the king is on place and rook is on place than retain otherwise clear
        let castle_tuple: [(usize, usize, CastlingRights, Color); 4] = [
            (H1 as usize, E1 as usize, CastlingRights::WKINGSIDE, WHITE),
            (A1 as usize, E1 as usize, CastlingRights::WQUEENSIDE, WHITE),
            (H8 as usize, E8 as usize, CastlingRights::BKINGSIDE, BLACK),
            (A8 as usize, E8 as usize, CastlingRights::BQUEENSIDE, BLACK),
        ];
        for c in castle_tuple {
            if !mv.castle.is_set(c.2)
                || !self.bitboard[(ROOK + c.3) as usize].is_set(c.0)
                || !self.bitboard[(KING + c.3) as usize].is_set(c.1)
            {
                self.castling.clear(c.2);
            }
        }

        if mv.piece.is_pawn() && mv.from.abs_diff(mv.to) == 16 {
            self.ep = match mv.active_color {
                WHITE => Some(Bitboard::init(mv.to - 8)),
                BLACK => Some(Bitboard::init(mv.to + 8)),
                _ => panic!("Invalid Color"),
            }
        } else {
            self.ep = None
        }

        if mv.piece.is_pawn() || mv.captured != None {
            self.half_move = 0
        } else {
            self.half_move = mv.half_move + 1;
        }

        if self.moves.len() % 2 == 0 {
            self.full_move += 1;
        }

        // FIXME: 1ms for 1000 nodes, generate the key better
        // mv.position_key = self.generate_pos_key();
        // self.moves.push(*mv);

        self.mv_idx += 1;
        self.moves[self.mv_idx] = Some(*mv);

        let king_sq = self.bitboard[(KING + mv.active_color) as usize].get_lsb();

        // if mv.from == 51 && mv.to == 35 && mv.piece.p_type == PieceType::Pawn {
        //     print_chess(self);
        //     println!("{:?}", "MAKE MOVE END");
        //     println!(
        //         "King is attacked: {:?}",
        //         gen_attacks(self, self.active_color).is_set(king_sq)
        //     );
        // }

        if sq_attack(&self, king_sq, mv.active_color) != 0 {
            // if gen_attacks(self, self.color).is_set(king_sq) {
            self.undo_move();
            return false;
        }

        return true;
    }

    fn undo_move(&mut self) {
        // println!("Before Taking move: {:#?}", self.moves);
        // print_chess(self);

        let mv = match self.moves[self.mv_idx] {
            Some(mv) => mv,
            None => return,
        };
        self.mv_idx -= 1;

        // println!("After Taking move: {:#?}", self.moves);
        // print_chess(self);

        self.full_move -= if self.moves.len() % 2 == 0 { 1 } else { 0 };
        self.half_move = mv.half_move;
        self.ep = mv.ep;
        self.castling = mv.castle;
        self.color.change_color();

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
                    (WHITE, Some(piece)) => self.add_piece(mv.to - 8, piece),
                    (BLACK, Some(piece)) => self.add_piece(mv.to + 8, piece),
                    (_, _) => panic!("Error regarding placing back ep move"),
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
                    WHITE => self.replace_piece(F1 as usize, H1 as usize),
                    BLACK => self.replace_piece(F8 as usize, H8 as usize),
                    _ => panic!("Error there should be only two colors"),
                }
            }
            Flag::QueenSideCastle => {
                self.replace_piece(mv.to, mv.from);
                match mv.active_color {
                    WHITE => self.replace_piece(D1 as usize, A1 as usize),
                    BLACK => self.replace_piece(D8 as usize, A8 as usize),
                    _ => panic!("Error there should be only two colors"),
                }
            }
        }
    }

    fn add_piece(&mut self, sq: usize, piece: Piece) {
        match self.squares[sq] {
            Square::Empty => (),
            Square::Occupied(_) => self.clear_piece(sq),
        }
        self.squares[sq] = Square::Occupied(piece);
        self.bitboard[piece.idx()].set_bit(sq);
        self.occupancy[piece.color().idx()].set_bit(sq);
        self.pos_key ^= PieceKeys[sq][piece.idx()];
    }

    fn clear_piece(&mut self, sq: usize) {
        match self.squares[sq] {
            Square::Empty => panic!("Clearing a Peace that does not exist"),
            Square::Occupied(piece) => {
                self.squares[sq] = Square::Empty;
                self.bitboard[piece.idx()].clear_bit(sq);
                self.occupancy[piece.color().idx()].clear_bit(sq);
                self.pos_key ^= PieceKeys[sq][piece.idx()];
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

        if self.color == WHITE {
            final_key ^= *SideKey;
        }

        match self.ep {
            Some(idx) => final_key ^= EpKeys[bit_scan_lsb(idx)],
            None => (),
        }

        if self.castling.idx() < 16 {
            final_key ^= CastleKeys[self.castling.idx()];
        }
        return final_key;
    }
}
