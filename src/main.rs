extern crate nalgebra;

mod tile;

use crate::tile::Tile;
use nalgebra::DVector;

fn main() {
    let _ = DVector::<Tile>::from_vec(vec![]);

    for i in 1..9 {
        dbg!(i);
    }

    println!("Hello, world!");
}
