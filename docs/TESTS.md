## Writing the Code

Write your code in `src/lib.rs`

You'll be writing a *library crate*.  Code for a crate is stored within the `src/` subdirectory.  *Library crates* must contain at least a file named `lib.rs`.  For more details, check out the rustlang book [chapter on crates and modules](http://doc.rust-lang.org/stable/book/crates-and-modules.html)

If you were starting from scratch you could use `cargo new` to create the filestructure: e.g. `cargo new $projectname`.
exercism has already created the projectname directory so you'll need to create `src/lib.rs` manually:

```bash
mkdir src
touch src/lib.rs
```

### Running Tests

To run the tests, all you need to do is run the following command:

```bash
$ cargo test
```

Only the first test is enabled by default.  After you are ready to pass the next test, remove the ignore flag from the next test (`#[ignore]`).  You can also remove the flag from all the tests at once if you prefer.

You should try to write as little code possible to get the tests to pass.  Let the test failures guide you to what should be written next.

