# Introduction

There are various idiomatic approaches to solve Grains.
You can use `pow` to calculate the number on grains on a square.
Or you can use bit-shifting.

## General guidance

The key to solving Grains is to focus on each square having double the amount of grains as the square before it.
This means that the amount of grains grows exponentially.
The first square has one grain, which is `2` to the power of `0`.
The second square has two grains, which is `2` to the power of `1`.
The third square has four grains, which is `2` to the power of `2`.
You can see that the exponent, or power, that `2` is raised by is always one less than the square number.

| Square  | Power      | Value                   |
| ------- | ---------- | ----------------------- |
|       1 |          0 | 2 to the power of 0 = 1 |
|       2 |          1 | 2 to the power of 1 = 2 |
|       3 |          2 | 2 to the power of 2 = 4 |
|       4 |          3 | 2 to the power of 4 = 8 |

## Approach: `pow`

```rust
pub fn square(s: u32) -> u64 {
    if (s < 1) | (s > 64) {
        panic!("Square must be between 1 and 64");
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (2_u128.pow(64) - 1) as u64
}
```

For more information, check the [`pow` approach][approach-pow].

## Approach: Bit-shifting

```rust
pub fn square(s: u32) -> u64 {
    if (s < 1) | (s > 64) {
        panic!("Square must be between 1 and 64");
    }
    1 << (s - 1)
}

pub fn total() -> u64 {
    ((1_u128 << 64) - 1) as u64
}
```

For more information, check the [Bit-shifting approach][approach-bit-shifting].

## Other approaches

Besides the aforementioned, idiomatic approaches, you could also approach the exercise as follows:

### Other approach: Max value

The maximum value for an unsigned integer gives the total number of grains on all sixty-four squares.
For more information, check the [`u64::MAX` for Total Approach][approach-max-value].

## Which approach to use?

`Pow` may be considered the most readable solution.

Bit-shifting is the same measurable performance as `pow`, but may be considered less readable.

For more information, check the [Performance article][article-performance].

[approach-pow]: https://exercism.org/tracks/rust/exercises/grains/approaches/pow
[approach-bit-shifting]: https://exercism.org/tracks/rust/exercises/grains/approaches/bit-shifting
[approach-max-value]: https://exercism.org/tracks/rust/exercises/grains/approaches/max-value
[article-performance]: https://exercism.org/tracks/rust/exercises/grains/articles/performance
