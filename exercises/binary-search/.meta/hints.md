## Hints

[Slices](https://doc.rust-lang.org/book/second-edition/ch04-03-slices.html) have additionally to
the normal element access via indexing (slice[index]) many useful functions like 
[split_at](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at) or [getting 
subslices](https://doc.rust-lang.org/std/primitive.slice.html#method.get) (slice[start..end]).

You can solve this exercise by just using boring old element access via indexing, but maybe the
other provided functions can make your code cleaner and safer. 

### Hints for Bonus Points

- To get your function working with all kind of elements which can be ordered,
  have a look at the [Ord Trait](https://doc.rust-lang.org/std/cmp/trait.Ord.html).
- To get your function working directly on Vec and Array, you can use the 
  [AsRef Trait](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
  