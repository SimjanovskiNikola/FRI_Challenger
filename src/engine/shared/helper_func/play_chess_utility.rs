use std::{
    io::{stdin, stdout, Write},
    process::exit,
};
use crate::engine::{
    game::Game,
    move_generation::{
        make_move::{self, GameMoveTrait},
        mv_gen::gen_moves,
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
        print!("Enter command (q | u | m | p | s): ");
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
                if game.mv_idx > 0 {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    game.undo_move();
                    print_chess(game);
                }
            }
            "s" => {
                println!("{:#?}", game.moves[game.mv_idx]);
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
                    }
                }
                move_list.clear();
            }
        }
    }
}
