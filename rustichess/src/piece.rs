use crate::piece::Piece::*;
use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

// Board constants
const BOARD_SIZE: usize      = 64;
const MAX_KNIGHT_MOVES: usize = 8;
const MAX_QUEEN_MOVES: usize = 27;
const MAX_KING_MOVES: usize  = 8;
const MAX_BISHOP_MOVES: usize = 13;
const MAX_ROOK_MOVES: usize = 14;

// Attack tables
const ROOK_TABLE: [[Option<usize>; MAX_ROOK_MOVES]; BOARD_SIZE] = init_rook_table();
const BISHOP_TABLE: [[Option<usize>; MAX_BISHOP_MOVES]; BOARD_SIZE] = init_bishop_table();
const QUEEN_TABLE: [[Option<usize>; MAX_QUEEN_MOVES]; BOARD_SIZE] = init_queen_table();
const KING_TABLE: [[Option<usize>; MAX_KING_MOVES]; BOARD_SIZE] = init_king_table();
const KNIGHT_TABLE: [[Option<usize>; MAX_KNIGHT_MOVES]; BOARD_SIZE] = init_knight_table();

// Pieces directions
const QUEEN_AND_KING_DIRS: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
];

const KNIGHT_MOVES: [(isize, isize); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

// Function to initialize attack tables
const fn push_to_direction_limited<const LEN: usize>(
    row: &mut [Option<usize>; LEN],
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
        row[*index] = Some(rank as usize * 8 + file as usize);
        *index += 1;
        number_of_moves += 1;
    }
}

