use crate::utils::{ bit_scan, bit_scan_backward };

macro_rules! make_rays {
    ($ray_fn:ident) => {
        {
            let mut rays = vec![];
            
            for row in 0..8 {
                for col in 0..8 {
                    rays.push($ray_fn(row, col));
                }
            }   

            rays
        }
    };
}

macro_rules! define_ray {
    ($name:ident, $offset_fn:expr) => {
        pub fn $name(row: i64, col: i64) -> u64 {
            let mut bitboard = 0;

            for offset in 1..8 {
                bitboard = set_bit(bitboard, $offset_fn(row, col, offset));
            }

            return bitboard;
        }
    };
}

pub struct Rays {
    n_rays: Vec<u64>,
    e_rays: Vec<u64>,
    nw_rays: Vec<u64>,
    ne_rays: Vec<u64>,
    w_rays: Vec<u64>,
    s_rays: Vec<u64>,
    sw_rays: Vec<u64>,
    se_rays: Vec<u64>,
}

impl Rays {
    fn initialize() -> Self {
        let mut n_rays: Vec<u64> = make_rays!(n_ray);
        let mut e_rays: Vec<u64> = make_rays!(e_ray);
        let mut nw_rays: Vec<u64> = make_rays!(nw_ray);
        let mut ne_rays: Vec<u64> = make_rays!(ne_ray);
        let mut w_rays: Vec<u64> = make_rays!(w_ray);
        let mut s_rays: Vec<u64> = make_rays!(s_ray);
        let mut sw_rays: Vec<u64> = make_rays!(sw_ray);
        let mut se_rays: Vec<u64> = make_rays!(se_ray);

        return Self {
            n_rays: n_rays,
            e_rays: e_rays,
            nw_rays: nw_rays,
            ne_rays: ne_rays,
            w_rays: w_rays,
            s_rays: s_rays,
            sw_rays: sw_rays,
            se_rays: se_rays,
        };
    }
}

define_ray!(n_ray, |row, col, offset| (row + offset, col));
define_ray!(e_ray, |row, col, offset| (row, col + offset));
define_ray!(nw_ray, |row, col, offset| (row + offset, col - offset));
define_ray!(ne_ray, |row, col, offset| (row + offset, col + offset));
define_ray!(w_ray, |row, col, offset| (row, col - offset));
define_ray!(s_ray, |row, col, offset| (row - offset, col));
define_ray!(sw_ray, |row, col, offset| (row - offset, col - offset));
define_ray!(se_ray, |row, col, offset| (row - offset, col + offset));

pub fn set_bit(bitboard: u64, row_col: (i64, i64)) -> u64 {
    let (row, col) = row_col;

    if row < 0 || row > 7 || col < 0 || col > 7 {
        return bitboard;
    }
    return bitboard | (1 << (col + row * 8));
}

pub fn blocked_ray_attack(
    ray: u64,
    ray_family: &Vec<u64>,
    forward_ray: bool,
    occupancy: u64
) -> u64 {
    let overlap = ray & occupancy;
    let bit_idx;

    if forward_ray {
        bit_idx = bit_scan(overlap);
    } else {
        bit_idx = bit_scan_backward(overlap);
    }

    let ray_after = ray_family[bit_idx];
    return ray ^ ray_after;
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

// TESTS: Here Are the tests for the above functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocked_ray() {
        let mut occupancy = 0;
        for i in 0..16 {
            if i == 5 {
                continue;
            }
            occupancy |= 1 << i;
        }
        occupancy |= 1 << 22;

        for i in 48..64 {
            if i == 57 || i == 49 {
                continue;
            }
            occupancy |= 1 << i;
        }
        occupancy |= 1 << 41;
        occupancy |= 1 << 42;

        let rays = Rays::initialize();
        let row = 5;
        let col = 6;
        let idx = row * 8 + col;
        // occupancy &= rays.nw_rays[idx];

        println!("{}", bitboard_to_string(occupancy, Some(idx)));
        println!("{}", bitboard_to_string(rays.sw_rays[idx], Some(idx)));
        println!(
            "{}",
            bitboard_to_string(occupancy & rays.sw_rays[idx], Some(idx))
        );
        let blocked_ray = blocked_ray_attack(
            rays.sw_rays[idx],
            &rays.sw_rays,
            false,
            occupancy
        );
        println!("{}", bitboard_to_string(blocked_ray, None));
    }

    #[test]
    fn print_n_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        println!(
            "Bitboard: \n--------Start---------\n{}--------End---------",
            bitboard_to_string(rays.n_rays[idx], Some(idx))
        );
    }
    #[test]
    fn print_ne_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        println!(
            "Bitboard: \n--------Start---------\n{}--------End---------",
            bitboard_to_string(rays.ne_rays[idx], Some(idx))
        );
    }
    #[test]
    fn print_e_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        println!(
            "Bitboard: \n--------Start---------\n{}--------End---------",
            bitboard_to_string(rays.e_rays[idx], Some(idx))
        );
    }
    #[test]
    fn print_se_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        println!(
            "Bitboard: \n--------Start---------\n{}--------End---------",
            bitboard_to_string(rays.se_rays[idx], Some(idx))
        );
    }
    #[test]
    fn print_s_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        println!(
            "Bitboard: \n--------Start---------\n{}--------End---------",
            bitboard_to_string(rays.s_rays[idx], Some(idx))
        );
    }
    #[test]
    fn print_sw_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        println!(
            "Bitboard: \n--------Start---------\n{}--------End---------",
            bitboard_to_string(rays.sw_rays[idx], Some(idx))
        );
    }
    #[test]
    fn print_w_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        println!(
            "Bitboard: \n--------Start---------\n{}--------End---------",
            bitboard_to_string(rays.w_rays[idx], Some(idx))
        );
    }
    #[test]
    fn print_nw_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        println!(
            "Bitboard: \n--------Start---------\n{}--------End---------",
            bitboard_to_string(rays.nw_rays[idx], Some(idx))
        );
    }
}
