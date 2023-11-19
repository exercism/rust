# Instructions append

## Performance Hint

All the inputs and outputs are in ASCII.
Rust `String`s and `&str` are utf8, so while one might expect `"Hello".chars()` to be simple, it actually has to check each char to see if it's 1, 2, 3 or 4 `u8`s long.
If we know a `&str` is ASCII then we can call `.as_bytes()` and refer to the underlying data as a `&[u8]` (byte slice).
Iterating over a slice of ASCII bytes is much quicker as there are no codepoints involved - every ASCII byte is one `u8` long.

Can you complete the challenge without cloning the input?
