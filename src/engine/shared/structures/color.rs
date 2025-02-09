pub type Color = u8;
pub const WHITE: Color = 0;
pub const BLACK: Color = 1;
pub const COLORS: [Color; 2] = [WHITE, BLACK];

pub trait ColorTrait {
    fn is_white(&self) -> bool;
    fn is_black(&self) -> bool;
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
