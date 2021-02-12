# Abstraction over Networks and Files

Network and file operations are implemented in terms of the [`io::Read`][read] and [`io::Write`][write] traits. It will therefore be necessary to implement those traits for your types.

[read]: https://doc.rust-lang.org/std/io/trait.Read.html
[write]: https://doc.rust-lang.org/std/io/trait.Write.html
