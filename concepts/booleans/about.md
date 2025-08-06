# About

Booleans in Rust are represented by the `bool` type, with values either
`true` or `false`.

## Declaration

The Boolean type in Rust is specified using `bool`.

```rust
let t = true;

let f: bool = false; // with explicit type annotation
```

The main way to use Boolean values is through conditionals, such as an if
expression.

### Use as a type
Booleans can be used as a type for a variable, function parameter or return
type.

```rust
let my_variable: bool;

fn my_function(input: bool) -> bool {
    // Do something
}

```

### Operators

Rust supports various operators on `bool` values.

- AND (`&&`) operator computes the logical AND of its operands.
  The result of `x && y` is `true` if both `x` and `y` evaluate to `true`.
  Otherwise, the result is `false`.
- OR (`||`) operator computes the logical OR of its operands.
  The result of `x || y` is `true` if either `x` or `y` evaluates to `true`. Otherwise, the result is `false`.
- NOT (`!`) operator computes logical negation of its operand.
  That is, it produces `true`, if the operand evaluates to `false`, and `false`,
  if the operand evaluates to `true`.

The following example demonstrates the effect of each operator on bool values:

```rust
let x = true;
let y = false;

!x // => false

x && y // => false

x || y // => true

```

The [comparison operators][comparison operators] can be used to evaluate the relationship of non-boolean values into a boolean value.
- `==` (Equal)
- `!=` (Not equal)
- `>` (Greater than)
- `<` (Less than)
- `>=` (Greater than or equal to)
- `<=` (Less than or equal to).

```rust
1 > 0 // true
1 < 0 // false
1 == 0 // false
1 != 0 // true
1 >= 0 // true
1 <= 0 // false
```

Unlike some other languages, `0` is not `false` and non-zero is not `true`.

Rust supports three [logical binary operators][logical binary operators]: `&`
(Logical And), `|` (Logical Or), `^` (Logical Xor).

- `|` (Logical Or)
    | `a` | `b` | `a \| b` |
    |- | - | - |
    | `true` | `true` | `true` |
    | `true` | `false` | `true` |
    | `false` | `true` | `true` |
    | `false` | `false` | `false` |

- `&` (Logical And)
    | `a` | `b` | [`a & b` |
    |- | - | - |
    | `true` | `true` | `true` |
    | `true` | `false` | `false` |
    | `false` | `true` | `false` |
    | `false` | `false` | `false` |

- `^` (Logical Xor)
    | `a` | `b` | `a ^ b` |
    |- | - | - |
    | `true` | `true` | `false` |
    | `true` | `false` | `true` |
    | `false` | `true` | `true` |
    | `false` | `false` | `false` |

The operators `||` (OR), `&&` (AND) differ from `|` and `&`.
The `||` only evaluates its right-hand operand when the left-hand operand
evaluates to `false`, and `&&` only when it evaluates to `true`.
They are also called [lazy boolean operators] for this reason.

```rust
true || false // == true; the right-hand side was not evaluated
true && false // == false
```

[comparison operators]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#comparison-operators
[logical binary operators]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators
[lazy boolean operators]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators
