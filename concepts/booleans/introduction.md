# Introduction

Booleans in Rust are represented by the `bool` type, which values can be either `true` or `false`.

Rust supports six comparison operators: `==` (Equal), `!=` (Not equal), `>` (Greater than), `<` (Less than), `>=`
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

Rust supports two boolean operators: `||` (Or), `&&` (And). They are only evaluated when the left-hand operand does not already
determine the result of the expression. That is, `||` only evaluates its right-hand operand when the left-hand operand evaluates
to `false`, and `&&` only when it evaluates to `true`.
