pub const DIR_OFFSET: [(i8, i8); 8] =
    [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];
pub const DIRECTIONS: [Dir; 8] = [
    Dir::NORTH,
    Dir::SOUTH,
    Dir::EAST,
    Dir::WEST,
    Dir::NORTHEAST,
    Dir::SOUTHWEST,
    Dir::NORTHWEST,
    Dir::SOUTHEAST,
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    NORTH = 0,     // 0b0000,
    SOUTH = 1,     // 0b0001,
    EAST = 2,      // 0b0010,
    WEST = 3,      // 0b0011,
    NORTHEAST = 4, // 0b0100,
    SOUTHWEST = 5, // 0b0101,
    NORTHWEST = 6, // 0b0110,
    SOUTHEAST = 7, // 0b0111,
}

impl Dir {
    pub fn val(&self) -> u8 {
        return *self as u8;
    }

    pub fn idx(&self) -> usize {
        return *self as usize;
    }

    pub fn dir_offset(&self) -> (i8, i8) {
        return DIR_OFFSET[*self as usize];
    }

    pub fn is_forward(&self) -> bool {
        return self.val() & 0b0001 == 0;
    }

    pub fn is_backward(&self) -> bool {
        return self.val() & 0b0001 != 0;
    }

    pub fn is_orthogonal(&self) -> bool {
        return self.val() & 0b0100 == 0;
    }

    pub fn is_diagonal(&self) -> bool {
        return self.val() & 0b0100 != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_diagonal() {
        assert!(!Dir::NORTH.is_diagonal());
        assert!(!Dir::SOUTH.is_diagonal());
        assert!(!Dir::EAST.is_diagonal());
        assert!(!Dir::WEST.is_diagonal());
        assert!(Dir::NORTHEAST.is_diagonal());
        assert!(Dir::SOUTHWEST.is_diagonal());
        assert!(Dir::NORTHWEST.is_diagonal());
        assert!(Dir::SOUTHEAST.is_diagonal());
    }

    #[test]
    fn test_direction_orthogonal() {
        assert!(Dir::NORTH.is_orthogonal());
        assert!(Dir::SOUTH.is_orthogonal());
        assert!(Dir::EAST.is_orthogonal());
        assert!(Dir::WEST.is_orthogonal());
        assert!(!Dir::NORTHEAST.is_orthogonal());
        assert!(!Dir::SOUTHWEST.is_orthogonal());
        assert!(!Dir::NORTHWEST.is_orthogonal());
        assert!(!Dir::SOUTHEAST.is_orthogonal());
    }

    #[test]
    fn test_direction_backward() {
        assert!(!Dir::NORTH.is_backward());
        assert!(Dir::SOUTH.is_backward());
        assert!(!Dir::EAST.is_backward());
        assert!(Dir::WEST.is_backward());
        assert!(!Dir::NORTHEAST.is_backward());
        assert!(Dir::SOUTHWEST.is_backward());
        assert!(!Dir::NORTHWEST.is_backward());
        assert!(Dir::SOUTHEAST.is_backward());
    }

    #[test]
    fn test_direction_forward() {
        assert!(Dir::NORTH.is_forward());
        assert!(!Dir::SOUTH.is_forward());
        assert!(Dir::EAST.is_forward());
        assert!(!Dir::WEST.is_forward());
        assert!(Dir::NORTHEAST.is_forward());
        assert!(!Dir::SOUTHWEST.is_forward());
        assert!(Dir::NORTHWEST.is_forward());
        assert!(!Dir::SOUTHEAST.is_forward());
    }
}
