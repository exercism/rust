# Bit field

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

This solution uses the [ASCII][ascii] value of the letter to set the corresponding bit position.
First, some [`const`][const] values are set.
These values will be used for readability in the body of the `is_pangram` function.
The ASCII value for `a` is `97`.
The ASCII value for `A` is `65`.
The value for all of the rightmost `26` bits being set in a [`u32`][u32] is `67108863`.

- The [`for` loop][for-loop] loops through the [chars][chars] of `sentence`.
We don't iterate by bytes because, as of this writing, some tests may include multi-byte characters in `sentence`.
- Each letter is tested for being `a` through `z` or `A` through `Z`.
- If the lower-cased letter is subtracted by `a`, then `a` will result in `0`, because `97` minus `97`  equals `0`.
`z` would result in `25`, because `122` minus `97` equals `25`.
So `a` would have `1` [shifted left][shift-left] 0 places (so not shifted at all) and `z` would have `1` shifted left 25 places.
- If the upper-cased letter is subtracted by `A`, then `A` will result in `0`, because `65` minus `65`  equals `0`.
`Z` would result in `25`, because `90` minus `65` equals `25`.
So `A` would have `1` [shifted left][shift-left] 0 places (so not shifted at all) and `Z` would have `1` shifted left 25 places.

In that way, both a lower-cased `z` and an upper-cased `Z` can share the same position in the bit field.

So, for an unsigned thirty-two bit integer, if the values for `a` and `Z` were both set, the bits would look like

```
      zyxwvutsrqponmlkjihgfedcba
00000010000000000000000000000001
```

We can use the [bitwise OR operator][or] to set the bit.
After the loop completes, the function returns if the `letter_flags` value is the same value as when all of the rightmost  `26` bits are set,
which is `67108863`.

This approach is relatively very fast compared with other approaches.

[ascii]: https://www.asciitable.com/
[const]: https://doc.rust-lang.org/std/keyword.const.html
[u32]: https://doc.rust-lang.org/std/primitive.u32.html
[for-loop]: https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops
[chars]: https://doc.rust-lang.org/std/primitive.str.html#method.chars
[shift-left]: https://doc.rust-lang.org/std/ops/trait.Shl.html
[or]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
