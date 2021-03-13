# Instructions

Your company has been hired to generate CSV records from user input.

We need a builder for a CSV record per the standard defined in [RFC 4180](https://tools.ietf.org/html/rfc4180).

While a formal specification exists, your company only needs to implement the following functionality:
- create a new record builder
- add an individual field to the record
- serialize the record with basic escaping

Basic escaping means:

- Fields containing line breaks, double-quotes, and commas should be enclosed in double-quotes.

- If double-quotes are used to enclose fields, then a double-quote appearing inside a field must be escaped by preceding it with another double-quote.


## 1. Create a new builder

You'll need to implement the `new` method to create a new builder.

```rust
let mut builder = CsvRecordBuilder::new();
```

## 2. Add individual fields to the record

You'll need to implement the `add` method to append fields to your record builder.
`add` takes immutable string slices, `&str`, as the argument.

```rust
builder.add("ant");
builder.add("ba\"t");
builder.add("cat");
```


## 3. Serialize the record with basic escaping

You'll need to implement the `build` method.
`build` consumes the builder and returns a CSV record.
```rust
let record = builder.build();
// Note that from now on we cannot use `builder`
assert_eq!(&record, r#"ant,"ba""t",cat"#);
```
