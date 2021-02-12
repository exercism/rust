# Concepts of Parallel letter frequency

This exercise boasts a wide variety of possible solutions in combination with the help of external crates. For keeping the amount of work manageable this document contains a small selection of possibilities. Crossbeam solutions have a similar set of concepts as listed here, and Rayon requires only a subset.

## Summary

- **Primitives**
  - `usize`
  - `char`
  - floating point values
    - Dividing floating points vs dividing integers
- **Immutability/explicit mutability**
- **Strings**
  - `&str` type
- **Unicode**
  - Unicode methods vs ascii methods
- **Slices**
- **Tuples**
- **Destructuring**
- **Functions**
  - Higher order functions
  - Closures
- **Visibility**
- **Data collections**
  - `HashMap`
- **`Option` type**
- **Iterators**
  - Lazy evaluation
  - Consuming/non-consuming
  - As per the second example: `FromIterator` trait
- **For loop**
- **References**
  - Reference counters (`Arc` in this situation)
  - Dereferencing
- **Lifetimes**
  - `'Static`
  - `move`
  - Dropping
- **Crates**
- **Parallelism**
  - Spawning and joining

### Concepts related to parallelism

- **Channels**
- **`Mutex` and `RwLock`**
- **Futures**
- **Reference counting**

\*<sub>Related but here not applicable for reasonable solutions</sub>

## General

- Strings: specifically the `str` is always a factor here
- Unicode: performance/feature concessions of ascii vs unicode methods. Benchmarking is skewed when benchmark uses unicode methods and the solution uses ascii methods.
- Iterators: iterating over input or chunks of it.
- Collections: Iterating over a `HashMap` and using `Entry`. Commonly handles are collected into `Vec`.
- References
- Numbers: `Add` trait specifically
- For loop: can't join threads in an Iterator because of move.
- **Opt** Visibility: keep implementation details non-public
- **Opt** slices: `.chunks()` is a often used method for splitting up input for worker threads. Alternatively subslices are used.
- **Opt** Higher-order functions: the HashMaps can be merged by a higher-order function with an Iterator.
- **Opt** Types: for dividing input we can cast integers to floating point values for applying rounding
- **Opt** Crates

## Std approach with or without channels

- Parallelism - subtopic spawning/joining threads.
- Iterators - subtopic lazily-evaluated: spawning and joining threads with the same Iterator prevents parallelism.
- Lifetimes - subtopics `'static lifetime` and explicit `move` semantics
- Iterators/lifetimes - subtopic consuming Iterators: threads can't be joined using Iterator.
- Option type: unwrapping thread or channel results
- Functions: Specifically higher order functions are required for thread spawning. Closures are an option here.

## Std approach with channels

- Parallelism - subtopic channels.
- Tuples: `mpsc::channel()` returns a Tuple
- Destructuring: tuple can be destructured on assignment
- Lifetimes: `drop()`

## Alternate approach with shared variables

- Reference counting: Sharing aggregate data between threads with a reference counted collection.
- **Honorable mention**: Parallelism - subtopic Mutexes. Not necessary/useful for this exercise, but a common concept nonetheless. Related: `RwLock`.

### Example std with channels

```rust
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel();

    let _handles = input.chunks((input.len() as f64 / worker_count as f64).ceil() as usize)
        .map(|slice| slice.concat())
        .map(|slice| {
            let tx = tx.clone();
            thread::spawn(move || {
                let mut map: HashMap<char, usize> = HashMap::new();
                for chr in slice.chars().filter(|c| c.is_alphabetic()) {
                    if let Some(c) = chr.to_lowercase().next() {
                        (*map.entry(c).or_insert(0)) += 1;
                    };
                }
                tx.send(map).unwrap()
            })
        }).collect::<Vec<_>>();

    drop(tx);

    let mut result: HashMap<char, usize> = HashMap::new();
    for received in rx {
        for (c, count) in received {
            *result.entry(c).or_insert(0) += count;
        }
    }

    result
}
```

### Example crate with Arc

```rust
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;
use dashmap::DashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let map: Arc<DashMap<char, usize>> = Arc::new(DashMap::new());

    let handles = input
        .chunks((input.len() as f64 / worker_count as f64).ceil() as usize)
        .map(|slice| slice.concat())
        .map(|slice| {
            let map = map.clone();
            std::thread::spawn(move || {
                for c in slice.chars().filter(|c|c.is_alphabetic())
                  .flat_map(|c| c.to_lowercase().next()) {
                    *map.entry(c).or_insert(0) += 1;
                }
            })
        }).collect::<Vec<_>>();

    for h in handles {
        h.join().unwrap();
    }

    map.iter().map(|x| (*x.key(), *x.value())).collect()
}
```
