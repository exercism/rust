# Testing on the Command Line

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored. After you get the first test to
pass, open the tests source file which is located in the `tests` directory
and remove the `#[ignore]` flag from the next test and get the tests to pass
again. Each separate test is a function with `#[test]` flag above it.
Continue, until you pass every test.

If you wish to run _only ignored_ tests without editing the tests source file, use:

```bash
$ cargo test -- --ignored
```

If you are using Rust 1.51 or later, you can run _all_ tests with

```bash
$ cargo test -- --include-ignored
```

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

If the specific test is ignored, use:

```bash
$ cargo test some_test -- --ignored
```

To learn more about Rust tests refer to the online [test documentation][rust-tests].

[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
