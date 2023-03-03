# Introduction

There are many ways to solve Secret Handshake.
One approach is to iterate only once, even when the signs are to be reversed.

## General guidance

Something to consider is to keep the number of iterations at a minimum to get the best performance.
However, if that is felt to adversely impact readability, then to use a series of `if` statements and then reverse is also valid.

## Approach: Iterate once

```rust
const SIGNS: [&'static str; 4] = ["wink", "double blink", "close your eyes", "jump"];
const REVERSE_SIGNS: u8 = 16;

pub fn actions(n: u8) -> Vec<&'static str> {
    let (mut action, action_incr, end) = match n & REVERSE_SIGNS {
        0 => (0, 1, 4),
        _ => (3, -1, -1),
    };
    let mut output: Vec<&'static str> = Vec::new();
    
    loop {
        if action == end {
            break;
        }
        if (n & (1 << action)) != 0 {
            output.push(SIGNS[action as usize])
        }
        action += action_incr
    }
    output
}
```

For more information, check the [Iterate once approach][approach-iterate-once].

[approach-iterate-once]: https://exercism.org/tracks/rust/exercises/secret-handshake/approaches/iterate-once
