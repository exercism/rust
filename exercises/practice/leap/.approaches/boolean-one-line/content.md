# One line of Boolean conditions

```rust
pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}
```

The first boolean condition uses the [remainder operator][remainder-operator] to check if the year is evenly divided by `4`.
- If the year is not evenly divisible by `4`, then the chain will "short circuit" due to the next operator being a [logical AND][operators] (`&&`), and will return `false`.
- If the year _is_ evenly divisible by `4`, then the year is checked to _not_ be evenly divisible by `100`.
- If the year is not evenly divisible by `100`, then the expression is `true` and the chain will "short-circuit" to return `true`,
since the next operator is a [logical OR][operators] (`||`).
- If the year _is_ evenly divisible by `100`, then the expression is `false`, and the returned value from the chain will be if the year is evenly divisible by `400`.

| year | year % 4 == 0 | year % 100 != 0 | year % 400 == 0 | is leap year |
| ---- | ------------- | --------------- | --------------- | ------------ |
| 2020 |          true |            true |   not evaluated |         true |
| 2019 |         false |   not evaluated |   not evaluated |        false |
| 2000 |          true |           false |            true |         true |
| 1900 |          true |           false |           false |        false |


The chain of boolean expressions is efficient, as it proceeds from testing the most likely to least likely conditions.

[remainder-operator]: https://doc.rust-lang.org/std/ops/trait.Rem.html
[operators]: https://doc.rust-lang.org/book/appendix-02-operators.html
