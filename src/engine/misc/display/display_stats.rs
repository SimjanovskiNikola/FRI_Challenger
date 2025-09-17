use crate::engine::search::iter_deepening::Search;

pub trait DisplayStatsTrait {
    fn print_info(&self, score: isize, line: String);
    fn print_pruning_info(&self, score: isize);
    fn print_ordering_info(&self, depth: i8);
}

impl DisplayStatsTrait for Search {
    fn print_info(&self, score: isize, line: String) {
        let time = self.uci.start_time.elapsed().as_millis();
        println!(
            "info depth {} nodes {} time {} score cp {} pv{}",
            self.info.curr_depth, self.info.nodes, time, score, line
        );
    }

    fn print_pruning_info(&self, _score: isize) {
        println!(
            "Fail Hard First: {:?}, Fail Hard: {:?}",
            self.info.fail_hard_first, self.info.fail_hard
        );
    }

    fn print_ordering_info(&self, depth: i8) {
        let avg_beta_idx = self.info.beta_cut_index_sum[depth as usize] as f64
            / (self.info.beta_cut_count[depth as usize] + 1) as f64;

        let avg_alpha_idx = self.info.alpha_raise_index_sum[depth as usize] as f64
            / (self.info.alpha_raise_count[depth as usize] + 1) as f64;

        let fhf = self.info.fail_hard_first as f64 / (self.info.fail_hard + 1) as f64;
        println!(
            "Depth: {:?}, Avg Beta Index: {:.4}, Avg Alpha Index: {:.4}, Fail Hard First: {:.4}",
            depth, avg_beta_idx, avg_alpha_idx, fhf
        );
    }
}
