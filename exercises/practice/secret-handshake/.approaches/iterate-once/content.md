# Iterate once

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

This approach starts by defining a fixed-size [array][array] to hold the signal values in normal order.

The `[&'static str; 4]` is used to give the type and length of the array.
To be a [`const`][const], the size of the array must be known at compile time, so setting the type and length must be done explicitly,
so the size in bytes of the fixed array can be deduced by the compiler from that and not by inspecting the element types and counting
the elements itself.

The value of `16` is defined as a `const` with a meaningful name so it won't be used as a [magic number][magic-number].

The `actions` function uses multiple assignment with a `match` expression to define the variables that control iterating through the signals array,
setting their values to iterate in either the normal or reverse order.

The [bitwise AND operator][bitand] is used to check if the input number contains the signal for reversing the order of the other signals.

For example, if the number passed in is `19`, which is `10011` in binary, then it is ANDed with `16`, which is `10000` in binary.
The `1` in `10000` is also at the same position in `10011`, so the two values ANDed will not be `0`.
- `10011` AND
- `10000` =
- `10000`

If the number passed in is `3`, which is `00011` in binary, then it is ANDed with `16`, which is `10000` in binary.
The `1` in `10000` is not at the same position in `00011`, so the two values ANDed will be `0`.
- `00011` AND
- `10000` =
- `00000`

If the number passed in does not contain the signal for reverse, then the iteration variables are set to iterate through the array of signals
in their normal order, otherwise they are set to iterate through the arrray backwards..

The output `vector` is defined, and then the [`loop`][loop] begins.

Normal iteration will start at index `0`.
Reverse iteration will start at index `3`.

Normal iteration will terminate when the index equals `4`.
Reverse iteration will terminate when the index equals `-1`.

Normal iteration will increase the index by `1` for each iteration.
Reverse iteration will decrease the index by `1` for each iteration.

For each iteration of the `loop`, the AND operator is used to check if the number passed in contains `1` [shifted left][shl] (`<<`) for the number of positions
as the value being iterated.

```rust
if (n & (1 << action)) != 0 {
    output.push(SIGNS[action as usize])
}
```

For example, if the number being iterated is `0`, then `1` is shifted left `0` times (so not shifted at all), and the number passed in is ANDed with `00001`.
If the number passed in is `3`, which is `00011` in binary, then it is ANDed with `00001`.
`00011` ANDed with `00001` is not equal to `0`, so the signal at the index of the array of signals is added to the output `vector`.
The index used is the number being iterated, which is `0`, so the element at index `0` (`"wink"`) would be added to the output `vector`
using the [push][push] function.

If the number being iterated is `1`, then `1` is shifted left `1` time, and the number passed in is ANDed with `00010`.
If the number passed in is `3`, which is `00011` in binary, then it is ANDed with `00010`.
`00011` ANDed with `00010` is not equal to `0`, so the signal at the index of the array of signals is added to the output `vector`.
The index used is the number being iterated, which is `1`, so the element at index `1` (`"double blink"`) would be added to the output `vector`.

If the number passed in ANDed with the number being iterated is equal to `0`, then the signal in the array for that index is not added to the output `vector`.

After iterating through the array of signals is done, the output `vector` is returned from the function.

[array]: https://doc.rust-lang.org/std/primitive.array.html
[const]: https://doc.rust-lang.org/std/keyword.const.html
[magic-number]: https://en.wikipedia.org/wiki/Magic_number_(programming)
[bitand]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
[shl]: https://doc.rust-lang.org/std/ops/trait.Shl.html
[loop]: https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
[push]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push
