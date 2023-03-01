# Looping

```rust
use std::cmp::Ordering;

pub fn find<U: AsRef<[T]>, T: Ord>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut left = 0usize;
    let mut right = array.len();
    let mut mid: usize;

    while left < right {
        mid = (left + right) / 2;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Greater => {
                right = mid;
            }
        }
    }
    None
}
```

This approach starts by using the [`Ordering`][ordering-enum] enum.

The `find()` function has a signature to support the optional generic tests.
To support slices, arrays and Vecs, which can be of varying lengths and sizes at runtime,
the compiler needs to be given informaton it can know at compile time.
A reference to any of those containers will always be of the same size (essentially the size of a pointer),
so [`AsRef`][asref] is used to constrain the generic type to be anything that is a reference or can be coerced to a reference.

The `<[T]>` is used to constrain the reference type to an indexable type `T`.
The `T` is constrained to be anything which implements the [`Ord`][ord] trait, which essentially means the values must be able to be ordered.

So, the `key` is of type `T` (orderable), and the `array` is of type `U` (a reference to an indexable container of orderable values
of the same type as the `key`.)

Although `array` is defined as generic type `U`, which is constrained to be of type `AsRef`, 
the [`as_ref()`][asref] method is used to get the reference to the actual type.
Without it, the compiler would complain that "no method named `len` found for type parameter `U` in the current scope" and
"cannot index into a value of type `U`".

Next. some variables of type [`usize`][usize] are defined to keep track of where we are in the container.

The loop runs while `left` is less than `right`.
If the `array` is empty, then `left` and `right` will both be `0`, and the loop won't run, and the function will return [`None`][none].

Inside the loop, the midpoint between `left` and `right` is used to get the element of the `array` at the index of the midpoint value.
The [`cmp()`][cmp] method of the `Ord` trait is used to compare that element value with the key value.
Since the element is a reference, the `key` must also be referenced.

The [`match`][match] arms each use a value from the `Ordering` enum.
- If the midpoint element value equals the `key`, then the midpoint is returned from the function wrapped in a [`Some`][some].
- If the midpoint element value is less than the `key`, then the `left` value is adjusted to be one to the right of the midpoint.
- If the midpoint element value is greater than the `key`, then the `right` value is adjusted to be the midpoint.

While the element value is not equal to the `key`, the number of elements being searched keeps getting halved until
either the `key` is found, or, if it is not in the `array`, the bounds meet and the loop no longer runs.

[ordering-enum]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html
[asref]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
[ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[asref]: https://doc.rust-lang.org/std/convert/trait.AsRef.html#tymethod.as_ref
[usize]: https://doc.rust-lang.org/std/primitive.usize.html
[match]: https://doc.rust-lang.org/rust-by-example/flow_control/match.html
[cmp]: https://doc.rust-lang.org/std/cmp/trait.Ord.html#tymethod.cmp
[none]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
[some]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
