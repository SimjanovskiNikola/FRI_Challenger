pub mod engine {
    pub mod game;
    pub mod attacks {
        pub mod all_attacks;
        pub mod pawn_attacks;
    }
    pub mod move_generation {
        pub mod fen;
        pub mod make_move;
        pub mod move_generation;
        pub mod perft;
    }
    pub mod shared {
        pub mod helper_func {
            pub mod bit_pos_utility;
            pub mod bitboard;
            pub mod const_utility;
            pub mod error_msg;
            pub mod generate_key_utility;
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
