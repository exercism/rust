# Bit field using a `for` loop

```rust
const A_LCASE: u8 = 97;
const Z_LCASE: u8 = 122;
const A_UCASE: u8 = 65;
const Z_UCASE: u8 = 90;

pub fn check(candidate: &str) -> bool {
    let mut letter_flags: u32 = 0;

    for letter in candidate.bytes() {
        if letter >= A_LCASE && letter <= Z_LCASE {
            if letter_flags & (1 << (letter - A_LCASE)) != 0 {
                return false;
            } else {
                letter_flags |= 1 << (letter - A_LCASE);
            }
        } else if letter >= A_UCASE && letter <= Z_UCASE {
            if letter_flags & (1 << (letter - A_UCASE)) != 0 {
                return false;
            } else {
                letter_flags |= 1 << (letter - A_UCASE);
            }
        }
    }
    return true;
}
```

This solution uses the [ASCII][ascii] value of the letter to set the corresponding bit position.

First, some [`const`][const] values are set.
These values will be used for readability in the body of the `check` function.

An unsigned 32-bit integer ([`u32`][u32]) will be used to hold the twenty-six bits needed
to keep track of the letters in the English alphabet.

The [`for` loop][for-loop] loops through the [bytes][bytes] of `candidate`.
Each `letter` is a [`u8`][u8] which is tested for being `a` through `z` or `A` through `Z`.
The ASCII values defined as the `const` values are used for that.
The ASCII value for `a` is `97`, and for `z` is `122`.
The ASCII value for `A` is `65`, and for `Z` is `90`.

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

We can use the [bitwise AND operator][and] to check if a bit has already been set.
If it has been set, we know the letter is duplicated and we can immediately return `false`.
If it has not been set, we can use the [bitwise OR operator][or] to set the bit.
If the loop completes without finding a duplicate letter (and returning `false`), the function returns `true`.

[ascii]: https://www.asciitable.com/
[const]: https://doc.rust-lang.org/std/keyword.const.html
[u32]: https://doc.rust-lang.org/std/primitive.u32.html
[for-loop]: https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops
[bytes]: https://doc.rust-lang.org/std/primitive.str.html#method.bytes
[u8]: https://doc.rust-lang.org/std/primitive.u8.html
[shift-left]: https://doc.rust-lang.org/std/ops/trait.Shl.html
[and]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
[or]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
