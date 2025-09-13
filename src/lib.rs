pub mod engine {

    pub mod board {
        pub mod board;
        pub mod castling;
        pub mod color;
        pub mod fen;
        pub mod moves;
        pub mod piece;
        pub mod square;
        pub mod state;
        pub mod zobrist;
    }
    pub mod generated;
    pub mod move_generator {
        pub mod make_move;
        pub mod mv_gen;
        pub mod mv_oredering;
        pub mod perft;
    }
    pub mod attacks {
        pub mod bishop;
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
        pub mod pawn_hash_table;
        pub mod pvs;
        pub mod quiescence;
        pub mod transposition_table;
    }

    pub mod evaluation {
        pub mod common_eval;
        pub mod eval_defs;
        pub mod evaluation;
        pub mod imbalance_eval;
        pub mod init_eval;
        pub mod king_eval;
        pub mod material_eval;
        pub mod mobility_eval;
        pub mod passed_pawn_eval;
        pub mod pawn_eval;
        pub mod piece_eval;
        pub mod psqt_eval;
        pub mod space_eval;
        pub mod tempo_eval;
        pub mod test_evaluation;
        pub mod threats_eval;
        pub mod trace_eval;
    }
    pub mod misc {
        pub mod bit_pos_utility;
        pub mod bitboard;
        pub mod const_utility;
        pub mod directions;
        pub mod display;
        pub mod generate_key_utility;
        pub mod play_chess_utility;
    }
}
