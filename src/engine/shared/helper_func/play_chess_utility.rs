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
use crate::engine::search::searcher::SearchInfo;
use crate::engine::search::transposition_table::get_line;
use crate::engine::shared::helper_func::print_utility::move_notation;
use crate::engine::shared::helper_func::print_utility::print_chess;
use crate::engine::shared::helper_func::print_utility::print_move_list;
use crate::engine::shared::structures::internal_move::Flag;
use crate::engine::shared::structures::internal_move::PositionRev;
use crate::engine::shared::structures::piece::PieceTrait;

pub fn play_chess(game: &mut Game, info: &mut SearchInfo) {
    // let mut move_list: Vec<InternalMove>;
    let mut move_list: Vec<PositionRev>;
    gen_moves(game.color, game);

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
                print_move_list(&get_line(game, game.pos_key));
            }
            "a" => {
                println!("{:#?}", game.tt.table);
            }
            "m" => {
                move_list = gen_moves(game.color, game);
                println!("{:?}", "Please choose a number: ");
                print_move_list(&move_list);
            }
            "u" => {
                if !game.moves.is_empty() {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    game.undo_move();
                    print_chess(game);
                }
            }
            "s" => {
                iterative_deepening(game, info);
                // println!("{:#?}", game.moves.last());
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
                while !game.make_move(&move_list[idx]) {
                    idx = rand::rng().random_range(0..(move_list.len() - 1));
                }
                game.tt.set(move_list[idx].position_key, move_list[idx]);
                println!("{:?}", move_list[idx].position_key);
                print_chess(game);
                move_list.clear();
            }
            str => {
                move_list = gen_moves(game.color, game);
                for mv in &move_list {
                    let promotion = match mv.flag.is_promo() {
                        true => Some(mv.flag.get_promo_piece()),
                        false => None,
                    };
                    if str == move_notation(mv.from.idx(), mv.to.idx(), promotion).as_str() {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        game.make_move(mv);
                        game.tt.set(mv.position_key, *mv);

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
