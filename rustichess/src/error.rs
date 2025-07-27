use std::fmt;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MoveError {
    NoPieceOnSource,
    IllegalKnightMove,
    IllegalKingMove,
    IllegalPawnMove,
    IllegalQueenMove,
    IllegalRookMove,
    IllegalBishopMove,
    KingInCheck,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            MoveError::NoPieceOnSource   => "first, select a piece to move ...",
            MoveError::IllegalKnightMove => "the knight horse cannot jump here",
            MoveError::IllegalKingMove   => "this square lacks of prestige for the king",
            MoveError::IllegalPawnMove   => "just a pawn, not enough will to go there",
            MoveError::IllegalQueenMove  => "the queen does almost what she wants, but not quite",
            MoveError::IllegalRookMove   => "the rook cannot fly here",
            MoveError::IllegalBishopMove => "the bishop does not have enough faith to go there",
            MoveError::KingInCheck       => "the king does not feel safe now"
        };
        write!(f, "{msg}")
    }
}

impl std::error::Error for MoveError {}
