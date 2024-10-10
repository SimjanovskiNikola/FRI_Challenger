pub struct Rays {
    n_rays: Vec<u64>,
}

impl Rays {
    fn initialize() -> Self {
        let mut n_rays = vec![];

        for row in 1..8 {
            for col in 1..8 {
                n_rays.push(Self::n_ray(row, col));
            }
        }

        for row in 1..8 {
            for col in 1..8 {
                n_rays.push(Self::e_ray(row, col));
            }
        }

        return Self {
            n_rays: n_rays,
        };
    }

    pub fn n_ray(row: u64, col: u64) -> u64 {
        let mut bitboard = 0;

        for offset in 1..8 {
            if row + offset > 8 {
                break;
            }
            bitboard = Self::set_bit(bitboard, row + offset, col);
        }

        return bitboard;
    }

    pub fn set_bit(bitboard: u64, row: u64, col: u64) -> u64 {
        return bitboard | (1 << (col - 1 + (row - 1) * 8));
    }

    pub fn bitboard_to_string(bitboard: u64, mark: Option<usize>) -> String {
        let mut row = "".to_owned();
        let mut board = "".to_owned();

        for i in 0..64 {
            let value = (bitboard >> i) & 1;
            let s = if value == 0 { ".".to_owned() } else { value.to_string() };
            match mark {
                Some(idx) => if i == idx {
                    row.push_str("X");
                } else {
                    row.push_str(&s);
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
}

// TESTS: Here Are the tests for the above functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_n_ray() {
        println!(
            "Bitboard: \n--------Start---------\n{}--------End---------",
            Rays::bitboard_to_string(Rays::n_ray(4, 4), None)
        );
    }
}
