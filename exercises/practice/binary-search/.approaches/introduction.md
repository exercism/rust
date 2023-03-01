# Introduction

There are at least two general ways to solve Binary Search.
One approach is to loop.
Another approach is to use recursion.

## General guidance

One thing to keep in mind is that Rust does not guarantee [tail call optimization][tco].

## Approach: Looping

```rust
use std::cmp::Ordering;

pub fn find<U: AsRef<[T]>, T: Ord>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut left = 0usize;
    let mut right = array.len();
    let mut mid: usize;

    while left < right {
        mid = (left + right) / 2;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Greater => {
                right = mid;
            }
        }
    }
    None
}
```

For more information, check the [Looping approach][approach-looping].

## Approach: Recursion

```rust
use std::cmp::Ordering;

fn find_rec<U: AsRef<[T]>, T: Ord>(array: U, key: T, offset: usize) -> Option<usize> {
    let array = array.as_ref();
    if array.len() == 0 {
        return None;
    }
    let mid = array.len() / 2;

    match array[mid].cmp(&key) {
        Ordering::Equal => Some(offset + mid),
        Ordering::Less => find_rec(&array[mid + 1..], key, offset + mid + 1),
        Ordering::Greater => find_rec(&array[..mid], key, offset),
    }
}

pub fn find<U: AsRef<[T]>, T: Ord>(array: U, key: T) -> Option<usize> {
    find_rec(array, key, 0)
}
```

For more information, check the [Recursion approach][approach-recursion].

## Which approach to use?

Since benchmarking is currently outside the scope of this document, which to use is pretty much a matter of personal preference,
but the looping approach may be considered to be more idiomatic, given that Rust does not guarantee tail call optimization.

[tco]: https://stackoverflow.com/questions/59257543/when-is-tail-recursion-guaranteed-in-rust/59258170#59258170
[approach-looping]: https://exercism.org/tracks/rust/exercises/binary-search/approaches/looping
[approach-recursion]: https://exercism.org/tracks/rust/exercises/binary-search/approaches/recursion
