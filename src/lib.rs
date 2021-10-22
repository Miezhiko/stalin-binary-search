mod vec;

use std::cmp;

pub trait StalinFind<T> {
  fn len(&self) -> usize;
  fn is_empty(&self) -> bool;

  fn stalin(&mut self, i: T, l: usize, r: usize) -> Option<usize>
    where T: cmp::PartialEq + cmp::PartialOrd;

  fn stalin_find(&mut self, i: T) -> Option<usize>
    where T: cmp::PartialEq + cmp::PartialOrd {
    if self.is_empty() {
      None
    } else {
      self.stalin(i, 0, self.len() - 1)
    }
  }
}
