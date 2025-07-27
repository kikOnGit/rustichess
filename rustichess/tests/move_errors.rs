use rustichess::board::Board;
use rustichess::pieces::Piece;
use rustichess::error::MoveError;

/// Tests for MoveError::NoPieceOnSource
#[test]
fn test_no_piece_on_source() {
    let mut board = Board::empty();
    let result = board.move_piece(0, 8);
    assert_eq!(result, Err(MoveError::NoPieceOnSource));
}

/// Tests for MoveError::IllegalKnightMove
#[test]
fn test_illegal_knight_move() {
    let mut board = Board::empty();
    board.squares[0] = Some(Piece::WhiteKnight);

    // Knight can't move in a straight line
    let result = board.move_piece(0, 2);
    assert_eq!(result, Err(MoveError::IllegalKnightMove));

    // Knight can't move to invalid positions
    let result = board.move_piece(0, 8); // Straight up (not L-shaped)
    assert_eq!(result, Err(MoveError::IllegalKnightMove));
}

/// Tests for MoveError::IllegalKingMove
#[test]
fn test_illegal_king_move() {
    let mut board = Board::empty();
    board.squares[4] = Some(Piece::WhiteKing);

    // King can only move one square in any direction
    let result = board.move_piece(4, 20); // Too far
    assert_eq!(result, Err(MoveError::IllegalKingMove));
}

/// Tests for MoveError::IllegalPawnMove
#[test]
fn test_illegal_pawn_move() {
    let mut board = Board::empty();

    // White pawn
    board.squares[8] = Some(Piece::WhitePawn);

    // Pawn can't move more than two squares
    let result = board.move_piece(8, 32);
    assert_eq!(result, Err(MoveError::IllegalPawnMove));

    // Pawn can't move diagonally without capturing
    let result = board.move_piece(8, 17); // Diagonal without capture
    assert_eq!(result, Err(MoveError::IllegalPawnMove));

    // Black pawn
    board = Board::empty();
    board.squares[48] = Some(Piece::BlackPawn);

    // Black pawn can't move more than two squares
    let result = board.move_piece(48, 24); // Three squares forward
    assert_eq!(result, Err(MoveError::IllegalPawnMove));
}

/// Tests for MoveError::IllegalQueenMove
#[test]
fn test_illegal_queen_move() {
    let mut board = Board::empty();
    board.squares[3] = Some(Piece::WhiteQueen);

    // Queen can't jump over pieces
    board.squares[12] = Some(Piece::WhitePawn); // Blocking the queen's path
    let result = board.move_piece(3, 21); // Try to move past the pawn
    assert_eq!(result, Err(MoveError::IllegalQueenMove));

    // Queen can't move in invalid patterns (knight-like)
    let result = board.move_piece(3, 13); // Knight-like move
    assert_eq!(result, Err(MoveError::IllegalQueenMove));
}

/// Tests for MoveError::IllegalRookMove
#[test]
fn test_illegal_rook_move() {
    let mut board = Board::empty();
    board.squares[0] = Some(Piece::WhiteRook);

    // Rook can't jump over pieces
    board.squares[8] = Some(Piece::WhitePawn); // Blocking the rook's path
    let result = board.move_piece(0, 16); // Try to move past the pawn
    assert_eq!(result, Err(MoveError::IllegalRookMove));

    // Rook can't move diagonally
    let result = board.move_piece(0, 9); // Diagonal move
    assert_eq!(result, Err(MoveError::IllegalRookMove));
}

/// Tests for MoveError::IllegalBishopMove
#[test]
fn test_illegal_bishop_move() {
    let mut board = Board::empty();
    board.squares[2] = Some(Piece::WhiteBishop);

    // Bishop can't jump over pieces
    board.squares[11] = Some(Piece::WhitePawn); // Blocking the bishop's path
    let result = board.move_piece(2, 20); // Try to move past the pawn
    assert_eq!(result, Err(MoveError::IllegalBishopMove));

    // Bishop can't move in straight lines
    let result = board.move_piece(2, 3); // Horizontal move
    assert_eq!(result, Err(MoveError::IllegalBishopMove));
}

