use std::vec;
use crate::engine::shared::helper_func::error_msg::Error;
use super::const_utility::*;

// DEPRECATE:
/**
 TEST: Not sure if the resonse is like that, and not sure if this is for the msb or lsb
 Get least segnificant bit from a bitboard(u64);
 * Ex: get_lsb(bitboard: 0....0111) -> 0
*/
pub fn bit_scan_lsb(bitboard: u64) -> usize {
    // Gets the least significant bit
    let bit: u64 = bitboard ^ (bitboard - 1) ^ (!bitboard & (bitboard - 1));
    return MOD67TABLE[(bit % 67) as usize];
}

// DEPRECATE:
/**
 TEST: Not sure if the resonse is like that, and not sure if this is for the msb or lsb
 Get most segnificant bit from a bitboard(u64);
 * Ex: get_msb(bitboard: 0....0111) -> 2
*/
pub fn bit_scan_msb(bitboard: u64) -> usize {
    return (bitboard as f64).log2().floor() as usize;
}

// DEPRECATE:
/**
 TEST: Not sure if the resonse is like that
 Extracts all of the bits from a bitboard(u64);
 * Ex: extract_bits(bitboard: 0....0111) -> [0, 1, 2]
*/
pub fn extract_all_bits(mut bitboard: u64) -> Vec<usize> {
    let mut result = vec![]; //Vec::with_capacity(64);

    while bitboard != 0 {
        let next_bit = bit_scan_lsb(bitboard);
        result.push(next_bit);
        bitboard ^= 1 << next_bit;
    }

    return result;
}

pub fn get_bit_rank(square: usize) -> Rank {
    match Rank::try_from(square / 8) {
        Ok(rank) => return rank,
        Err(_) => panic!("Invalid Thing"),
    }
}

pub fn get_bit_file(square: usize) -> File {
    match File::try_from(square % 8) {
        Ok(file) => return file,
        Err(_) => panic!("Invalid Thing"),
    }
}

pub fn get_rank_bits(square: usize) -> File {
    match File::try_from(square % 8) {
        Ok(file) => return file,
        Err(_) => panic!("Invalid Thing"),
    }
}

pub fn exclude_file_rank(bitboard: u64, file: Option<usize>, rank: Option<usize>) -> u64 {
    match (rank, file) {
        (Some(r), Some(f)) => return (bitboard & !RANK_BITBOARD[r]) & !FILE_BITBOARD[f],
        (Some(r), None) => return bitboard & !RANK_BITBOARD[r],
        (None, Some(f)) => return bitboard & !FILE_BITBOARD[f],
        (None, None) => return bitboard,
    }
}

pub fn include_only_file_rank(bitboard: u64, file: Option<usize>, rank: Option<usize>) -> u64 {
    match (rank, file) {
        (Some(r), Some(f)) => return (bitboard & RANK_BITBOARD[r]) & FILE_BITBOARD[f],
        (Some(r), None) => return bitboard & RANK_BITBOARD[r],
        (None, Some(f)) => return bitboard & FILE_BITBOARD[f],
        (None, None) => return bitboard,
    }
}

// DEPRECATE:
pub fn is_bit_set(bitboard: u64, square: usize) -> bool {
    return bitboard & (1 << square) != 0;
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
    return bitboard | (1 << position_to_idx(row, col, None));
}

/**
 Converts index to row and col tuple.
 If Index is not in bounds it panics if the check_bounds is enabled.
 * Ex: idx_to_position(idx: 50, check_bounds: true) -> (6, 2)
*/
pub fn idx_to_position(idx: usize, check_bounds: Option<bool>) -> (usize, usize) {
    let check_bounds = check_bounds.unwrap_or(true);
    if check_bounds && !is_inside_board_bounds_idx(idx) {
        panic!("{}", Error::InvalidIdxToPos { idx }.to_string());
    }
    return ((idx - (idx % 8)) / 8, idx % 8);
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

    return row * 8 + col;
}

/**
Checks if the row and col are inside the board. They should be between 0 and 7 included.
* Ex: is_inside_board_bounds_row_col(row: 8, col: 4) -> false
*/
pub fn is_inside_board_bounds_row_col(row: i8, col: i8) -> bool {
    return 0 <= row && row <= 7 && 0 <= col && col <= 7;
}

/**
Checks if the idx is inside the board. It should be between 0 and 63 included .
* Ex: is_inside_board_bounds_idx(63) -> true
*/
pub fn is_inside_board_bounds_idx(idx: usize) -> bool {
    return 0 <= idx && idx <= 63;
}

// TODO: Needs a rework in the future
pub fn position_to_bit(position: &str) -> Result<u64, String> {
    if position.len() != 2 {
        return Err(format!("Invalid length: {}, string: '{}'", position.len(), position));
    }

    let bytes = position.as_bytes();
    let byte0 = bytes[0];
    if byte0 < 97 || byte0 >= 97 + 8 {
        return Err(format!("Invalid Column character: {}, string: '{}'", byte0 as char, position));
    }

    let column = (byte0 - 97) as u32;

    let byte1 = bytes[1];
    let row;
    match (byte1 as char).to_digit(10) {
        Some(number) => {
            if number < 1 || number > 8 {
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
    let bit = (1 as u64) << square_number;

    Ok(bit)
}

//**** START: TESTS ****
#[cfg(test)]
mod tests {
    use crate::engine::shared::helper_func::print_utility::split_on;

    use super::*;

    #[test]
    fn split_on_space_works() {
        let test_string = "A B C D";
        let (item, rest) = split_on(test_string, ' ');
        assert_eq!(item, "A");
        assert_eq!(rest, "B C D");
    }

    // #[test]
    // fn index_to_position_works() {
    //     let test_arr: [usize; 3] = [1, 10, 62];
    //     assert_eq!(index_to_position(test_arr[0]), "b1");
    //     assert_eq!(index_to_position(test_arr[1]), "c2");
    //     assert_eq!(index_to_position(test_arr[2]), "g8");
    // }

    #[test]
    fn bit_scan_works() {
        for i in 0..64 {
            let bit = (1 as u64) << i;
            let index = bit_scan_lsb(bit);
            assert_eq!(i, index);
        }
    }

    #[test]
    fn bit_scan_with_multiple_bits() {
        for lowest_bit in 0..64 {
            let mut bit = 1 << lowest_bit;

            for other_bit in lowest_bit + 1..64 {
                if (other_bit + 37) % 3 != 0 {
                    bit |= 1 << other_bit;
                }
            }
            let bit_scan_result = bit_scan_lsb(bit);
            assert_eq!(lowest_bit, bit_scan_result);
        }
    }

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
