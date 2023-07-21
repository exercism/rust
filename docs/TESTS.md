# Writing the Code

Write your code in `src/lib.rs`.
The exercises come with a stub file in `src/lib.rs` that will show you the signatures of the code you'll need to write.

For most exercises, it is best to write all your code in the file `src/lib.rs`.
If you would like to split your solution into several files, consult the Rust book's [chapter on modules][chapter-modules].

[chapter-modules]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

## Running Tests

To run the tests, all you need to do is run the following command:

```bash
$ cargo test
```

Only the first test is enabled by default.
After you are ready to pass the next test, remove the ignore flag from the next test (`#[ignore]`).
You can also remove the flag from all the tests at once if you prefer.

Feel free to write as little code as possible to get the tests to pass.
The test failures will guide you to what should be written next.

Because Rust checks all code at compile time, you may find that your tests won't compile until you write the required code.
Even `ignore`d tests are checked at compile time.
You can [comment out][comments] tests that won't compile by starting each line with a `//`.
Then, when you're ready to work on that test, you can un-comment it.
Rust also has a special macro called `todo!()`, which you can use on unfinished code paths to make your program compile.

[comments]: https://doc.rust-lang.org/book/ch03-04-comments.html
