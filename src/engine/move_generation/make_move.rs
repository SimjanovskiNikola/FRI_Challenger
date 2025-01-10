use crate::engine::{
    game::Game,
    shared::{
        helper_func::{
            bit_pos_utility::clear_bit,
            const_utility::{CLEAR_MASK, SET_MASK},
        },
        structures::{internal_move::InternalMove, piece_struct::Color, square_struct::Square},
    },
};

//  let mut new_move = InternalMove {
//             position_key: 0,
//             active_color: piece.p_color,
//             from: bit_scan_lsb(piece.pos),
//             to: p_move,
//             piece: *piece,
//             captured: match game.squares[p_move] {
//                 Square::Empty => None,
//                 Square::Occupied(piece) => Some(piece),
//             },
//             promotion: None, //NOTE:
//             ep: None,
//             castle: None,
//         };

//     pub squares: [Square; 64],
//     pub occupancy: [u64; 2],
//     pub piece_bitboard: [[u64; 6]; 2],
//     pub active_color: Color,
//     pub castling_rights: CastlingRights,
//     pub en_passant: Option<PiecePosition>,
//     pub halfmove_clock: usize,
//     pub fullmove_number: usize,

//     pub moves: Vec<InternalMove>,

// pub fn make_move(game: &mut Game, mv: &InternalMove) {

//     if let Some(piece) = mv.promotion {
//         game.squares[mv.from] = Square::Empty;
//         game.squares[mv.to] = Square::Occupied(piece);

//         game.piece_bitboard[mv.piece.p_color as usize][mv.piece.p_type as usize]  &= CLEAR_MASK[mv.from];
//         game.piece_bitboard[piece.p_color as usize][piece.p_type as usize] |= SET_MASK[mv.from];

//         if let Some(cap_piece) = mv.captured {
//             game.piece_bitboard[cap_piece.p_color as usize][cap_piece.p_type as usize]  &= CLEAR_MASK[mv.from];
//         }
//     }

//     if let Some(castling_rights) = mv.castle {
//         game.squares[mv.from] = Square::Empty;
//         game.squares[mv.to] = Square::Occupied(piece);

//         game.piece_bitboard[mv.piece.p_color as usize][mv.piece.p_type as usize]  &= CLEAR_MASK[mv.from];
//         game.piece_bitboard[piece.p_color as usize][piece.p_type as usize] |= SET_MASK[mv.from];

//         if let Some(cap_piece) = mv.captured {
//             game.piece_bitboard[cap_piece.p_color as usize][cap_piece.p_type as usize]  &= CLEAR_MASK[mv.from];
//         }
//     }

//     game.squares[mv.from] = Square::Empty;
//     game.squares[mv.to] = Square::Occupied(mv.piece);

//     game.piece_bitboard

//     game.set_occupancy(Color::White);
//     game.set_occupancy(Color::Black);

//     // TODO: Change Color
//     // TODO: UPDATE HALF MOVE
//     // TODO: UPDATE FULL MOVE
//     // TODO: GENERATE POSITION KEY
//     game.moves.push(*mv);
// }
