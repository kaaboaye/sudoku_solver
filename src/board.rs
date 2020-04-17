use crate::tile::Tile;
use nalgebra::{MatrixMN, U9};
use std::fmt::Display;

type BoardData = MatrixMN<Tile, U9, U9>;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
  data: BoardData,
}

impl Board {
  pub fn new_from_string(data: String) -> Result<Board, ()> {
    let raw: Vec<_> = data
      .chars()
      .map(|c| match c {
        '.' => Tile::new_full(),
        d => {
          let value = d.to_digit(10).unwrap();
          let mut tile = Tile::new();
          tile.insert(value as u16);
          tile
        }
      })
      .collect();

    if raw.len() != 81 {
      Err(())?
    }

    let data = BoardData::from_vec(raw);
    Ok(Board { data })
  }

  pub fn apply_constraints(&mut self) -> Result<(), ()> {
    apply_constraints(&mut self.data)
  }

  pub fn solve(self) -> (Board, usize) {
    let mut versions = Vec::<(BoardData, usize, usize)>::new();
    let mut data = self.data;
    let mut skip_tiles = 0 as usize;
    let mut skip_values = 0 as usize;
    let mut iterations = 0 as usize;

    loop {
      iterations += 1;

      let tile = data
        .iter()
        .enumerate()
        .skip(skip_tiles)
        .find(|(_idx, tile)| tile.len() > 1);

      match tile {
        None => {
          // Check if it's a valid solution
          let tile = data.iter().take(skip_tiles).find(|tile| tile.len() > 1);
          if let None = tile {
            return (Board { data }, iterations);
          }

          if let Some((new_data, new_skip_tiles, new_skip_values)) = versions.pop() {
            data = new_data;
            skip_tiles = new_skip_tiles;
            skip_values = new_skip_values + 1;
            continue;
          }
        }
        Some((idx, _tile)) => {
          let mut data_candidate = data.clone();
          let tile = data_candidate.iter_mut().nth(idx).unwrap();

          match tile.iter().skip(skip_values).next() {
            // try next tile
            None => {
              skip_tiles = idx + 1;
              skip_values = 0;
              continue;
            }
            Some(value) => {
              *tile = Tile::new();
              tile.insert(value);
            }
          };

          match apply_constraints(&mut data_candidate) {
            // Reject candidate
            // and try next value in tile, if there is no value next tile will be selected
            Err(()) => {
              skip_values += 1;
              continue;
            }

            // Choose candidate
            Ok(()) => {
              versions.push((data.clone(), skip_tiles, skip_values));
              data = data_candidate;
              skip_tiles = 0;
              skip_values = 0;
              continue;
            }
          }
        }
      }
    }
  }
}

#[inline]
fn apply_constraints(data: &mut BoardData) -> Result<(), ()> {
  let previous_data = data.clone();
  do_apply_constraints(data, previous_data)
}

fn do_apply_constraints(data: &mut BoardData, previous_data: BoardData) -> Result<(), ()> {
  // apply rows
  for mut row in data.row_iter_mut() {
    let values = values_from_slice(row.iter());
    remove_invalid_values(row.iter_mut(), &values)?;
  }

  // apply cols
  for mut col in data.column_iter_mut() {
    let values = values_from_slice(col.iter());
    remove_invalid_values(col.iter_mut(), &values)?;
  }

  // apply squares
  for i in (0..9).step_by(3) {
    for j in (0..9).step_by(3) {
      let mut slice = data.slice_mut((i, j), (3, 3));
      let values = values_from_slice(slice.iter());
      remove_invalid_values(slice.iter_mut(), &values)?;
    }
  }

  if *data == previous_data {
    return Ok(());
  } else {
    // check if it's possible to eliminate more possibilities
    let previous_data = data.clone();
    do_apply_constraints(data, previous_data)
  }
}

fn values_from_slice<'a, T>(iter: T) -> Vec<u16>
where
  T: Iterator<Item = &'a Tile>,
{
  iter
    .filter(|tile| tile.len() == 1)
    .map(|tile| tile.next())
    .collect()
}

fn remove_invalid_values<'a, T>(tiles: T, values: &Vec<u16>) -> Result<(), ()>
where
  T: Iterator<Item = &'a mut Tile>,
{
  for tile in tiles.filter(|tile| tile.len() != 1) {
    for &value in values.iter() {
      tile.remove(value);

      // impossible sudoku
      if tile.len() == 0 {
        return Err(());
      }
    }
  }

  Ok(())
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", &self.data)
  }
}
