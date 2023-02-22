# Introduction

There is more than one way to solve Allergies.
One approach can be to create a [`Vec`][vec] of `Allergen` values when the `Allergies` struct is created.
Another approach can be to store the `Allergen` values as a [`u32`][u32] and only output a `Vec` of `Allergen` values when requested.

## General guidance

Something to keep in mind is to leverage [bitwise][bitwise] operations to implement the logic.


## Approach: Create `Vec` on `new()`

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

For more information, check the [create `Vec` on `new()` approach][approach-vec-on-new].

## Approach: Create `Vec` when requested

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

For more information, check the [create `Vec` when requested approach][approach-vec-when-requested].

## Which approach to use?

Since benchmarking is currently outside the scope of this document, which to use is pretty much a matter of personal preference.
To create the `Vec` on `new()` may be considered wasteful if the `allergies()` method is never called.
On the other hand, if the `allergies()` method is called more than once on the same struct, then it may be considered wasteful
to create the `Vec` multiple times.

[vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[u32]: https://doc.rust-lang.org/std/primitive.u32.html
[bitwise]: https://www.tutorialspoint.com/rust/rust_bitwise_operators.htm
[approach-vec-on-new]: https://exercism.org/tracks/rust/exercises/allergies/approaches/vec-on-new
[approach-vec-when-requested]: https://exercism.org/tracks/rust/exercises/allergies/approaches/vec-when-requested
