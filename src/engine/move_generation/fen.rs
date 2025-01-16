use crate::engine::{game::Game, shared::structures::piece_struct::Color};

pub trait FenTrait {
    fn change_active_color(&mut self);
    fn set_en_passant(&mut self, sq: Option<usize>);
}

impl FenTrait for Game {
    fn change_active_color(&mut self) {
        match self.active_color {
            Color::White => self.active_color = Color::Black,
            Color::Black => self.active_color = Color::White,
        }
    }

    fn set_en_passant(&mut self, sq: Option<usize>) {
        // TODO: Add Validation
        match sq {
            Some(pos) => self.en_passant = Some(1 << pos),
            None => self.en_passant = None,
        }
    }
}