const fn push_to_direction<const LEN: usize>(
    row: &mut [Option<usize>; LEN],
    index: &mut usize,
    square: usize,
    vertical_dir: isize,
    horizontal_dir: isize,
) {
    push_to_direction_limited(row, index, square, vertical_dir, horizontal_dir, 8)
}
const fn init_rook_table() -> [[Option<usize>; 14]; BOARD_SIZE] {
    let mut table = [[None; 14]; BOARD_SIZE];
    let mut square = 0usize;
    while square < BOARD_SIZE {
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

const fn init_bishop_table() -> [[Option<usize>; 13]; BOARD_SIZE] {
    let mut table = [[None; 13]; BOARD_SIZE];
    let mut square = 0usize;
    while square < BOARD_SIZE {
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

const fn init_queen_table() -> [[Option<usize>; 27]; BOARD_SIZE] {
    let mut table = [[None; 27]; BOARD_SIZE];
    let mut square = 0usize;
    while square < BOARD_SIZE {
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

const fn init_king_table() -> [[Option<usize>; 8]; BOARD_SIZE] {
    let mut table = [[None; 8]; BOARD_SIZE];
    let mut square = 0usize;
    while square < BOARD_SIZE {
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

const fn init_knight_table() -> [[Option<usize>; 8]; BOARD_SIZE] {
    let mut table = [[None; 8]; BOARD_SIZE];
    let mut square = 0usize;
    while square < BOARD_SIZE {
        let file = (square % 8) as isize;
        let rank = (square / 8) as isize;
        let row = &mut table[square];
        let mut m = 0usize;
        while m < KNIGHT_MOVES.len() {
            let (v_dir, h_dir) = KNIGHT_MOVES[m];
            let file = file + v_dir;
            let rank = rank + h_dir;
            if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
                row[m] = Some(rank as usize * 8 + file as usize);
            }
            m += 1;
        }
        square += 1;
    }
    table
}

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

pub fn pawn_attacks_king(rank: usize, file: usize, king_square: usize, piece: Piece) -> bool {
    if piece != BlackPawn && piece != WhitePawn {
        return false;
    }
    
    let dir: i32 = if piece == BlackPawn { -1 } else { 1 };
    
    let rank_i = rank as i32 + dir;
    if !(0..=7).contains(&rank_i) {
        return false; // would go off the board
    }
    let rank_u = rank_i as usize;

    // Left diagonal
    if file > 0 {
        if rank_u * 8 + (file - 1) == king_square {
            return true;
        }
    }

    // Right diagonal
    if file < 7 {
        if rank_u * 8 + (file + 1) == king_square {
            return true;
        }
    }

    false
}

pub fn knight_attacks_king(rank: usize, file: usize, king_square: usize, piece: Piece) -> bool {
    if piece != WhiteKnight && piece != BlackKnight {
        return false;
    }
    let row = &KNIGHT_TABLE[rank * 8 + file];
    row.iter().flatten().any(|&d| d == king_square)
}

/// Generic helper for every sliding piece (bishop, rook, queen).
fn sliding_piece_attacks_king<const N: usize>(
    piece: Piece,
    rank: usize,
    file: usize,
    king_square: usize,
    defended_square: [Option<usize>; 16],
    // two valid variants of the piece (white / black)
    valid_pieces: (Piece, Piece),
    // one row of the appropriate ATTACK_TABLE
    attack_row: &[Option<usize>; N],
) -> bool {
    let (white, black) = valid_pieces;
    
    if piece != white && piece != black {
        return false;
    }
    
    if !attack_row.contains(&Some(king_square)) {
        return false;
    }

    // 3. Trace the ray; any defended square encountered blocks the check.
    let king_rank = king_square / 8;
    let king_file = king_square % 8;

    let dir_r = (king_rank as isize - rank as isize).signum();
    let dir_f = (king_file as isize - file as isize).signum();

    let (mut r, mut f) = (rank as isize + dir_r, file as isize + dir_f);

    while (r as usize, f as usize) != (king_rank, king_file) {
        let sq = (r as usize) * 8 + f as usize;
        if defended_square.iter().flatten().any(|&d| d == sq) {
            return false; // ray is blocked
        }
        r += dir_r;
        f += dir_f;
    }
    true // nothing blocked us – the king is in check
}

pub fn queen_attacks_king(
    rank: usize,
    file: usize,
    king_square: usize,
    defended_square: [Option<usize>; 16],
    piece: Piece,
) -> bool {
    sliding_piece_attacks_king(
        piece,
        rank,
        file,
        king_square,
        defended_square,
        (WhiteQueen, BlackQueen),
        &QUEEN_TABLE[rank * 8 + file],
    )
}

pub fn bishop_attacks_king(
    rank: usize,
    file: usize,
    king_square: usize,
    defended_square: [Option<usize>; 16],
    piece: Piece,
) -> bool {
    sliding_piece_attacks_king(
        piece,
        rank,
        file,
        king_square,
        defended_square,
        (WhiteBishop, BlackBishop),
        &BISHOP_TABLE[rank * 8 + file],
    )
}

pub fn rook_attacks_king(
    rank: usize,
    file: usize,
    king_square: usize,
    defended_square: [Option<usize>; 16],
    piece: Piece,
) -> bool {
    sliding_piece_attacks_king(
        piece,
        rank,
        file,
        king_square,
        defended_square,
        (WhiteRook, BlackRook),
        &ROOK_TABLE[rank * 8 + file],
    )
}

#[derive(Copy, Clone)]
pub struct Board {
    squares: [Option<Piece>; BOARD_SIZE],
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
            squares: [None; BOARD_SIZE],
        }
    }
    pub fn set_up() -> Board {
        let squares: [Option<Piece>; BOARD_SIZE] = [
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

    fn collect_piece_squares<F>(&self, mut pred: F) -> [Option<usize>; 16]
    where
        F: FnMut(Piece) -> bool,
    {
        let mut squares = [None; 16];
        let mut idx = 0;

        for (i, piece_opt) in self.squares.iter().enumerate() {
            if let Some(piece) = piece_opt {
                if pred(*piece) {
                    squares[idx] = Some(i);
                    idx += 1;
                    if idx == 16 {
                        break; // array is full
                    }
                }
            }
        }
        squares
    }
    
    pub fn get_white_pieces_squares_without_king(&self) -> [Option<usize>; 16] {
        self.collect_piece_squares(|p| p.is_white_piece() && p != WhiteKing)
    }

    pub fn get_black_pieces_squares_without_king(&self) -> [Option<usize>; 16] {
        self.collect_piece_squares(|p| p.is_black_piece() && p != BlackKing)
    }
    //TODO: add collision with same color pieces
    pub fn move_piece(&mut self, from: usize, to: usize) -> Result<Option<Piece>, String> {
        let piece_to_move = match self.squares[from] {
            Some(piece) => piece,
            None => return Err(String::from("No piece to move")),
        };
        let captured_piece = self.squares[to];
        if (piece_to_move == WhiteRook || piece_to_move == BlackRook)
            && !ROOK_TABLE[from].contains(&Some(to))
        {
            return Err(String::from(
                "The rook can only move up or down vertically on any file",
            ));
        }
        if (piece_to_move == WhiteBishop || piece_to_move == BlackBishop)
            && !BISHOP_TABLE[from].contains(&Some(to))
        {
            return Err(String::from("The bishop can only move diagonally"));
        }

        if (piece_to_move == WhiteQueen || piece_to_move == BlackQueen)
            && !QUEEN_TABLE[from].contains(&Some(to))
        {
            return Err(String::from(
                "The queen does almost what she wants, but not quite",
            ));
        }
        if (piece_to_move == WhiteKnight || piece_to_move == BlackKnight)
            && !KNIGHT_TABLE[from].contains(&Some(to))
        {
            return Err(String::from("The knight cannot jump here"));
        }

        if (piece_to_move == WhiteKing || piece_to_move == BlackKing)
            && !KING_TABLE[from].contains(&Some(to))
        {
            return Err(String::from("The king cannot jump here"));
        }
        self.squares[from] = None;
        self.squares[to] = Some(piece_to_move);
        if !self.is_king_safe(piece_to_move.is_white_piece()) {
            return Err(String::from("The king is not safe"));
        }
        Ok(captured_piece)
    }

    fn find_piece(&self, piece: Piece) -> [usize; 8] {
        let mut pieces: [usize; 8] = [0; 8];
        let mut index = 0;
        for i in 0..BOARD_SIZE {
            if self.squares[i].is_some() && piece == self.squares[i].unwrap() {
                pieces[index] = i;
                index += 1;
            }
        }
        pieces
    }
    //TODO: take care of pawn attack move
    fn pawn_move_is_valid(&self,piece: Piece, from: usize, to: usize) -> bool {
        let (rank_from, file_from) = (from / 8, from % 8);
        let (rank_to, file_to) = (to / 8, to % 8);
        let dir: i32 = if piece == BlackPawn { -1 } else { 1 };
        if (piece == WhitePawn && rank_from == 0) || (piece == BlackPawn && rank_from == 7) {
            return false;
        }
        if self.squares[to].is_some() {
            return false;
        }
        if (rank_from as i32 + dir, file_from) == (rank_to as i32, file_to) {
            return true;
        }
        if (rank_from as i32 + 2 * dir, file_from) == (rank_to as i32, file_to) 
            && (piece == WhitePawn && rank_from == 1) || (piece == BlackPawn && rank_from == 6) 
            && (self.squares[(rank_from as i32 + dir) as usize * 8 + file_from].is_none()){
            return true;
        }
        if piece == WhitePawn && file_to == file_from {}
        true
    }
    fn is_king_safe(&self, is_white_turn: bool) -> bool {
        let king_square;
        let defended_squares;
        if is_white_turn {
            king_square = self.find_piece(WhiteKing)[0];
            defended_squares = self.get_white_pieces_squares_without_king();
        } else {
            king_square = self.find_piece(BlackKing)[0];
            defended_squares = self.get_black_pieces_squares_without_king()
        }
        for i in 0..BOARD_SIZE {
            let rank = i / 8;
            let file = i % 8;
            if self.squares[i].is_some()
                && self.squares[i].unwrap() != WhiteKing
                && self.squares[i].unwrap() != BlackKing
            {
                let piece = self.squares[i].unwrap();
                // Keep only the enemy pieces.
                let is_enemy = if is_white_turn {
                    piece.is_black_piece()
                } else {
                    piece.is_white_piece()
                };
                if is_enemy {
                    if queen_attacks_king(rank, file, king_square, defended_squares, piece) {
                        return false;
                    }
                    if rook_attacks_king(rank, file, king_square, defended_squares, piece) {
                        return false;
                    }
                    if bishop_attacks_king(rank, file, king_square, defended_squares, piece) {
                        return false;
                    }
                    if knight_attacks_king(rank, file, king_square, piece) {
                        return false;
                    }
                    if pawn_attacks_king(rank, file, king_square, piece) {
                        return false;
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
        let result = board.move_piece(8, 16);
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