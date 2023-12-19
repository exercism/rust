# Debug

When a test fails, a message is displayed describing what went wrong and for which input.
You can also use the fact that anything printed to "standard out" will be shown too.
You can write to standard out using `println!`:

```rust
println!("Debug message");
```

Use braces `{}` to print the value of a variable or expression:

```rust
let secret_number = 4321;
println!("My banking password is {}, which is better than {}.", secret_number, 1200 + 34);
```

Note that not all types can be printed this way, only the ones that can be presented to users unambiguously.
For example, vectors (lists) cannot be printed like this, because it's not clear how a vector should be displayed to users!
For these other types, you can tell the `println!` macro to use _debug-formatting_ with: `{:?}`.
This representation is only intended to be seen by developers.
An example:

```rust
let words = vec!["hello", "world"];
println!("words that are often combined: {:?}", words);
```

There is even the handy `dbg!` macro, which can be used to debug large expressions inline, without having to pick it apart into separate variable declarations.

Assume we have the following expression:

```rust
"hello world".split(' ').collect::<Vec<_>>().join("-")
```

Now we would like to see what the call to `collect` returns.
We could do the following:

```rust
let my_temporary_debug_variable = "hello world".split(' ').collect::<Vec<_>>();
println!("{:?}", my_temporary_debug_variable);
my_temporary_debug_variable.join("-")
```

But that's tedious!
We don't wan't to rewrite our nice expressions just to be able to debug them.
Please welcome to the stage: `dbg!`

```rust
dbg!(
    "hello world".split(' ').collect::<Vec<_>>()
).join("-")
```

The whitespace is only for clarity.
You can wrap any expression in `dbg!()`, which will print that value (with debug-formatting).
Afterwards you can keep using the value in a larger expression, as if the `dbg!` wasn't even there.

Happy debugging!
