Your company has been hired to generate CSV documents from user input.

While a formal specification exists, your company only needs to implement the following functionality:
- build a single record with basic escaping

## 1. Build a single record with basic escaping

The builder needs to concatenate immutable string slices (`&str`) and when `build` is called.
Then the builder will be consumed and will return the created CSV record (one line of a CSV file).

```rust
let mut builder = CSVRecordBuilder::new();

builder.add("ant");
builder.add("ba\"t");
builder.add("cat");

let list = builder.build();
// Note that from now on we cannot use `builder`
assert_eq!(r#"ant,"ba""t",cat"#, &list);
```
