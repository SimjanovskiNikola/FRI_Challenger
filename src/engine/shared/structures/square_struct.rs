#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SquareType {
    Empty,
    Occupied(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Square {
    pub square_type: SquareType,
}

impl Square {}
