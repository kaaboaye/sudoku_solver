use nalgebra::Scalar;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]
/// Tile is a set of possible values
pub struct Tile {
  // Bits from 10 to 1 are representing possible values
  //
  // Examples
  // `0b0000000000010110` means `Set([1, 2, 4])`
  // `0b0000001000000010` means `Set([1, 9])`
  data: u16,
}

impl Tile {
  #[inline]
  pub fn new() -> Tile {
    Tile { data: 0 }
  }

  #[inline]
  pub fn new_full_set() -> Tile {
    Tile {
      data: 0b0000001111111110,
    }
  }

  #[inline]
  pub fn insert(&mut self, value: u16) {
    debug_assert!((1..10).contains(&value));

    self.data |= 1 << value;
  }

  #[inline]
  pub fn remove(&mut self, value: u16) {
    debug_assert!((1..10).contains(&value));

    self.data &= !(1 << value);
  }

  #[inline]
  pub fn len(&self) -> u32 {
    self.data.count_ones()
  }

  #[inline]
  pub fn get_single_value(&self) -> u16 {
    self.data.trailing_zeros() as u16
  }

  #[inline]
  pub fn iter(&self) -> TileIter {
    TileIter { tile: self.clone() }
  }
}

impl Debug for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    let values: Vec<_> = self.iter().collect();

    if values.len() == 1 {
      write!(f, "{}", values[0])
    } else {
      write!(f, "{:?}", &values)
    }
  }
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{:?}", &self)
  }
}

impl Scalar for Tile {}

pub struct TileIter {
  tile: Tile,
}

impl Iterator for TileIter {
  type Item = u16;

  fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
    if self.tile.len() == 0 {
      return None;
    }

    let next_value = self.tile.get_single_value();
    self.tile.remove(next_value);
    Some(next_value)
  }
}

impl IntoIterator for Tile {
  type Item = u16;
  type IntoIter = TileIter;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn contains(tile: &Tile, value: u16) -> bool {
    debug_assert!((1..10).contains(&value));

    (tile.data & (1 << value)) != 0
  }

  #[test]
  fn test_new() {
    let tile = Tile::new();
    assert_eq!(tile.len(), 0);
  }

  #[test]
  fn test_new_full_set() {
    let tile = Tile::new_full_set();
    assert_eq!(tile.len(), 9);
  }

  #[test]
  fn test_insert() {
    let mut tile = Tile::new();

    assert_eq!(tile.len(), 0);

    tile.insert(3);
    assert_eq!(tile.len(), 1);
    assert!(contains(&tile, 3));

    tile.insert(3);
    assert_eq!(tile.len(), 1);
    assert!(contains(&tile, 3));

    tile.insert(4);
    assert_eq!(tile.len(), 2);
    assert!(contains(&tile, 3));
    assert!(contains(&tile, 4));
  }

  #[test]
  fn set_tile_contains() {
    let mut tile = Tile { data: 0b110 };

    assert!(contains(&tile, 1));
    assert!(contains(&tile, 2));

    tile.remove(1);

    assert!(!contains(&tile, 1));
    assert!(contains(&tile, 2));
  }

  #[test]
  fn set_tile_remove() {
    let mut tile = Tile::new_full_set();

    for i in 1..10 {
      assert!(contains(&tile, i));
    }

    tile.remove(3);
    tile.remove(8);

    for i in 1..10 {
      assert_eq!(contains(&tile, i), i != 3 && i != 8);
    }
  }

  #[test]
  fn set_tile_len1() {
    let tile = Tile { data: 0b110 };

    assert_eq!(tile.len(), 2);
  }

  #[test]
  fn set_tile_len2() {
    let mut set_tile = Tile::new_full_set();

    assert_eq!(set_tile.len(), 9);

    set_tile.remove(3);
    set_tile.remove(8);

    assert_eq!(set_tile.len(), 7);
  }

  #[test]
  fn test_get_single_value_some() {
    for i in 1..10 {
      let mut tile = Tile::new_full_set();

      // leave only one value in set
      for x in 1..10 {
        if x != i {
          tile.remove(x);
        }
      }

      let value = tile.get_single_value();
      assert_eq!(value, i);
    }
  }
}
