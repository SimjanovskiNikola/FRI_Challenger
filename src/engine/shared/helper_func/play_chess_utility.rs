use std::{
    io::{stdin, stdout, Write},
    process::exit,
};
use rand::Rng;

use crate::engine::{
    game::Game,
    move_generation::{
        make_move::GameMoveTrait,
        mv_gen::{gen_moves, is_repetition},
    },
    shared::{
        helper_func::print_utility::{move_notation, print_chess, print_move_list},
        structures::internal_move::{Flag, InternalMove},
    },
};

pub fn play_chess(game: &mut Game) {
    let mut move_list: Vec<InternalMove> = Vec::with_capacity(256);
    gen_moves(game.color, game);

    loop {
        let mut s = String::new();
        print!("Enter command (q | m | u | c | p | s | g | r): ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        match s.trim() {
            "q" => exit(400),
            "c" => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
            "p" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                print_chess(game);
            }
            "m" => {
                move_list = gen_moves(game.color, game);
                println!("{:?}", "Please choose a number: ");
                print_move_list(&move_list);
            }
            "u" => {
                if game.moves.len() > 0 {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    game.undo_move();
                    print_chess(game);
                }
            }
            "s" => {
                println!("{:#?}", game.moves.last());
            }
            "g" => {
                println!("Color: {:#?}", game.color);
                println!("Castling: {:#?}", game.castling);
                println!("EP: {:#?}", game.ep);
                println!("Half Move: {:#?}", game.half_move);
                println!("Full Move: {:#?}", game.full_move);
                println!("Position Key: {:#?}", game.pos_key);
                for i in &game.moves {
                    println!("{:?}", i.position_key);
                }
            }
            "r" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                move_list = gen_moves(game.color, game);
                let mut idx: usize = rand::rng().random_range(0..(move_list.len() - 1));
                while !game.make_move(&mut move_list[idx]) {
                    idx = rand::rng().random_range(0..(move_list.len() - 1));
                }
                print_chess(game);
                move_list.clear();
            }
            str => {
                move_list = gen_moves(game.color, game);
                for idx in 0..move_list.len() {
                    let promotion = match move_list[idx].flag {
                        Flag::Promotion(piece, _) => Some(piece),
                        _ => None,
                    };
                    if str
                        == move_notation(move_list[idx].from, move_list[idx].to, promotion).as_str()
                    {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        game.make_move(&mut move_list[idx]);
                        print_chess(game);
                        if is_repetition(game) {
                            println!("Repetition of a position");
                        }
                    }
                }
                move_list.clear();
            }
        }
    }
}
