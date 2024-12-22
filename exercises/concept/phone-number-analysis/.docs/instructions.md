# Instructions

This exercise has you analyze phone numbers.

You are asked to implement 2 features.

Phone numbers passed to the routines are guaranteed to be in the form
NNN-NNN-NNNN e.g. 212-515-9876.

## 1. Analyze a phone number

Your analysis should return 3 pieces of data

1. An indication of whether the number has a New York dialing code ie. 212 as the first 3 digits
2. An indication of whether the number is fake having 555 as a prefix code in positions 5 to 7 (numbering from 1)
3. The last 4 digits of the number.

Implement the function `analyze()` to produce the phone number info.

```rust
analyze("631-555-1234");
// => (false, true, "1234")
```

## 2. Detect if a phone number has a fake prefix code (555)

Implement the function `is_fake()` to detect whether the phone number is fake using the phone number info produced in task 1.

```rust
is_fake(analyze("631-555-1234"));
// => true
```
