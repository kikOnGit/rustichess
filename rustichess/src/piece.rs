use crate::piece::Piece::*;
use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

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
    fn to_char(self) -> char {
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
}

#[derive(Copy, Clone)]
pub struct Board {
    squares: [Option<Piece>; 64],
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Ranks are printed 8 → 1 so that rank 8 is on top
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?; // left rank label
            for file in 0..8 {
                let idx = rank * 8 + file;
                let symbol = self.squares[idx].map(|p| p.to_char()).unwrap_or('.');
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?; // newline after each rank
        }
        // bottom file labels
        writeln!(f, "  a b c d e f g h")?;
        Ok(())
    }
}

impl Board {
    pub fn empty() -> Board {
        Board {
            squares: [None; 64],
        }
    }
    pub fn set_up() -> Board {
        let squares: [Option<Piece>; 64] = [
            Some(WhiteRook),
            Some(WhiteKnight),
            Some(WhiteBishop),
            Some(WhiteQueen),
            Some(WhiteKing),
            Some(WhiteBishop),
            Some(WhiteKnight),
            Some(WhiteRook),
            Some(WhitePawn),
            Some(WhitePawn),
            Some(WhitePawn),
            Some(WhitePawn),
            Some(WhitePawn),
            Some(WhitePawn),
            Some(WhitePawn),
            Some(WhitePawn),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(BlackPawn),
            Some(BlackPawn),
            Some(BlackPawn),
            Some(BlackPawn),
            Some(BlackPawn),
            Some(BlackPawn),
            Some(BlackPawn),
            Some(BlackPawn),
            Some(BlackRook),
            Some(BlackKnight),
            Some(BlackBishop),
            Some(BlackQueen),
            Some(BlackKing),
            Some(BlackBishop),
            Some(BlackKnight),
            Some(BlackRook),
        ];
        Board { squares }
    }

    //TODO: add collision detection and check if the king is safe
    pub fn move_piece(&mut self, from: usize, to: usize) -> Result<Option<Piece>, String> {
        let piece_to_move = match self.squares[from] {
            Some(piece) => piece,
            None => return Err(String::from("No piece to move")),
        };
        let captured_piece = self.squares[to];
        if (piece_to_move == WhiteRook || piece_to_move == BlackRook)
            && !can_move_like_a_rook(from, to)
        {
            return Err(String::from(
                "The rook can only move up or down vertically on any file",
            ));
        }
        if (piece_to_move == WhiteBishop || piece_to_move == BlackBishop)
            && !can_move_like_a_bishop(from, to)
        {
            return Err(String::from("The bishop can only move diagonally"));
        }

        if (piece_to_move == WhiteQueen || piece_to_move == BlackQueen)
            && !(can_move_like_a_bishop(from, to) || can_move_like_a_rook(from, to))
        {
            return Err(String::from(
                "The queen does almost what she wants, but not quite",
            ));
        }

        if (piece_to_move == WhiteKnight || piece_to_move == BlackKnight)
            && !can_move_like_a_knight(from, to)
        {
            return Err(String::from("The knight cannot jump here"));
        }

        if piece_to_move == WhiteKing || piece_to_move == BlackKing {
            
        }
        
        fn can_move_like_a_rook(from: usize, to: usize) -> bool {
            (from / 8 == to / 8) || (from % 8 == to % 8)
        }

        fn can_move_like_a_bishop(from: usize, to: usize) -> bool {
            let ranks_dif = (from / 8).abs_diff(to / 8);
            let file_dif = (from % 8).abs_diff(to % 8);
            ranks_dif == file_dif
        }

        fn can_move_like_a_knight(from: usize, to: usize) -> bool {
            let ranks_dif = (from / 8).abs_diff(to / 8);
            let file_dif = (from % 8).abs_diff(to % 8);
            (ranks_dif == 2 && file_dif == 1) || (ranks_dif == 1 && file_dif == 2)
        }
        Ok(captured_piece)
    }

    fn is_king_safe(&self, king_index: usize) -> bool {
        let king_square = self.squares[king_index];
        let king_is_safe = king_square.is_some();
        king_is_safe
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::{Board, Piece};

    #[test]
    fn test_board() {
        let board = Board::set_up();
        assert_eq!(board.squares.iter().filter(|x| x.is_some()).count(), 32);
    }
    #[test]
    fn test_display() {
        let mut board = Board::set_up();
        println!("{board}");
        board.move_piece(8, 16);
        println!("{board}");
    }
    #[test]
    fn test_move_rook() {
        // a1 -> a2 legal move
        let mut board = Board::empty();
        board.squares[0] = Some(Piece::WhiteRook);
        assert_eq!(board.move_piece(0, 8), Ok(None));
    }

    #[test]
    fn test_move_rook_wrong_direction() {
        // a1 -> b2 illegal move
        let mut board = Board::empty();
        board.squares[0] = Some(Piece::WhiteRook);
        assert!(board.move_piece(0, 9).is_err());
    }

    #[test]
    fn test_move_bishop() {
        // c1 -> e3 legal move
        let mut board = Board::empty();
        board.squares[2] = Some(Piece::WhiteBishop);

        assert_eq!(board.move_piece(2, 20), Ok(None));
    }

    #[test]
    fn test_move_bishop_wrong_direction() {
        // c1 -> d1 illegal move
        let mut board = Board::empty();
        board.squares[2] = Some(Piece::WhiteBishop);
        assert!(board.move_piece(2, 3).is_err());
    }
}
