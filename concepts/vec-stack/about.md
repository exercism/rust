# Vectors as a Stack

A stack is a linear data structure optimized for access from one end. An appropriate metaphor is a pile of sheets of paper on a spindle: new pieces of paper are always added to the same end, and the sheets are removed in the opposite order that they were added. The sheets added first are removed last, and those added last are removed first.

Rust's `Vec` data structure is appropriate for use as a stack. The [`push` method](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push) adds items to the tail of the vector. The [`pop` method](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop) removes the final item from the vector. Both `push` and `pop` require only amortized `O(1)` time.

## Example

In this example, we use a vector as a stack for a simple [RPN](https://en.wikipedia.org/wiki/Reverse_Polish_notation) calculator. Full source can be found on the [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b40514e282cb753d0c3272778f92d736).

```rust
#[derive(Default)]
pub struct RpnCalculator {
    stack: Vec<Value>,
}

impl RpnCalculator {
    pub fn evaluate(&mut self, argument: Argument) -> Result<(), Error> {
        match argument {
            Argument::Value(value) => self.stack.push(value),
            Argument::Operator(operator) => {
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let output = operator.execute()(a, b);
                self.stack.push(output);
            }
        }
        Ok(())
    }

    pub fn conclude(self) -> Result<Value, Error> {
        if self.stack.len() == 1 {
            Ok(self.stack[0])
        } else {
            Err(Error::IncompleteCalculation)
        }
    }
}
```
