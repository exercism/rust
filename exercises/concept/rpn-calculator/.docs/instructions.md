# Instructions

Reverse Polish notation (RPN) is a way of writing mathematical expressions in which operators follow their operands.
For example, instead of writing:

```
2 + 2
```

you would write:

```
2 2 +
```

A major benefit of Reverse Polish notation is that it eliminates the need for parentheses in complex expressions.

```
(4 + 8) / (7 - 5)
```

can be written as

```
4 8 + 7 5 - /
```

In both cases, the expression evaluates to 6.

Your goal is to write a calculator to evaluate a list of inputs ordered by Reverse Polish notation.
You are given the following enum and stubbed function as a starting point.

```rust
#[derive(Debug)]
enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
  
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    unimplemented!(
		"Given the inputs: {:?}, evaluate them as though they were a Reverse Polish notation expression",
		inputs,
	);
}
```

