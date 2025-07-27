use crate::pieces::Piece;
use crate::pieces::Piece::*;
use crate::error::MoveError;
use crate::utils::{BISHOP_TABLE, BOARD_SIZE, KING_TABLE, KNIGHT_TABLE, QUEEN_TABLE, ROOK_TABLE};
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone)]
pub struct Board {
    pub squares: [Option<Piece>; BOARD_SIZE],
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

    pub fn pawn_can_attack_move_square(
        &self,
        from_square: usize,
        to_square: usize,
        piece: Piece,
    ) -> bool {
        if piece != BlackPawn && piece != WhitePawn {
            return false;
        }

        if self.squares[to_square].is_none() {
            return false;
        }

        let (rank, file) = crate::utils::square_to_coords(from_square);
        let dir: i32 = if piece == BlackPawn { -1 } else { 1 };

        let rank_i = rank as i32 + dir;
        if !(0..=7).contains(&rank_i) {
            return false; // would go off the board
        }
        let rank_u = rank_i as usize;

        // Left diagonal
        if file > 0 {
            if rank_u * 8 + (file - 1) == to_square {
                return true;
            }
        }

        // Right diagonal
        if file < 7 {
            if rank_u * 8 + (file + 1) == to_square {
                return true;
            }
        }

        false
    }

    /// Generic helper for every sliding piece (bishop, rook, queen).
    fn sliding_piece_can_move_to_square<const N: usize>(
        &self,
        piece: Piece,
        from_square: usize,
        to_square: usize,
        valid_pieces: (Piece, Piece),
        attack_row: &[Option<usize>; N],
    ) -> bool {
        let (white, black) = valid_pieces;

        if piece != white && piece != black {
            return false;
        }

        if !attack_row.contains(&Some(to_square)) {
            return false;
        }

        // Trace the ray; any piece encountered blocks the movement.
        let (from_rank, from_file) = crate::utils::square_to_coords(from_square);
        let (to_rank, to_file) = crate::utils::square_to_coords(to_square);

        let dir_r = (to_rank as isize - from_rank as isize).signum();
        let dir_f = (to_file as isize - from_file as isize).signum();

        let (mut r, mut f) = (from_rank as isize + dir_r, from_file as isize + dir_f);

        // Check if the ray is blocked by a piece
        while (r as usize, f as usize) != (to_rank, to_file) {
            let sq = (r as usize) * 8 + f as usize;
            if self.squares[sq].is_some() {
                return false; // ray is blocked
            }
            r += dir_r;
            f += dir_f;
        }

        self.can_move_to_square(from_square, to_square)
    }

    pub fn can_move_to_square(&self, from_square: usize, to_square: usize) -> bool {
        let piece = self.squares[from_square].unwrap();
        // Check if the destination square has a piece
        if let Some(target_piece) = self.squares[to_square] {
            // If the piece is of the same color, we can't move there
            if target_piece.is_white_piece() == piece.is_white_piece() {
                return false;
            }
            return true;
        }
        true
    }
    pub fn queen_can_move_to_square(
        &self,
        from_square: usize,
        to_square: usize,
        piece: Piece,
    ) -> bool {
        self.sliding_piece_can_move_to_square(
            piece,
            from_square,
            to_square,
            (WhiteQueen, BlackQueen),
            &QUEEN_TABLE[from_square],
        )
    }

    pub fn king_can_move_to_square(
        &self,
        from_square: usize,
        to_square: usize,
        piece: Piece,
    ) -> bool {
        self.sliding_piece_can_move_to_square(
            piece,
            from_square,
            to_square,
            (WhiteKing, BlackKing),
            &KING_TABLE[from_square],
        )
    }

    pub fn bishop_can_move_to_square(
        &self,
        from_square: usize,
        to_square: usize,
        piece: Piece,
    ) -> bool {
        self.sliding_piece_can_move_to_square(
            piece,
            from_square,
            to_square,
            (WhiteBishop, BlackBishop),
            &BISHOP_TABLE[from_square],
        )
    }

    pub fn rook_can_move_to_square(
        &self,
        from_square: usize,
        to_square: usize,
        piece: Piece,
    ) -> bool {
        self.sliding_piece_can_move_to_square(
            piece,
            from_square,
            to_square,
            (WhiteRook, BlackRook),
            &ROOK_TABLE[from_square],
        )
    }

