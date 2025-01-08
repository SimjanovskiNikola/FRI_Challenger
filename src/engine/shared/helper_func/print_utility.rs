use crate::engine::{
    game::Game,
    shared::{helper_func::bit_pos_utility::*, structures::square_struct::Square},
};

pub fn print_bitboard(bitboard: u64, mark: Option<i8>) {
    println!(
        "Bitboard: \n--------Start---------\n{}--------End---------",
        bitboard_to_string(bitboard, mark)
    );
}

pub fn bitboard_to_string(bitboard: u64, mark: Option<i8>) -> String {
    let mut row = "".to_owned();
    let mut board = "".to_owned();

    for i in 0..64 {
        let value = (bitboard >> i) & 1;
        let s = if value == 0 { ".".to_owned() } else { value.to_string() };
        match mark {
            Some(idx) => {
                if i == idx {
                    row.push_str("X");
                } else {
                    row.push_str(&s);
                }
            }
            None => row.push_str(&s),
        }

        if (i + 1) % 8 == 0 {
            row.push_str("\n");
            board.insert_str(0, &row);
            row.clear();
        }
    }
    return board;
}

pub fn print_chess(game: &Game) {
    let mut chess_board = "".to_owned();
    for _ in 0..64 {
        chess_board.push_str(".");
    }

    for i in 0..2 {
        for bitboard in game.piece_bitboard[i] {
            for pos in extract_all_bits(bitboard) {
                let piece = match game.squares[pos] {
                    Square::Empty => continue,
                    Square::Occupied(piece) => piece,
                };
                chess_board.replace_range(pos..pos + 1, piece.chess_figure());
            }
        }
    }

    for (idx, c) in chess_board.chars().into_iter().enumerate() {
        print!("{}", c);
        if (idx + 1) % 8 == 0 {
            println!("");
        }
    }
}

pub fn split_on(s: &str, sep: char) -> (&str, &str) {
    for (i, item) in s.chars().enumerate() {
        if item == sep {
            return (&s[0..i], &s[i + 1..]);
        }
    }
    return (&s[..], "");
}
