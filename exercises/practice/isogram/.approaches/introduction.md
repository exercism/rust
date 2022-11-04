# Introduction

There are various idiomatic ways to solve Isogram.
You can use `filter` with `all` on a `HashSet`.
Or you could use a bit field.
The bitfield could use a `for` loop or a chain of functions.

## General guidance

The key to solving Isogram is to determine if any of the letters in the `&str` being checked are repeated.
A repeated letter means the `&str` is not an Isogram.
The occurrence of the letter `a` and the letter `A` count as a repeated letter, so `Alpha` would not be an isogram.
At the time of this writing, all tests use [ASCII][ascii] characters, so the letters can be processed as bytes instead of Unicode characters.

## Approach: `filter` and `map` with `all` on a `HashSet`

```rust
pub fn check(candidate: &str) -> bool {
    let mut hs = HashSet::new();
    candidate
        .bytes()
        .filter(|&c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .all(|c| hs.insert(c))
```

For more information, check the [`filter` and `map` with `all` on a `HashSet` approach][approach-filter-all]

## Approach: bit field using a `for` loop

```rust
const A_LCASE: u8 = 97;
const Z_LCASE: u8 = 122;
const A_UCASE: u8 = 65;
const Z_UCASE: u8 = 90;

pub fn check(candidate: &str) -> bool {
    let mut letter_flags = 0;

    for letter in candidate.bytes() {
        if letter >= A_LCASE && letter <= Z_LCASE {
            if letter_flags & (1 << (letter - A_LCASE)) != 0 {
                return false;
            } else {
                letter_flags |= 1 << (letter - A_LCASE);
            }
        } else if letter >= A_UCASE && letter <= Z_UCASE {
            if letter_flags & (1 << (letter - A_UCASE)) != 0 {
                return false;
            } else {
                letter_flags |= 1 << (letter - A_UCASE);
            }
        }
    }
    return true;
}
```
For more information, check the [bit field using a `for` loop approach][approach-bitfield].

## bit field used functionally

```rust
const A_LCASE: u8 = 97;

pub fn check(candidate: &str) -> bool {
    candidate
        .bytes()
        .filter_map(|c| {
            c.is_ascii_alphabetic()
                .then(|| 1u32 << (c.to_ascii_lowercase() - A_LCASE))
        })
        .try_fold(0u32, |ltr_flags, ltr| {
            (ltr_flags & ltr == 0).then(|| ltr_flags | ltr)
        })
        .is_some()
}
```

For more information, check the [bit field used functionally approach][approach-bitfield-func].

## Which approach to use?

All three approaches are idiomatic.
The two approaches using a bit field are more than forty times faster than the one using a `HashSet`.
Of the two bit field approaches, the `for` loop is few nanoseconds faster than the one using a chain of functions.
Since they are so close in performance, choosing between them would be a matter of stylistic preference.

To compare performance of the approaches, check the [Performance article][article-performance].

[approach-filter-all]: https://exercism.org/tracks/rust/exercises/isogram/approaches/filter-all
[approach-bitfield]: https://exercism.org/tracks/rust/exercises/isogram/approaches/bitfield
[approach-bitfield-func]: https://exercism.org/tracks/rust/exercises/isogram/approaches/bitfield-func
[article-performance]: https://exercism.org/tracks/rust/exercises/isogram/articles/performance
[ascii]: https://www.asciitable.com/
