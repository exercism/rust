# Concepts of Decimal

Decimal explores a lossless alternative for float points values. This highlights the fundamental difference between floating-point arithmetic and the way humans interpret decimal values. A good topic to get right for new programmers.

Some other useful concepts are handled too, like the Traits implemented for primitives.

Variation in approach is very minimal, as the solution doesn't have to be algorithmic-ally complex.

## Extracted concepts

This includes all concepts shown in the examples, and some related concepts not applied here.

**In bold: concept level items**

- **Signed-ness**
- **literals**
  - **integers**
  - **floats**
    - **Floating point characteristics**
  - **chars**
- **`u128` and `i128`** types
- **`Option`** type
  - `unwrap`
  - `unwrap_or`
- **`Result`** type
- **`Self`** type
- **`usize`/`isize`** type
- **`str` and `String`** type
  - Difference + `as_str()`
  - `trim` and variants
- **Iterator**
  - `filter`
  - `map`
  - `collect`
  - `rev`
  - `position`
- Related, not applied here: **Generics**
- **Structs**
- **Traits**
  - **Ordering**
  - **Equality**
  - **Debug**
  - **Default**
  - `From`, `TryFrom` and `FromStr`
- **Number traits**
  - Add
  - Sub
  - Mul
  - Div
  - `*Assign` variants of above
  - Pow
    - `powf32`/`powif32` and `powf64`/`powif64`
  - **saturating / overflowing / wrapping / checked variants of above**
- **`type` keyword**
- **Deriving**
- **Functions**
  - **Methods**
- **Method syntax** (`Type::` vs `self.`)
- **Visibility**
- **References**
  - Dereferencing
- **`impl` blocks**
- `new()` - common practice
- **conditionals**
  - **if/else if/else**
  - `||` and `&&`
    - Don't forget **order of operations**!
- **`while` loop**
- **`match`**
- **`if let`/`while let`**
- **variables**
  - **`let`/`static`/`const`**
  - Casting
  - `let x = { //scope }` assign syntax
- **Immutability/explicit mutability**
- **Ownership**
  - `to_owned`
- **Tuples**
- **Turbofish operator**
- **`?` operator**
- **explicit early `return`**
- **`use` syntax**
  - using multiple modules on single line
- **Opt:** **Crates**

## Example exercise 1

Taken from ["lewisclement"](https://exercism.io/my/solutions/c87d24f45c284c83b0da780d132f1c4b)

```rust
use std::str::FromStr;
use std::ops::{Add, Sub, Mul};
use std::cmp::{max, Ordering};
use num_bigint::BigInt;
use num_traits::pow::Pow;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(PartialEq, Eq, Ord, Debug)]
pub struct Decimal {
    value: BigInt,
    point: usize
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (v1, v2) = self.normalize(&other);

        Some(v1.cmp(&v2))
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (v1, v2) = self.normalize(&other);

        Decimal::new(v1 + v2, max(self.point, other.point))
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (v1, v2) = self.normalize(&other);

        Decimal::new(v1 - v2, max(self.point, other.point))
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let (v1, v2) = self.normalize(&other);

        Decimal::new(v1 * v2, (self.point + other.point) * 2)
    }
}

impl Decimal {
    pub fn new(value: BigInt, point: usize) -> Self {
        let mut point = point.clone();
        let mut value = value.clone();

        while &value % 10 == BigInt::from(0) && point > 0 {
            value /= 10;
            point -= 1;
        }

        Self {
            value,
            point
        }
    }

    pub fn try_from(input: &str) -> Option<Decimal> {
        let point = input.chars().rev().position(|c| c == '.');
        let value = BigInt::from_str(
            input.chars().filter(|c| *c != '.').collect::<String>().as_str()
        );

        if let Ok(value) = value {
            Some(Decimal::new(value, point.unwrap_or(0)))
        } else {
            None
        }
    }

    fn normalize(&self, other: &Self) -> (BigInt, BigInt) {
        (
            (self.value.clone() * BigInt::from(10).pow(other.point.saturating_sub(self.point) as u32)),
            (other.value.clone() * BigInt::from(10).pow(self.point.saturating_sub(other.point) as u32))
        )
    }
}
```

## Example exercise 2

Taken from ["yawpitch"](https://exercism.io/tracks/rust/exercises/decimal/solutions/16d3747cccca4ee9aeba5a0e8f0ed429)

```rust
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Decimal {
    significand: BigInt,
    exponent: usize,
}

impl Decimal {
    pub fn new(significand: BigInt, exponent: usize) -> Self {
        let mut this = Self {
            significand,
            exponent,
        };
        this.normalize();
        this
    }

    pub fn try_from(input: &str) -> Option<Self> {
        // trim any whitespace padding
        let mut input = input.trim();
        if input.is_empty() {
            return Some(Self::default());
        }

        let sign = if input.starts_with('-') {
            Sign::Minus
        } else {
            Sign::Plus
        };

        // trim sign and leading 0s
        input = input.trim_start_matches(|c| c == '-' || c == '+' || c == '0');

        let mut parts = input.split('.');
        let integer = parts.next().unwrap_or("");
        let fraction = parts.next().unwrap_or("0");
        let parsed = BigUint::parse_bytes((integer.to_owned() + fraction).as_bytes(), 10)?;
        let significand = BigInt::from_biguint(sign, parsed);
        let exponent = fraction.len();
        Some(Self::new(significand, exponent))
    }

    fn normalize(&mut self) {
        let ten = 10.to_bigint().unwrap();
        let zero = 0.to_bigint().unwrap();
        while self.significand.clone() % ten.clone() == zero && self.exponent > 0 {
            self.significand /= ten.clone();
            self.exponent -= 1;
        }
    }

    fn promote(&self, exponent: usize) -> BigInt {
        let ten = 10.to_bigint().unwrap();
        let mut result = self.significand.clone();

        match self.exponent.cmp(&exponent) {
            Ordering::Equal => result,
            Ordering::Less => {
                for _ in 0..(exponent - self.exponent) {
                    result *= ten.clone();
                }
                result
            }
            Ordering::Greater => {
                for _ in 0..(self.exponent - exponent) {
                    result /= ten.clone();
                }
                result
            }
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        match self.exponent.cmp(&other.exponent) {
            Ordering::Equal => Some(self.significand.cmp(&other.significand)),
            Ordering::Less => Some(self.promote(other.exponent).cmp(&other.significand)),
            Ordering::Greater => Some(self.significand.cmp(&other.promote(self.exponent))),
        }
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Self {
        match self.exponent.cmp(&other.exponent) {
            Ordering::Equal => Self::new(self.significand + other.significand, self.exponent),
            Ordering::Less => Self::new(
                self.promote(other.exponent) + other.significand,
                other.exponent,
            ),
            Ordering::Greater => Self::new(
                self.significand + other.promote(self.exponent),
                self.exponent,
            ),
        }
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Self {
        self.add(Self {
            significand: -other.significand,
            exponent: other.exponent,
        })
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Self {
        Self::new(
            self.significand * other.significand,
            self.exponent + other.exponent,
        )
    }
}
```
