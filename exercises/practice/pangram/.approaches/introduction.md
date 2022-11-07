# Introduction

There are various idomatic approaches to Pangram.
You can use the `all` method with the `contains` method on lower-cased letters.
You can see if the `HashSet` of the alphabet `is_substring` of a `HashSet` of the lower-cased `sentence`.
Or you can see if the `HashSet` `len` of the lowercased `sentence` filtered to just ASCII letters is `26`.
Or you can use a bit field to keep track of used letters.


## General guidance

The key to solving Pangram is determining if all of the letters in the alphabet are in the `&str` being tested.
The occurrence of either the letter `a` or the letter `A` would count as the same letter.

## Approach: `all` with `contains` on lowercased letters

```rust
pub fn is_pangram(sentence: &str) -> bool {
    let sentence_lowered = sentence.to_lowercase();
    ('a'..='z').all(|ltr| sentence_lowered.contains(ltr))
}
```

For more information, check the [`all` with `contains` approach][approach-all-contains].

## Approach: `HashSet` with `is_subset` on lowercased characters

```rust
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let all: HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
    let used: HashSet<char> = HashSet::from_iter(sentence.to_lowercase().chars());
    all.is_subset(&used)
}
```

For more information, check the [`HashSet` with `is_subset` approach][approach-hashset-is-subset].

## Approach: `HashSet` with `len` on lowercased characters

```rust
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect::<HashSet<char>>()
        .len()
        == 26
}
```

For more information, check the [`HashSet` with `len` approach][approach-hashset-len].

## Bit field

```rust
const A_LCASE: u8 = 97;
const A_UCASE: u8 = 65;
const ALL_26_BITS_SET: u32 = 67108863;

pub fn is_pangram(sentence: &str) -> bool {
    let mut letter_flags = 0;

    for letter in sentence.chars() {
        if letter >= 'a' && letter <= 'z' {
            letter_flags |= 1 << (letter as u8 - A_LCASE);
        } else if letter >= 'A' && letter <= 'Z' {
            letter_flags |= 1 << (letter as u8 - A_UCASE);
        }
    }
    letter_flags == ALL_26_BITS_SET
}
```

For more information, check the [Bit field approach][approach-bitfield].

## Which approach to use?

The fastest is the `Bit field` approach.

To compare performance of the approaches, check the [Performance approach][approach-performance].

[approach-all-contains]: https://exercism.org/tracks/rust/exercises/pangram/approaches/all-contains
[approach-hashset-is-subset]: https://exercism.org/tracks/rust/exercises/pangram/approaches/hashset-is-subset
[approach-hashset-len]: https://exercism.org/tracks/rust/exercises/pangram/approaches/hashset-len
[approach-bitfield]: https://exercism.org/tracks/rust/exercises/pangram/approaches/bitfield
[approach-performance]: https://exercism.org/tracks/rust/exercises/pangram/approaches/performance
