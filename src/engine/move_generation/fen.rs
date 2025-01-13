use crate::engine::{game::Game, shared::structures::piece_struct::Color};

pub trait FenTrait {
    fn change_active_color(&mut self);
}

impl FenTrait for Game {
    fn change_active_color(&mut self) {
        match self.active_color {
            Color::White => self.active_color = Color::Black,
            Color::Black => self.active_color = Color::White,
        }
    }
}
