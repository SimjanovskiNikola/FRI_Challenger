use crate::{Board, Color, Sq64TOSq120, SqPos, BOARD_SQ_NUM};
use super::print_utility::split_on;

// pub fn read_fen(fen: &str, board: &mut Board) -> Board {
//     let mut game: Board = Board {
//         pieces: vec![],
//         squares: vec![],
//         active_color: PieceColor::White,
//         castling_rights: CastlingRights::ALL,
//         en_passant: None,
//         halfmove_clock: 0,
//         fullmove_number: 1,
//         white_occupancy: 0,
//         black_occupancy: 0,
//     };

//     let (position, rest) = split_on(fen, ' ');
//     let mut deque_squares = VecDeque::new();
//     let mut piece_index = 0;
//     let mut piece_position = 64;

//     for row in position.splitn(8, '/') {
//         piece_position -= 8;
//         let (pieces, sqares) =
//             Self::parse_row(&mut game, &row, piece_index, piece_position);
//         for p in pieces {
//             game.pieces.push(p);
//             piece_index += 1;
//         }
//         for s in sqares {
//             deque_squares.push_front(s);
//         }
//     }
//     game.squares = Vec::from(deque_squares);

//     // COLOR
//     let (color, rest) = split_on(rest, ' ');
//     game.active_color = match color {
//         "w" => PieceColor::White,
//         "b" => PieceColor::Black,
//         _ => panic!("Unknown color: {}", color),
//     };

//     // CASTLING RIGHTS
//     let (castling_rights, rest) = split_on(rest, ' ');
//     let mut castling: CastlingRights = CastlingRights::NONE;
//     for ch in castling_rights.chars() {
//         match ch {
//             'K' => {
//                 castling |= CastlingRights::WKINGSIDE;
//             }
//             'Q' => {
//                 castling |= CastlingRights::WQUEENSIDE;
//             }
//             'k' => {
//                 castling |= CastlingRights::BKINGSIDE;
//             }
//             'q' => {
//                 castling |= CastlingRights::BQUEENSIDE;
//             }
//             '-' => (),
//             _ => panic!("Unknown Castling Rights: {}", ch),
//         }
//     }
//     game.castling_rights = castling;

//     // EnPassant
//     let (en_passant, rest) = split_on(rest, ' ');
//     match en_passant {
//         "-" => {
//             game.en_passant = None;
//         }
//         s => match position_to_bit(s) {
//             Err(msg) => panic!("{}", msg),
//             Ok(bit) => {
//                 game.en_passant = Some(bit);
//             }
//         },
//     }
//     // halfmove_clock
//     let (halfmove_clock, rest) = split_on(rest, ' ');
//     match halfmove_clock.parse() {
//         Ok(number) => {
//             game.halfmove_clock = number;
//         }
//         Err(_) => panic!("Invalid halfmove: {}", halfmove_clock),
//     }
//     // fullmove_number
//     let (fullmove_number, _) = split_on(rest, ' ');
//     match fullmove_number.parse() {
//         Ok(number) => {
//             game.fullmove_number = number;
//         }
//         Err(_) => panic!("Invalid halfmove: {}", fullmove_number),
//     }

//     return game;
// }

pub fn reset_board(board: &mut Board) {
    for idx in 0..BOARD_SQ_NUM {
        board.pieces[idx] = SqPos::NoSq as u64;
    }

    for idx in 0..64 {
        board.pieces[Sq64TOSq120[idx]] = SqPos::NoSq as u64;
    }

    for idx in 0..3 {
        board.big_pieces[idx] = 0;
        board.major_pieces[idx] = 0;
        board.minor_pieces[idx] = 0;
        board.pawns[idx] = 0;
    }

    for idx in 0..13 {
        board.piece_num[idx] = 0;
    }

    board.kings[Color::WHITE as usize] = 0;
    board.kings[Color::BLACK as usize] = 0;
    board.side = Color::BOTH;
    board.en_passant = None;
    board.fifty_move = 0;
    board.ply = 0;
    board.history_ply = 0;
    board.castle_permision = 0;
    board.positionKey = 0;
}
