# Ternary expression of Boolean conditions

```rust
pub fn is_leap_year(year: u64) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}
```

Unlike other languages, Rust does not have a ternary operator such as `?` that is used like so

```c
// this is NOT valid Rust code
a > b ? a : b
```

where `a` is tested to be greater than `b`, and , if so, returns `a`, otherwise it returns `b`.
Note that the line of code returns `a` or `b` without using a `return` keyword.

Because Rust is an [expression-based][expression-based] language, a simple `if/else` works like a ternary expression in other languages.

In this code

```rust
if year % 100 == 0 {
    year % 400 == 0
}
```

`year` is tested to be evenly divislbe by `100` by using the [remainder operator][remainder-operator].
If so, it returns if `year` is evenly divisible by `400`, as the last expression evaluated in the function.

```exercism/note
Note that the line is just `year % 400 == 0`.
This is because the [last expression can be returned](https://doc.rust-lang.org/rust-by-example/fn.html)
from a function without using `return` and a semicolon.
```

If `year` is _not_ evenly divisible by `100`, then `else` is the last expression evaluated in the function

```rust
else {
    year % 4 == 0
}
```

and returns if `year` is evenly divisible by `4`.

| year | year % 100 == 0 | year % 400 == 0 | year % 4 == 0  | is leap year |
| ---- | --------------- | --------------- | -------------- | ------------ |
| 2020 |           false |   not evaluated |           true |        true  |
| 2019 |           false |   not evaluated |          false |       false  |
| 2000 |           true  |            true |  not evaluated |        true  |
| 1900 |           true  |           false |  not evaluated |        false |

Although it uses a maximum of only two checks, the ternary operator tests an outlier condition first,
making it less efficient than another approach that would first test if the year is evenly divisible by `4`,
which is more likely than the year being evenly divisible by `100`.

[remainder-operator]: https://doc.rust-lang.org/std/ops/trait.Rem.html
[expression-based]: https://doc.rust-lang.org/reference/statements-and-expressions.html
