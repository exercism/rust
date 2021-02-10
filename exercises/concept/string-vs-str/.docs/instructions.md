Your company has been hired to generate CSV documents from user input.

While a formal specification exists, your company only needs to implement the following functionality:
- create a new builder
- add an individual record
- serialize records with basic escaping

## 1. Create a new builder

You'll need to implement the `new` method to create a new builder.

```rust
let mut builder = CSVRecordBuilder::new();
```

## 2. Add individual records

You'll need to implement the `add` method to append records to your builder.
`add` takes immutable string slices, `&str`, as the argument.

```rust
builder.add("ant");
builder.add("ba\"t");
builder.add("cat");
```


## 3. Serialize records with basic escaping

You'll need to implement the `build` method.
`build` consumes the builder and returns a CSV record.
```rust
let list = builder.build();
// Note that from now on we cannot use `builder`
assert_eq!(r#"ant,"ba""t",cat"#, &list);
```
