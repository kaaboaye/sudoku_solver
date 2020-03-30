use nalgebra::Scalar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
  // Tile can be either constant value or set of possible values
  //
  // When it is constant it has 11'th bit from right side set to 1
  // and left byte has this constant value binary encoded
  //
  // Examples
  // `0b0000010000000100` means `Constant(4)`
  // `0b0000010000000001` means `Constant(1)`
  //
  // When tile is a set of possible values 11'th bit is set to 0
  // and bits from 10 to 1 are representing possible values
  //
  // Examples
  // `0b0000000000010110` means `Set([1, 2, 4])`
  // `0b0000001000000010` means `Set([1, 9])`
  data: u16,
}

impl Tile {
  pub fn new_constant(value: u16) -> Tile {
    debug_assert!((1..10).contains(&value));

    let data = (1 << 10) | value;
    Tile { data }
  }

  pub fn new_full_set() -> Tile {
    let data = 0b0000001111111110;
    Tile { data }
  }

  pub fn is_constant(&self) -> bool {
    (self.data & (1 << 10)) != 0
  }

  pub fn is_set(&self) -> bool {
    (self.data & (1 << 10)) == 0
  }

  pub fn value_of_constant(&self) -> u16 {
    assert!(self.is_constant());

    (self.data & 0x00ff)
  }

  pub fn transform_set_into_constant(&mut self) {
    assert_eq!(self.set_len(), 1);

    // x86 tzcnt instruction <3
    let value = self.data.trailing_zeros();
    let Tile { data } = Tile::new_constant(value as u16);
    self.data = data;
  }

  pub fn set_contains(&self, value: u16) -> bool {
    debug_assert!((1..10).contains(&value));
    assert!(self.is_set());

    (self.data & (1 << value)) != 0
  }

  pub fn set_remove(&mut self, value: u16) {
    debug_assert!((1..10).contains(&value));
    assert!(self.is_set());

    self.data &= !(1 << value);
  }

  pub fn set_len(&self) -> u32 {
    assert!(self.is_set());

    self.data.count_ones()
  }
}

impl Scalar for Tile {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn constant_tile() {
    let constant_tile = Tile::new_constant(3);
    assert!(constant_tile.is_constant());
    assert!(!constant_tile.is_set());
    assert_eq!(constant_tile.value_of_constant(), 3);
  }

  #[test]
  fn set_tile_contains() {
    let mut tile = Tile { data: 0b110 };

    assert!(tile.set_contains(1));
    assert!(tile.set_contains(2));

    tile.set_remove(1);

    assert!(!tile.set_contains(1));
    assert!(tile.set_contains(2));
  }

  #[test]
  fn set_tile_remove() {
    let mut set_tile = Tile::new_full_set();

    assert!(set_tile.is_set());
    assert!(!set_tile.is_constant());

    for i in 1..10 {
      assert!(set_tile.set_contains(i));
    }

    set_tile.set_remove(3);
    set_tile.set_remove(8);

    for i in 1..10 {
      assert_eq!(set_tile.set_contains(i), i != 3 && i != 8);
    }
  }

  #[test]
  fn set_tile_len1() {
    let tile = Tile { data: 0b110 };

    assert_eq!(tile.set_len(), 2);
  }

  #[test]
  fn set_tile_len2() {
    let mut set_tile = Tile::new_full_set();

    assert!(set_tile.is_set());
    assert!(!set_tile.is_constant());

    assert_eq!(set_tile.set_len(), 9);

    set_tile.set_remove(3);
    set_tile.set_remove(8);

    assert_eq!(set_tile.set_len(), 7);
  }

  #[test]
  fn test_transform_set_into_constant() {
    for i in 1..10 {
      let mut tile = Tile::new_full_set();

      // leave only one value in set
      for x in 1..10 {
        if x != i {
          tile.set_remove(x);
        }
      }

      tile.transform_set_into_constant();

      assert_eq!(tile.value_of_constant(), i);
    }
  }
}
