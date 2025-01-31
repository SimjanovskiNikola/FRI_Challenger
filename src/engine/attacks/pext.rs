use std::{
    arch::x86_64::{_pdep_u64, _pext_u64},
    array,
    collections::HashMap,
};
use crate::engine::shared::structures::{directions::*, piece::*};
use super::all_attacks::{blocked_ray_att, ATTACKS};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ROOK_MASKS: [u64; 64] =
        array::from_fn(|sq| gen_rook_mov(sq) & !edges_of_board());
    pub static ref BISHOP_MASKS: [u64; 64] =
        array::from_fn(|sq| gen_bishop_mov(sq) & !edges_of_board());
    pub static ref ROOK_PEXT_TABLES: [[u64; 1024]; 64] = gen_pext_table_rook();
    pub static ref BISHOP_PEXT_TABLES: [[u64; 512]; 64] = gen_pext_table_bishop();
}

fn gen_pext_table_bishop() -> [[u64; 512]; 64] {
    let mut lookup_table = [[0u64; 512]; 64];
    for sq in 0..64 {
        println!("{:?}. {:?}", sq, ((2 as u64).pow(BISHOP_MASKS[sq].count_ones())));
        for occ in 0..((2 as u64).pow(BISHOP_MASKS[sq].count_ones())) {
            let extracted = insert_bits(BISHOP_MASKS[sq], occ as u64);
            let mut moves = 0u64;

            for dir in DIRECTIONS {
                if dir.is_diagonal() {
                    let ray = ATTACKS.rays[dir.idx()][sq];
                    moves |= blocked_ray_att(
                        DIRECTIONS[dir.idx()],
                        &ATTACKS.rays[dir.idx()],
                        ray,
                        0,
                        extracted,
                    );
                }
            }
            lookup_table[sq][occ as usize] = moves;
        }
    }
    return lookup_table;
}

pub fn gen_pext_table_rook() -> [[u64; 1024]; 64] {
    let mut lookup_table = [[0u64; 1024]; 64];
    for sq in 0..64 {
        for occ in 0..((2 as u64).pow(ROOK_MASKS[sq].count_ones())) {
            let extracted = insert_bits(ROOK_MASKS[sq], occ as u64);
            let mut moves = 0u64;

            for dir in DIRECTIONS {
                if dir.is_orthogonal() {
                    let ray = ATTACKS.rays[dir.idx()][sq];
                    moves |= blocked_ray_att(
                        DIRECTIONS[dir.idx()],
                        &ATTACKS.rays[dir.idx()],
                        ray,
                        0,
                        extracted,
                    );
                }
            }
            lookup_table[sq][occ as usize] = moves;
        }
    }

    return lookup_table;
}

pub fn gen_rook_mov(pos: usize) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= ATTACKS.rays[Dir::NORTH.idx()][pos];
    attacks |= ATTACKS.rays[Dir::SOUTH.idx()][pos];
    attacks |= ATTACKS.rays[Dir::EAST.idx()][pos];
    attacks |= ATTACKS.rays[Dir::WEST.idx()][pos];

    return attacks;
}

pub fn gen_bishop_mov(pos: usize) -> u64 {
    let mut attacks: u64 = 0;
    attacks |= ATTACKS.rays[Dir::NORTHEAST.idx()][pos];
    attacks |= ATTACKS.rays[Dir::NORTHWEST.idx()][pos];
    attacks |= ATTACKS.rays[Dir::SOUTHEAST.idx()][pos];
    attacks |= ATTACKS.rays[Dir::SOUTHWEST.idx()][pos];

    return attacks;
}

pub fn gen_moves(sq: usize, own: u64, enemy: u64, is_bishop: bool) -> u64 {
    let mask = if is_bishop { BISHOP_MASKS[sq] } else { ROOK_MASKS[sq] };
    let occupancy = own | enemy;
    let key = pext(occupancy, mask);

    if is_bishop {
        BISHOP_PEXT_TABLES[sq][key as usize] & !own
    } else {
        ROOK_PEXT_TABLES[sq][key as usize] & !own
    }
}

// fn generate_pext_table(square: usize, diagonal: bool) -> HashMap<u64, u64> {
//     let mask = sliding_piece_mask(square, diagonal);
//     let num_entries = 1 << mask.count_ones(); // 2^(# relevant bits)
//     println!("{:?}", num_entries);
//     let mut table = HashMap::with_capacity(num_entries);

//     for occupancy in 0..num_entries {
//         let extracted = insert_bits(mask, occupancy as u64);
//         let mut moves = 0u64;

//         // Calculate sliding moves based on extracted occupancy
//         for dir in DIRECTIONS {
//             if diagonal == dir.is_diagonal() {
//                 let ray = ATTACKS.rays[dir.idx()][square];
//                 moves |= blocked_ray_att(
//                     DIRECTIONS[dir.idx()],
//                     &ATTACKS.rays[dir.idx()],
//                     ray,
//                     extracted,
//                     0,
//                 );
//             }
//         }
//         table.insert(occupancy as u64, moves);
//     }
//     table
// }

// pub fn sliding_piece_mask(square: usize, diagonal: bool) -> u64 {
//     let mut mask = 0;
//     for dir in DIRECTIONS {
//         if diagonal == dir.is_diagonal() {
//             mask |= ATTACKS.rays[dir.idx()][square];
//         }
//     }
//     mask & !edges_of_board()
// }

