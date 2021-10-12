# Match Basics

The `match` operator compares a value against a series of patterns and then executes code based on which pattern matches.

## Matching Literals

At its most fundamental, `match` compares a value to several potential matches and chooses the first arm which matches. Only one arm is ever chosen, and control never "flows through" to another arm. `match` is an expression which can evaluate to a value.

```rust
enum TrollNumber {
    One,
    Two,
    Many,
    Lots,
}

fn estimated_value(troll_number: TrollNumber) -> u32 {
    match troll_number {
        TrollNumber::One => 1,
        TrollNumber::Two => 2,
        TrollNumber::Many => 10,
        TrollNumber::Lots => 100,
    }
}
```

## Exhaustivity

Matching is always exhaustive: Rust will not compile a program if it is possible for a value to slip through the cracks. To aid in this, `_` is a wildcard which matches anything.

```rust
fn ordinal(n: u32) -> String {
    let ordinal = match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{}{}", n, ordinal)
}
```

## Matching as Control Flow

While `match` can be used as an expression to produce a value, it can also be used as a statement purely for the purposes of control flow.

```rust
// pretend that we've already filtered the key events relevant to the space bar (we don't want a keylogger)
fn space_timer(&mut self, event: Event) {
    match event {
        Event::KeyDown => self.timer = Some(Instant::now()),
        Event::KeyUp => {
            let duration = Instant::now() - self.timer.take().expect("can't have KeyUp without KeyDown");
            self.durations.push(duration);
        }
        // we don't care about any other events; the empty scope is a no-op
        _ => {}
    }
}
```

## Matching Ranges

Match arms don't have to be specified as single values; they can also be ranges.

```rust
fn ascii_type(ch: u8) -> &'static str {
    match ch {
        0..=31 => "non-printable control code",
        32..=126 => "printable character",
        127 => "DEL",
        _ => "extended ascii; region-specific",
    }
}
```

## Or Patterns

A match arm can match one of several possible options by using the `|` token as a separator.

```rust
fn should_retry(error: std::io::Error) -> bool {
    use std::io::ErrorKind;

    match error.kind() {
        ErrorKind::TimedOut | ErrorKind::Interrupted => true,
        _ => false,
    }
}
```

## Structural Matching and Binding

Matching can produce a binding valid within a particular match arm. This is particularly useful for unpacking enums which carry data in their variants.

```rust
/// Consume a result and proceed, logging any errors.
trait OrLog {
    fn or_log(self);
}

impl<E: std::fmt::Display> OrLog for Result<(), E> {
    fn or_log(self) {
        match self {
            // discard Ok results
            Ok(()) => {},
            // log errors
            Err(err) => println!("ERROR: {}", err),
        }
    }
}
```

More on this topic in the `match-advanced` concept.
