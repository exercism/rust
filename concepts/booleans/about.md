# About

Booleans in Rust are represented by the `bool` type, which values can be either `true` or `false`.

Rust supports one [boolean negation operator][negation operators]: `!` (Not).

Rust supports six [comparison operators][comparison operators]: `==` (Equal), `!=` (Not equal), `>` (Greater than), `<` (Less than), `>=`
(Greater than or equal to), `<=` (Less than or equal to). They can be used to evaluate the relationship of non-boolean
values into a boolean value.

```rust
1 > 0 // true
1 < 0 // false
1 == 0 // false
1 != 0 // true
1 >= 0 // true
1 <= 0 // false
```

Unlike some other languages, `0` is not `false` and non-zero is not `true`.

Rust supports three [logical binary operators][logical binary operators]: `&` (Logical And), `|` (Logical Or), `^` (Logical Xor).

Rust supports two [boolean operators][lazy boolean operators]: `||` (Or), `&&` (And). They differ from `|` and `&` in that the right-hand operand
is only evaluated when the left-hand operand does not already determine the result of the expression. That is, `||` only evaluates
its right-hand operand when the left-hand operand evaluates to `false`, and `&&` only when it evaluates to `true`.

```rust
true || false // == true; the right-hand side was not evaluated
true && false // == false
```

[negation operators]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#negation-operators
[comparison operators]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#comparison-operators
[logical binary operators]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators
[lazy boolean operators]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators
