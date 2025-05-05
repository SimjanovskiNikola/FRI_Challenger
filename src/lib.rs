pub mod engine {
    pub mod game;

    pub mod attacks {
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
    pub mod fen {
        pub mod fen;
    }
    pub mod move_generation {
        pub mod make_move;
        pub mod mv_gen;
        pub mod perft;
    }
    pub mod evaluation {
        pub mod evaluation;
    }
    pub mod protocols {
        pub mod time;
        pub mod uci;
    }
    pub mod search {
        pub mod searcher;
        pub mod time;
        pub mod transposition_table;
    }
    pub mod shared {
        pub mod helper_func {
            pub mod bit_pos_utility;
            pub mod bitboard;
            pub mod const_utility;
            pub mod generate_key_utility;
            pub mod play_chess_utility;
            pub mod print_utility;
        }
        pub mod structures {
            pub mod castling_struct;
            pub mod color;
            pub mod directions;
            pub mod internal_move;
            pub mod piece;
            pub mod square;
        }
    }
}
