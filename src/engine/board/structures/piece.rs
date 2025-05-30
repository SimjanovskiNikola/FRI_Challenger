use super::color::*;

pub type Piece = u8;

pub const EMPTY: Piece = 0b0000; // 0
pub const PAWN: Piece = 0b0010; // 2
pub const KNIGHT: Piece = 0b0100; // 4
pub const KING: Piece = 0b0110; // 6
pub const BISHOP: Piece = 0b1000; // 8
pub const ROOK: Piece = 0b1010; // 10
pub const QUEEN: Piece = 0b1100; // 12

pub const WHITE_PAWN: Piece = WHITE | PAWN;
pub const WHITE_KNIGHT: Piece = WHITE | KNIGHT;
pub const WHITE_BISHOP: Piece = WHITE | BISHOP;
pub const WHITE_ROOK: Piece = WHITE | ROOK;
pub const WHITE_QUEEN: Piece = WHITE | QUEEN;
pub const WHITE_KING: Piece = WHITE | KING;
pub const BLACK_PAWN: Piece = BLACK | PAWN;
pub const BLACK_KNIGHT: Piece = BLACK | KNIGHT;
pub const BLACK_BISHOP: Piece = BLACK | BISHOP;
pub const BLACK_ROOK: Piece = BLACK | ROOK;
pub const BLACK_QUEEN: Piece = BLACK | QUEEN;
pub const BLACK_KING: Piece = BLACK | KING;

pub const PIECES_WITHOUT_PAWN: [Piece; 5] = [KNIGHT, BISHOP, ROOK, QUEEN, KING];
pub const PROMO_PIECES: [Piece; 4] = [KNIGHT, BISHOP, ROOK, QUEEN];
pub const PIECES: [Piece; 6] = [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING];
pub const CLR_PIECES: [Piece; 12] = [
    WHITE_PAWN,
    WHITE_KNIGHT,
    WHITE_BISHOP,
    WHITE_ROOK,
    WHITE_QUEEN,
    WHITE_KING,
    BLACK_PAWN,
    BLACK_KNIGHT,
    BLACK_BISHOP,
    BLACK_ROOK,
    BLACK_QUEEN,
    BLACK_KING,
];

const KING_WT: isize = 20000;
const QUEEN_WT: isize = 900;
const ROOK_WT: isize = 500;
const BISHOP_WT: isize = 350;
const KNIGHT_WT: isize = 325;
const PAWN_WT: isize = 100;

pub const PIECE_WT: [isize; 6] = [PAWN_WT, KNIGHT_WT, KING_WT, BISHOP_WT, ROOK_WT, QUEEN_WT];

pub trait PieceTrait {
    fn color(&self) -> Color;
    fn kind(&self) -> Piece;

    fn idx(&self) -> usize;

    fn is_pawn(&self) -> bool;
    fn is_knight(&self) -> bool;
    fn is_bishop(&self) -> bool;
    fn is_rook(&self) -> bool;
    fn is_queen(&self) -> bool;
    fn is_king(&self) -> bool;

    fn weight(&self) -> isize;

    fn from_char(c: char) -> Self;
    fn to_char(&self) -> char;
    fn to_figure(&self) -> String;
    fn change_color(&mut self);
}

impl PieceTrait for Piece {
    #[inline(always)]
    fn change_color(&mut self) {
        *self ^= 0b0001;
    }

    #[inline(always)]
    fn color(&self) -> Color {
        *self & 0b0001
    }

    #[inline(always)]
    fn kind(&self) -> Piece {
        *self & 0b1110
    }

    #[inline(always)]
    fn idx(&self) -> usize {
        *self as usize
    }

    #[inline(always)]
    fn is_pawn(&self) -> bool {
        self.kind() == PAWN
    }

    #[inline(always)]
    fn is_knight(&self) -> bool {
        self.kind() == KNIGHT
    }

    #[inline(always)]
    fn is_bishop(&self) -> bool {
        self.kind() == BISHOP
    }

    #[inline(always)]
    fn is_rook(&self) -> bool {
        self.kind() == ROOK
    }

    #[inline(always)]
    fn is_queen(&self) -> bool {
        self.kind() == QUEEN
    }

    #[inline(always)]
    fn is_king(&self) -> bool {
        self.kind() == KING
    }

    #[inline(always)]
    fn weight(&self) -> isize {
        PIECE_WT[(self.kind() / 2).idx() - 1]
    }

