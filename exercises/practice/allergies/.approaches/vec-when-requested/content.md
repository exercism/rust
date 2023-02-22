# Create `Vec` when requested

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

use self::Allergen::*;
const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];
pub struct Allergies {
    allergens: u32,
}

impl Allergies {
    pub fn new(n: u32) -> Allergies {
        Allergies { allergens: n }
    }
    
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens & *allergen as u32 != 0
    }
    
    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .cloned()
            .collect()
    }
}
```

This approach starts by defining the `Allergen` enum.
Since the values of the `Allergen` enum are primitive numeric types, the [`Copy`][copy] trait is derived when defining the enum.
[`Clone`][clone] is a supertrait of `Copy`, so everything which is `Copy` must also implement `Clone`, so `Clone` is derived as well.

The `use self::Allergen::*;` line is so that the `Allergen` values can be referred to directly in the file,
instead of needing to prefix every value with `Allergen`, e.g. 'Allergen::Eggs`.

A fixed-size [array][array] is defined to hold the `Allergen` values as a way to iterate the enum values without requiring an external crate.

The `[Allergen; 8]` is used to give the type and length of the array.
To be a [`const`][const], the size of the array must be known at compile time, so setting the type and length must be done explicitly,
so the size in bytes of the fixed array can be deduced by the compiler from that and not by inspecting the element types and counting
the elements itself.

The `Allergies` struct is defined with its `allergens` field given the type of [`u32`][u32].

Next, the methods for the `Allergies` struct are implemented.

The `new()` method sets its `allergens` field to the `u232` value passed in.

The `is_allergic_to()` method uses the [bitwise AND operator][bitand] (`&`) to compare the `Allergen` passed in with the `allergens` `u32` field.
The dereferenced `Allergen` passed in is [cast][cast] to a `u32` for the purpose of comparison with the `allergens` `u32` value.
The method returns if the comparison is not `0`. 
If the comparison is not `0`, then the `allergens` field contains the value of the `Allergen`, and `true` is returned.

For example, if the `allergens` field is decimal `3`, it is binary `11`.
If the passed-in `Allergen` is `Peanuts`, which is the value `2` decimal, `10` binary, then `10` ANDed with `11` is `10`, not `0`,
and so the method would return `true`.

If the passed-in `Allergen` is `Shellfish`, which is the value `4` decimal, `100` binary, then `100` ANDed with `011` is `0`,
and so the method would return `false`.

The `allergies()` method iterates through the `ALLERGENS` array, passing each element of the array to the [`filter()`][filter] method.
The [closure][closure] (also known as a lambda), returns the result of passing the `Allergen` to the `is_allergic_to()` method.
The elements that survive the test of being present in the `allergens` field are passed to the [`cloned()`][cloned] method. which
converts the iterator of references to `Allergen` values to an iterator of copies of the `Allergen` values.
Since the `Allergen` values derive from `Copy`, they are inexpensively copied.
The cloned elements are chained to the [`collect()`][collect] method, which uses [type inference][type-inference] to collect
the elements into a `Vec<Allergen>` as defined in the method signature for the return value.

[copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[array]: https://doc.rust-lang.org/std/primitive.array.html
[const]: https://doc.rust-lang.org/std/keyword.const.html
[u32]: https://doc.rust-lang.org/std/primitive.u32.html
[any]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
[clone-method]: https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone
[for-and-range]: https://doc.rust-lang.org/rust-by-example/flow_control/for.html
[bitand]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
[shl]: https://doc.rust-lang.org/std/ops/trait.Shl.html
[cast]: https://doc.rust-lang.org/1.8.0/book/casting-between-types.html
[filter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[closure]: https://doc.rust-lang.org/rust-by-example/fn/closures.html
[cloned]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned
[collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
[type-inference]: https://doc.rust-lang.org/rust-by-example/types/inference.html
