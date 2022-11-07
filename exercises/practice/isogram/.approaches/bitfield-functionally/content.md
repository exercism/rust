# Bit field used functionally

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

This solution uses the [ASCII][ascii] value of the letter to set the corresponding bit position.

First, a [`const`][const] value is set with the ASCII value for `a`.

- Since all of the characters are [ASCII][ascii], they can be iterated with the [`bytes`][bytes] method.
Each byte is iterated as a [`u8`][u8], which is an unsigned 8-bit integer, and is passed to the [filter_map][filter-map] method.
- The [closure][closure] inside `filter_map` first tests if the byte [is_ascii_alphabetic][is-ascii-alphabetic].
If so, the byte is passed to the [`then`][then] method, where the byte is set [`to_ascii_lowercase`][to-ascii-lowercase] in its closure.
To understand what else is happening in `then`, consider the following:

- If the lower-cased letter is subtracted by `a`, then `a` will result in `0`, because `97` minus `97`  equals `0`.
`z` would result in `25`, because `122` minus `97` equals `25`.
So `a` would have `1u32` [shifted left][shift-left] 0 places (so not shifted at all) and `z` would have `1` shifted left 25 places.

So, for the [unsigned thirty-two bit integer][u32] (`1u32`), the value for `a` would look like

```
      zyxwvutsrqponmlkjihgfedcba
00000000000000000000000000000001
```

and the value for `z` would look like

```
      zyxwvutsrqponmlkjihgfedcba
00000010000000000000000000000000
```

- The `filter map` passes only lowercased ASCII letter bytes converted to `u32` values to the [try_fold][try-fold] method.

```rust
        // code snipped
        .try_fold(0u32, |ltr_flags, ltr| {
            (ltr_flags & ltr == 0).then(|| ltr_flags | ltr)
        })
        .is_some()
}
```

- The `try_fold` has its accumulator set to a `u32` initialized to `0`.
The closure inside `try_fold` uses the [bitwise AND operator][and] to check if the bit for the letter position has not already been set.
- If it has been set, you know the letter is duplicated and `try_fold` will "short circuit"
and immediately pass [`None`][none] to the [`is_some`][is-some] method, which willl return `false`.
- If it has not been set, the [bitwise OR operator][or] is used in the `then` method to set the bit.
If all of the iterations of `try_fold` complete without finding a duplicate letter (and returning `None`),
the function returns `true` from the `is_some` method.

## Refactoring

Since a `filter_map` is used, this approach could be refactored to use [`filter`][filter] and a [`map`][map].

```rust
pub fn check_bits(candidate: &str) -> bool {
    candidate
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| 1u32 << (c.to_ascii_lowercase() - A_LCASE))
        .try_fold(0u32, |ltr_flags, ltr| {
            (ltr_flags & ltr == 0).then(|| ltr_flags | ltr)
        })
        .is_some()
}
```

In benchmarking, this approach was slightly slower, but its style may be prefered.

[ascii]: https://www.asciitable.com/
[const]: https://doc.rust-lang.org/std/keyword.const.html
[bytes]: https://doc.rust-lang.org/std/primitive.str.html#method.bytes
[u8]: https://doc.rust-lang.org/std/primitive.u8.html
[filter-map]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
[closure]: https://doc.rust-lang.org/rust-by-example/fn/closures.html
[is-ascii-alphabetic]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphabetic
[then]: https://doc.rust-lang.org/core/primitive.bool.html#method.then
[to-ascii-lowercase]: https://doc.rust-lang.org/std/primitive.u8.html#method.to_ascii_lowercase
[u32]: https://doc.rust-lang.org/std/primitive.u32.html
[try-fold]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold
[shift-left]: https://doc.rust-lang.org/std/ops/trait.Shl.html
[and]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
[none]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
[is-some]: https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some
[or]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
[filter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
