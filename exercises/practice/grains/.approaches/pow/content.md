## `pow`

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

Other languages may have an exponential operator such as `**` or `^` to raise a number by a specified power.
Rust does not have an exponential operator, but uses the [`pow`][pow-u64] method.

`pow` is nicely suited to the problem, since we start with one grain and keep doubling the number of grains on each successive square.
`1` grain is `2u64.pow(0)`, `2` grains is `2u64.pow(1)`, `4` is `2u64.pow(2)`, and so on.

```exercism/note
Note that the type of a binding can be defined by appending it to the binding name.
It can optionally be appended using one or more `_` separators between the value and the type.
More info can be found in the [Literals](https://doc.rust-lang.org/rust-by-example/types/literals.html) section of
[Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
```

So, to get the right exponent, we subtract `1` from the square number `s`.

The easiest way to get `total` is to use [`u128::pow`][pow-u128] to get the value for an imaginary 65th square,
and then subtract `1` from it.
To understand how that works, consider a board that has only two squares.
If we could grow the board to three squares, then we could get the number of grains on the imaginary third square,
which would be `4.`
You could then subtract `4` by `1` to get `3`, which is the number of grains on the first square (`1`) and the second square (`2`).
Remembering that the exponent must be one less than the square you want,
you might like to call `2u64.pow(64)` to get the number of grains on the imaginary 65th square.
However, that won't work, because the resulting number is too large for a `u64`.
So, you can use `2u128.pow(64)`, and when it is subtracted by `1` it will then fit in the `u64` value which is returned.

[pow-u64]: https://doc.rust-lang.org/std/primitive.u64.html#method.pow
[pow-u128]: https://doc.rust-lang.org/std/primitive.u128.html#method.pow
