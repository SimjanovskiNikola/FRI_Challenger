use bitflags::bitflags;

// TODO: Needs a little reaserch because it does not sum all of this.
bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct CastlingRights: u8 {
        const NONE = 0;
        const WKINGSIDE = 1 << 0;
        const WQUEENSIDE = 1 << 1;
        const BKINGSIDE = 1 << 2;
        const BQUEENSIDE = 1 << 3;
        const ALL = 15;
    }
}

impl CastlingRights {
    pub fn as_u8(&self) -> u8 {
        return self.bits() as u8;
    }

    pub fn as_usize(&self) -> usize {
        return self.bits() as usize;
    }
}

// TODO: Add u4
// NOTE: Remove the Castling Rights struct and add the rights in one u4 integer
