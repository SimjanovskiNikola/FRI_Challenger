use super::piece::PieceTrait;

pub type Color = u8;
pub const WHITE: Color = 0;
pub const BLACK: Color = 1;
pub const COLORS: [Color; 2] = [WHITE, BLACK];
pub const COLOR_SIGN: [isize; 2] = [1, -1];

pub trait ColorTrait {
    fn is_white(&self) -> bool;
    fn is_black(&self) -> bool;
    fn opp(&self) -> Self;
    fn sign(&self) -> isize;
}

impl ColorTrait for Color {
    #[inline(always)]
    fn is_black(&self) -> bool {
        *self & BLACK != 0
    }

    #[inline(always)]
    fn is_white(&self) -> bool {
        *self & BLACK == 0
    }

    #[inline(always)]
    fn opp(&self) -> Self {
        *self ^ 1
    }

    #[inline(always)]
    fn sign(&self) -> isize {
        COLOR_SIGN[self.idx()]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_color() {
        assert!(WHITE.is_white());
        assert!(!WHITE.is_black());
        assert!(!BLACK.is_white());
        assert!(BLACK.is_black());
    }
}
