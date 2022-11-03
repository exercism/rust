# `u64::MAX` for total

```rust
pub fn total() -> u64 {
    u64::MAX
}
```

The maximum value for an unsigned integer gives the total number of grains on all sixty-four squares.
It is essentially getting all of the bits of a 64-bit number set to `1`.

This might seem like a "cheat".
To understand how it works, consider just two squares that are represented in bits as `00`.

- If there is a grain on square One, then the first bit (on the right) is set to binary `01`, which is the decimal value of `1`.
- If there are two grains on square Two, then the second bit (from the right) is set to binary `10`, which is the decimal value of `2`.
- If there are is one grain on Square One and two grains on square Two, then both bits are set to binary `11`, which is the decimal value of `3`.
- So, with all of the two bits set you get `3`, which is the number of all grains on both squares.

Getting the value of [`u64::MAX`][u64-max] is getting 64 bits set to `1`, which is just what we need to get the total number of grains on
sixty-four squares, given that the grains double on each square, starting with one grain on the first square.

[u64-max]: https://doc.rust-lang.org/std/u64/constant.MAX.html
