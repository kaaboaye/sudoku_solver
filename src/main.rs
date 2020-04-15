extern crate nalgebra;

mod board;
mod tile;

use crate::board::Board;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut board = Board::new_from_string(args[1].clone());

    board.apply_constraints().unwrap();

    println!("{}", board);

    let (solution, iterations) = board.solve();
    println!("Calculated in {} iterations {}", iterations, solution);
}
