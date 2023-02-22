# Create `Vec` on `new()`

```rust
pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            allergens: Self::calculate_allergens(score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.iter().any(|allergy| allergy == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }

    fn calculate_allergens(score: u32) -> Vec<Allergen> {
        let mut allergens = Vec::<Allergen>::new();

        for flag in 0..=7 {
            if score & 1 << flag != 0 {
                allergens.push(ALLERGENS[flag as usize]);
            }
        }
        allergens
    }
}
```

This approach starts by defining the `Allergies` struct with an `allergens` field of `Vec<Allergens>` type.

Since the values of the `Allergen` enum are primitive numeric types, the [`Copy`][copy] trait is derived when defining the enum.
[`Clone`][clone] is a supertrait of `Copy`, so everything which is `Copy` must also implement `Clone`, so `Clone` is derived as well.

The `use self::Allergen::*;` line is so that the `Allergen` values can be referred to directly in the file,
instead of needing to prefix every value with `Allergen`, e.g. 'Allergen::Eggs`.

A fixed-size [array][array] is defined to hold the `Allergen` values in order, as enum values are not ordered to be acccessible by index.

The `[Allergen; 8]` is used to give the type and length of the array.
To be a [`const`][const], the size of the array must be known at compile time, so setting the type and length must be done explicitly,
so the size in bytes of the fixed array can be deduced by the compiler from that and not by inspecting the element types and counting
the elements itself.

Next, the methods for the `Allergies` struct are implemented.

The `new()` method uses the `calculate_allergens()` method to set the `allergens` field for a new `Allergies` struct.

The `is_allergic_to()` method uses the [`any()`][any] method to see if the passed-in `Allergen` is contained in the `allergens`
field, which is a `Vec` of `Allergen` values.

The `allergies()` method uses the [`clone()`][clone-method] method to return a copy of the `allergens` `Vec`.
Since the `Allergen` values derive from `Copy`, they are inexpensively copied.

The `calculate_allergens()` method uses [`for` and `range`][for-and-range] to iterate through the indexes of the `ALLERGENS` array.
The [bitwise AND operator][bitand] (`&`) is used to compare the score with `1` [shifted left][shl] (`<<`) for the value of the `flag`.

So, if `flag` is `0`, then `1` is shifted left `0` places, and the score is compared with bitwise `1`, which is also decimal `1`.
If the comparison is not `0`, then the score contains the `Allergen`, and the `ALLERGEN` at the index of the `flag` value (`Eggs`) is pushed
to the output `Vec`.

if `flag` is `7`, then `1` is shifted left `7` places, and the score is compared with bitwise `10000000`, which is decimal `128`.
If the comparison is not `0`, then the score contains the `Allergen`, and the `ALLERGEN` at the index of the `flag` value (`Cats`) is pushed
to the output `Vec`.

[copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[array]: https://doc.rust-lang.org/std/primitive.array.html
[const]: https://doc.rust-lang.org/std/keyword.const.html
[any]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
[clone-method]: https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone
[for-and-range]: https://doc.rust-lang.org/rust-by-example/flow_control/for.html
[bitand]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
[shl]: https://doc.rust-lang.org/std/ops/trait.Shl.html
