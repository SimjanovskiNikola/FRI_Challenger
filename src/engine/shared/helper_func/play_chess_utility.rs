use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::exit;

use rand::Rng;

use crate::engine::game::Game;
use crate::engine::move_generation::make_move::GameMoveTrait;
use crate::engine::move_generation::mv_gen::gen_moves;
use crate::engine::move_generation::mv_gen::is_repetition;
use crate::engine::search::searcher::iterative_deepening;
use crate::engine::search::transposition_table::get_line;
use crate::engine::shared::helper_func::print_utility::move_notation;
use crate::engine::shared::helper_func::print_utility::print_chess;
use crate::engine::shared::helper_func::print_utility::print_move_list;
use crate::engine::shared::structures::internal_move::Flag;

pub fn play_chess(game: &mut Game) {
    // let mut move_list: Vec<InternalMove>;
    let (mut irr, mut pos_rev) = gen_moves(game.color, game);

    loop {
        let mut s = String::new();
        print!("Enter command (q | m | u | c | p | s | g | r | l | a): ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        match s.trim() {
            "q" => exit(400),
            "c" => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
            "p" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                print_chess(game);
            }
            "l" => {
                print_move_list(&get_line(game, game.key));
            }
            "a" => {
                println!("{:#?}", game.tt.table);
            }
            "m" => {
                (irr, pos_rev) = gen_moves(game.color, game);
                println!("{:?}", "Please choose a number: ");
                print_move_list(&pos_rev);
            }
            "u" => {
                if !game.pos_rev.is_empty() {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    game.undo_move();
                    print_chess(game);
                }
            }
            "s" => {
                iterative_deepening(game);
                // println!("{:#?}", game.moves.last());
            }
            "g" => {
                println!("Color: {:#?}", game.color);
                println!("Castling: {:#?}", game.castling);
                println!("EP: {:#?}", game.ep);
                println!("Half Move: {:#?}", game.half_move);
                println!("Full Move: {:#?}", game.full_move);
                println!("Position Key: {:#?}", game.key);
                for irr in &game.pos_irr {
                    println!("{:?}", irr.key);
                }
            }
            "r" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                (irr, pos_rev) = gen_moves(game.color, game);
                let mut idx: usize = rand::rng().random_range(0..(pos_rev.len() - 1));
                while !game.make_move(&pos_rev[idx], &irr) {
                    idx = rand::rng().random_range(0..(pos_rev.len() - 1));
                }
                println!("{:?}", irr.key);
                print_chess(game);
                pos_rev.clear();
            }
            str => {
                (irr, pos_rev) = gen_moves(game.color, game);
                for rev in &pos_rev {
                    let promotion = match rev.flag {
                        Flag::Promotion(piece, _) => Some(piece),
                        _ => None,
                    };
                    if str == move_notation(rev.from, rev.to, promotion).as_str() {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        game.make_move(rev, &irr);

                        print_chess(game);
                        if is_repetition(game) {
                            println!("Repetition of a position");
                        }
                    }
                }
                pos_rev.clear();
            }
        }
    }
}
