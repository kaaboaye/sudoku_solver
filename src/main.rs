extern crate nalgebra;

mod board;
mod tile;

use crate::board::Board;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut board = Board::new_from_string(args[1].clone());

    board.apply_constraints().unwrap();

    println!("{}", board);

    let solutions = board.solve();

    println!("SOLUTIONS {}", solutions.len());
    for solution in solutions {
        println!("{}", solution);
    }

    // if args.len() >= 3 {
    //     let control_board = Board::new_from_string(args[2].clone());
    //     println!("{} The same {}", &control_board, board == control_board);
    // }
}
