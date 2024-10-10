#[derive(PartialEq, Eq)]
pub enum SquareType {
    Empty,
    Occupied(usize),
}

pub struct Square {
    pub square_type: SquareType,
}

impl Square {}
