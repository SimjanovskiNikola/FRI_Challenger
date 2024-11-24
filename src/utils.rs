/* A Chess board has files and ranks */
/* Rank (Row) - horizontal from A to H */
/* Files (Columns) - vertical from 1 to 8*/
use crate::game::PiecePosition;

static RANK_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23, 3, 12, 16, 59, 41, 19, 24, 54, 4, 64, 13, 10,
    17, 62, 60, 28, 42, 30, 20, 51, 25, 44, 55, 47, 5, 32, 64, 38, 14, 22, 11, 58,
    18, 53, 63, 9, 61, 27, 29, 50, 43, 46, 31, 37, 21, 57, 52, 8, 26, 49, 45, 36,
    56, 7, 48, 35, 6, 34, 33,
];

// DEPRECATE:
fn bit_scan_simple(mut bit: u64) -> usize {
    let mut leading_zeros = 0;
    while (bit & 1) == 0 {
        bit >>= 1;
        leading_zeros += 1;
    }
    return leading_zeros;
}

pub fn bit_scan(bit: u64) -> usize {
    // Gets the least significant bit
    let one_bit = bit ^ (bit - 1) ^ (!bit & (bit - 1));
    return MOD67TABLE[(one_bit % 67) as usize];
}

pub fn bit_scan_backward(bit: u64) -> usize {
    return (bit as f64).log2().floor() as usize;
}

pub fn bit_to_position(bit: u64) -> Result<String, String> {
    if bit == 0 {
        return Err("No piece present!".to_string());
    } else {
        let bit = bit_scan(bit);
        return Ok(index_to_position(bit).to_string());
    }
}

pub fn index_to_position(index: usize) -> String {
    let file = index / 8 + 1;
    let rank = index % 8;
    return format!("{}{}", RANK_MAP[rank], file);
}

pub fn split_on(s: &str, sep: char) -> (&str, &str) {
    for (i, item) in s.chars().enumerate() {
        if item == sep {
            return (&s[0..i], &s[i + 1..]);
        }
    }
    return (&s[..], "");
}

pub fn position_to_bit(position: &str) -> Result<PiecePosition, String> {
    if position.len() != 2 {
        return Err(
            format!(
                "Invalid length: {}, string: '{}'",
                position.len(),
                position
            )
        );
    }

    let bytes = position.as_bytes();
    let byte0 = bytes[0];
    if byte0 < 97 || byte0 >= 97 + 8 {
        return Err(
            format!(
                "Invalid Column character: {}, string: '{}'",
                byte0 as char,
                position
            )
        );
    }

    let column = (byte0 - 97) as u32;

    let byte1 = bytes[1];
    let row;
    match (byte1 as char).to_digit(10) {
        Some(number) => {
            if number < 1 || number > 8 {
                return Err(
                    format!(
                        "Invalid Row character: {}, string: '{}'",
                        byte1 as char,
                        position
                    )
                );
            } else {
                row = number - 1;
            }
        }
        None => {
            return Err(
                format!(
                    "Invalid Row character: {}, string: '{}'",
                    byte1 as char,
                    position
                )
            );
        }
    }

    let square_number = row * 8 + column;
    let bit = (1 as u64) << square_number;

    Ok(bit)
}

pub fn bitboard_to_string(bitboard: u64, mark: Option<usize>) -> String {
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

pub fn idx_to_position(index: usize) -> (usize, usize) {
    return ((index - (index % 8)) / 8, index % 8);
}

pub fn position_to_idx(row: usize, col: usize) -> usize {
    return row * 8 + col;
}

// TESTS: Here Are the tests for the above functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_on_space_works() {
        let test_string = "A B C D";
        let (item, rest) = split_on(test_string, ' ');
        assert_eq!(item, "A");
        assert_eq!(rest, "B C D");
    }

    #[test]
    fn index_to_position_works() {
        let test_arr: [usize; 3] = [1, 10, 62];
        assert_eq!(index_to_position(test_arr[0]), "b1");
        assert_eq!(index_to_position(test_arr[1]), "c2");
        assert_eq!(index_to_position(test_arr[2]), "g8");
    }

    #[test]
    fn bit_scan_works() {
        for i in 0..64 {
            let bit = (1 as u64) << i;
            let index = bit_scan(bit);
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
            let bit_scan_result = bit_scan(bit);
            assert_eq!(lowest_bit, bit_scan_result);
        }
    }
}
