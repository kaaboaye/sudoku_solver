extern crate nalgebra;

mod board;
mod tile;

use crate::board::Board;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let is_csv = match args.get(2) {
        Some(output) if output == "csv" => true,
        _ => false,
    };

    let mut board = Board::new_from_string(args[1].clone()).expect("incorrect sudoku");

    board.apply_constraints().expect("incorrect sudoku");

    if !is_csv {
        println!("{}", board);
    }

    let solution = board.solve();

    if is_csv {
        println!(
            "{},{},{}",
            solution.tried_values,
            solution.field_attempts,
            solution.tried_values - solution.field_attempts
        );
    } else {
        println!(
            "Calculated with {} tried values and {} wrong guesses\n{}",
            solution.tried_values, solution.field_attempts, solution.board
        );
    }
}
