use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Square {
    pub x: u8, // 0‥7  (a‥h)
    pub y: u8, // 0‥7  (1‥8 → 0‥7)
}

impl Square {
    pub fn from_algebraic(file: &str, rank: u8) -> Option<Self> {
        let b = *file.as_bytes().get(0)?;
        let fx = b.checked_sub(b'a')?;
        (fx < 8 && (1..=8).contains(&rank)).then(|| Self { x: fx, y: rank - 1 })
    }

    pub fn to_algebraic(self) -> (String, u8) {
        (String::from_utf8_lossy(&[b'a' + self.x]).into(), self.y + 1)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (file, rank) = self.to_algebraic();
        write!(f, "{file}{rank}")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Debug)]
pub struct Piece {
    piece_type: PieceType,
    square:     Square,
    color:      Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color, file: &str, rank: u8) -> Self {
        let square =
            Square::from_algebraic(file, rank).expect("invalid chess coordinate");
        Self { piece_type, square, color }
    }

    pub fn coord(&self) -> (String, u8) {
        self.square.to_algebraic()
    }
}

pub struct Board {
    pub pieces: [Piece; 32]
}

impl Board {
    /// Creates a fresh board with all 32 pieces in their initial positions.
    pub fn new() -> Self {
        use Color::*;
        use PieceType::*;

        let mk = |pt: PieceType, col: Color, file: char, rank: u8| {
            Piece::new(pt, col, &file.to_string(), rank)
        };

        // Order: white back-rank, white pawns, black back-rank, black pawns
        let pieces = [
            // ── White major pieces ──
            mk(Rook,   White, 'a', 1),
            mk(Knight, White, 'b', 1),
            mk(Bishop, White, 'c', 1),
            mk(Queen,  White, 'd', 1),
            mk(King,   White, 'e', 1),
            mk(Bishop, White, 'f', 1),
            mk(Knight, White, 'g', 1),
            mk(Rook,   White, 'h', 1),

            // ── White pawns ──
            mk(Pawn, White, 'a', 2),
            mk(Pawn, White, 'b', 2),
            mk(Pawn, White, 'c', 2),
            mk(Pawn, White, 'd', 2),
            mk(Pawn, White, 'e', 2),
            mk(Pawn, White, 'f', 2),
            mk(Pawn, White, 'g', 2),
            mk(Pawn, White, 'h', 2),

            // ── Black major pieces ──
            mk(Rook,   Black, 'a', 8),
            mk(Knight, Black, 'b', 8),
            mk(Bishop, Black, 'c', 8),
            mk(Queen,  Black, 'd', 8),
            mk(King,   Black, 'e', 8),
            mk(Bishop, Black, 'f', 8),
            mk(Knight, Black, 'g', 8),
            mk(Rook,   Black, 'h', 8),

            // ── Black pawns ──
            mk(Pawn, Black, 'a', 7),
            mk(Pawn, Black, 'b', 7),
            mk(Pawn, Black, 'c', 7),
            mk(Pawn, Black, 'd', 7),
            mk(Pawn, Black, 'e', 7),
            mk(Pawn, Black, 'f', 7),
            mk(Pawn, Black, 'g', 7),
            mk(Pawn, Black, 'h', 7),
        ];

        Board { pieces }
    }

    pub fn try_a_move(&self, pt: PieceType, col: Color, file: char, rank: u8) -> bool {
        true
    }
}
