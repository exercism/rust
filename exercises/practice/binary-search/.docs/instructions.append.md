# Instructions append

## Restrictions

Rust provides in its standard library already a
[binary search function](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search).
For this exercise you should not use this function but just other basic tools instead.

## For bonus points

Did you get the tests passing and the code clean? If you want to, there
are some additional things you could try.

- Currently your find function will probably only work for slices of numbers,
  but the Rust type system is flexible enough to create a find function which
  works on all slices which contains elements which can be ordered.
- Additionally this find function can work not only on slices, but at the
  same time also on a Vec or an Array.

To run the bonus tests, remove the `#[ignore]` flag and execute the tests with
the `generic` feature, like this:

```bash
$ cargo test --features generic
```

Then please share your thoughts in a comment on the submission. Did this
experiment make the code better? Worse? Did you learn anything from it?
