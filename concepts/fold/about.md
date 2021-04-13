# About

[`fold`][fold] takes in an iterator and optionally applies the results of processing each item to an accumulator, returning the accumulator.

Each iteration of `fold` takes two arguments. The first is an initial value for the accumulator. The second is a closure which itself takes
two arguments: the accumulator and an item. The closure's return value is the subsequent value of the accumulator.
Once the iterator drains, the final value of the accumulator is returned.

`fold` can be used when it is desired to transform a collection into a single value. For example, if the sum of all even-numbers is desired

```rust
pub fn main() {
    let even_sum = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .fold(0, |acc, num| match num % 2 == 0 {
            true => acc + num,
            _ => acc,
        });
    println!("{:?}", even_sum);
}
```

Prints `30`.

An accumulator can be used for other than numeric values. For example

```rust
pub fn i_smell_the_blood(n: u32) -> String {
    match [(3, "Fee"), (5, "Fi"), (4, "Fo"), (7, "Fum")].iter().fold(
        "".to_string(),
        |acc, factor| match n % factor.0 == 0 {
            true => acc + factor.1,
            _ => acc,
        },
    ) {
        output if !output.is_empty() => output,
        _ => "No Englishman!".to_string(),
    }
}

pub fn main() {
    let song = i_smell_the_blood(420);
    println!("{:?}", song);
}
```

Prints `FeeFiFoFum`

[fold]: https://doc.rust-lang.org/beta/std/iter/trait.Iterator.html#method.fold
