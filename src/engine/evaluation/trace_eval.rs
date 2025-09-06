use crate::engine::board::structures::board::Board;
use crate::engine::board::structures::color::{Color, BLACK, WHITE};
use crate::engine::board::structures::piece::{Piece, PieceTrait};
use crate::engine::misc::display::display_board::print_eval;

pub trait TraceEvalTrait {
    // NOTE: TRACE [Debugging purposes]
    fn trace(
        &mut self,
        color: Color,
        square: Option<usize>,
        piece: Option<Piece>,
        value: (isize, isize),
    );
    fn print_trace_board(&mut self, name: &str);
    fn print_trace_log(&mut self, name: &str);
    fn print_trace_score(&mut self, name: &str);
    fn reset_trace(&mut self);
}

impl TraceEvalTrait for Board {
    #[inline(always)]
    fn trace(
        &mut self,
        color: Color,
        square: Option<usize>,
        piece: Option<Piece>,
        value: (isize, isize),
    ) {
        self.eval.vec_test.push(format!(
            "Piece:{:?} ,Color: {:?}, Square: {:?}, Value Mg: {:?}, Value Eg: {:?}",
            piece, color, square, value.0, value.1
        ));

        if let Some(sq) = square {
            self.eval.mg_test[color.idx()][sq] += value.0;
            self.eval.eg_test[color.idx()][sq] += value.1;
        }
    }

    #[inline(always)]
    fn reset_trace(&mut self) {
        self.eval.vec_test.clear();
        self.eval.mg_test = [[0; 64]; 2];
        self.eval.eg_test = [[0; 64]; 2];
    }

    #[inline(always)]
    fn print_trace_board(&mut self, name: &str) {
        let mg_test = self.eval.mg_test.map(|row| row.map(|num| num.to_string()));
        let eg_test = self.eval.eg_test.map(|row| row.map(|num| num.to_string()));

        println!("--------------Print Evaluation Board for: {:?}-------------", name);
        println!("{:?}", "");
        println!("******* Color White, Phase: Middle Game *******");
        print_eval(&mg_test[WHITE.idx()]);
        println!("******* Color Black, Phase: Middle Game *******");
        print_eval(&mg_test[BLACK.idx()]);
        println!("******* Color White, Phase: End Game *******");
        print_eval(&eg_test[WHITE.idx()]);
        println!("******* Color Black, Phase: End Game *******");
        print_eval(&eg_test[BLACK.idx()]);
    }

    #[inline(always)]
    fn print_trace_log(&mut self, name: &str) {
        println!("--------------Print Evaluation Log for: {:?}-------------", name);
        for log in &self.eval.vec_test {
            println!("{:?}", log);
        }
    }

    #[inline(always)]
    fn print_trace_score(&mut self, name: &str) {
        println!("-------------- Print Evaluation Score for: {:?} -------------", name);
        println!("-> Color White, Phase: Mg, Score: {:?} ", self.eval.score[WHITE.idx()].0);
        println!("-> Color Black, Phase: Mg, Score: {:?} ", self.eval.score[BLACK.idx()].0);
        println!("-> Color White, Phase: Eg, Score: {:?} ", self.eval.score[WHITE.idx()].1);
        println!("-> Color Black, Phase: Eg, Score: {:?} ", self.eval.score[BLACK.idx()].1);
    }
}
