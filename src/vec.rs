use super::StalinFind;

use std::cmp;

impl<T: PartialOrd> StalinFind<T> for Vec<T> {
  #[inline]
  fn len(&self) -> usize {
    self.len()
  }

  #[inline]
  fn is_empty(&self) -> bool {
    self.is_empty()
  }

    fn stalin(&mut self, i: T, l: usize, r: usize) -> Option<usize>
    where T: cmp::PartialEq + cmp::PartialOrd 
    {
        let m = (l + r) / 2;
        if self[m] == i 
        {
            Some(m)
        }
        else 
        {
            if l != r
            {
                if self[m] > i
                {
                    self.remove(m);
                    // a special care is only needed for the left edge 
                    // since "r" always is always bigger than "m" at least by 1
                    if m == l { self.stalin(i, l, m) } 
                    else { self.stalin(i, l, m - 1) }
                } 
                else 
                {
                    self.remove(m);
                    self.stalin(i, m, r - 1)
                }
            }
            else { None }
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_on_sorted() {
    let mut sorted = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let find = sorted.stalin_find(3);
    assert!(find.is_some());
    assert_eq!(
      find,
      Some(1),
    );
    assert_eq!(
      sorted,
      vec![1, 3, 4, 6, 7, 8, 9],
    );
    if let Some(find_3) = find {
      assert_eq!(
        3, sorted[find_3]
      );
    }
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
    if let Some(find_7) = unsorted.stalin_find(7) {
      assert_eq!(
        7, unsorted[find_7]
      );
    }
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
