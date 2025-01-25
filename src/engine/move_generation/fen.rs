use crate::engine::game::Game;

pub trait FenTrait {
    fn set_en_passant(&mut self, sq: Option<usize>);
}

impl FenTrait for Game {
    fn set_en_passant(&mut self, sq: Option<usize>) {
        // TODO: Add Validation
        match sq {
            Some(pos) => self.en_passant = Some(1 << pos),
            None => self.en_passant = None,
        }
    }
}
