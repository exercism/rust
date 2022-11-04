# Bit field using a `for` loop

```rust
const A_LCASE: u8 = 97;
const Z_LCASE: u8 = 122;
const A_UCASE: u8 = 65;
const Z_UCASE: u8 = 90;

pub fn check(candidate: &str) -> bool {
    let mut letter_flags = 0;

    for letter in candidate.bytes() {
        if letter >= A_LCASE && letter <= Z_LCASE {
            if letter_flags & (1 << (letter - A_LCASE)) != 0 {
                return false;
            } else {
                letter_flags |= 1 << (letter - A_LCASE);
            }
        } else if letter >= A_UCASE && letter <= Z_UCASE {
            if letter_flags & (1 << (letter - A_UCASE)) != 0 {
                return false;
            } else {
                letter_flags |= 1 << (letter - A_UCASE);
            }
        }
    }
    return true;
}
```
