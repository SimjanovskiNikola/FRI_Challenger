pub mod engine {

    pub mod board {
        pub mod fen;
        pub mod make_move;
        pub mod mv_gen;
        pub mod perft;
        pub mod structures;
    }
    pub mod move_generator {
        pub mod bishop;
        pub mod generated;
        pub mod king;
        pub mod knight;
        pub mod pawn;
        pub mod queen;
        pub mod rays;
        pub mod rook;
        pub mod utility;
    }
    pub mod protocols {
        pub mod time;
        pub mod uci;
    }

    pub mod search {
        pub mod alpha_beta;
        pub mod iter_deepening;
        pub mod pvs;
        pub mod quiescence;
        pub mod transposition_table;
    }

    pub mod evaluation {
        pub mod eval_defs;
        pub mod evaluation;
        pub mod new_evaluation;
    }
    pub mod misc {
        pub mod bit_pos_utility;
        pub mod bitboard;
        pub mod const_utility;
        pub mod directions;
        pub mod generate_key_utility;
        pub mod play_chess_utility;
        pub mod print_utility;
    }
}
