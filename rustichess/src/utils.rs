// Board constants
pub const BOARD_SIZE: usize = 64;
const MAX_KNIGHT_MOVES: usize = 8;
const MAX_QUEEN_MOVES: usize = 27;
const MAX_KING_MOVES: usize = 8;
const MAX_BISHOP_MOVES: usize = 13;
const MAX_ROOK_MOVES: usize = 14;

/// Convert a square index (0-63) to rank (0-7) and file (0-7) coordinates
#[inline]
pub fn square_to_coords(square: usize) -> (usize, usize) {
    let rank = square / 8;
    let file = square % 8;
    (rank, file)
}

/// Convert rank (0-7) and file (0-7) coordinates to a square index (0-63)
#[inline]
pub fn coords_to_square(rank: usize, file: usize) -> usize {
    rank * 8 + file
}

// Attack tables
pub const ROOK_TABLE: [[Option<usize>; MAX_ROOK_MOVES]; BOARD_SIZE] = init_rook_table();
pub const BISHOP_TABLE: [[Option<usize>; MAX_BISHOP_MOVES]; BOARD_SIZE] = init_bishop_table();
pub const QUEEN_TABLE: [[Option<usize>; MAX_QUEEN_MOVES]; BOARD_SIZE] = init_queen_table();
pub const KING_TABLE: [[Option<usize>; MAX_KING_MOVES]; BOARD_SIZE] = init_king_table();
pub const KNIGHT_TABLE: [[Option<usize>; MAX_KNIGHT_MOVES]; BOARD_SIZE] = init_knight_table();

// Pieces directions
pub const QUEEN_AND_KING_DIRS: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
];

pub const KNIGHT_MOVES: [(isize, isize); 8] = [
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
    // We can't use square_to_coords in const functions,
    // so we'll keep the direct calculation here
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
        // We can't use square_to_coords in const functions,
        // so we'll keep the direct calculation here
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


