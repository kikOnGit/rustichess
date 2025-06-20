use std::cmp::PartialEq;
use crate::piece::Piece::*;
use std::fmt::{self, Display, Formatter};


#[derive(Copy, Clone,PartialEq, Eq, Debug)]
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
            WhitePawn   => 'P',
            WhiteKnight => 'N',
            WhiteBishop => 'B',
            WhiteRook   => 'R',
            WhiteQueen  => 'Q',
            WhiteKing   => 'K',
            BlackPawn   => 'p',
            BlackKnight => 'n',
            BlackBishop => 'b',
            BlackRook   => 'r',
            BlackQueen  => 'q',
            BlackKing   => 'k',
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
                let symbol = self.squares[idx]
                    .map(|p| p.to_char())
                    .unwrap_or('.');
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
    pub fn set_up() -> Board {
        let squares: [Option<Piece>; 64] =
            [Some(WhiteRook), Some(WhiteKnight), Some(WhiteBishop), Some(WhiteQueen), Some(WhiteKing), Some(WhiteBishop), Some(WhiteKnight), Some(WhiteRook),
                Some(WhitePawn), Some(WhitePawn), Some(WhitePawn), Some(WhitePawn), Some(WhitePawn), Some(WhitePawn), Some(WhitePawn), Some(WhitePawn),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                Some(BlackPawn), Some(BlackPawn), Some(BlackPawn), Some(BlackPawn), Some(BlackPawn), Some(BlackPawn), Some(BlackPawn), Some(BlackPawn),
                Some(BlackRook), Some(BlackKnight), Some(BlackBishop), Some(BlackQueen), Some(BlackKing), Some(BlackBishop), Some(BlackKnight), Some(BlackRook)];
        Board { squares }
    }
    
    
    pub fn move_piece(&mut self, from: usize, to: usize) -> Result<Option<Piece>, String> {
        let piece_to_move = match self.squares[from] {
            Some(piece) => piece,
            None => return Err(String::from("No piece to move"))
        };
        if piece_to_move == WhitePawn {
            
        } 
        
        
        let mut board_after_the_move = self.clone();
        
        
        Ok(board_after_the_move.squares[from])
    }
    
    fn is_king_safe(&self, king_index: usize) -> bool {
        let king_square = self.squares[king_index];
        let king_is_safe = king_square.is_some();
        king_is_safe
    }
    
    
}

#[cfg(test)]
mod tests {
    use crate::piece::Board;

    #[test]
    fn test_board() {
        let board = Board::set_up();
        assert_eq!(board.squares.iter().filter(|x| x.is_some()).count(), 32);
    }
    #[test]
    fn test_display() {
        let mut board = Board::set_up();
        println!("{board}");
        board.move_piece(8,16);
        println!("{board}");
    }
}