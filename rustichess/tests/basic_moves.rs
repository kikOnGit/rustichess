use rustichess::board::Board;
use rustichess::pieces::Piece;

#[test]
fn test_basic_board_setup() {
    let board = Board::set_up();
    assert_eq!(board.squares.iter().filter(|x| x.is_some()).count(), 32);
}

#[test]
fn test_legal_knight_move() {
    let mut board = Board::empty();
    board.squares[0] = Some(Piece::WhiteKnight);

    // Knight moves in L-shape (2,1)
    assert_eq!(board.move_piece(0, 10), Ok(None));

    // Another legal knight move
    board = Board::empty();
    board.squares[36] = Some(Piece::BlackKnight);
    assert_eq!(board.move_piece(36, 21), Ok(None));
}

#[test]
fn test_legal_king_move() {
    let mut board = Board::empty();
    board.squares[4] = Some(Piece::WhiteKing);

    // King moves one square
    assert_eq!(board.move_piece(4, 5), Ok(None)); // Horizontal

    // Setup another test
    board = Board::empty();
    board.squares[60] = Some(Piece::BlackKing);
    assert_eq!(board.move_piece(60, 51), Ok(None)); // Diagonal
}

#[test]
fn test_legal_pawn_move() {
    let mut board = Board::empty();
    board.squares[8] = Some(Piece::WhitePawn);

    // Move one square forward
    assert_eq!(board.move_piece(8, 16), Ok(None));

    // Setup for two square move from starting position
    board = Board::empty();
    board.squares[8] = Some(Piece::WhitePawn);
    assert_eq!(board.move_piece(8, 24), Ok(None)); // Two squares forward from rank 2

    // Setup for pawn capture
    board = Board::empty();
    board.squares[8] = Some(Piece::WhitePawn);
    board.squares[17] = Some(Piece::BlackPawn); // Diagonal to the pawn
    assert_eq!(board.move_piece(8, 17), Ok(Some(Piece::BlackPawn))); // Capture
}

#[test]
fn test_legal_queen_move() {
    let mut board = Board::empty();
    board.squares[3] = Some(Piece::WhiteQueen);

    // Queen moves diagonally
    assert_eq!(board.move_piece(3, 30), Ok(None));

    // Setup for horizontal move
    board = Board::empty();
    board.squares[3] = Some(Piece::WhiteQueen);
    assert_eq!(board.move_piece(3, 7), Ok(None)); // Horizontal
}

#[test]
fn test_legal_rook_move() {
    let mut board = Board::empty();
    board.squares[0] = Some(Piece::WhiteRook);

    // Rook moves vertically
    assert_eq!(board.move_piece(0, 8), Ok(None));

    // Setup for horizontal move
    board = Board::empty();
    board.squares[7] = Some(Piece::WhiteRook);
    assert_eq!(board.move_piece(7, 5), Ok(None)); // Horizontal
}

#[test]
fn test_legal_bishop_move() {
    let mut board = Board::empty();
    board.squares[2] = Some(Piece::WhiteBishop);

    // Bishop moves diagonally
    assert_eq!(board.move_piece(2, 20), Ok(None));

    // Setup for another diagonal move
    board = Board::empty();
    board.squares[61] = Some(Piece::BlackBishop);
    assert_eq!(board.move_piece(61, 52), Ok(None)); // Diagonal
}

#[test]
fn test_piece_capture() {
    let mut board = Board::empty();
    board.squares[0] = Some(Piece::WhiteRook);
    board.squares[8] = Some(Piece::BlackPawn);

    // Rook captures pawn
    assert_eq!(board.move_piece(0, 8), Ok(Some(Piece::BlackPawn)));

    // Setup for queen capture
    board = Board::empty();
    board.squares[3] = Some(Piece::WhiteQueen);
    board.squares[21] = Some(Piece::BlackKnight);
    assert_eq!(board.move_piece(3, 21), Ok(Some(Piece::BlackKnight)));
}