    pub fn knight_can_move_to_square(&self, from_square: usize, to_square: usize, piece: Piece) -> bool {
        if piece != WhiteKnight && piece != BlackKnight {
            return false;
        }
        let row = &KNIGHT_TABLE[from_square];
        row.iter().flatten().any(|&d| d == to_square) && self.can_move_to_square(from_square, to_square)
    }

    fn pawn_can_move_to_square(&self, piece: Piece, from: usize, to: usize) -> bool {
        let (rank_from, file_from) = crate::utils::square_to_coords(from);
        let (rank_to, file_to) = crate::utils::square_to_coords(to);
        let dir: i32 = if piece == BlackPawn { -1 } else { 1 };
        if (piece == WhitePawn && rank_from == 0) || (piece == BlackPawn && rank_from == 7) {
            return false;
        }
        if (rank_from as i32 + dir, file_from) == (rank_to as i32, file_to)
            && self.squares[to].is_none()
        {
            return true;
        }
        if ((rank_from as i32 + 2 * dir, file_from) == (rank_to as i32, file_to))
            && ((piece == WhitePawn && rank_from == 1) || (piece == BlackPawn && rank_from == 6))
            && self.squares[crate::utils::coords_to_square((rank_from as i32 + dir) as usize, file_from)].is_none()
            && self.squares[crate::utils::coords_to_square((rank_from as i32 + 2 * dir) as usize, file_from)].is_none()
        {
            return true;
        }
        self.pawn_can_attack_move_square(from, to, piece)
    }
    //TODO: add castle and en passant
    pub fn move_piece(
        &mut self,
        from_square: usize,
        to_square: usize,
    ) -> Result<Option<Piece>, MoveError> {
 
        let piece = self
            .squares[from_square]
            .ok_or(MoveError::NoPieceOnSource)?;
        
        let is_the_piece_happy = match piece {
            WhiteKnight | BlackKnight =>
                self.knight_can_move_to_square(from_square, to_square, piece)
                    .then_some(())
                    .ok_or(MoveError::IllegalKnightMove),

            WhiteKing | BlackKing =>
                self.king_can_move_to_square(from_square, to_square, piece)
                    .then_some(())
                    .ok_or(MoveError::IllegalKingMove),

            WhitePawn | BlackPawn =>
                self.pawn_can_move_to_square(piece, from_square, to_square)
                    .then_some(())
                    .ok_or(MoveError::IllegalPawnMove),

            WhiteQueen | BlackQueen =>
                self.queen_can_move_to_square(from_square, to_square, piece)
                    .then_some(())
                    .ok_or(MoveError::IllegalQueenMove),

            WhiteRook | BlackRook =>
                self.rook_can_move_to_square(from_square, to_square, piece)
                    .then_some(())
                    .ok_or(MoveError::IllegalRookMove),

            WhiteBishop | BlackBishop =>
                self.bishop_can_move_to_square(from_square, to_square, piece)
                    .then_some(())
                    .ok_or(MoveError::IllegalBishopMove),
        };
        is_the_piece_happy?;
        
        let captured = self.squares[to_square];
        self.squares[to_square]   = Some(piece);
        self.squares[from_square] = None;

        if !self.is_king_safe(piece.is_white_piece()) {
            self.squares[from_square] = Some(piece);
            self.squares[to_square]   = captured;
            return Err(MoveError::KingInCheck);
        }
        
        Ok(captured)
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
    
    fn is_king_safe(&self, is_white_turn: bool) -> bool {
        let king_square;
        if is_white_turn {
            king_square = self.find_piece(WhiteKing)[0];
        } else {
            king_square = self.find_piece(BlackKing)[0];
        }
        for i in 0..BOARD_SIZE {
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
                    if self.queen_can_move_to_square(i, king_square, piece) {
                        return false;
                    }
                    if self.rook_can_move_to_square(i, king_square, piece) {
                        return false;
                    }
                    if self.bishop_can_move_to_square(i, king_square, piece) {
                        return false;
                    }
                    if self.knight_can_move_to_square(i, king_square, piece) {
                        return false;
                    }
                    if self.pawn_can_attack_move_square(i, king_square, piece) {
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
    use super::*;

    #[test]
    fn test_board() {
        let board = Board::set_up();
        assert_eq!(board.squares.iter().filter(|x| x.is_some()).count(), 32);
    }
    #[test]
    fn test_display() {
        let mut board = Board::set_up();
        println!("{board}");
        let _result = board.move_piece(8, 16);
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
