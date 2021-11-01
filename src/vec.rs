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
    where T: cmp::PartialEq + cmp::PartialOrd {
    let m = (l + r) / 2;
    if self[m] == i {
      Some(m)
    } else {
      self.remove(m);
      if m == 0 || l > r {
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
  use rand::Rng;

  #[test]
  fn randomised_test() {
    // generate a random vector of length 0-n
    let n = 100; // n was supposed to be a paramer of the function but whatever
    let mut rng = rand::thread_rng();
    let sought = rng.gen_range(0..n);
    let amount = rng.gen_range(0..n);
    let range = rand::distributions::uniform::Uniform::new(0, n);
    let mut vector: Vec<u64> = (0..amount).map(|_| rng.sample(&range)).collect();

    // check if the value is within the array
    let i = vector.iter().position(|x| x == &sought);
    if i.is_some() {
      if let Some(find) = vector.stalin_find(sought) {
        assert_eq!(vector[find], sought)
      }
    } else {
      let find = vector.stalin_find(sought);
      assert_eq!(find, None)
    }
  }

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

  #[test]
  fn find_not_fingon() {
    let mut unsorted = vec![12, 11, 4, 5, 7, 8, 2];
    assert_eq!(
      unsorted.stalin_find(9),
      None,
    );
    assert_eq!(
      unsorted,
      vec![],
    );
  }
}