/// Tests for MoveError::KingInCheck
#[test]
fn test_king_in_check() {
    let mut board = Board::empty();
    board.squares[4] = Some(Piece::WhiteKing);   // e1
    board.squares[13] = Some(Piece::WhitePawn);  // f2
    board.squares[20] = Some(Piece::BlackQueen); // g3 - queen creating check

    // Can't move a piece that block the check
    let result = board.move_piece(13, 21); // Move pawn, doesn't block check
    assert_eq!(result, Err(MoveError::KingInCheck));

    // Different scenario - can't move king into check
    board = Board::empty();
    board.squares[4] = Some(Piece::WhiteKing);  // e1
    board.squares[14] = Some(Piece::BlackQueen); // g2 - controls the g-file

    // Can't move king into line of attack
    let result = board.move_piece(4, 5); // Move king to g1, into check
    assert_eq!(result, Err(MoveError::KingInCheck));
}

/// Test all MoveError variants in a single test
#[test]
fn test_all_move_errors() {
    // Test each error type with pattern matching

    // NoPieceOnSource
    let mut board = Board::empty();
    match board.move_piece(0, 8) {
        Err(MoveError::NoPieceOnSource) => {},
        _ => panic!("Expected NoPieceOnSource error")
    }

    // IllegalKnightMove
    board.squares[0] = Some(Piece::WhiteKnight);
    match board.move_piece(0, 2) {
        Err(MoveError::IllegalKnightMove) => {},
        _ => panic!("Expected IllegalKnightMove error")
    }

    // IllegalKingMove
    board.squares[4] = Some(Piece::WhiteKing);
    match board.move_piece(4, 20) {
        Err(MoveError::IllegalKingMove) => {},
        _ => panic!("Expected IllegalKingMove error")
    }

    // IllegalPawnMove
    board.squares[8] = Some(Piece::WhitePawn);
    match board.move_piece(8, 32) {
        Err(MoveError::IllegalPawnMove) => {},
        _ => panic!("Expected IllegalPawnMove error")
    }

    // IllegalQueenMove
    board.squares[3] = Some(Piece::WhiteQueen);
    board.squares[12] = Some(Piece::WhitePawn); // Blocking the queen's path
    match board.move_piece(3, 21) {
        Err(MoveError::IllegalQueenMove) => {},
        _ => panic!("Expected IllegalQueenMove error")
    }

    // IllegalRookMove
    board = Board::empty();
    board.squares[0] = Some(Piece::WhiteRook);
    board.squares[8] = Some(Piece::WhitePawn); // Blocking the rook's path
    match board.move_piece(0, 16) {
        Err(MoveError::IllegalRookMove) => {},
        _ => panic!("Expected IllegalRookMove error")
    }

    // IllegalBishopMove
    board = Board::empty();
    board.squares[2] = Some(Piece::WhiteBishop);
    board.squares[11] = Some(Piece::WhitePawn); // Blocking the bishop's path
    match board.move_piece(2, 20) {
        Err(MoveError::IllegalBishopMove) => {},
        _ => panic!("Expected IllegalBishopMove error")
    }

    // KingInCheck
    board = Board::empty();
    board.squares[4] = Some(Piece::WhiteKing);   // e1
    board.squares[12] = Some(Piece::WhitePawn);  // e2 - pawn blocking check
    board.squares[60] = Some(Piece::BlackQueen); // e8 - queen creating check
    board.squares[21] = Some(Piece::BlackPawn);  // e7 - pawn to be taken
    match board.move_piece(12, 21) {
        Err(MoveError::KingInCheck) => {},
        _ => panic!("Expected KingInCheck error")
    }
}
