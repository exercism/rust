## General

* A doubly linked does not have a clear ownership hierarchy, which is why it requires either the use
  of unsafe or abstractions for shared ownership like `Rc`. The latter has some overhead that is unnecessary
  for this case.

* Refer to the [Rustonomicon](https://doc.rust-lang.org/nomicon/) for details on how to use `unsafe {}` correctly.

* Several functions require similar behaviour in different directions (towards front or back). Try not to duplicate
  shared code paths.