// TODO: This can be optimized with one u64 which can be constant
fn edges_of_board() -> u64 {
    let mut edges = 0xff818181818181ffu64; // All board edges
                                           // edges ^= (1u64 << 0) | (1u64 << 7) | (1u64 << 56) | (1u64 << 63); // Remove corners
    edges
}

fn insert_bits(mask: u64, occupancy: u64) -> u64 {
    let mut result = 0;
    let mut bit = 0;
    for i in 0..64 {
        if (mask >> i) & 1 == 1 {
            if (occupancy >> bit) & 1 == 1 {
                result |= 1 << i;
            }
            bit += 1;
        }
    }
    result
}

// pub fn generate_moves(square: usize, own: u64, enemy: u64, is_bishop: bool) -> u64 {
//     let mask = sliding_piece_mask(square, is_bishop);
//     let occupancy = own | enemy;
//     let index = pext(occupancy, mask);

//     if is_bishop {
//         BISHOP_PEXT_TABLES[square][index]
//     } else {
//         ROOK_PEXT_TABLES[square][index]
//     }
// }
// pub fn get_moves(square: usize, own: u64, enemy: u64, piece: Piece) -> u64 {
//     match piece.kind() {
//         KING => ATTACKS.king[square],
//         KNIGHT => ATTACKS.knight[square],
//         BISHOP => generate_moves(square, own, enemy, true),
//         ROOK => generate_moves(square, own, enemy, false),
//         QUEEN => {
//             return generate_moves(square, own, enemy, true)
//                 | generate_moves(square, own, enemy, false)
//         }
//         // PAWN => ATTACKS.pawn.get_attacks(square, own, enemy), // Fix This
//         _ => panic!("The provided Peace is not of valid Kind !!!"),
//     }
// }

pub fn pext(bitboard: u64, mask: u64) -> u64 {
    unsafe { _pext_u64(bitboard, mask) }
}

pub fn pdep(bitboard: u64, mask: u64) -> u64 {
    unsafe { _pdep_u64(bitboard, mask) }
}

#[cfg(test)]
mod tests {

    use engine::engine::{
        game::Game, move_generation::fen::FenTrait, shared::helper_func::const_utility::SqPos,
    };

    use crate::engine::{
        attacks::all_attacks::Attacks,
        shared::{
            helper_func::{
                bit_pos_utility::extract_all_bits,
                bitboard::BitboardTrait,
                const_utility::{FEN_CASTLE_ONE, FEN_PAWNS_WHITE},
                print_utility::print_bitboard,
            },
            structures::{
                color::{BLACK, WHITE},
                directions::Dir,
            },
        },
    };

    use super::*;

    // #[test]
    // fn test_king_knight_attacks_init() {
    //     let game = Game::read_fen(FEN_PAWNS_WHITE);
    //     let sq = extract_all_bits(game.bitboard[BLACK_ROOK.idx()]);

    //     print_bitboard(
    //         get_moves(sq[1], game.occupancy[BLACK.idx()], game.occupancy[BLACK.idx()], BLACK_ROOK),
    //         Some(sq[1] as i8),
    //     );

    // let bla = ROOK_PEXT_TABLES[sq[1]].get(&1u64);

    // if let Some(num) = bla {
    //     print_bitboard(*num, Some(sq[1] as i8));
    // }
    // }

    #[test]
    fn test_king_knight_attacks_init1() {
        // let arrRookBase = ATTACKS.rays[Dir::EAST.idx()][SqPos::D6 as usize]
        //     | ATTACKS.rays[Dir::WEST.idx()][SqPos::D6 as usize]
        //     | ATTACKS.rays[Dir::SOUTH.idx()][SqPos::D6 as usize]
        //     | ATTACKS.rays[Dir::NORTH.idx()][SqPos::D6 as usize];

        // let hor_occupancy = (1u64 << SqPos::D2 as usize);
        // // | (1u64 << SqPos::D6 as usize)
        // // | (1u64 << SqPos::D5 as usize)
        // // | (1u64 << SqPos::D4 as usize)
        // // | (1u64 << SqPos::D3 as usize);
        // let ver_occupancy = 0;
        // // (1u64 << SqPos::D6 as usize)
        // // | (1u64 << SqPos::A6 as usize)
        // // | (1u64 << SqPos::C6 as usize);
        // let occupancy = hor_occupancy | ver_occupancy;
        // let bla = pext(occupancy, arrRookBase & !edges_of_board());

        // print_bitboard(arrRookBase, None);
        // print_bitboard(arrRookBase & !edges_of_board(), None);
        // print_bitboard(occupancy, None);
        // print_bitboard(bla, None);
        // print_bitboard(arrRookBase + bla, None);

        let rook_mv = gen_moves(
            SqPos::D2 as usize,
            (1u64 << (SqPos::D2 as usize)),
            (1u64 << (SqPos::D6 as usize)),
            false,
        );
        print_bitboard(rook_mv, Some(SqPos::D2 as i8));

        print_bitboard(BISHOP_MASKS[27], None);

        let bishop_mv = gen_moves(
            SqPos::D2 as usize,
            (1u64 << (SqPos::E3 as usize)) | (1u64 << (SqPos::F2 as usize)),
            (1u64 << (SqPos::D6 as usize)) | (1u64 << (SqPos::B4 as usize)),
            true,
        );
        print_bitboard(bishop_mv, Some(SqPos::D2 as i8));
    }
}
