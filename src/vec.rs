use super::StalinFind;

use std::cmp;

impl<T: PartialOrd> StalinFind<T> for Vec<T> {
  #[inline]
  fn len(&self) -> usize {
    self.len()
  }

  fn stalin(&mut self, i: T, l: usize, r: usize) -> Option<usize>
    where T: cmp::PartialEq + cmp::PartialOrd {
    let m = (l + r) / 2;
    if self[m] == i {
      Some(m)
    } else {
      self.remove(m);
      if m == 0 {
        if self.is_empty() {
          None
        } else {
          self.stalin(i, 0, self.len() - 1)
        }
      } else if self[m - 1] > i {
        self.stalin(i, l, m - 1)
      } else {
        self.stalin(i, m - 1, r - 1)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_on_sorted() {
    let mut sorted = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(
      sorted.stalin_find(3),
      Some(1),
    );
    assert_eq!(
      sorted,
      vec![1, 3, 4, 6, 7, 8, 9],
    );
  }

  #[test]
  fn find_on_unsorted() {
    let mut unsorted = vec![33, 55, 3, 4, 7657, 6, 7, 8];
    assert_eq!(
      unsorted.stalin_find(3),
      Some(2),
    );
    assert_eq!(
      unsorted,
      vec![33, 55, 3, 7657, 7, 8],
    );
  }

  #[test]
  fn find_fail() {
    let mut unsorted = vec![33, 55, 3, 4, 7657, 6, 7, 8];
    assert_eq!(
      unsorted.stalin_find(77),
      None,
    );
    assert_eq!(
      unsorted,
      vec![],
    );
  }

  #[test]
  fn find_not_fail() {
    let mut unsorted = vec![33, 55, 3, 4, 7657, 6, 7, 8, 2];
    assert_eq!(
      unsorted.stalin_find(2),
      Some(0),
    );
    assert_eq!(
      unsorted,
      vec![2],
    );
  }
}