    #[inline(always)]
    fn from_char(c: char) -> Piece {
        match c {
            'P' => WHITE_PAWN,
            'N' => WHITE_KNIGHT,
            'B' => WHITE_BISHOP,
            'R' => WHITE_ROOK,
            'Q' => WHITE_QUEEN,
            'K' => WHITE_KING,
            'p' => BLACK_PAWN,
            'n' => BLACK_KNIGHT,
            'b' => BLACK_BISHOP,
            'r' => BLACK_ROOK,
            'q' => BLACK_QUEEN,
            'k' => BLACK_KING,
            _ => EMPTY, // FIXME
        }
    }

    #[inline(always)]
    fn to_char(&self) -> char {
        match *self {
            WHITE_PAWN => 'P',
            WHITE_KNIGHT => 'N',
            WHITE_BISHOP => 'B',
            WHITE_ROOK => 'R',
            WHITE_QUEEN => 'Q',
            WHITE_KING => 'K',
            BLACK_PAWN => 'p',
            BLACK_KNIGHT => 'n',
            BLACK_BISHOP => 'b',
            BLACK_ROOK => 'r',
            BLACK_QUEEN => 'q',
            BLACK_KING => 'k',
            c => panic!("Invalid Character: {c}"),
        }
    }

    #[inline(always)]
    fn to_figure(&self) -> String {
        match *self {
            WHITE_PAWN => "♟".to_string(),
            WHITE_KNIGHT => "♞".to_string(),
            WHITE_BISHOP => "♝".to_string(),
            WHITE_ROOK => "♜".to_string(),
            WHITE_QUEEN => "♛".to_string(),
            WHITE_KING => "♚".to_string(),
            BLACK_PAWN => "♙".to_string(),
            BLACK_KNIGHT => "♘".to_string(),
            BLACK_BISHOP => "♗".to_string(),
            BLACK_ROOK => "♖".to_string(),
            BLACK_QUEEN => "♕".to_string(),
            BLACK_KING => "♔".to_string(),
            EMPTY => ' '.to_string(),
            _ => '?'.to_string(),
        }
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

    #[test]
    fn test_piece_change_color() {
        let mut piece = WHITE_KING;
        piece.change_color();
        assert_eq!(piece, BLACK_KING);
        piece.change_color();
        assert_eq!(piece, WHITE_KING);
    }

    #[test]
    fn test_piece_color() {
        assert_eq!(WHITE_ROOK.color(), WHITE);
        assert_eq!(BLACK_BISHOP.color(), BLACK);
        assert_eq!(BLACK_QUEEN.color(), BLACK);
        assert_eq!(WHITE_PAWN.color(), WHITE);
        assert_eq!(WHITE_KING.color(), WHITE);
        assert_eq!(BLACK_KNIGHT.color(), BLACK);
    }

    #[test]
    fn test_piece_kind() {
        assert_eq!(WHITE_ROOK.kind(), ROOK);
        assert_eq!(BLACK_BISHOP.kind(), BISHOP);
        assert_eq!(BLACK_QUEEN.kind(), QUEEN);
        assert_eq!(WHITE_PAWN.kind(), PAWN);
        assert_eq!(WHITE_KING.kind(), KING);
        assert_eq!(BLACK_KNIGHT.kind(), KNIGHT);
    }

    #[test]
    fn test_piece_idx() {
        assert_eq!(WHITE_ROOK.idx(), 10);
        assert_eq!(BLACK_BISHOP.idx(), 9);
        assert_eq!(BLACK_QUEEN.idx(), 13);
        assert_eq!(WHITE_PAWN.idx(), 2);
        assert_eq!(WHITE_KING.idx(), 6);
        assert_eq!(BLACK_KNIGHT.idx(), 5);
    }

    #[test]
    fn test_piece_is_kind() {
        assert!(!WHITE_PAWN.is_king());
        assert!(WHITE_KNIGHT.is_knight());
        assert!(!WHITE_BISHOP.is_queen());
        assert!(WHITE_ROOK.is_rook());
        assert!(!WHITE_QUEEN.is_rook());
        assert!(WHITE_KING.is_king());
        assert!(BLACK_PAWN.is_pawn());
        assert!(!BLACK_KNIGHT.is_pawn());
        assert!(BLACK_BISHOP.is_bishop());
        assert!(!BLACK_ROOK.is_bishop());
        assert!(BLACK_QUEEN.is_queen());
        assert!(!BLACK_KING.is_rook());
    }
}
