# Bit-shifting

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

Instead of using math to calculate the number of grains on a square, you can simply set a bit in the correct position of a [`u64`][u64] or [`u128`][u128] value.

To understand how this works, consider just two squares that are represented in binary bits as `00`.

You use the [left-shift operator][left-shift-operator] to set `1` at the position needed to make the correct decimal value.
- To set the one grain on Square One you shift `1` for `0` positions to the left.
So, if `n` is `1` for square One, you subtract `n` by `1` to get `0`, which will not move it any positions to the left.
The result is binary `01`, which is decimal `1`.
- To set the two grains on Square Two you shift `1` for `1` position to the left.
So, if `n` is `2` for square Two, you subtract `n` by `1` to get `1`, which will move it `1` position to the left.
The result is binary `10`, which is decimal `2`.

| Square  | Shift Left By | Binary Value | Decimal Value |
| ------- | ------------- | ------------ | ------------- |
|       1 |             0 |         0001 |             1 |
|       2 |             1 |         0010 |             2 |
|       3 |             2 |         0100 |             4 |
|       4 |             3 |         1000 |             8 |

For `total` we want all of the 64 bits set to `1` to get the sum of grains on all sixty-four squares.
The easy way to do this is to set the 65th bit to `1` and then subtract `1`.
However, we can't do this with a `u64` which has only `64` bits, so we need to use a `u128`.
To go back to our two-square example, if we can grow to three squares, then we can shift `1_u128` two positions to the left for binary `100`,
which is decimal `4`.

```exercism/note
Note that the type of a binding can be defined by appending it to the binding name.
It can optionally be appended using one or more `_` separators between the value and the type.
More info can be found in the [Literals](https://doc.rust-lang.org/rust-by-example/types/literals.html) section of
[Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
```
By subtracting `1` we get `3`, which is the total amount of grains on the two squares.

| Square  | Binary Value | Decimal Value |
| ------- | ------------ | ------------- |
|       3 |         0100 |             4 |

| Square  | Sum Binary Value | Sum Decimal Value |
| ------- | ---------------- | ----------------- |
|       1 |             0001 |                 1 |
|       2 |             0011 |                 3 |

[u64]: https://doc.rust-lang.org/std/primitive.u64.html
[u128]: https://doc.rust-lang.org/std/primitive.u128.html
[left-shift-operator]: https://doc.rust-lang.org/std/ops/trait.Shl.html
