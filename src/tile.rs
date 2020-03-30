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
  pub fn contains(&self, value: u16) -> bool {
    debug_assert!((1..10).contains(&value));

    (self.data & (1 << value)) != 0
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
    debug_assert_eq!(self.len(), 1);

    self.data.trailing_zeros() as u16
  }
}

impl Debug for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    let mut values = Vec::new();

    for i in 1..10 {
      if self.contains(i) {
        values.push(i)
      }
    }

    write!(f, "{:?}", &values)
  }
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{:?}", &self)
  }
}

impl Scalar for Tile {}

#[cfg(test)]
mod tests {
  use super::*;

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
    assert!(tile.contains(3));

    tile.insert(3);
    assert_eq!(tile.len(), 1);
    assert!(tile.contains(3));

    tile.insert(4);
    assert_eq!(tile.len(), 2);
    assert!(tile.contains(3));
    assert!(tile.contains(4));
  }

  #[test]
  fn set_tile_contains() {
    let mut tile = Tile { data: 0b110 };

    assert!(tile.contains(1));
    assert!(tile.contains(2));

    tile.remove(1);

    assert!(!tile.contains(1));
    assert!(tile.contains(2));
  }

  #[test]
  fn set_tile_remove() {
    let mut tile = Tile::new_full_set();

    for i in 1..10 {
      assert!(tile.contains(i));
    }

    tile.remove(3);
    tile.remove(8);

    for i in 1..10 {
      assert_eq!(tile.contains(i), i != 3 && i != 8);
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
