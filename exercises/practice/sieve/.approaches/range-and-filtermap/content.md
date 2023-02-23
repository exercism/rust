# `Range` and `filter-map()`

```rust
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (0..=upper_bound).map(Option::from).collect();
    numbers[1] = None;
    let upper_bound = upper_bound as usize;
    (2..numbers.len())
        .filter_map(|i| {
            let prime = numbers[i].take()? as usize;
            (prime + prime..=upper_bound)
                .step_by(prime)
                .for_each(|j| numbers[j] = None);
            Some(prime as u64)
        })
        .collect()
}
```

This approach starts by TODO
