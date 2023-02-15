# `match` on a `tuple`

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

A [tuple][tuple] is made from the conditions for the year being evenly divisible by `4`, `100` and `400`.

- The `tuple` is tested in a [match expression][match].
- It tests from top to bottom, so the `(_, _, 0) => true,` arm is tested first.
- It checks the values of the expressions in the `tuple`, and uses the `_` [wildcard pattern][wildcard] to disregard a value inside the tuple.
- If the pattern matches, then the arm returns the value to the right of the `=>`.
- If the pattern does not match, then the pattern in the next arm is tested.
- In the final arm of the `match`, the wildcard pattern is applied to the entire tuple.
This is similar to `default` used in `switch` statements in other languages.
It returns `false` no matter what the values in the tuple are.

| year | year % 4 | year % 100 |   year % 400    | is leap year |
| ---- | -------- | ---------- | --------------- | ------------ |
| 2020 |        0 |         20 |              20 |         true |
| 2019 |        3 |         19 |              19 |        false |
| 2000 |        0 |          0 |               0 |         true |
| 1900 |        0 |          0 |             300 |        false |

Although some may consider it to be a more "functional" approach, the `switch` on a `tuple` approach is somewhat more verbose than other approaches,
and may also be considered less readable.

[match]: https://doc.rust-lang.org/rust-by-example/flow_control/match.html
[tuple]: https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
[wildcard]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern
