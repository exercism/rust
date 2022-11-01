# `match` on a `tuple`

```rust
pub fn reply(msg: &str) -> &str {
    let message = msg.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = message.ends_with('?');
    let is_yelling =
        message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase();

    match (is_yelling, is_questioning) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever.",
    }
}
```
