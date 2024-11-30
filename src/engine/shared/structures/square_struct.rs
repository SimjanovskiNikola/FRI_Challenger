/** 
    TODO: Maybe needs rework. How can i determine what peace is that and what is that number ? Should i include id for every peace ?
    Determines if the square is empty or occupied. 
    If it is occupied, it indicated the piece number that is on that particular square. 
*/
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SquareType {
    Empty,
    Occupied(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Square {
    pub square_type: SquareType,
}
