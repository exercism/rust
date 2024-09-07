# Instructions

In this exercise you'll be writing code to analyze the production of an assembly line in a car factory.
The assembly line's speed can range from `0` (off) to `10` (maximum).

At its lowest speed (`1`), `221` cars are produced each hour. The production increases linearly with the speed.
So with the speed set to `4`, it should produce `4 * 221 = 884` cars per hour. 
However, higher speeds increase the likelihood that faulty cars are produced, which then have to be discarded.

You have three tasks.

## 1. Calculate the success rate

Implement the `success_rate()` function to calculate the ratio of an item being
created without error for a given speed.
The following table shows how speed influences the success rate:

- `0`: 0% success rate.
- `1` to `4`: 100% success rate.
- `5` to `8`: 90% success rate.
- `9`: 80% success rate.
- `10`: 77% success rate.

```rust
success_rate(10)
// => 0.77
```

Note that the value returned is a `f64`.

## 2. Calculate the production rate per hour

Implement the `production_rate_per_hour()` function to calculate the assembly line's production rate per hour, taking into account its success rate:

```rust
production_rate_per_hour(6)
// => 1193.4
```

Note that the value returned is a `f64`.

## 3. Calculate the number of working items produced per minute

Implement the `working_items_per_minute()` function to calculate how many working cars are produced per minute:

```rust
working_items_per_minute(6)
// => 19
```

Note that the value returned is an `i32`.
