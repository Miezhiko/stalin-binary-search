[![Rust](https://github.com/Miezhiko/stalin-binary-search/actions/workflows/rust.yml/badge.svg)](https://github.com/Miezhiko/stalin-binary-search/actions/workflows/rust.yml)
[![crates.io version]][crates.io link]

Stalin Binary Search
====================

Idea is based on Stalin Sort ![ss](https://i.redd.it/x9triplll1v11.jpg)

It's alike binary search but any checking element which is not target one is eliminated.

Complexity is~ O(log n) on first run however after~ n/2 runs Complxity will be O(1) guaranteed.
Should not fail on unsorted.

(there analysis not from Fingon but with pictures and fancy text targetting working of this on unsorted arrays [The underlying analysis](docs/complexity_analysis.md).)


```rust
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
```

[crates.io link]: https://crates.io/crates/stalin-binary-search
[crates.io version]: https://img.shields.io/crates/v/stalin-binary-search.svg
