# Introduction

There are various idiomatic approaches to solve Leap.
You can use one line of boolean expressions to test the conditions.
Or you can use a [ternary expression][ternary-expression].
Another approach you can use is a [match][match] on a [tuple][tuple].

## General guidance

The key to solving Leap is to know if the year is evenly divisible by `4`, `100` and `400`.
For determining that, you will use the [remainder operator][remainder-operator].

## Approach: One line of Boolean conditions

```rust
pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}
```

For more information, check the [Boolean one-line approach][approach-boolean-line].

## Approach: Ternary expression of Boolean conditions

```rust
pub fn is_leap_year(year: u64) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}
```

For more information, check the [Ternary expression approach][approach-ternary-expression].

## Approach: `match` on a `tuple`

```rust
pub fn is_leap_year(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (_, _, 0) => true,
        (_, 0, _) => false,
        (0, _, _) => true,
        _ => false,
    }
}
```

For more information, check the [`match` on a `tuple` approach][approach-match-on-a-tuple].

## Other approaches

Besides the aforementioned, idiomatic approaches, you could also approach the exercise as follows:

### Other approach: `time::Date` addition approach:

Add a day to February 28th for the year and see if the new day is the 29th. For more information, see the [`time::Date` addition approach][approach-date-add-time].

### Other approach: `chrono::Date` addition approach:

Add a day to February 28th for the year and see if the new day is the 29th. For more information, see the [`chrono::Date` addition approach][approach-date-add-chrono].

## Which approach to use?

- The one line of boolean conditions is most efficient, as it proceeds from the most likely to least likely conditions.
It has a maximum of three checks.
- The ternary expression has a maximum of only two checks, but it starts from a less likely condition.
- The `match` on a `tuple` may be considered more "functional", but it is more verbose and may be considered less readable.
- Using `Date` addition may be considered "cheats" for the exercise,
and they are less performant than the three main approaches.

For more information, check the [Performance article][article-performance].

[remainder-operator]: https://doc.rust-lang.org/std/ops/trait.Rem.html
[match]: https://doc.rust-lang.org/rust-by-example/flow_control/match.html
[tuple]: https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
[ternary-expression]: (https://doc.rust-lang.org/reference/expressions/if-expr.html)
[approach-boolean-line]: https://exercism.org/tracks/rust/exercises/leap/approaches/boolean-one-line
[approach-ternary-expression]: https://exercism.org/tracks/rust/exercises/leap/approaches/ternary-expression
[approach-match-on-a-tuple]: https://exercism.org/tracks/rust/exercises/leap/approaches/match-on-a-tuple
[approach-date-add-time]: https://exercism.org/tracks/rust/exercises/leap/approaches/date-addition-time
[approach-date-add-chrono]: https://exercism.org/tracks/rust/exercises/leap/approaches/date-addition-chrono
[article-performance]: https://exercism.org/tracks/rust/exercises/leap/articles/performance
