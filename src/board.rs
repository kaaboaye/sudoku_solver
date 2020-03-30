use crate::tile::Tile;
use nalgebra::{MatrixMN, U9};
use std::fmt::Display;

type BoardData = MatrixMN<Tile, U9, U9>;

#[derive(Debug)]
pub struct Board {
  data: BoardData,
}

impl Board {
  pub fn new_from_string(data: String) -> Board {
    let raw: Vec<_> = data
      .chars()
      .map(|c| match c {
        '.' => Tile::new_full_set(),
        d => {
          let value = d.to_digit(10).unwrap();
          let mut tile = Tile::new();
          tile.insert(value as u16);
          tile
        }
      })
      .collect();

    let data = BoardData::from_vec(raw);
    Board { data }
  }

  pub fn apply_constraints(&mut self) {
    for mut row in self.data.row_iter_mut() {
      let values: Vec<_> = row
        .iter()
        .filter(|tile| tile.len() == 1)
        .map(|tile| tile.get_single_value())
        .collect();

      for set in row.iter_mut().filter(|tile| tile.len() != 1) {
        for &value in values.iter() {
          set.remove(value);
        }
      }
    }

    for mut col in self.data.column_iter_mut() {
      let values: Vec<_> = col
        .iter()
        .filter(|tile| tile.len() == 1)
        .map(|tile| tile.get_single_value())
        .collect();

      for set in col.iter_mut().filter(|tile| tile.len() != 1) {
        for &value in values.iter() {
          set.remove(value);
        }
      }
    }

    for i in (0..9).step_by(3) {
      for j in (0..9).step_by(3) {
        let mut slice = self.data.slice_mut((i, j), (3, 3));

        let values: Vec<_> = slice
          .iter()
          .filter(|tile| tile.len() == 1)
          .map(|tile| tile.get_single_value())
          .collect();

        for set in slice.iter_mut().filter(|tile| tile.len() != 1) {
          for &value in values.iter() {
            set.remove(value);
          }
        }
      }
    }
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", &self.data)
  }
}
