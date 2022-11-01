# Answer array

```rust
const ANSWERS: &'static [&'static str] = &[
    "Whatever.",
    "Sure.",
    "Whoa, chill out!",
    "Calm down, I know what I'm doing!",
];

pub fn reply(msg: &str) -> &str {
    let message = msg.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = if message.ends_with('?') { 1 } else { 0 };
    let is_yelling =
        if message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase() {
            2
        } else {
            0
        };

    ANSWERS[is_questioning + is_yelling]
}
```
