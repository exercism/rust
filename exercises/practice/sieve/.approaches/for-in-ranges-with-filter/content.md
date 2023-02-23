# `for` in ranges with `filter()`

```rust
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut work: Vec<u64> = (0..=upper_bound).collect();
    work[1] = 0;
    let stop = (upper_bound as f64).sqrt() as usize + 1usize;
    let upper_bound = upper_bound as usize;
    for i in 2..stop {
        if (work[i]) != 0 {
            for idx in (i * i..=upper_bound).step_by(i) {
                work[idx] = 0;
            }
        }
    }
    work.iter().filter(|num| *num != &0u64).copied().collect()
}
```

This approach starts by TODO
