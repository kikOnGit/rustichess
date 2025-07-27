#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Piece {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

impl Piece {
    /// Map every piece to a single UTF-8 character.
    #[inline]
    pub fn to_char(self) -> char {
        use Piece::*;
        match self {
            WhitePawn => 'P',
            WhiteKnight => 'N',
            WhiteBishop => 'B',
            WhiteRook => 'R',
            WhiteQueen => 'Q',
            WhiteKing => 'K',
            BlackPawn => 'p',
            BlackKnight => 'n',
            BlackBishop => 'b',
            BlackRook => 'r',
            BlackQueen => 'q',
            BlackKing => 'k',
        }
    }

    pub fn is_white_piece(self) -> bool {
        use Piece::*;
        match self {
            WhitePawn | WhiteKnight | WhiteBishop | WhiteRook | WhiteQueen | WhiteKing => true,
            BlackPawn | BlackKnight | BlackBishop | BlackRook | BlackQueen | BlackKing => false,
        }
    }

    pub fn is_black_piece(self) -> bool {
        !self.is_white_piece()
    }
}
