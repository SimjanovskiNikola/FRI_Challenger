use std::vec;
use super::{bitboard::BitboardTrait, const_utility::*};

/**
 TEST: Not sure if the resonse is like that
 Extracts all of the bits from a bitboard(u64);
 * Ex: extract_bits(bitboard: 0....0111) -> [0, 1, 2]
*/
pub fn extract_all_bits(mut bitboard: u64) -> Vec<usize> {
    let mut result = vec![]; //Vec::with_capacity(64);

    while bitboard != 0 {
        let next_bit = bitboard.get_lsb();
        result.push(next_bit);
        bitboard ^= 1 << next_bit;
    }

    result
}

pub fn get_bit_rank(square: usize) -> Rank {
    match Rank::try_from(square / 8) {
        Ok(rank) => rank,
        Err(_) => panic!("Invalid Thing"),
    }
}

pub fn get_bit_file(square: usize) -> File {
    match File::try_from(square % 8) {
        Ok(file) => file,
        Err(_) => panic!("Invalid Thing"),
    }
}

pub fn get_rank_bits(square: usize) -> File {
    match File::try_from(square % 8) {
        Ok(file) => file,
        Err(_) => panic!("Invalid Thing"),
    }
}

pub fn exclude_file_rank(bitboard: u64, file: Option<usize>, rank: Option<usize>) -> u64 {
    match (rank, file) {
        (Some(r), Some(f)) => (bitboard & !RANK_BITBOARD[r]) & !FILE_BITBOARD[f],
        (Some(r), None) => bitboard & !RANK_BITBOARD[r],
        (None, Some(f)) => bitboard & !FILE_BITBOARD[f],
        (None, None) => bitboard,
    }
}

pub fn include_only_file_rank(bitboard: u64, file: Option<usize>, rank: Option<usize>) -> u64 {
    match (rank, file) {
        (Some(r), Some(f)) => (bitboard & RANK_BITBOARD[r]) & FILE_BITBOARD[f],
        (Some(r), None) => bitboard & RANK_BITBOARD[r],
        (None, Some(f)) => bitboard & FILE_BITBOARD[f],
        (None, None) => bitboard,
    }
}

/**
 * FIXME: Remove this or change name
   Sets one bit to the given bitboard(u64) by getting the row and col.
   It also checks if the row and col re in bounds.
 * Ex: set_bit(bitboard: 0, row: 0, col: 1) -> 0...010
*/
pub fn set_bit(bitboard: u64, row: i8, col: i8) -> u64 {
    if !is_inside_board_bounds_row_col(row, col) {
        return bitboard;
    }
    bitboard | (1 << position_to_idx(row, col, None))
}

/**
   Converts index to row and col tuple.
   If Index is not in bounds it panics if the check_bounds is enabled.
 * Ex: idx_to_position(idx: 50, check_bounds: true) -> (6, 2)
*/
pub fn idx_to_position(idx: usize, check_bounds: Option<bool>) -> (usize, usize) {
    let check_bounds = check_bounds.unwrap_or(true);
    if check_bounds && !is_inside_board_bounds_idx(idx) {
        panic!("Invalid position index: {}", idx);
    }
    ((idx - (idx % 8)) / 8, idx % 8)
}

/**
 Converts given row and col to position index.
 If row and col are not in bounds it panics if the check_bounds is enabled
 * Ex: position_to_idx(row: 6, col: 2, check_bounds: true) -> 50
*/
pub fn position_to_idx(row: i8, col: i8, check_bounds: Option<bool>) -> i8 {
    let check_bounds = check_bounds.unwrap_or(true);
    if check_bounds && !is_inside_board_bounds_row_col(row, col) {
        panic!("The row and col are not inside bounds");
    }

    row * 8 + col
}

/**
Checks if the row and col are inside the board. They should be between 0 and 7 included.
* Ex: is_inside_board_bounds_row_col(row: 8, col: 4) -> false
*/
pub fn is_inside_board_bounds_row_col(row: i8, col: i8) -> bool {
    (0..=7).contains(&row) && (0..=7).contains(&col)
}

/**
Checks if the idx is inside the board. It should be between 0 and 63 included .
* Ex: is_inside_board_bounds_idx(63) -> true
*/
pub fn is_inside_board_bounds_idx(idx: usize) -> bool {
    idx <= 63
}

// TODO: Needs a rework in the future
pub fn position_to_bit(position: &str) -> Result<u64, String> {
    if position.len() != 2 {
        return Err(format!("Invalid length: {}, string: '{}'", position.len(), position));
    }

    let bytes = position.as_bytes();
    let byte0 = bytes[0];
    if !(97..97 + 8).contains(&byte0) {
        return Err(format!("Invalid Column character: {}, string: '{}'", byte0 as char, position));
    }

    let column = (byte0 - 97) as u32;

    let byte1 = bytes[1];
    let row;
    match (byte1 as char).to_digit(10) {
        Some(number) => {
            if !(1..=8).contains(&number) {
                return Err(format!(
                    "Invalid Row character: {}, string: '{}'",
                    byte1 as char, position
                ));
            } else {
                row = number - 1;
            }
        }
        None => {
            return Err(format!(
                "Invalid Row character: {}, string: '{}'",
                byte1 as char, position
            ));
        }
    }

    let square_number = row * 8 + column;
    let bit = 1u64 << square_number;

    Ok(bit)
}

//**** START: TESTS ****
#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn index_to_position_works() {
    //     let test_arr: [usize; 3] = [1, 10, 62];
    //     assert_eq!(index_to_position(test_arr[0]), "b1");
    //     assert_eq!(index_to_position(test_arr[1]), "c2");
    //     assert_eq!(index_to_position(test_arr[2]), "g8");
    // }

    #[test]
    fn test_get_bits() {
        let bits = (1 << 2) | (1 << 5) | (1 << 55);
        let resp = extract_all_bits(bits);

        assert_eq!(vec![2, 5, 55], resp)
    }

    #[test]
    fn test_get_bit_file() {
        assert_eq!(get_bit_file(0), File::A);
        assert_eq!(get_bit_file(3), File::D);
        assert_eq!(get_bit_file(15), File::H);
        assert_eq!(get_bit_file(17), File::B);
        assert_eq!(get_bit_file(26), File::C);
    }

    #[test]
    fn test_get_bit_rank() {
        assert_eq!(get_bit_rank(0), Rank::One);
        assert_eq!(get_bit_rank(3), Rank::One);
        assert_eq!(get_bit_rank(15), Rank::Two);
        assert_eq!(get_bit_rank(17), Rank::Three);
        assert_eq!(get_bit_rank(26), Rank::Four);
    }
}
//**** END: TESTS ****
