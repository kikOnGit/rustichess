mod board;
mod pieces;
mod utils;
mod error;

fn main() {
    let board = board::Board::set_up();
    println!("Hello, world!");
    println!("{}", board);
}
