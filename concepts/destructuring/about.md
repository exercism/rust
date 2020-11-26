# Destructuring

Destructuring is the process of breaking down items into their component parts, binding each to smaller variables.

## Destructuring Tuples

Destructuring tuples is simple: just assign new variable names to each field of the tuple:

```rust
let (first, second) = (1, 2);
```

## Destructuring Tuple Structs

Destructuring tuple structs is identical to destructuring tuples, but with the struct name added:

```rust
struct TupleStruct(&'static str, i32);
let my_tuple_struct = TupleStruct("foo", 123);
let TupleStruct(foo, num) = my_tuple_struct;
```

## Destructuring Structs

Destructuring structs is similar to destructuring tuple structs. Just use the field names:

```rust
struct ArabianNights {
    name: String,
    stories: usize,
}
let teller = ArabianNights { name: "Scheherazade".into(), stories: 1001 };
{
    let ArabianNights { name, stories } = teller;
    println!("{} told {} stories", name, stories);
}
```

## Partial Destructuring

The `..` operator can be used to ignore some fields when destructuring:

```rust
let ArabianNights { name, .. } = teller;
println!("{} survived by her wits", name);
```

## Conditional Destructuring

Destructuring structs and tuples is infallible. However, enums can also be destructured, using the `if let`, `while let`, and `match` language constructs.

```rust
if let Some(foo) = foo {
    println!("{:?}", foo);
}
```
