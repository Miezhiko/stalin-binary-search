mod vec;

use std::cmp;

pub trait StalinFind<T> {
  fn len(&self) -> usize;

  fn stalin(&mut self, i: T, l: usize, r: usize) -> Option<usize>
    where T: cmp::PartialEq + cmp::PartialOrd;

  fn stalin_find(&mut self, i: T) -> Option<usize>
    where T: cmp::PartialEq + cmp::PartialOrd {
    if self.len() > 0 {
      self.stalin(i, 0, self.len() - 1)
    } else {
      None
    }
  }
}
