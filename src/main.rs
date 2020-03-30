extern crate nalgebra;
extern crate stacker;

mod board;
mod tile;

use crate::board::Board;

fn main() {
    let board: String = std::env::args().nth(1).unwrap();
    let mut board = Board::new_from_string(board);

    println!("{}", board);

    board.apply_constraints().unwrap();

    println!("{}", board);

    let board = board.try_solve().unwrap();

    println!("{}", board);
}
