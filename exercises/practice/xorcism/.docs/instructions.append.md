# Instructions append

## Lifetime of `munge` return value

Due to the usage of the `impl Trait` feature, lifetime management may be a bit
tricky when implementing the `munge` method. You may find it easier to write
your own `struct` with an `Iterator` implementation and return that concrete
type, at least to get started. Ultimately, it's a good idea to try and implement
the solution using `Iterator` combinators directly.
