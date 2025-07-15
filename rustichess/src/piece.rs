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

const ROOK_TABLE: [[usize; 14]; 64] = init_rook_table();
const BISHOP_TABLE: [[usize; 13]; 64] = init_bishop_table();
const QUEEN_TABLE: [[usize; 27]; 64] = init_queen_table();
const KING_TABLE: [[usize; 8]; 64] = init_king_table();
const KNIGHT_TABLE: [[usize; 8]; 64] = init_knight_table();

const QUEEN_AND_KING_DIRS: [(isize, isize); 8] = [
    (0, -1), (0, 1),
    (-1, 0), (1, 0),
    (-1, -1), (-1, 1),
    (1, 1),  (1, -1),
];

const KNIGHT_MOVES: [(isize, isize); 8] = [
    (-2, -1), (-2, 1),
    (-1, -2), (-1, 2),
    (1, -2), (1, 2),
    (2, -1), (2, 1),
];
const fn push_to_direction_limited<const LEN: usize>(
    row: &mut [usize; LEN],
    index: &mut usize,
    square: usize,
    vertical_dir: isize,
    horizontal_dir: isize,
    move_limit: isize,
) {
    let mut file = (square % 8) as isize;
    let mut rank = (square / 8) as isize;
    let mut number_of_moves = 0;
    loop {
        file += vertical_dir;
        rank += horizontal_dir;
        if rank < 0 || rank >= 8 || file < 0 || file >= 8 || number_of_moves >= move_limit {
            break;
        }
        row[*index] = rank as usize * 8 + file as usize;
        *index += 1;
        number_of_moves += 1;
    }
}

const fn push_to_direction<const LEN: usize>(
    row: &mut [usize; LEN],
    index: &mut usize,
    square: usize,
    vertical_dir: isize,
    horizontal_dir: isize,
) {
    push_to_direction_limited(row, index, square, vertical_dir, horizontal_dir, 8)
}
const fn init_rook_table() -> [[usize; 14]; 64] {
    let mut table = [[0usize; 14]; 64];
    let mut square = 0usize;
    while square < 64 {
        let mut index = 0usize;
        let row = &mut table[square];
        push_to_direction(row, &mut index, square, 0, -1); // left
        push_to_direction(row, &mut index, square, 0, 1); // right
        push_to_direction(row, &mut index, square, -1, 0); // down
        push_to_direction(row, &mut index, square, 1, 0); // up
        square += 1;
    }
    table
}

const fn init_bishop_table() -> [[usize; 13]; 64] {
    let mut table = [[0usize; 13]; 64];
    let mut square = 0usize;
    while square < 64 {
        let mut index = 0usize;
        let row = &mut table[square];
        push_to_direction(row, &mut index, square, -1, -1);
        push_to_direction(row, &mut index, square, -1, 1);
        push_to_direction(row, &mut index, square, 1, 1);
        push_to_direction(row, &mut index, square, 1, -1);
        square += 1;
    }
    table
}

const fn init_queen_table() -> [[usize; 27]; 64] {
    let mut table = [[0usize; 27]; 64];
    let mut square = 0usize;
    while square < 64 {
        let mut idx = 0usize;
        let row = &mut table[square];
        let mut d = 0usize;
        while d < QUEEN_AND_KING_DIRS.len() {
            let (v_dir, h_dir) = QUEEN_AND_KING_DIRS[d];
            push_to_direction(row, &mut idx, square, v_dir, h_dir);
            d += 1;
        }
        square += 1;
    }

    table
}

const fn init_king_table() -> [[usize; 8]; 64] {
    let mut table = [[0usize; 8]; 64];
    let mut square = 0usize;
    while square < 64 {
        let mut idx = 0usize;
        let row = &mut table[square];
        let mut d = 0usize;
        while d < QUEEN_AND_KING_DIRS.len() {
            let (v_dir, h_dir) = QUEEN_AND_KING_DIRS[d];
            push_to_direction_limited(row, &mut idx, square, v_dir, h_dir, 1);
            d += 1;
        }
        square += 1;
    }
    table
}

const fn init_knight_table() -> [[usize; 8]; 64] {
    let mut table = [[0usize; 8]; 64];
    let mut square = 0usize;
    while square < 64 {
        let file = (square % 8) as isize;
        let rank = (square / 8) as isize;
        let row = &mut table[square];
        let mut m = 0usize;
        while m < KNIGHT_MOVES.len() {
            let (v_dir, h_dir) = KNIGHT_MOVES[m];
            let file = file + v_dir;
            let rank = rank + h_dir;
            if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
                row[m] = rank as usize * 8 + file as usize;
            }
            m += 1;
        }
        square += 1;   
    }
    table
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

    fn is_white_piece(self) -> bool {
        match self {
            WhitePawn | WhiteKnight | WhiteBishop | WhiteRook | WhiteQueen | WhiteKing => true,
            BlackPawn | BlackKnight | BlackBishop | BlackRook | BlackQueen | BlackKing => false,
        }
    }
    
    fn is_black_piece(self) -> bool {
        !self.is_white_piece()
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
        if (piece_to_move == WhiteRook || piece_to_move == BlackRook) && !ROOK_TABLE[from].contains(&to) {
            return Err(String::from(
                "The rook can only move up or down vertically on any file",
            ));
        }
        if (piece_to_move == WhiteBishop || piece_to_move == BlackBishop)
            && !BISHOP_TABLE[from].contains(&to)
        {
            return Err(String::from("The bishop can only move diagonally"));
        }

        if (piece_to_move == WhiteQueen || piece_to_move == BlackQueen)
            && !QUEEN_TABLE[from].contains(&to)
        {
            return Err(String::from(
                "The queen does almost what she wants, but not quite",
            ));
        }
        if (piece_to_move == WhiteKnight || piece_to_move == BlackKnight)
            && !KNIGHT_TABLE[from].contains(&to)
        {
            return Err(String::from("The knight cannot jump here"));
        }

        if (piece_to_move == WhiteKing || piece_to_move == BlackKing)
            && !KING_TABLE[from].contains(&to)
        {
            return Err(String::from("The king cannot jump here"));
        }
        
        Ok(captured_piece)
    }

    fn find_piece(&self, piece: Piece) -> [usize; 8] {
        let mut  pieces: [usize; 8] = [0; 8];
        let  mut index = 0;
        for  i in 0..64 {
            if self.squares[i].is_some() && piece == self.squares[i].unwrap() {
                pieces[index] = i;
                index += 1;
            }
        }
        pieces
    }
    fn is_king_safe(&self, from: usize, to: usize) -> bool {
        let mut next_board = self.clone();
        next_board.squares[to] = self.squares[from];
        next_board.squares[from] = None;
        let mut attacking_squares = [0usize; 64];
        if next_board.squares[to].unwrap().is_white_piece() {
            let king_square = next_board.find_piece(WhiteKing)[0];
            for i in 0..64 {
                if next_board.squares[i].is_some() {
                    let piece = next_board.squares[i].unwrap();
                    if piece.is_black_piece() {
                        
                    }
                }
            }
            
        }
        true
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